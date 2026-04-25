use std::ffi::CString;

use pcsc::{Context, Disposition, Protocols, ReaderState, Scope, ShareMode, State};

use crate::core::pcsc::{
    PcscError,
    types::{CardInfo, PcscResult, ReaderInfo},
};

pub struct PcscService {
    context: Context,
}

impl PcscService {
    pub fn new() -> PcscResult<Self> {
        let context = Context::establish(Scope::User).map_err(PcscError::EstablishContext)?;
        Ok(Self { context })
    }

    pub fn list_readers(&self) -> PcscResult<Vec<ReaderInfo>> {
        let readers = self
            .context
            .list_readers_owned()
            .map_err(PcscError::ListReaders)?;

        if readers.is_empty() {
            return Ok(vec![]);
        }

        let mut result = Vec::with_capacity(readers.len());
        for reader in readers {
            let name = reader.into_string().map_err(PcscError::Utf8ReaderName)?;
            result.push(ReaderInfo {
                name,
                card_present: false,
                atr: None,
            });
        }

        Ok(result)
    }

    pub fn reader_status(&self, reader_name: &str) -> PcscResult<ReaderInfo> {
        let name = CString::new(reader_name).map_err(PcscError::InvalidReaderName)?;
        let mut states = [ReaderState::new(name, State::UNAWARE)];

        self.context
            .get_status_change(std::time::Duration::from_secs(0), &mut states)
            .map_err(PcscError::GetStatusChange)?;

        let state = states[0].event_state();
        let card_present = state.intersects(State::PRESENT);

        let atr = if card_present {
            Some(states[0].atr().to_vec())
        } else {
            None
        };

        Ok(ReaderInfo {
            name: reader_name.to_string(),
            card_present,
            atr,
        })
    }

    pub fn read_card_info(&self, reader_name: &str) -> PcscResult<Option<CardInfo>> {
        let reader_cstr = CString::new(reader_name).map_err(PcscError::InvalidReaderName)?;

        let card = match self
            .context
            .connect(&reader_cstr, ShareMode::Shared, Protocols::ANY)
        {
            Ok(card) => card,
            Err(pcsc::Error::NoSmartcard) => {
                return Ok(None);
            }
            Err(err) => {
                return Err(PcscError::ConnectCard(err));
            }
        };

        let uid = Self::read_card_uid(&card)?;
        let card_info = CardInfo { uid };

        card.disconnect(Disposition::LeaveCard)
            .map_err(|(_, err)| PcscError::DisconnectCard(err))?;

        Ok(Some(card_info))
    }

    fn read_card_uid(card: &pcsc::Card) -> PcscResult<Vec<u8>> {
        let command = [0xFF, 0xCA, 0x00, 0x00, 0x00];
        let mut response_buf = [0u8; 258];

        let res = card
            .transmit2(&command, &mut response_buf)
            .map_err(|(err, _)| PcscError::TransmitApdu(err))?;

        if res.len() < 2 {
            return Err(PcscError::InvalidApduResponse);
        }

        let sw1 = res[res.len() - 2];
        let sw2 = res[res.len() - 1];

        if sw1 != 0x90 || sw2 != 0x00 {
            return Err(PcscError::ApduStatus(sw1, sw2));
        }

        let uid = res[..res.len() - 2].to_vec();
        if uid.is_empty() {
            return Err(PcscError::EmptyUid);
        }

        Ok(uid)
    }
}
