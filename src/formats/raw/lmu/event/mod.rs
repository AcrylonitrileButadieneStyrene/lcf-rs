use crate::helpers::{Array, Chunk, Number, ToChunkID};

pub mod command;
pub mod commands;
pub mod instruction;
pub mod move_route;
pub mod page;
pub mod condition;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum EventChunk {
    #[br(pre_assert(id.0 == 1))]
    Name(#[br(count = length.0)] Vec<u8>),

    #[br(pre_assert(id.0 == 2))]
    PositionX(Number),

    #[br(pre_assert(id.0 == 3))]
    PositionY(Number),

    #[br(pre_assert(id.0 == 5))]
    Pages {
        #[bw(calc = Number(chunks.len() as u32))]
        count: Number,

        #[br(count = count.0)]
        chunks: Vec<(Number, Array<Chunk<page::EventPageChunk>>)>,
    },

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for EventChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Name(_) => 1,
            Self::PositionX(_) => 2,
            Self::PositionY(_) => 3,
            Self::Pages { .. } => 5,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
