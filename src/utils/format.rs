use crate::core::pcsc::types::{CardCapacity, CardKind};

pub fn format_card_kind(kind: &CardKind) -> String {
    match kind {
        CardKind::MifareClassic1K => "MIFARE Classic 1K".to_string(),
        CardKind::MifareClassic4K => "MIFARE Classic 4K".to_string(),
        CardKind::MifareUltralight => "MIFARE Ultralight".to_string(),
        CardKind::Unknown => "Unknown".to_string(),
    }
}

pub fn format_capacity(capacity: Option<&CardCapacity>) -> String {
    match capacity {
        Some(capacity) => {
            let mut parts = vec![format!("{} bytes", capacity.bytes)];

            if let Some(sectors) = capacity.sectors {
                parts.push(format!("{sectors} sectors"));
            }

            if let Some(blocks) = capacity.blocks {
                parts.push(format!("{blocks} blocks"));
            }

            parts.join(" · ")
        }
        None => "Unknown".to_string(),
    }
}
