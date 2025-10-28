use crate::helpers::{Array, Array2D, Chunk, ToChunkID, UnknownChunk};

pub mod chipset;
pub mod common_event;
pub mod switch;
pub mod variable;

#[binrw::binrw]
#[brw(magic(b"\x0bLcfDataBase"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfDataBase(pub Array<Chunk<LcfDataBaseChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum LcfDataBaseChunk {
    #[br(pre_assert(id == 11))]
    Actors(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 12))]
    Skills(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 13))]
    Items(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 14))]
    Enemies(Array2D<UnknownChunk>),
    #[br(pre_assert(id == 15))]
    Troops(Array<Chunk<UnknownChunk>>),
    #[br(pre_assert(id == 16))]
    Terrain(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 17))]
    Attributes(Array<Chunk<UnknownChunk>>),
    #[br(pre_assert(id == 18))]
    States(Array<Chunk<UnknownChunk>>),
    #[br(pre_assert(id == 19))]
    Animations(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 20))]
    ChipSet(Array2D<chipset::ChipSetChunk>),
    #[br(pre_assert(id == 21))]
    Terms(Array<Chunk<UnknownChunk>>),
    #[br(pre_assert(id == 22))]
    System(Array<Chunk<UnknownChunk>>),
    #[br(pre_assert(id == 23))]
    Switches(Array2D<switch::SwitchChunk>),
    #[br(pre_assert(id == 24))]
    Variables(Array2D<variable::VariableChunk>),
    #[br(pre_assert(id == 25))]
    CommonEvents(Array2D<common_event::CommonEventChunk>),
    // for old r2k, this field is absent.
    // for r2k3, this field is present but empty.
    #[br(pre_assert(id == 26))]
    Version(#[br(count = length)] Vec<u8>),

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
            Self::Actors(_) => 11,
            Self::Skills(_) => 12,
            Self::Items(_) => 13,
            Self::Enemies { .. } => 14,
            Self::Troops(_) => 15,
            Self::Terrain(_) => 16,
            Self::Attributes(_) => 17,
            Self::States(_) => 18,
            Self::Animations(_) => 19,
            Self::ChipSet(_) => 20,
            Self::Terms(_) => 21,
            Self::System(_) => 22,
            Self::Switches { .. } => 23,
            Self::Variables { .. } => 24,
            Self::CommonEvents { .. } => 25,
            Self::Version(_) => 26,
            Self::Unknown { id, .. } => *id,
        }
    }
}
