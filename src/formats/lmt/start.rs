use crate::{
    helpers::{Chunk, Maybe, Number},
    lmt::LcfMapTreeReadError,
    raw::lmt::start::StartChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Start {
    party: Option<Position>,
    boat: Option<Position>,
    ship: Option<Position>,
    airship: Option<Position>,
}

impl Start {
    pub fn from_chunks(chunks: Vec<Chunk<StartChunk>>) -> Result<Self, LcfMapTreeReadError> {
        let mut party: Maybe<Position> = Maybe::default();
        let mut boat: Maybe<Position> = Maybe::default();
        let mut ship: Maybe<Position> = Maybe::default();
        let mut airship: Maybe<Position> = Maybe::default();

        for chunk in chunks {
            match chunk.data {
                StartChunk::PartyMapID(val) => party.map = val.0,
                StartChunk::PartyX(val) => party.x = val.0,
                StartChunk::PartyY(val) => party.y = val.0,
                StartChunk::BoatMapID(val) => boat.map = val.0,
                StartChunk::BoatX(val) => boat.x = val.0,
                StartChunk::BoatY(val) => boat.y = val.0,
                StartChunk::ShipMapID(val) => ship.map = val.0,
                StartChunk::ShipX(val) => ship.x = val.0,
                StartChunk::ShipY(val) => ship.y = val.0,
                StartChunk::AirshipMapID(val) => airship.map = val.0,
                StartChunk::AirshipX(val) => airship.x = val.0,
                StartChunk::AirshipY(val) => airship.y = val.0,
                StartChunk::Unknown { id, bytes } => {
                    return Err(LcfMapTreeReadError::UnknownStartData(id, bytes));
                }
            }
        }

        Ok(Self {
            party: party.finish(),
            boat: boat.finish(),
            ship: ship.finish(),
            airship: airship.finish(),
        })
    }

    pub fn to_chunks(&self) -> Vec<Chunk<StartChunk>> {
        let mut chunks = Vec::new();

        if let Some(position) = &self.party {
            chunks.extend_from_slice(&[
                StartChunk::PartyMapID(Number(position.map)),
                StartChunk::PartyX(Number(position.x)),
                StartChunk::PartyY(Number(position.y)),
            ]);
        }

        if let Some(position) = &self.boat {
            chunks.extend_from_slice(&[
                StartChunk::BoatMapID(Number(position.map)),
                StartChunk::BoatX(Number(position.x)),
                StartChunk::BoatY(Number(position.y)),
            ]);
        }

        if let Some(position) = &self.ship {
            chunks.extend_from_slice(&[
                StartChunk::ShipMapID(Number(position.map)),
                StartChunk::ShipX(Number(position.x)),
                StartChunk::ShipY(Number(position.y)),
            ]);
        }

        if let Some(position) = &self.airship {
            chunks.extend_from_slice(&[
                StartChunk::AirshipMapID(Number(position.map)),
                StartChunk::AirshipX(Number(position.x)),
                StartChunk::AirshipY(Number(position.y)),
            ]);
        }

        chunks.into_iter().map(Into::into).collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub map: u32,
    pub x: u32,
    pub y: u32,
}
