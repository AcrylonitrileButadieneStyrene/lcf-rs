use serde::{Deserialize, Serialize};

use crate::{
    helpers::{Array, Chunk},
    raw::shared::MusicChunk,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sound {
    pub file: Vec<u8>,
    pub volume: u32,
    pub tempo: u32,
    pub balance: u32,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            file: b"<OFF>".to_vec(),
            volume: 100,
            tempo: 100,
            balance: 50,
        }
    }
}

impl Sound {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<MusicChunk>>,
    ) -> Result<(Self, Option<u32>), (u32, Vec<u8>)> {
        let mut fade_in_time = None;
        for chunk in chunks.inner_vec {
            match chunk.data {
                MusicChunk::FileName(items) => self.file = items,
                MusicChunk::FadeInTime(number) => fade_in_time = Some(number.0),
                MusicChunk::Volume(number) => self.volume = number.0,
                MusicChunk::Tempo(number) => self.tempo = number.0,
                MusicChunk::Balance(number) => self.balance = number.0,
                MusicChunk::Unknown { id, bytes } => {
                    return Err((id, bytes));
                }
            }
        }

        Ok((self, fade_in_time))
    }

    pub fn to_chunks(&self) -> Array<Chunk<MusicChunk>> {
        self.to_chunks_late(self.to_chunks_early())
    }

    pub(crate) fn to_chunks_early(&self) -> Vec<MusicChunk> {
        let mut chunks = Vec::new();
        chunks.push(MusicChunk::FileName(self.file.clone()));
        chunks
    }

    pub(crate) fn to_chunks_late(&self, mut chunks: Vec<MusicChunk>) -> Array<Chunk<MusicChunk>> {
        if self.volume != 100 {
            chunks.push(MusicChunk::Volume(self.volume.into()));
        }

        if self.tempo != 100 {
            chunks.push(MusicChunk::Tempo(self.tempo.into()));
        }

        if self.balance != 50 {
            chunks.push(MusicChunk::Balance(self.balance.into()));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
