use serde::{Deserialize, Serialize};

use crate::{
    helpers::{Array, Chunk},
    raw::shared::MusicChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Music {
    pub sound: super::Sound,
    pub fade_in_time: u32,
}

impl std::ops::Deref for Music {
    type Target = super::Sound;

    fn deref(&self) -> &Self::Target {
        &self.sound
    }
}

impl std::ops::DerefMut for Music {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sound
    }
}

impl Music {
    pub fn with_chunks(mut self, chunks: Array<Chunk<MusicChunk>>) -> Result<Self, (u32, Vec<u8>)> {
        let (sound, fade_in_time) = super::Sound::default().with_chunks(chunks)?;
        self.sound = sound;
        if let Some(fade_in_time) = fade_in_time {
            self.fade_in_time = fade_in_time;
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<MusicChunk>> {
        let mut chunks = self.sound.to_chunks_early();

        if self.fade_in_time != 0 {
            chunks.push(MusicChunk::FadeInTime(self.fade_in_time.into()));
        }

        self.sound.to_chunks_late(chunks)
    }
}
