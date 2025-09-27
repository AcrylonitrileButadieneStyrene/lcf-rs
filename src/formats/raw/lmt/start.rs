use crate::helpers::{Number, ToChunkID};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum StartChunk {
    #[br(pre_assert(id == 1))]
    PartyMapID(Number),
    #[br(pre_assert(id == 2))]
    PartyX(Number),
    #[br(pre_assert(id == 3))]
    PartyY(Number),
    #[br(pre_assert(id == 11))]
    BoatMapID(Number),
    #[br(pre_assert(id == 12))]
    BoatX(Number),
    #[br(pre_assert(id == 13))]
    BoatY(Number),
    #[br(pre_assert(id == 21))]
    ShipMapID(Number),
    #[br(pre_assert(id == 22))]
    ShipX(Number),
    #[br(pre_assert(id == 23))]
    ShipY(Number),
    #[br(pre_assert(id == 31))]
    AirshipMapID(Number),
    #[br(pre_assert(id == 32))]
    AirshipX(Number),
    #[br(pre_assert(id == 33))]
    AirshipY(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for StartChunk {
    fn id(&self) -> u32 {
        match self {
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
            Self::Unknown { id, .. } => *id,
        }
    }
}
