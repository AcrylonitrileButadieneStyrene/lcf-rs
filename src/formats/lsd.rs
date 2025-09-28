use crate::raw::lsd::RawLcfSaveData;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LcfSaveData {}

#[derive(Debug, thiserror::Error)]
pub enum LcfSaveDataReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),
}

impl TryFrom<RawLcfSaveData> for LcfSaveData {
    type Error = LcfSaveDataReadError;

    fn try_from(_value: RawLcfSaveData) -> Result<Self, Self::Error> {
        todo!("instead use `lcf::raw::lsd::RawLcfSaveData`")
    }
}

impl From<&LcfSaveData> for RawLcfSaveData {
    fn from(_value: &LcfSaveData) -> Self {
        todo!("instead use `lcf::raw::lsd::RawLcfSaveData`")
    }
}
