use crate::helpers::Number;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum EventPageConditionChunk {
    #[br(pre_assert(id == 1))]
    Flags(Number),

    #[br(pre_assert(id == 2))]
    SwitchA(Number),

    #[br(pre_assert(id == 3))]
    SwitchB(Number),

    #[br(pre_assert(id == 4))]
    Variable(Number),

    #[br(pre_assert(id == 5))]
    Value(Number),

    #[br(pre_assert(id == 6))]
    Item(Number),

    #[br(pre_assert(id == 7))]
    Actor(Number),

    #[br(pre_assert(id == 8))]
    Timer(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for EventPageConditionChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Flags(_) => 1,
            Self::SwitchA(_) => 2,
            Self::SwitchB(_) => 3,
            Self::Variable(_) => 4,
            Self::Value(_) => 5,
            Self::Item(_) => 6,
            Self::Actor(_) => 7,
            Self::Timer(_) => 8,
            Self::Unknown { id, .. } => *id,
        }
    }
}
