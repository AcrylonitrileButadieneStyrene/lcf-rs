use crate::helpers::{Array, Chunk, Number, ToChunkID};

pub mod chipset;

#[binrw::binrw]
#[brw(magic(b"\x0bLcfDataBase"), little)]
#[derive(Debug)]
pub struct RawLcfDataBase(pub Array<Chunk<LcfDataBaseChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum LcfDataBaseChunk {
    #[br(pre_assert(id.0 == 20))]
    ChipSet(chipset::ChipSet),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfDataBaseChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::ChipSet(_) => 20,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
