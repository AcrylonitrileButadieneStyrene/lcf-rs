use crate::{
    helpers::{Array, Chunk},
    lmu::LcfMapUnitReadError,
    raw::lmu::event::move_route::EventMoveRouteChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MoveRoute {
    commands: Vec<u8>,
    repeat: bool,
    skippable: bool,
}

impl MoveRoute {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<EventMoveRouteChunk>>,
    ) -> Result<Self, LcfMapUnitReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                EventMoveRouteChunk::CommandsSize(_) => (),
                EventMoveRouteChunk::Commands(bytes) => self.commands = bytes,
                EventMoveRouteChunk::Repeat(val) => self.repeat = val.0 != 0,
                EventMoveRouteChunk::Skippable(val) => self.skippable = val.0 != 0,
                EventMoveRouteChunk::Unknown { id, bytes } => {
                    Err(LcfMapUnitReadError::UnknownEventMoveRouteData(id, bytes))?;
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<EventMoveRouteChunk>> {
        let mut chunks = Vec::new();

        if self.commands.is_empty() {
            chunks.push(EventMoveRouteChunk::Commands(Vec::new()));
        } else {
            chunks.push(EventMoveRouteChunk::CommandsSize(
                self.commands.len().into(),
            ));
            chunks.push(EventMoveRouteChunk::Commands(self.commands.clone()));
            chunks.push(EventMoveRouteChunk::Repeat(self.repeat.into()));
            chunks.push(EventMoveRouteChunk::Skippable(self.skippable.into()));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
