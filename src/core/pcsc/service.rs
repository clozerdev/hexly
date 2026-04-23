use pcsc::{Context, Scope};

use crate::core::pcsc::{PcscError, types::PcscResult};

pub struct PcscService {
    context: Context,
}

impl PcscService {
    pub fn new() -> PcscResult<Self> {
        let context = Context::establish(Scope::User).map_err(PcscError::EstablishContext)?;
        Ok(Self { context })
    }
}
