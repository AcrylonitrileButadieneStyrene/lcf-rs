use crate::helpers::{Array, Chunk, Number, ToChunkID};

#[binrw::binrw]
#[derive(Clone, Debug)]
pub struct ChipSet {
    #[br(temp)]
    #[bw(calc = Number(data.len() as u32))]
    count: Number,

    #[br(count = count.0)]
    pub data: Vec<ChipSetChunks>,
}

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
pub struct ChipSetChunks {
    pub index: Number,
    pub chunks: Array<Chunk<ChipSetChunk>>,
}

impl ToChunkID for ChipSetChunks {
    fn id(&self) -> u32 {
        self.index.0
    }
}

#[binrw::binrw]
#[br(import(id: u32, length: u32))]
#[derive(Clone, Debug)]
#[brw(little)]
pub enum ChipSetChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 2))]
    File(#[br(count = length)] Vec<u8>),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for ChipSetChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name(_) => 1,
            Self::File(_) => 2,
            Self::Unknown { id, .. } => *id,
        }
    }
}
