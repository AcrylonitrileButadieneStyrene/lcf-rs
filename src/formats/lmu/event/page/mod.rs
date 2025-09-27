mod graphic;
mod movement;

pub use graphic::Graphic;
pub use movement::Movement;

use crate::{
    enums::{AnimationType, Priority, Trigger},
    helpers::{Array, Chunk, Number},
    lmu::LcfMapUnitReadError,
    raw::lmu::event::{command::Command, page::EventPageChunk},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventPage {
    // pub trigger_term: Array<Chunk<EventTriggerChunk>>,
    pub graphic: Graphic,
    pub movement: Movement,
    pub trigger: Trigger,
    pub priority: Priority,
    pub forbid_event_overlap: bool,
    pub animation_type: AnimationType,
    pub commands: Vec<Command>,
}

impl EventPage {
    pub fn from_chunks(chunks: Vec<Chunk<EventPageChunk>>) -> Result<Self, LcfMapUnitReadError> {
        let mut value = Self::default();

        for chunk in chunks {
            match chunk.data {
                EventPageChunk::Condition(_chunks) => (),
                EventPageChunk::GraphicFile(bytes) => value.graphic.file = bytes,
                EventPageChunk::GraphicIndex(val) => value.graphic.index = val.0,
                EventPageChunk::GraphicDirection(val) => value.graphic.direction = val.0,
                EventPageChunk::GraphicPattern(val) => value.graphic.pattern = val.0,
                EventPageChunk::GraphicTransparent(val) => value.graphic.transparent = val.0 != 0,
                EventPageChunk::MovementType(val) => value.movement.r#type = val.0,
                EventPageChunk::MovementFrequency(val) => value.movement.frequency = val.0,
                EventPageChunk::MovementRoute(_chunks) => (),
                EventPageChunk::Trigger(val) => {
                    value.trigger = Trigger::from_repr(val.0)
                        .ok_or(LcfMapUnitReadError::InvalidTriggerType(val.0))?;
                }
                EventPageChunk::Priority(val) => {
                    value.priority = Priority::from_repr(val.0)
                        .ok_or(LcfMapUnitReadError::InvalidPriorityType(val.0))?;
                }
                EventPageChunk::PriorityForbidEventOverlap(val) => {
                    value.forbid_event_overlap = val.0 != 0;
                }
                EventPageChunk::AnimationType(val) => {
                    value.animation_type = AnimationType::from_repr(val.0)
                        .ok_or(LcfMapUnitReadError::InvalidAnimationType(val.0))?;
                }
                EventPageChunk::MoveSpeed(val) => value.movement.speed = val.0,
                EventPageChunk::CommandsSize(_) => (),
                EventPageChunk::Commands(commands) => value.commands = commands.0,
                EventPageChunk::Unknown { id, bytes } => {
                    return Err(LcfMapUnitReadError::UnknownEventPageData(id, bytes));
                }
            }
        }

        Ok(value)
    }

    #[must_use]
    pub fn to_chunks(&self) -> (Number, Array<Chunk<EventPageChunk>>) {
        todo!()
    }
}
