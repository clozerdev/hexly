// El hilo del worker actúa como productor (Sender) e inyecto eventos en la cola. La UI actúa como
// consumidor (Receiver), procesando los eventos de forma segura dentro del hilo de GTK.

use async_channel::{Receiver, Sender};

use crate::core::nfc::NfcWorkerState;
use crate::core::nfc::messages::{NfcCommand, NfcEvent};
use crate::core::nfc::pcsc::PcscService;
use crate::core::state::{Reader, ReaderId};

pub struct NfcWorker {
    cmd_sender: Sender<NfcCommand>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl NfcWorker {
    pub fn new(event_sender: Sender<NfcEvent>) -> Self {
        let (cmd_sender, cmd_receiver) = async_channel::unbounded::<NfcCommand>();

        let handle = std::thread::spawn(move || {
            worker_loop(cmd_receiver, event_sender);
        });

        Self {
            cmd_sender,
            handle: Some(handle),
        }
    }

    pub fn send_command(
        &self,
        command: NfcCommand,
    ) -> Result<(), async_channel::SendError<NfcCommand>> {
        self.cmd_sender.send_blocking(command)
    }

    pub fn shutdown(&mut self) {
        let _ = self.send_command(NfcCommand::Shutdown);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for NfcWorker {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn worker_loop(cmd_receiver: Receiver<NfcCommand>, event_sender: Sender<NfcEvent>) {
    let mut service: Option<PcscService> = None;

    while let Ok(command) = cmd_receiver.recv_blocking() {
        match command {
            NfcCommand::Start => handle_nfc_start(&event_sender, &mut service),
            NfcCommand::Shutdown => break,
            NfcCommand::ReadCard { reader_id } => {
                handle_card_reading(&event_sender, service.as_ref(), reader_id)
            }
        }
    }

    let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Stopped));
}

fn handle_nfc_start(event_sender: &Sender<NfcEvent>, service: &mut Option<PcscService>) {
    if service.is_none() {
        match PcscService::establish() {
            Ok(s) => *service = Some(s),
            Err(error) => {
                let _ = event_sender.send_blocking(NfcEvent::Log(error.to_string()));
                let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Stopped));
                return;
            }
        }
    }

    let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Running));

    let Some(service) = service.as_ref() else {
        return;
    };

    let readers = match service.list_readers() {
        Ok(readers) => readers
            .into_iter()
            .map(|reader| Reader {
                id: ReaderId(reader.name.clone()),
                label: reader.name,
            })
            .collect::<Vec<_>>(),
        Err(error) => {
            let _ = event_sender.send_blocking(NfcEvent::Log(error.to_string()));
            let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Stopped));
            return;
        }
    };

    let _ = event_sender.send_blocking(NfcEvent::ReadersChanged(readers));
}

fn handle_card_reading(
    event_sender: &Sender<NfcEvent>,
    service: Option<&PcscService>,
    reader_id: ReaderId,
) {
    let _ = event_sender.send_blocking(NfcEvent::Log("Reading requested".to_string()));

    let Some(_service) = service else {
        let _ = event_sender.send_blocking(NfcEvent::Log("PC/SC not initialized".to_string()));
        return;
    };

    let _ = event_sender.send_blocking(NfcEvent::ReadStarted { reader_id });
}
