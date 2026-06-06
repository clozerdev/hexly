use crate::core::nfc::NfcWorkerState;

// Commands (UI -> Worker)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfcCommand {
    Start,
    Shutdown,
}

// Commands (Worker -> UI)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NfcEvent {
    StateChanged(NfcWorkerState),
}
