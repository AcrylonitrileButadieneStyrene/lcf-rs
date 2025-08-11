use crate::helpers::{Chunk, Array, Number, ToChunkID};

#[binrw::binrw]
#[brw(magic(b"\x0aLcfMapUnit"), little)]
#[derive(Debug)]
pub struct LcfMapUnit(pub Array<Chunk<LcfMapUnitChunk>>);

crate::impl_rw!(LcfMapUnit);

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum LcfMapUnitChunk {
    /// Default: 1
    #[br(pre_assert(id.0 == 1))]
    ChipSet(Number),

    /// Default: 20
    #[br(pre_assert(id.0 == 2))]
    Width(Number),

    /// Default: 15
    #[br(pre_assert(id.0 == 3))]
    Height(Number),

    /// * 0: No Loop
    /// * 1: Vertical Loop Only
    /// * 2: Horizontal Loop Only
    /// * 3: Vertical and Horizontal Loop
    #[br(pre_assert(id.0 == 11))]
    ScrollType(Number),

    #[br(pre_assert(id.0 == 71))]
    Lower(Array<u16>),

    #[br(pre_assert(id.0 == 72))]
    Upper(Array<u16>),

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
            Self::Lower(_) => 71,
            Self::Upper(_) => 72,
            Self::SaveTime(_) => 91,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
