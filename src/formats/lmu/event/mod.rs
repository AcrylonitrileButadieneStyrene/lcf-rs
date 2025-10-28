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
    pub fn with_chunks(
        mut self,
        chunks: Vec<Chunk<EventChunk>>,
    ) -> Result<Self, LcfMapUnitReadError> {
        for chunk in chunks {
            match chunk.data {
                EventChunk::Name(bytes) => self.name = bytes,
                EventChunk::PositionX(val) => self.x = val.0,
                EventChunk::PositionY(val) => self.y = val.0,
                EventChunk::Pages(chunks) => {
                    chunks
                        .inner_vec
                        .iter()
                        .enumerate()
                        .for_each(|(index, (id, _))| {
                            debug_assert_eq!(
                                index,
                                id.0 as usize - 1,
                                "event page id must match index"
                            );
                        });

                    self.pages = chunks
                        .inner_vec
                        .into_iter()
                        .map(|(_, item)| EventPage::default().with_chunks(item))
                        .try_collect()?;
                }
                EventChunk::Unknown { id, bytes } => {
                    return Err(LcfMapUnitReadError::UnknownEventData(id, bytes));
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> (Number, Array<Chunk<EventChunk>>) {
        let mut chunks = Vec::new();

        chunks.push(EventChunk::Name(self.name.clone()));

        if self.x != 0 {
            chunks.push(EventChunk::PositionX(Number(self.x)));
        }

        if self.y != 0 {
            chunks.push(EventChunk::PositionY(Number(self.y)));
        }

        chunks.push(EventChunk::Pages(
            self.pages
                .iter()
                .map(EventPage::to_chunks)
                .enumerate()
                .map(|(id, chunks)| (Number(id as u32 + 1), chunks))
                .collect(),
        ));

        (
            Number(self.id),
            Array {
                inner_vec: chunks.into_iter().map(Into::into).collect(),
                null_terminated: true,
            },
        )
    }
}
