pub mod page;

use crate::{
    helpers::{Array, Chunk, Number},
    lmu::{LcfMapUnitReadError, event::page::EventPage},
    raw::lmu::event::EventChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: u32,
    pub name: Vec<u8>,
    pub x: u32,
    pub y: u32,
    pub pages: Vec<EventPage>,
}

impl Event {
    pub fn from_chunks(
        id: u32,
        chunks: Vec<Chunk<EventChunk>>,
    ) -> Result<Self, LcfMapUnitReadError> {
        let mut value = Self {
            id,
            ..Default::default()
        };

        for chunk in chunks {
            match chunk.data {
                EventChunk::Name(bytes) => value.name = bytes,
                EventChunk::PositionX(val) => value.x = val.0,
                EventChunk::PositionY(val) => value.y = val.0,
                EventChunk::Pages { chunks } => {
                    chunks.iter().enumerate().for_each(|(index, (page, _))| {
                        debug_assert_eq!(
                            index,
                            (page.0 - 1) as usize,
                            "event page id must match index"
                        );
                    });

                    value.pages = chunks
                        .into_iter()
                        .map(|(_, chunks)| EventPage::from_chunks(chunks.inner_vec))
                        .try_collect()?;
                }
                EventChunk::Unknown { id, bytes } => {
                    return Err(LcfMapUnitReadError::UnknownEventData(id.0, bytes));
                }
            }
        }

        Ok(value)
    }

    pub fn to_chunks(&self) -> (Number, Array<Chunk<EventChunk>>) {
        let mut chunks = Vec::new();

        chunks.push(EventChunk::Name(self.name.clone()));

        if self.x != 0 {
            chunks.push(EventChunk::PositionX(Number(self.x)));
        }

        if self.y != 0 {
            chunks.push(EventChunk::PositionX(Number(self.y)));
        }

        chunks.push(EventChunk::Pages {
            chunks: self.pages.iter().map(EventPage::to_chunks).collect(),
        });

        (
            Number(self.id),
            Array {
                inner_vec: chunks.into_iter().map(Into::into).collect(),
                null_terminated: true,
            },
        )
    }
}
