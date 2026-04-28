use crate::core::pcsc::error::PcscError;

pub type PcscResult<T> = Result<T, PcscError>;

#[derive(Debug, Clone)]
pub struct ReaderInfo {
    pub reader_name: String,
}

#[derive(Debug, Clone)]
pub struct CardInfo {
    pub atr: Vec<u8>,
    pub uid: Vec<u8>,
    pub kind: CardKind,
    pub capacity: Option<CardCapacity>,
}

#[derive(Debug)]
pub enum PcscCommand {
    RefreshReaders,
    WatchReader { reader_name: String },
    StopWatchingReader,
    ReadCardInfo { reader_name: String },
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum PcscEvent {
    ReadersUpdated {
        readers: Vec<ReaderInfo>,
    },
    ReaderStatusUpdated {
        reader_name: String,
        card_present: bool,
    },
    CardInfoUpdated {
        card: Option<CardInfo>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardProtocol {
    Iso14443TypeA,
    Iso14443TypeB,
    Iso15693,
    Felica,
    Unknown(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardKind {
    MifareClassic1K,
    MifareClassic4K,
    MifareUltralight,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CardCapacity {
    pub bytes: usize,
    pub sectors: Option<usize>,
    pub blocks: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ParsedAtr {
    pub protocol: CardProtocol,
    pub kind: CardKind,
}
