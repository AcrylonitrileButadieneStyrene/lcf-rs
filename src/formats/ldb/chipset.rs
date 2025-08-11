use crate::helpers::{Chunk, Array, ToChunkID, Number};

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
    fn id(&self) -> Number {
        self.index
    }
}

#[binrw::binrw]
#[br(import(id: Number, length: Number))]
#[derive(Clone, Debug)]
#[brw(little)]
pub enum ChipSetChunk {
    #[br(pre_assert(id.0 == 1))]
    Name(#[br(count = length.0)] Vec<u8>),
    #[br(pre_assert(id.0 == 2))]
    File(#[br(count = length.0)] Vec<u8>),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for ChipSetChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Name(_) => 1,
            Self::File(_) => 2,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
