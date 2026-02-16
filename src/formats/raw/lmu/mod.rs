use crate::helpers::{Array, Array2D, Chunk, Number, ToChunkID};

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

    /// - Type: boolean
    #[br(pre_assert(id == 40))]
    GeneratorEnabled(Number),

    /// - 0: Road
    /// - 1: Room
    /// - 2: Maze
    /// - 3: Obstacle
    #[br(pre_assert(id == 41))]
    GeneratorMode(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 42))]
    TopLevel(Number),

    #[br(pre_assert(id == 48))]
    GeneratorTiles(Number),

    /// - Default: 4
    #[br(pre_assert(id == 49))]
    GeneratorWidth(Number),

    /// - Default: 2
    #[br(pre_assert(id == 50))]
    GeneratorHeight(Number),

    /// "Surround Exterior with Wall Ceiling"
    /// - Type: boolean
    #[br(pre_assert(id == 51))]
    GeneratorSurround(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 52))]
    GeneratorUseWallUpper(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 53))]
    GeneratorUseFloorB(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 54))]
    GeneratorUseFloorC(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 55))]
    GeneratorUseObstacleB(Number),

    /// - Type: boolean
    #[br(pre_assert(id == 56))]
    GeneratorUseObstacleC(Number),

    #[br(pre_assert(id == 60))]
    GeneratorX([u32; 9]),

    #[br(pre_assert(id == 61))]
    GeneratorY([u32; 9]),

    #[br(pre_assert(id == 62))]
    GeneratorIDs(#[br(count = length / 2)] Vec<u16>),

    #[br(pre_assert(id == 71))]
    Lower(#[br(count = length / 2)] Vec<u16>),

    #[br(pre_assert(id == 72))]
    Upper(#[br(count = length / 2)] Vec<u16>),

    #[br(pre_assert(id == 81))]
    Events(Array2D<event::EventChunk>),

    #[br(pre_assert(id == 90))]
    SaveTimeA(Number),

    #[br(pre_assert(id == 91))]
    SaveTimeB(Number),

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
            Self::GeneratorEnabled(_) => 40,
            Self::GeneratorMode(_) => 41,
            Self::TopLevel(_) => 42,
            Self::GeneratorTiles(_) => 48,
            Self::GeneratorWidth(_) => 49,
            Self::GeneratorHeight(_) => 50,
            Self::GeneratorSurround(_) => 51,
            Self::GeneratorUseWallUpper(_) => 52,
            Self::GeneratorUseFloorB(_) => 53,
            Self::GeneratorUseFloorC(_) => 54,
            Self::GeneratorUseObstacleB(_) => 55,
            Self::GeneratorUseObstacleC(_) => 56,
            Self::GeneratorX(_) => 60,
            Self::GeneratorY(_) => 61,
            Self::GeneratorIDs(_) => 62,
            Self::Lower(_) => 71,
            Self::Upper(_) => 72,
            Self::Events { .. } => 81,
            Self::SaveTimeA(_) => 90,
            Self::SaveTimeB(_) => 91,
            Self::Unknown { id, .. } => *id,
        }
    }
}
