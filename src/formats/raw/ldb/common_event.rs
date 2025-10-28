use crate::{
    helpers::{Number, ToChunkID},
    raw::lmu::event::commands::Commands,
};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum CommonEventChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),

    #[br(pre_assert(id == 11))]
    Trigger(Number),

    #[br(pre_assert(id == 12))]
    SwitchState(Number),

    #[br(pre_assert(id == 13))]
    SwitchID(Number),

    /// - Type: size in bytes of [`Self::Commands`] chunk. Can be ignored.
    #[br(pre_assert(id == 21))]
    CommandsSize(Number),

    #[br(pre_assert(id == 22))]
    Commands(Commands),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for CommonEventChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name { .. } => 1,
            Self::Trigger(_) => 11,
            Self::SwitchState(_) => 12,
            Self::SwitchID(_) => 13,
            Self::CommandsSize(_) => 21,
            Self::Commands(_) => 22,
            Self::Unknown { id, .. } => *id,
        }
    }
}
