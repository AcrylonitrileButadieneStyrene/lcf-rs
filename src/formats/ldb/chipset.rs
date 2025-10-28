use crate::{
    helpers::{Array, Chunk, Number},
    ldb::LcfDataBaseReadError,
    raw::ldb::chipset::ChipSetChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChipSet {
    pub name: Vec<u8>,
    pub file: Vec<u8>,
    pub terrain: Vec<u16>,
    pub passability_lower: Vec<u32>,
    pub passability_upper: Vec<u32>,
    pub animation_type: u32,
    pub animation_speed: u32,
}

impl ChipSet {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<ChipSetChunk>>,
    ) -> Result<Self, LcfDataBaseReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                ChipSetChunk::Name(bytes) => self.name = bytes,
                ChipSetChunk::File(bytes) => self.file = bytes,
                ChipSetChunk::Terrain(items) => self.terrain = items.to_vec(),
                ChipSetChunk::PassabilityLower(items) => {
                    self.passability_lower = items.into_iter().map(|number| number.0).collect();
                }
                ChipSetChunk::PassabilityUpper(items) => {
                    self.passability_upper = items.into_iter().map(|number| number.0).collect();
                }
                ChipSetChunk::AnimationType(val) => self.animation_type = val.0,
                ChipSetChunk::AnimationSpeed(val) => self.animation_speed = val.0,
                ChipSetChunk::Unknown { id, bytes } => {
                    return Err(LcfDataBaseReadError::UnknownData(id, bytes));
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<ChipSetChunk>> {
        let mut chunks = Vec::new();

        if !self.name.is_empty() {
            chunks.push(ChipSetChunk::Name(self.name.clone()));
        }

        if !self.file.is_empty() {
            chunks.push(ChipSetChunk::File(self.file.clone()));
        }

        // TODO: the other fields (i don't know their default values)

        if self.animation_type != 0 {
            chunks.push(ChipSetChunk::AnimationType(Number(self.animation_type)));
        }

        if self.animation_speed != 0 {
            chunks.push(ChipSetChunk::AnimationSpeed(Number(self.animation_speed)));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
