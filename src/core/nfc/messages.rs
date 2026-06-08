use crate::core::nfc::NfcWorkerState;
use crate::core::state::{Reader, ReaderId};

// Commands (UI -> Worker)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfcCommand {
    Start,
    Shutdown,
    ReadCard { reader_id: ReaderId },
}

// Commands (Worker -> UI)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfcEvent {
    StateChanged(NfcWorkerState),
    ReadersChanged(Vec<Reader>),
    ReadStarted { reader_id: ReaderId },
    Log(String),
}
