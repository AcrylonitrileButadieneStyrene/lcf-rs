use serde::{Deserialize, Serialize};

use crate::{
    helpers::{Array, Chunk},
    raw::lmt::bgm::MapBGMChunk,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BGM {
    pub file: Vec<u8>,
    pub fade_in_time: u32,
    pub volume: u32,
    pub tempo: u32,
    pub balance: u32,
}

impl Default for BGM {
    fn default() -> Self {
        Self {
            file: b"<OFF>".to_vec(),
            fade_in_time: 0,
            volume: 100,
            tempo: 100,
            balance: 50,
        }
    }
}

impl BGM {
    pub fn to_chunks(&self) -> Array<Chunk<MapBGMChunk>> {
        let mut chunks = Vec::new();
        chunks.push(MapBGMChunk::FileName(self.file.clone()));

        if self.fade_in_time != 0 {
            chunks.push(MapBGMChunk::FadeInTime(self.fade_in_time.into()));
        }

        if self.volume != 100 {
            chunks.push(MapBGMChunk::Volume(self.volume.into()));
        }

        if self.tempo != 100 {
            chunks.push(MapBGMChunk::Tempo(self.tempo.into()));
        }

        if self.balance != 50 {
            chunks.push(MapBGMChunk::Balance(self.balance.into()));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
