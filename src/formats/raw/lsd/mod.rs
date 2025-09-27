use crate::helpers::{Array, Chunk, ToChunkID};

mod chunks;
pub use chunks::*;

#[binrw::binrw]
#[brw(magic(b"\x0bLcfSaveData"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfSaveData(pub Array<Chunk<LcfSaveDataChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum LcfSaveDataChunk {
    #[br(pre_assert(id == 100))]
    Title {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 101))]
    System(Array<Chunk<SaveSystemChunk>>),
    #[br(pre_assert(id == 102))]
    Screen {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 103))]
    Pictures {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 104))]
    PartyLocation {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 105))]
    BoatLocation {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 106))]
    ShipLocation {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 107))]
    AirshipLocation {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 108))]
    Actors {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 109))]
    Inventory {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 110))]
    Targets {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 111))]
    MapInfo {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 112))]
    Panorama {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 113))]
    ExecutionState {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 114))]
    CommonEvents {
        #[br(count = length)]
        bytes: Vec<u8>,
    },
    #[br(pre_assert(id == 200))]
    EasyRPG {
        #[br(count = length)]
        bytes: Vec<u8>,
    },

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfSaveDataChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Title { .. } => 100,
            Self::System { .. } => 101,
            Self::Screen { .. } => 102,
            Self::Pictures { .. } => 103,
            Self::PartyLocation { .. } => 104,
            Self::BoatLocation { .. } => 105,
            Self::ShipLocation { .. } => 106,
            Self::AirshipLocation { .. } => 107,
            Self::Actors { .. } => 108,
            Self::Inventory { .. } => 109,
            Self::Targets { .. } => 110,
            Self::MapInfo { .. } => 111,
            Self::Panorama { .. } => 112,
            Self::ExecutionState { .. } => 113,
            Self::CommonEvents { .. } => 114,
            Self::EasyRPG { .. } => 200,
            Self::Unknown { id, .. } => *id,
        }
    }
}
