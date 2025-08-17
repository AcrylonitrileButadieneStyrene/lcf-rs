use crate::helpers::{Number, ToChunkID};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum EventTriggerChunk {
    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for EventTriggerChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Unknown { id, .. } => id.0,
        })
    }
}
