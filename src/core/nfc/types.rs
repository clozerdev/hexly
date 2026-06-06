pub mod nfc {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NfcWorkerState {
        Running,
        Stopped,
    }
}
