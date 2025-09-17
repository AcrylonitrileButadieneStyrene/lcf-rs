use crate::helpers::{Number, ToChunkID};

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum EventMoveRouteChunk {
    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for EventMoveRouteChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Unknown { id, .. } => id.0,
        })
    }
}
