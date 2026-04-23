use crate::core::pcsc::error::PcscError;

pub type PcscResult<T> = Result<T, PcscError>;
