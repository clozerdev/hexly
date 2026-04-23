use std::fmt;

#[derive(Debug)]
pub enum PcscError {
    EstablishContext(pcsc::Error),
}

impl std::fmt::Display for PcscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EstablishContext(err) => {
                write!(f, "Failed establishing context: {}", err)
            }
        }
    }
}

impl std::error::Error for PcscError {}
