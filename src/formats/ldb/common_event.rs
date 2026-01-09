use binrw::BinWrite as _;

use crate::{
    enums::Trigger,
    helpers::{Array, Chunk, Number},
    ldb::LcfDataBaseReadError,
    raw::{
        ldb::common_event::CommonEventChunk,
        lmu::event::{command::Command, commands::Commands},
    },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommonEvent {
    pub id: u32,
    pub name: Vec<u8>,
    pub trigger: Trigger,
    pub switch: u32,
    pub state: bool,
    pub commands: Vec<Command>,
}

impl CommonEvent {
    #[must_use]
    pub const fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<CommonEventChunk>>,
    ) -> Result<Self, LcfDataBaseReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                CommonEventChunk::Name(bytes) => self.name = bytes,
                CommonEventChunk::Trigger(val) => {
                    self.trigger = Trigger::try_from(val.0)
                        .map_err(|_| LcfDataBaseReadError::InvalidTriggerType(val.0))?;

                    if matches!(
                        self.trigger,
                        Trigger::ActionButton | Trigger::PlayerTouch | Trigger::EventTouch
                    ) {
                        return Err(LcfDataBaseReadError::InvalidTriggerType(val.0));
                    }
                }
                CommonEventChunk::SwitchState(number) => self.state = number.0 != 0,
                CommonEventChunk::SwitchID(number) => self.switch = number.0,
                CommonEventChunk::CommandsSize(_) => (),
                CommonEventChunk::Commands(commands) => self.commands = commands.0,
                CommonEventChunk::Unknown { id, bytes } => {
                    return Err(LcfDataBaseReadError::UnknownCommonEventData(id, bytes));
                }
            }
        }

        Ok(self)
    }

    #[must_use]
    pub fn to_chunks(&self) -> Array<Chunk<CommonEventChunk>> {
        let mut chunks = Vec::new();

        chunks.push(CommonEventChunk::Name(self.name.clone()));
        chunks.push(CommonEventChunk::Trigger(Number(self.trigger as u32)));
        chunks.push(CommonEventChunk::CommandsSize(Number({
            let mut buf = std::io::Cursor::new(Vec::new());
            self.commands.write(&mut buf).unwrap();
            buf.into_inner().len() as u32 + 4
        })));
        if self.state {
            chunks.push(CommonEventChunk::SwitchState(Number(u32::from(self.state))));
        }
        if self.switch != 0 {
            chunks.push(CommonEventChunk::SwitchID(Number(self.switch)));
        }
        chunks.push(CommonEventChunk::Commands(Commands(self.commands.clone())));

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
