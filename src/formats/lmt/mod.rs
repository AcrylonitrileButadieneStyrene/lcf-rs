use crate::{
    helpers::{Array, Number},
    raw::lmt::RawLcfMapTree,
};

use serde::{Deserialize, Serialize};

mod bgm;
mod map;
mod start;
pub use bgm::BGM;
pub use map::{Map, MapType, Setting};
pub use start::{Position, Start};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LcfMapTree {
    /// The last map opened in the editor
    active: u32,
    /// Information for starting new saves
    start: Start,
    /// The 0th map is the game itself.
    maps: Vec<(u32, Map)>,
}

#[derive(Debug, thiserror::Error)]
pub enum LcfMapTreeReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),

    #[error("invalid setting {0} on field {1} on map {2}")]
    InvalidSetting(u32, &'static str, u32),
    #[error("invalid map type {0} on map {1}")]
    InvalidMapType(u32, u32),
    #[error("map order was invalid")]
    InvalidMapOrder,

    #[error("contained unknown bgm data. chunk: {0} bytes: {1:?}")]
    UnknownBGMData(u32, Vec<u8>),
    #[error("contained unknown start data. chunk: {0} bytes: {1:?}")]
    UnknownStartData(u32, Vec<u8>),
    #[error("contained unknown data. chunk: {0} bytes: {1:?}")]
    UnknownData(u32, Vec<u8>),
}

impl TryFrom<RawLcfMapTree> for LcfMapTree {
    type Error = LcfMapTreeReadError;

    fn try_from(value: RawLcfMapTree) -> Result<Self, Self::Error> {
        assert_eq!(value.maps.len(), value.order.len());

        // i think i had a better way of doing this before
        let mut basis = value
            .maps
            .into_iter()
            .map(|(id, chunks)| {
                Map::from_chunks(id.0, chunks.inner_vec).map(|map| Some((id.0, map)))
            })
            .try_collect::<Vec<_>>()?;
        let mut maps = Vec::with_capacity(value.order.len());
        for index in value.order {
            let item = std::mem::take(&mut basis[index.0 as usize]);
            if item.is_none() {
                return Err(LcfMapTreeReadError::InvalidMapOrder);
            }
            maps.push(item);
        }

        Ok(Self {
            active: value.active.0,
            maps: maps.into_iter().map(Option::unwrap).collect(),
            start: Start::from_chunks(value.start.inner_vec)?,
        })
    }
}

impl From<&LcfMapTree> for RawLcfMapTree {
    fn from(value: &LcfMapTree) -> Self {
        Self {
            active: Number(value.active),
            start: Array {
                inner_vec: value.start.to_chunks(),
                null_terminated: true,
            },
            order: value.maps.iter().map(|(id, _)| Number(*id)).collect(),
            maps: {
                let mut maps = value.maps.clone();
                maps.sort_by_key(|(id, _)| *id);
                maps.iter()
                    .map(|(id, map)| (Number(*id), map.to_chunks()))
                    .collect()
            },
        }
    }
}
