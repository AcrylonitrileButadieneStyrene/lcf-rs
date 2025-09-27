use crate::helpers::{Array, Chunk, Number, ToChunkID};

pub mod command;
pub mod commands;
pub mod condition;
pub mod instruction;
pub mod move_route;
pub mod page;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum EventChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),

    #[br(pre_assert(id == 2))]
    PositionX(Number),

    #[br(pre_assert(id == 3))]
    PositionY(Number),

    #[br(pre_assert(id == 5))]
    Pages {
        #[bw(calc = Number(chunks.len() as u32))]
        count: Number,

        #[br(count = count.0)]
        chunks: Vec<(Number, Array<Chunk<page::EventPageChunk>>)>,
    },

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for EventChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name(_) => 1,
            Self::PositionX(_) => 2,
            Self::PositionY(_) => 3,
            Self::Pages { .. } => 5,
            Self::Unknown { id, .. } => *id,
        }
    }
}
