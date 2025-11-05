mod condition;
mod graphic;
mod move_route;
mod movement;

use binrw::BinWrite;
pub use condition::Condition;
pub use graphic::Graphic;
pub use move_route::MoveRoute;
pub use movement::Movement;

use crate::{
    enums::{AnimationType, Direction, Priority, Trigger},
    helpers::{Array, Chunk, Number},
    lmu::LcfMapUnitReadError,
    raw::lmu::event::{command::Command, commands::Commands, page::EventPageChunk},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventPage {
    pub condition: Condition,
    pub graphic: Graphic,
    pub movement: Movement,
    pub trigger: Trigger,
    pub priority: Priority,
    pub forbid_event_overlap: bool,
    pub animation_type: AnimationType,
    pub commands: Vec<Command>,
}

impl EventPage {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<EventPageChunk>>,
    ) -> Result<Self, LcfMapUnitReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                EventPageChunk::Condition(chunks) => {
                    self.condition = self.condition.with_chunks(chunks)?;
                }
                EventPageChunk::GraphicFile(bytes) => self.graphic.file = bytes,
                EventPageChunk::GraphicIndex(val) => self.graphic.index = val.0,
                EventPageChunk::GraphicDirection(val) => {
                    self.graphic.direction = Direction::try_from(val.0)
                        .map_err(|_| LcfMapUnitReadError::InvalidDirection(val.0))?;
                }
                EventPageChunk::GraphicPattern(val) => self.graphic.pattern = val.0,
                EventPageChunk::GraphicTransparent(val) => self.graphic.transparent = val.0 != 0,
                EventPageChunk::MovementType(val) => self.movement.r#type = val.0,
                EventPageChunk::MovementFrequency(val) => self.movement.frequency = val.0,
                EventPageChunk::MovementRoute(_chunks) => (),
                EventPageChunk::Trigger(val) => {
                    self.trigger = Trigger::try_from(val.0)
                        .map_err(|_| LcfMapUnitReadError::InvalidTriggerType(val.0))?;
                }
                EventPageChunk::Priority(val) => {
                    self.priority = Priority::try_from(val.0)
                        .map_err(|_| LcfMapUnitReadError::InvalidPriorityType(val.0))?;
                }
                EventPageChunk::PriorityForbidEventOverlap(val) => {
                    self.forbid_event_overlap = val.0 != 0;
                }
                EventPageChunk::AnimationType(val) => {
                    self.animation_type = AnimationType::try_from(val.0)
                        .map_err(|_| LcfMapUnitReadError::InvalidAnimationType(val.0))?;
                }
                EventPageChunk::MoveSpeed(val) => self.movement.speed = val.0,
                EventPageChunk::CommandsSize(_) => (),
                EventPageChunk::Commands(commands) => self.commands = commands.0,
                EventPageChunk::Unknown { id, bytes } => {
                    return Err(LcfMapUnitReadError::UnknownEventPageData(id, bytes));
                }
            }
        }

        Ok(self)
    }

    #[must_use]
    pub fn to_chunks(&self) -> Array<Chunk<EventPageChunk>> {
        let mut chunks = Vec::new();
        chunks.push(EventPageChunk::Condition(self.condition.to_chunks()));
        if !self.graphic.file.is_empty() {
            chunks.push(EventPageChunk::GraphicFile(self.graphic.file.clone()));
        }
        if self.graphic.index != 0 {
            chunks.push(EventPageChunk::GraphicIndex(Number(self.graphic.index)));
        }
        chunks.push(EventPageChunk::GraphicDirection(Number(
            self.graphic.direction as u32,
        )));
        if self.graphic.pattern != 0 {
            chunks.push(EventPageChunk::GraphicPattern(Number(self.graphic.pattern)));
        }
        chunks.push(EventPageChunk::GraphicTransparent(Number(u32::from(
            self.graphic.transparent,
        ))));
        chunks.push(EventPageChunk::MovementType(Number(self.movement.r#type)));
        if self.movement.frequency != 3 {
            chunks.push(EventPageChunk::MovementFrequency(Number(
                self.movement.frequency,
            )));
        }
        chunks.push(EventPageChunk::Trigger(Number(self.trigger as u32)));
        chunks.push(EventPageChunk::Priority(Number(self.priority as u32)));
        chunks.push(EventPageChunk::PriorityForbidEventOverlap(Number(
            u32::from(self.forbid_event_overlap),
        )));
        chunks.push(EventPageChunk::AnimationType(Number(
            self.animation_type as u32,
        )));
        if self.movement.speed != 3 {
            chunks.push(EventPageChunk::MoveSpeed(Number(self.movement.speed)));
        }
        chunks.push(EventPageChunk::MovementRoute(
            self.movement.route.to_chunks(),
        ));

        chunks.push(EventPageChunk::CommandsSize(Number({
            let mut buf = std::io::Cursor::new(Vec::new());
            self.commands.write(&mut buf).unwrap();
            buf.into_inner().len() as u32 + 4
        })));
        chunks.push(EventPageChunk::Commands(Commands(self.commands.clone())));

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
