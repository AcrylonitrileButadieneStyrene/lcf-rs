use crate::{
    helpers::{Array, Chunk, Number, ToChunkID},
    raw::lmt::bgm::MapBGMChunk,
};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum MapChunk {
    /// For the 0th map, this is the game title.
    #[br(pre_assert(id.0 == 1))]
    Name(#[br(count = length.0)] Vec<u8>),
    #[br(pre_assert(id.0 == 2))]
    Parent(Number),
    #[br(pre_assert(id.0 == 3))]
    Indentation(Number),
    #[br(pre_assert(id.0 == 4))]
    Type(Number),
    #[br(pre_assert(id.0 == 5))]
    HorizontalScrollBar(Number),
    #[br(pre_assert(id.0 == 6))]
    VerticalScrollBar(Number),
    /// - Type: boolean
    #[br(pre_assert(id.0 == 7))]
    Expanded(Number),
    /// * 0: Inherit
    /// * 1: Set by map event
    /// * 2: Preset
    #[br(pre_assert(id.0 == 11))]
    BGM(Number),
    #[br(pre_assert(id.0 == 12))]
    BGMData(Array<Chunk<MapBGMChunk>>),
    /// * 0: Inherit
    /// * 1: Set by map event
    /// * 2: Preset
    #[br(pre_assert(id.0 == 21))]
    Background(Number),
    #[br(pre_assert(id.0 == 22))]
    BackgroundFile(#[br(count = length.0)] Vec<u8>),
    /// * 0: Inherit
    /// * 1: Set by map event
    /// * 2: Preset
    #[br(pre_assert(id.0 == 31))]
    Teleport(Number),
    #[br(pre_assert(id.0 == 32))]
    Escape(Number),
    #[br(pre_assert(id.0 == 33))]
    Save(Number),
    #[br(pre_assert(id.0 == 41))]
    EncounterEnemyGroup(#[br(count = length.0)] Vec<u8>),
    /// Default 25
    #[br(pre_assert(id.0 == 44))]
    EnemyAppearStep(Number),
    /// Normal map will be all 0
    #[br(pre_assert(id.0 == 51))]
    AreaRange {
        begin_x: u32,
        begin_y: u32,
        end_x: u32,
        end_y: u32,
    },

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for MapChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::Name(_) => 1,
            Self::Parent(_) => 2,
            Self::Indentation(_) => 3,
            Self::Type(_) => 4,
            Self::HorizontalScrollBar(_) => 5,
            Self::VerticalScrollBar(_) => 6,
            Self::Expanded(_) => 7,
            Self::BGM(_) => 11,
            Self::BGMData(_) => 12,
            Self::Background(_) => 21,
            Self::BackgroundFile(_) => 22,
            Self::Teleport(_) => 31,
            Self::Escape(_) => 32,
            Self::Save(_) => 33,
            Self::EncounterEnemyGroup(_) => 41,
            Self::EnemyAppearStep(_) => 44,
            Self::AreaRange { .. } => 51,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
