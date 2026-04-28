use std::{thread, time::Duration};

use async_channel::{Receiver, Sender, TryRecvError};

use crate::core::pcsc::{
    service::PcscService,
    types::{PcscCommand, PcscEvent},
};

#[derive(Debug)]
pub struct PcscWorker {
    command_sender: Sender<PcscCommand>,
}

struct WorkerState {
    running: bool,
    watched_reader: Option<WatchedReader>,
}

#[derive(Debug)]
struct WatchedReader {
    reader_name: String,
    card_present: bool,
}

impl PcscWorker {
    pub fn start(event_sender: Sender<PcscEvent>) -> Self {
        let (command_sender, command_receiver) = async_channel::unbounded();

        thread::spawn(move || {
            let mut runner = PcscWorkerRunner::new(event_sender, command_receiver);
            runner.run();
        });

        Self { command_sender }
    }

    pub fn sender(&self) -> Sender<PcscCommand> {
        self.command_sender.clone()
    }
}

struct PcscWorkerRunner {
    event_sender: Sender<PcscEvent>,
    command_receiver: Receiver<PcscCommand>,
    state: WorkerState,
}

impl PcscWorkerRunner {
    fn new(event_sender: Sender<PcscEvent>, command_receiver: Receiver<PcscCommand>) -> Self {
        Self {
            event_sender,
            command_receiver,
            state: WorkerState {
                running: true,
                watched_reader: None,
            },
        }
    }

    fn run(&mut self) {
        while self.state.running {
            self.drain_commands();
            self.poll_watched_reader();

            thread::sleep(Duration::from_millis(150));
        }
    }

    fn drain_commands(&mut self) {
        loop {
            match self.command_receiver.try_recv() {
                Ok(command) => self.handle_command(command),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => {
                    self.state.running = false;
                    break;
                }
            }
        }
    }

    fn handle_command(&mut self, command: PcscCommand) {
        match command {
            PcscCommand::RefreshReaders => self.refresh_readers(),
            PcscCommand::WatchReader { reader_name } => {
                self.state.watched_reader = Some(WatchedReader {
                    reader_name,
                    card_present: false,
                });
            }
            PcscCommand::ReadCardInfo { reader_name } => self.read_card_info(reader_name),
            PcscCommand::StopWatchingReader => self.state.watched_reader = None,
            PcscCommand::Shutdown => self.state.running = false,
        }
    }

    fn refresh_readers(&self) {
        match PcscService::list_readers() {
            Ok(readers) => {
                self.send_event(PcscEvent::ReadersUpdated { readers });
            }
            Err(err) => {
                self.send_event(PcscEvent::Error {
                    message: err.to_string(),
                });
            }
        }
    }

    fn poll_watched_reader(&mut self) {
        let mut should_clear_watched_reader = false;

        let event = {
            let Some(watched_reader) = self.state.watched_reader.as_mut() else {
                return;
            };

            match PcscService::is_card_present(&watched_reader.reader_name) {
                Ok(card_present) => {
                    if watched_reader.card_present == card_present {
                        None
                    } else {
                        watched_reader.card_present = card_present;

                        Some(PcscEvent::ReaderStatusUpdated {
                            reader_name: watched_reader.reader_name.clone(),
                            card_present,
                        })
                    }
                }
                Err(err) => {
                    should_clear_watched_reader = true;

                    Some(PcscEvent::Error {
                        message: err.to_string(),
                    })
                }
            }
        };

        if should_clear_watched_reader {
            self.state.watched_reader = None;
        }

        if let Some(event) = event {
            self.send_event(event);
        }
    }

    fn read_card_info(&self, reader_name: String) {
        match PcscService::read_card_info(&reader_name) {
            Ok(card) => self.send_event(PcscEvent::CardInfoUpdated { card }),
            Err(err) => self.send_event(PcscEvent::Error {
                message: err.to_string(),
            }),
        }
    }

    fn send_event(&self, event: PcscEvent) {
        println!("WORKER: Sending event {:?}", event);

        if let Err(err) = self.event_sender.try_send(event) {
            eprintln!("Failed to send PC/SC event: {err}");
        }
    }
}
