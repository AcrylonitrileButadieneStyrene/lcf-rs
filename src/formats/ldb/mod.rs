use crate::{
    helpers::Array,
    raw::ldb::{LcfDataBaseChunk, RawLcfDataBase},
};

pub mod chipset;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LcfDataBase {
    pub chipsets: Vec<chipset::ChipSet>,
}

#[derive(Debug, thiserror::Error)]
pub enum LcfDataBaseReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),
    #[error("out of order array")]
    OutOfOrderArray,
    #[error("contained unknown data. chunk: {0} bytes: {1:?}")]
    UnknownData(u32, Vec<u8>),
}

impl TryFrom<RawLcfDataBase> for LcfDataBase {
    type Error = LcfDataBaseReadError;

    fn try_from(raw: RawLcfDataBase) -> Result<Self, Self::Error> {
        let mut value = Self::default();

        for chunk in raw.0.inner_vec {
            match chunk.data {
                LcfDataBaseChunk::Actors(_) => (),
                LcfDataBaseChunk::Skills(_) => (),
                LcfDataBaseChunk::Items(_) => (),
                LcfDataBaseChunk::Enemies(_) => (),
                LcfDataBaseChunk::Troops(_) => (),
                LcfDataBaseChunk::Terrain(_) => (),
                LcfDataBaseChunk::Attributes(_) => (),
                LcfDataBaseChunk::States(_) => (),
                LcfDataBaseChunk::Animations(_) => (),
                LcfDataBaseChunk::ChipSet(items) => {
                    value.chipsets = items
                        .to_vec()
                        .ok_or(LcfDataBaseReadError::OutOfOrderArray)?
                        .into_iter()
                        .map(|chunks| chipset::ChipSet::default().with_chunks(chunks))
                        .try_collect()?;
                }
                LcfDataBaseChunk::Terms(_) => (),
                LcfDataBaseChunk::System(_) => (),
                LcfDataBaseChunk::Switches(_) => (),
                LcfDataBaseChunk::Variables(_) => (),
                LcfDataBaseChunk::CommonEvents(_) => (),
                LcfDataBaseChunk::Version(_) => (),
                LcfDataBaseChunk::Unknown { id, bytes } => {
                    return Err(LcfDataBaseReadError::UnknownData(id, bytes));
                }
            }
        }

        Ok(value)
    }
}

impl From<&LcfDataBase> for RawLcfDataBase {
    fn from(value: &LcfDataBase) -> Self {
        let chunks = vec![LcfDataBaseChunk::ChipSet(
            value
                .chipsets
                .iter()
                .map(chipset::ChipSet::to_chunks)
                .collect(),
        )];

        Self(Array {
            null_terminated: false,
            inner_vec: chunks.into_iter().map(Into::into).collect(),
        })
    }
}
