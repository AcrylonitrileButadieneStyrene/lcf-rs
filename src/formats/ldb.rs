use crate::raw::ldb::RawLcfDataBase;

pub struct LcfDataBase {}

#[derive(Debug, thiserror::Error)]
pub enum LcfDataBaseReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),
}

impl TryFrom<RawLcfDataBase> for LcfDataBase {
    type Error = LcfDataBaseReadError;

    fn try_from(_value: RawLcfDataBase) -> Result<Self, Self::Error> {
        todo!("instead use `lcf::raw::ldb::LcfDataBase`")
    }
}

impl From<&LcfDataBase> for RawLcfDataBase {
    fn from(_value: &LcfDataBase) -> Self {
        todo!("instead use `lcf::raw::ldb::LcfDataBase`")
    }
}
