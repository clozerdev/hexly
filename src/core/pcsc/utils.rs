use crate::core::pcsc::types::{CardCapacity, CardKind, CardProtocol, ParsedAtr};

#[derive(Debug, Clone, Copy)]
pub enum ByteFormat {
    Hex,
    Decimal,
}

pub fn format_bytes(bytes: &[u8], format: ByteFormat) -> String {
    match format {
        ByteFormat::Hex => bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" "),

        ByteFormat::Decimal => bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(""),
    }
}

pub fn parse_atr(atr: &[u8]) -> Option<ParsedAtr> {
    let aid = [0x0A, 0x00, 0x00, 0x03, 0x06];

    let aid_pos = atr.windows(aid.len()).position(|w| w == aid)?;
    let ss_index = aid_pos + aid.len();
    let nn_index = ss_index + 1;

    if atr.len() <= nn_index + 1 {
        return None;
    }

    let ss = atr[ss_index];
    let nn = u16::from_be_bytes([atr[nn_index], atr[nn_index + 1]]);

    Some(ParsedAtr {
        protocol: parse_protocol(ss),
        kind: parse_card_kind(nn),
    })
}

pub fn capacity_for_kind(kind: &CardKind) -> Option<CardCapacity> {
    match kind {
        CardKind::MifareClassic1K => Some(CardCapacity {
            bytes: 1024,
            sectors: Some(16),
            blocks: Some(64),
        }),
        CardKind::MifareClassic4K => Some(CardCapacity {
            bytes: 4096,
            sectors: Some(40),
            blocks: Some(256),
        }),
        CardKind::MifareUltralight => Some(CardCapacity {
            bytes: 64,
            sectors: None,
            blocks: Some(16),
        }),
        _ => None,
    }
}

fn parse_protocol(ss: u8) -> CardProtocol {
    match ss {
        0x01 | 0x02 | 0x03 => CardProtocol::Iso14443TypeA,
        0x05 | 0x06 | 0x07 => CardProtocol::Iso14443TypeB,
        0x09 | 0x0A | 0x0B | 0x0C => CardProtocol::Iso15693,
        0x11 => CardProtocol::Felica,
        other => CardProtocol::Unknown(other),
    }
}

fn parse_card_kind(nn: u16) -> CardKind {
    match nn {
        0x0001 => CardKind::MifareClassic1K,
        0x0002 => CardKind::MifareClassic4K,
        0x0003 => CardKind::MifareUltralight,
        _ => CardKind::Unknown,
    }
}
