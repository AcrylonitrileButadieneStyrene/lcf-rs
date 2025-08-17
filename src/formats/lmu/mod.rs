use crate::helpers::{Array, Chunk, Number, ToChunkID};

pub mod event;

#[binrw::binrw]
#[brw(magic(b"\x0aLcfMapUnit"), little)]
#[derive(Debug)]
pub struct LcfMapUnit(pub Array<Chunk<LcfMapUnitChunk>>);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum LcfMapUnitChunk {
    /// - Default: 1
    #[br(pre_assert(id.0 == 1))]
    ChipSet(Number),

    /// - Default: 20
    #[br(pre_assert(id.0 == 2))]
    Width(Number),

    /// - Default: 15
    #[br(pre_assert(id.0 == 3))]
    Height(Number),

    /// * 0: No Loop
    /// * 1: Vertical Loop Only
    /// * 2: Horizontal Loop Only
    /// * 3: Vertical and Horizontal Loop
    #[br(pre_assert(id.0 == 11))]
    ScrollType(Number),

    /// - Type: boolean
    /// - Default: false
    #[br(pre_assert(id.0 == 31))]
    PanoramaEnabled(Number),

    #[br(pre_assert(id.0 == 32))]
    PanoramaFile(#[br(count = length.0)] Vec<u8>),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 33))]
    PanoramaHorizontalLoop(Number),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 34))]
    PanoramaVerticalLoop(Number),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 35))]
    PanoramaHorizontalAutoScroll(Number),

    // - Range: -8 to 8
    #[br(pre_assert(id.0 == 36))]
    PanoramaHorizontalAutoScrollSpeed(Number),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 37))]
    PanoramaVerticalAutoScroll(Number),

    // - Range: -8 to 8
    #[br(pre_assert(id.0 == 38))]
    PanoramaVerticalAutoScrollSpeed(Number),

    #[br(pre_assert(id.0 == 71))]
    Lower(Array<u16>),

    #[br(pre_assert(id.0 == 72))]
    Upper(Array<u16>),

    #[br(pre_assert(id.0 == 81))]
    Events {
        #[bw(calc = Number(chunks.len() as u32))]
        count: Number,
        #[br(count = count.0)]
        chunks: Vec<(Number, Array<Chunk<event::EventChunk>>)>,
    },

    #[br(pre_assert(id.0 == 91))]
    SaveTime(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for LcfMapUnitChunk {
    fn id(&self) -> Number {
        Number(match self {
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
            Self::Unknown { id, .. } => id.0,
        })
    }
}
