use crate::helpers::{Array, Chunk, ToChunkID};

pub mod chipset;

#[binrw::binrw]
#[brw(magic(b"\x0bLcfDataBase"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfDataBase(pub Array<Chunk<LcfDataBaseChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum LcfDataBaseChunk {
    #[br(pre_assert(id == 20))]
    ChipSet(chipset::ChipSet),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfDataBaseChunk {
    fn id(&self) -> u32 {
        match self {
            Self::ChipSet(_) => 20,
            Self::Unknown { id, .. } => *id,
        }
    }
}
