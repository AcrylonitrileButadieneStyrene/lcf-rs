use crate::helpers::{Array, Chunk, Number, ToChunkID};

pub mod event;

#[binrw::binrw]
#[brw(magic(b"\x0aLcfMapUnit"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfMapUnit(pub Array<Chunk<LcfMapUnitChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum LcfMapUnitChunk {
    /// - Default: 1
    #[br(pre_assert(id == 1))]
    ChipSet(Number),

    /// - Default: 20
    #[br(pre_assert(id == 2))]
    Width(Number),

    /// - Default: 15
    #[br(pre_assert(id == 3))]
    Height(Number),

    /// * 0: No Loop
    /// * 1: Vertical Loop Only
    /// * 2: Horizontal Loop Only
    /// * 3: Vertical and Horizontal Loop
    #[br(pre_assert(id == 11))]
    ScrollType(Number),

    /// - Type: boolean
    /// - Default: false
    #[br(pre_assert(id == 31))]
    PanoramaEnabled(Number),

    #[br(pre_assert(id == 32))]
    PanoramaFile(#[br(count = length)] Vec<u8>),

    /// - Type: boolean
    #[br(pre_assert(id == 33))]
    PanoramaHorizontalLoop(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 34))]
    PanoramaVerticalLoop(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 35))]
    PanoramaHorizontalAutoScroll(Number),

    /// - Range: -8 to 8
    #[br(pre_assert(id == 36))]
    PanoramaHorizontalAutoScrollSpeed(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 37))]
    PanoramaVerticalAutoScroll(Number),

    /// - Range: -8 to 8
    #[br(pre_assert(id == 38))]
    PanoramaVerticalAutoScrollSpeed(Number),

    #[br(pre_assert(id == 71))]
    Lower(#[br(count = length / 2)] Vec<u16>),

    #[br(pre_assert(id == 72))]
    Upper(#[br(count = length / 2)] Vec<u16>),

    #[br(pre_assert(id == 81))]
    Events {
        #[bw(calc = Number(chunks.len() as u32))]
        count: Number,

        #[br(count = count.0)]
        chunks: Vec<(Number, Array<Chunk<event::EventChunk>>)>,
    },

    #[br(pre_assert(id == 91))]
    SaveTime(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfMapUnitChunk {
    fn id(&self) -> u32 {
        match self {
            Self::ChipSet(_) => 1,
            Self::Width(_) => 2,
            Self::Height(_) => 3,
            Self::ScrollType(_) => 11,
            Self::PanoramaEnabled(_) => 31,
            Self::PanoramaFile(_) => 32,
            Self::PanoramaHorizontalLoop(_) => 33,
            Self::PanoramaVerticalLoop(_) => 34,
            Self::PanoramaHorizontalAutoScroll(_) => 35,
            Self::PanoramaHorizontalAutoScrollSpeed(_) => 36,
            Self::PanoramaVerticalAutoScroll(_) => 37,
            Self::PanoramaVerticalAutoScrollSpeed(_) => 38,
            Self::Lower(_) => 71,
            Self::Upper(_) => 72,
            Self::Events { .. } => 81,
            Self::SaveTime(_) => 91,
            Self::Unknown { id, .. } => *id,
        }
    }
}
