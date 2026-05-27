use std::collections::HashMap;

use crate::{helpers::Number, raw::lmt::RawLcfMapTree};

use serde::{Deserialize, Serialize};

mod map;
mod start;
pub use map::{Map, MapType, Setting};
pub use start::{Position, Start};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LcfMapTree {
    /// The last map opened in the editor
    pub active: u16,
    /// Information for starting new saves
    pub start: Start,
    /// The 0th map is the game itself.
    ///
    /// A map may be [None] if it was specified in the order but never defined
    pub maps: indexmap::IndexMap<u16, Map>,
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
        assert_eq!(value.maps.inner_vec.len(), value.order.len());

        let mut basis = HashMap::new();
        for (Number(id), chunks) in value.maps.inner_vec {
            let map = Map::from_chunks(id, chunks.inner_vec).unwrap();
            basis.insert(id, Some(map));
        }

        let mut maps = indexmap::IndexMap::new();
        for Number(id) in value.order {
            let item = match basis.get_mut(&id).map(std::mem::take) {
                Some(Some(item)) => item,
                Some(None) => {
                    return Err(LcfMapTreeReadError::InvalidMapOrder);
                }
                None => return Err(LcfMapTreeReadError::InvalidMapOrder),
            };

            maps.insert(id as u16, item);
        }

        Ok(Self {
            active: value.active.0 as u16,
            maps,
            start: Start::from_chunks(value.start.inner_vec)?,
        })
    }
}

impl From<&LcfMapTree> for RawLcfMapTree {
    fn from(value: &LcfMapTree) -> Self {
        Self {
            active: value.active.into(),
            start: value.start.to_chunks(),
            order: value.maps.iter().map(|(id, _)| (*id).into()).collect(),
            maps: {
                value
                    .maps
                    .iter()
                    .map(|(id, map)| ((*id).into(), map.to_chunks()))
                    .collect()
            },
        }
    }
}
