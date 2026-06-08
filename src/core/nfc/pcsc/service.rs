use std::ffi::CString;

use crate::core::nfc::pcsc::{PcscError, PcscReader, PcscResult};

pub struct PcscService {
    context: pcsc::Context,
}

impl PcscService {
    pub fn establish() -> PcscResult<Self> {
        let context = pcsc::Context::establish(pcsc::Scope::User).map_err(PcscError::from)?;
        Ok(Self { context })
    }

    pub fn list_readers(&self) -> PcscResult<Vec<PcscReader>> {
        let readers_vec = self.context.list_readers_owned().map_err(PcscError::from)?;
        let readers = readers_vec.into_iter().map(map_readers).collect();

        Ok(readers)
    }
}

fn map_readers(name: CString) -> PcscReader {
    PcscReader {
        name: name.to_string_lossy().into_owned(),
    }
}
