use crate::{
    helpers::{Array, Chunk, Number},
    lmu::LcfMapUnitReadError,
    raw::lmu::event::condition::EventPageConditionChunk,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Condition {
    pub switch_a: (bool, u32),
    pub switch_b: (bool, u32),
    pub variable: (bool, u32),
    pub value: u32,
    pub item: (bool, u32),
    pub actor: (bool, u32),
    pub timer: (bool, u32),
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            switch_a: (false, 0),
            switch_b: (false, 0),
            variable: (false, 0),
            value: 0,
            item: (false, 0),
            actor: (false, 1),
            timer: (false, 0),
        }
    }
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
        self.switch_a.0 = flags & 1 != 0;
        self.switch_b.0 = flags & 2 != 0;
        self.variable.0 = flags & 4 != 0;
        self.item.0 = flags & 8 != 0;
        self.actor.0 = flags & 16 != 0;
        self.timer.0 = flags & 32 != 0;

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
                    Err(LcfMapUnitReadError::UnknownEventConditionData(id, bytes))?;
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<EventPageConditionChunk>> {
        let mut chunks = Vec::new();
        let flags = u32::from(self.switch_a.0)
            | u32::from(self.switch_b.0) << 1
            | u32::from(self.variable.0) << 2
            | u32::from(self.item.0) << 3
            | u32::from(self.actor.0) << 4
            | u32::from(self.timer.0) << 5;
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

        if self.value != 0 {
            chunks.push(EventPageConditionChunk::Value(Number(self.value)));
        }

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
