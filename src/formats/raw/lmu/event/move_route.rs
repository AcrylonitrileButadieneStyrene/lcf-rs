use crate::helpers::Number;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum EventMoveRouteChunk {
    #[br(pre_assert(id == 11))]
    CommandsSize(Number),
    #[br(pre_assert(id == 12))]
    Commands(#[br(parse_with = binrw::helpers::until_eof)] Vec<u8>),
    #[br(pre_assert(id == 21))]
    Repeat(Number),
    #[br(pre_assert(id == 22))]
    Skippable(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for EventMoveRouteChunk {
    fn id(&self) -> u32 {
        match self {
            Self::CommandsSize(_) => 11,
            Self::Commands(_) => 12,
            Self::Repeat(_) => 21,
            Self::Skippable(_) => 22,
            Self::Unknown { id, .. } => *id,
        }
    }
}
