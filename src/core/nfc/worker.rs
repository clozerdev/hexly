// El hilo del worker actúa como productor (Sender) e inyecto eventos en la cola. La UI actúa como
// consumidor (Receiver), procesando los eventos de forma segura dentro del hilo de GTK.

use async_channel::{Receiver, Sender};

use crate::core::nfc::NfcWorkerState;
use crate::core::nfc::messages::{NfcCommand, NfcEvent};

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
    while let Ok(command) = cmd_receiver.recv_blocking() {
        match command {
            NfcCommand::Start => handle_nfc_start(&event_sender),
            NfcCommand::Shutdown => break,
        }
    }

    let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Stopped));
}

fn handle_nfc_start(event_sender: &Sender<NfcEvent>) {
    let _ = event_sender.send_blocking(NfcEvent::StateChanged(NfcWorkerState::Running));
}
