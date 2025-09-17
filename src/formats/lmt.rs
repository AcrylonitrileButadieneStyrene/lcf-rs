use crate::raw::lmt::RawLcfMapTree;

pub struct LcfMapTree {}

#[derive(Debug, thiserror::Error)]
pub enum LcfMapTreeReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),
}

impl TryFrom<RawLcfMapTree> for LcfMapTree {
    type Error = LcfMapTreeReadError;

    fn try_from(_value: RawLcfMapTree) -> Result<Self, Self::Error> {
        todo!("instead use `lcf::raw::lmt::RawLcfMapTree`")
    }
}

impl From<&LcfMapTree> for RawLcfMapTree {
    fn from(_value: &LcfMapTree) -> Self {
        todo!("instead use `lcf::raw::lmt::RawLcfMapTree`")
    }
}
