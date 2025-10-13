use crate::{
    helpers::{Array, Chunk, Number},
    lmu::LcfMapUnitReadError,
    raw::lmu::event::condition::EventPageConditionChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Condition {
    pub switch_a: (bool, u32),
    pub switch_b: (bool, u32),
    pub variable: (bool, u32),
    pub value: u32,
    pub item: (bool, u32),
    pub actor: (bool, u32),
    pub timer: (bool, u32),
}

impl Condition {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<EventPageConditionChunk>>,
    ) -> Result<Self, LcfMapUnitReadError> {
        let flags = chunks
            .inner_vec
            .iter()
            .find_map(|chunk| match chunk.data {
                EventPageConditionChunk::Flags(flags) => Some(flags),
                _ => None,
            })
            .ok_or(LcfMapUnitReadError::MissingEventConditionFlags)?
            .0;
        self.switch_a.0 = flags & 1 == 1;
        self.switch_b.0 = flags & 2 == 1;
        self.variable.0 = flags & 4 == 1;
        self.item.0 = flags & 8 == 1;
        self.actor.0 = flags & 16 == 1;
        self.timer.0 = flags & 32 == 1;

        for chunk in chunks.inner_vec {
            match chunk.data {
                EventPageConditionChunk::Flags(_) => (),
                EventPageConditionChunk::SwitchA(val) => self.switch_a.1 = val.0,
                EventPageConditionChunk::SwitchB(val) => self.switch_b.1 = val.0,
                EventPageConditionChunk::Variable(val) => self.variable.1 = val.0,
                EventPageConditionChunk::Value(val) => self.value = val.0,
                EventPageConditionChunk::Item(val) => self.item.1 = val.0,
                EventPageConditionChunk::Actor(val) => self.actor.1 = val.0,
                EventPageConditionChunk::Timer(val) => self.timer.1 = val.0,
                EventPageConditionChunk::Unknown { id, bytes } => {
                    Err(LcfMapUnitReadError::UnknownEventConditionData(id, bytes))?
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<EventPageConditionChunk>> {
        let mut chunks = Vec::new();
        let flags = (self.switch_a.0 as u32)
            | (self.switch_b.0 as u32) << 1
            | (self.variable.0 as u32) << 2
            | (self.item.0 as u32) << 3
            | (self.actor.0 as u32) << 4
            | (self.timer.0 as u32) << 5;
        chunks.push(EventPageConditionChunk::Flags(Number(flags)));

        if self.switch_a.1 != 0 {
            chunks.push(EventPageConditionChunk::SwitchA(Number(self.switch_a.1)));
        }

        if self.switch_b.1 != 0 {
            chunks.push(EventPageConditionChunk::SwitchB(Number(self.switch_b.1)));
        }

        if self.variable.1 != 0 {
            chunks.push(EventPageConditionChunk::Variable(Number(self.variable.1)));
        }

        chunks.push(EventPageConditionChunk::Value(Number(self.value)));

        if self.item.1 != 0 {
            chunks.push(EventPageConditionChunk::Item(Number(self.item.1)));
        }

        // this field defaults to 1 and is always pushed. r2k bug?
        chunks.push(EventPageConditionChunk::Actor(Number(self.actor.1)));

        if self.timer.1 != 0 {
            chunks.push(EventPageConditionChunk::Timer(Number(self.timer.1)));
        }

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
