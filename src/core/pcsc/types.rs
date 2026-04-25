use crate::core::pcsc::error::PcscError;

pub type PcscResult<T> = Result<T, PcscError>;

#[derive(Debug, Default)]
pub struct ReaderInfo {
    pub name: String,
    pub card_present: bool,
    pub atr: Option<Vec<u8>>,
}

#[derive(Debug, Default)]
pub struct CardInfo {
    pub uid: Vec<u8>,
}
