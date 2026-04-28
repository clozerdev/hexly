use std::{ffi::CString, time::Duration};

use gtk::gsk::SerializationError;
use pcsc::{Card, Context, Disposition, Protocols, ReaderState, Scope, ShareMode, State};

use crate::core::pcsc::{
    error::PcscError,
    types::{CardInfo, CardKind, PcscResult, ReaderInfo},
    utils::{capacity_for_kind, parse_atr},
};

pub struct PcscService;

impl PcscService {
    pub fn list_readers() -> PcscResult<Vec<ReaderInfo>> {
        let context = Context::establish(Scope::User).map_err(PcscError::EstablishContext)?;

        match context.list_readers_owned() {
            Ok(names) => {
                let readers = names
                    .into_iter()
                    .map(|name| ReaderInfo {
                        reader_name: name.into_string().unwrap_or(String::from("Default reader")),
                    })
                    .collect();

                Ok(readers)
            }
            Err(pcsc::Error::NoReadersAvailable) => Ok(Vec::new()),
            Err(err) => Err(PcscError::ListReaders(err)),
        }
    }

    pub fn is_card_present(reader_name: &str) -> PcscResult<bool> {
        let context = Context::establish(Scope::User).map_err(PcscError::EstablishContext)?;
        let reader_cstr = CString::new(reader_name).map_err(PcscError::InvalidReaderName)?;

        let mut states = [ReaderState::new(reader_cstr, State::UNAWARE)];

        context
            .get_status_change(Some(Duration::from_millis(0)), &mut states)
            .map_err(PcscError::GetStatusChange)?;

        Ok(states[0].event_state().intersects(State::PRESENT))
    }

    pub fn read_card_info(reader_name: &str) -> PcscResult<Option<CardInfo>> {
        let context = Context::establish(Scope::User).map_err(PcscError::EstablishContext)?;
        let reader_cstr = CString::new(reader_name).map_err(PcscError::InvalidReaderName)?;

        let card = match context.connect(&reader_cstr, ShareMode::Shared, Protocols::ANY) {
            Ok(card) => card,
            Err(pcsc::Error::NoSmartcard) => return Ok(None),
            Err(err) => return Err(PcscError::CardStatus(err)),
        };

        let service = card.status2_owned().map_err(PcscError::CardStatus)?;
        let uid = Self::read_card_uid(&card)?;
        let atr = service.atr().to_vec();

        let parsed_atr = parse_atr(&atr);
        let kind = parsed_atr
            .as_ref()
            .map(|p| p.kind.clone())
            .unwrap_or(CardKind::MifareClassic1K);
        let capacity = capacity_for_kind(&kind);

        let info = CardInfo {
            atr,
            uid,
            kind,
            capacity,
        };

        let _ = card.disconnect(Disposition::LeaveCard);

        Ok(Some(info))
    }

    fn read_card_uid(card: &Card) -> PcscResult<Vec<u8>> {
        let command = [0xFF, 0xCA, 0x00, 0x00, 0x00];
        let mut response_buf = [0u8; 258];

        let response = card
            .transmit2(&command, &mut response_buf)
            .map_err(|(err, _)| PcscError::TransmitApdu(err))?;

        if response.len() < 2 {
            return Err(PcscError::InvalidApduResponse);
        }

        let sw1 = response[response.len() - 2];
        let sw2 = response[response.len() - 1];

        if sw1 != 0x90 || sw2 != 0x00 {
            return Err(PcscError::InvalidApduResponse);
        }

        let uid = response[..response.len() - 2].to_vec();
        if uid.is_empty() {
            return Err(PcscError::EmptyUid);
        }

        Ok(uid)
    }
}
