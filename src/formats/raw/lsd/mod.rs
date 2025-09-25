use crate::helpers::{Array, Chunk, Number, ToChunkID};

#[binrw::binrw]
#[brw(magic(b"\x0bLcfSaveData"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfSaveData(pub Array<Chunk<LcfSaveDataChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum LcfSaveDataChunk {
    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfSaveDataChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Unknown { id, .. } => id.0,
        })
    }
}
