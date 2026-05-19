use crate::{
    helpers::{Array, Chunk},
    ldb::LcfDataBaseReadError,
    raw::ldb::chipset::ChipSetChunk,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChipSet {
    pub name: Vec<u8>,
    pub file: Vec<u8>,
    #[serde(with = "serde_big_array::BigArray")]
    pub terrain: [u16; 162],
    #[serde(with = "serde_big_array::BigArray")]
    pub passability_lower: [u32; 162],
    #[serde(with = "serde_big_array::BigArray")]
    pub passability_upper: [u32; 144],
    pub animation_type: u32,
    pub animation_speed: u32,
}

impl Default for ChipSet {
    fn default() -> Self {
        // todo: find actual defaults
        Self {
            name: Vec::new(),
            file: Vec::new(),
            terrain: [0; 162],
            passability_lower: [0; 162],
            passability_upper: [0; 144],
            animation_type: 0,
            animation_speed: 0,
        }
    }
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
                ChipSetChunk::Terrain(items) => self.terrain = items,
                ChipSetChunk::PassabilityLower(items) => {
                    self.passability_lower = items.map(|number| number.0);
                }
                ChipSetChunk::PassabilityUpper(items) => {
                    self.passability_upper = items.map(|number| number.0);
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

        chunks.push(ChipSetChunk::Terrain(self.terrain.into()));
        chunks.push(ChipSetChunk::PassabilityLower(
            self.passability_lower.map(Into::into),
        ));
        chunks.push(ChipSetChunk::PassabilityUpper(
            self.passability_upper.map(Into::into),
        ));

        if self.animation_type != 0 {
            chunks.push(ChipSetChunk::AnimationType(self.animation_type.into()));
        }

        if self.animation_speed != 0 {
            chunks.push(ChipSetChunk::AnimationSpeed(self.animation_speed.into()));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
