use std::{
    ffi::{IntoStringError, NulError},
    fmt,
};

#[derive(Debug)]
pub enum PcscError {
    EstablishContext(pcsc::Error),
    ListReaders(pcsc::Error),
    Utf8ReaderName(IntoStringError),
    InvalidReaderName(NulError),
    GetStatusChange(pcsc::Error),
    ConnectCard(pcsc::Error),
    CardStatus(pcsc::Error),
    DisconnectCard(pcsc::Error),
    TransmitApdu(pcsc::Error),
    InvalidApduResponse,
    ApduStatus(u8, u8),
    EmptyUid,
}

impl std::fmt::Display for PcscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EstablishContext(err) => write!(f, "Failed establishing context: {err}"),
            Self::ListReaders(err) => write!(f, "Failed listing readers: {err}"),
            Self::Utf8ReaderName(err) => write!(f, "Failed converting reader name: {err}"),
            Self::GetStatusChange(err) => write!(f, "Failed getting status change: {err}"),
            Self::InvalidReaderName(err) => write!(f, "Invalid reader name: {err}"),
            Self::ConnectCard(err) => write!(f, "Failed connecting to card: {err}"),
            Self::CardStatus(err) => write!(f, "Failed getting card status: {err}"),
            Self::DisconnectCard(err) => write!(f, "Failed disconnecting card: {err}"),
            Self::TransmitApdu(err) => write!(f, "Failed transmitting APDU: {err}"),
            Self::InvalidApduResponse => write!(f, "Invalid APDU response"),
            Self::ApduStatus(sw1, sw2) => {
                write!(f, "APDU failed with status {:02X} {:02X}", sw1, sw2)
            }
            Self::EmptyUid => write!(f, "Card returned an empty UID"),
        }
    }
}

impl std::error::Error for PcscError {}
