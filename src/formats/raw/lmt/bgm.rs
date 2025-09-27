use crate::helpers::{Number, ToChunkID};

#[binrw::binrw]
#[br(import(id: u32, length: u32))]
#[derive(Clone, Debug)]
#[brw(little)]
pub enum MapBGMChunk {
    #[br(pre_assert(id == 1))]
    FileName(#[br(count = length)] Vec<u8>),
    /// - Unit: milliseconds
    /// - Range: 0 to 10000
    /// - Default: 0
    #[br(pre_assert(id == 2))]
    FadeInTime(Number),
    /// - Unit: Percentage
    /// - Range: 0 to 100
    /// - Default: 100
    #[br(pre_assert(id == 3))]
    Volume(Number),
    /// - Range: 50 to 150
    /// - Default: 100
    #[br(pre_assert(id == 4))]
    Tempo(Number),
    /// - Range: 0 (left) to 50 (center) to 100 (right)
    /// - Default: 50
    #[br(pre_assert(id == 5))]
    Balance(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for MapBGMChunk {
    fn id(&self) -> u32 {
        match self {
            Self::FileName(_) => 1,
            Self::FadeInTime(_) => 2,
            Self::Volume(_) => 3,
            Self::Tempo(_) => 4,
            Self::Balance(_) => 5,
            Self::Unknown { id, .. } => *id,
        }
    }
}
