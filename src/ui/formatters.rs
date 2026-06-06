use crate::core::state::{CardType, ReaderStatus};

pub(crate) fn format_reader_status(status: &ReaderStatus) -> &'static str {
    match status {
        ReaderStatus::Ready => "Waiting for card",
        ReaderStatus::Working => "Card detected",
        ReaderStatus::Disconnected => "No reader",
    }
}

pub(crate) fn format_card_type(kind: Option<&CardType>) -> Option<&str> {
    match kind {
        Some(CardType::MifareClassic1K) => Some("MIFARE Classic 1K"),
        Some(CardType::MifareClassic4K) => Some("MIFARE Classic 4K"),
        Some(CardType::Unknown(raw)) => Some(raw.as_str()),
        None => None,
    }
}
