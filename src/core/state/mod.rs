pub mod types;
pub use types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    NoReaders,
    ReadersAvailable {
        readers: Vec<Reader>,
        selected_reader: Option<ReaderId>,
        reader_status: ReaderStatus,
    },
    CardPresent {
        readers: Vec<Reader>,
        selected_reader: ReaderId,
        reader_status: ReaderStatus,
        card: Card,
    },
}

impl Default for AppState {
    fn default() -> Self {
        Self::NoReaders
    }
}
