use crate::helpers::Number;

#[binrw::binrw]
#[br(import(id: u32, length: u32))]
#[derive(Clone, Debug)]
#[brw(little)]
pub enum ChipSetChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),

    #[br(pre_assert(id == 2))]
    File(#[br(count = length)] Vec<u8>),

    #[br(pre_assert(id == 3))]
    Terrain([u16; 162]),

    #[br(pre_assert(id == 4))]
    PassabilityLower([Number; 162]),

    #[br(pre_assert(id == 5))]
    PassabilityUpper([Number; 144]),

    #[br(pre_assert(id == 11))]
    AnimationType(Number),

    #[br(pre_assert(id == 12))]
    AnimationSpeed(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for ChipSetChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name(_) => 1,
            Self::File(_) => 2,
            Self::Terrain(_) => 3,
            Self::PassabilityLower(_) => 4,
            Self::PassabilityUpper(_) => 5,
            Self::AnimationType(_) => 11,
            Self::AnimationSpeed(_) => 12,
            Self::Unknown { id, .. } => *id,
        }
    }
}
