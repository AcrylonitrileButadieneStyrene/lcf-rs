use crate::helpers::{Number, ToChunkID};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum StartChunk {
    #[br(pre_assert(id.0 == 1))]
    PartyMapID(Number),
    #[br(pre_assert(id.0 == 2))]
    PartyX(Number),
    #[br(pre_assert(id.0 == 3))]
    PartyY(Number),
    #[br(pre_assert(id.0 == 11))]
    BoatMapID(Number),
    #[br(pre_assert(id.0 == 12))]
    BoatX(Number),
    #[br(pre_assert(id.0 == 13))]
    BoatY(Number),
    #[br(pre_assert(id.0 == 21))]
    ShipMapID(Number),
    #[br(pre_assert(id.0 == 22))]
    ShipX(Number),
    #[br(pre_assert(id.0 == 23))]
    ShipY(Number),
    #[br(pre_assert(id.0 == 31))]
    AirshipMapID(Number),
    #[br(pre_assert(id.0 == 32))]
    AirshipX(Number),
    #[br(pre_assert(id.0 == 33))]
    AirshipY(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for StartChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::PartyMapID(_) => 1,
            Self::PartyX(_) => 2,
            Self::PartyY(_) => 3,
            Self::BoatMapID(_) => 11,
            Self::BoatX(_) => 12,
            Self::BoatY(_) => 13,
            Self::ShipMapID(_) => 21,
            Self::ShipX(_) => 22,
            Self::ShipY(_) => 23,
            Self::AirshipMapID(_) => 31,
            Self::AirshipX(_) => 32,
            Self::AirshipY(_) => 33,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
