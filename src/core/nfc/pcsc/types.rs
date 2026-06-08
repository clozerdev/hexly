use std::fmt;

pub type PcscResult<T> = Result<T, PcscError>;

#[derive(Debug)]
pub struct PcscReader {
    pub name: String,
}

#[derive(Debug)]
pub struct PcscCard {
    pub uid: Vec<u8>,
    pub atr: Vec<u8>,
}

#[derive(Debug)]
pub enum PcscError {
    Native(pcsc::Error),
}

impl std::error::Error for PcscError {}

impl fmt::Display for PcscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PcscError::Native(error) => write!(f, "PC/SC error: {error}"),
        }
    }
}

impl From<pcsc::Error> for PcscError {
    fn from(value: pcsc::Error) -> Self {
        Self::Native(value)
    }
}
