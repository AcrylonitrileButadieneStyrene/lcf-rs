use binrw::{BinRead, BinWrite};
use byteorder::{LittleEndian, ReadBytesExt as _, WriteBytesExt as _};

use crate::{
    helpers::{Array, Chunk, Number, ToChunkID},
    lmu::event::{
        instruction::Instruction, move_route::EventMoveRouteChunk, trigger::EventTriggerChunk,
    },
};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum EventPageChunk {
    #[br(pre_assert(id.0 == 2))]
    TriggerTerm(Array<Chunk<EventTriggerChunk>>),

    /// "If this element is empty event graphic will be upper ChipSet."
    /// - Type: string
    #[br(pre_assert(id.0 == 21))]
    GraphicFile(#[br(count = length.0)] Vec<u8>),

    /// * When CharSet: 0 to 7
    /// * When Upper ChipSet: 0 to 143
    #[br(pre_assert(id.0 == 22))]
    GraphicIndex(Number),

    /// * 0: Up
    /// * 1: Right
    /// * 2: Down
    /// * 3: Left
    #[br(pre_assert(id.0 == 23))]
    GraphicDirection(Number),

    /// - Type: "boolean"
    /// - Values: 0 or 2
    #[br(pre_assert(id.0 == 24))]
    GraphicTransparent(Number),

    /// - 0: Fixed
    /// - 1: Random
    /// - 2: Vertical
    /// - 3: Horizontal
    /// - 4: Approach Player
    /// - 5: Away from Player
    /// - 6: Custom
    #[br(pre_assert(id.0 == 31))]
    MovementType(Number),

    /// - Range: 1 to 8
    #[br(pre_assert(id.0 == 32))]
    MovementFrequency(Number),

    #[br(pre_assert(id.0 == 41))]
    MovementRoute(Array<Chunk<EventMoveRouteChunk>>),

    /// - 0: Action Button
    /// - 1: Player Touch
    /// - 2: Event Touch
    /// - 3: Autorun
    /// - 4: Parallel process
    #[br(pre_assert(id.0 == 33))]
    Trigger(Number),

    /// - 0: Below Characters
    /// - 1: Same as Characters
    /// - 2: Above Characters
    #[br(pre_assert(id.0 == 34))]
    Priority(Number),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 35))]
    PriorityForbidEventOverlap(Number),

    /// - 0: Standing Animation
    /// - 1: Walking Animation
    /// - 2: Direction Fix/Inanimated
    /// - 3: Direction Fix/Animated
    /// - 4: Fixed Graphic
    /// - 5: Spin
    #[br(pre_assert(id.0 == 36))]
    AnimationType(Number),

    #[br(pre_assert(id.0 == 51))]
    InstructionsSize(Number),

    #[br(pre_assert(id.0 == 52))]
    Instructions(
        #[br(parse_with = parse_instructions)]
        #[bw(write_with = write_instructions)]
        Vec<Instruction>,
    ),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: Number,

        #[br(count = length.0)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for EventPageChunk {
    fn id(&self) -> Number {
        Number(match self {
            Self::TriggerTerm(_) => 2,
            Self::GraphicFile(_) => 21,
            Self::GraphicIndex(_) => 22,
            Self::GraphicDirection(_) => 23,
            Self::GraphicTransparent(_) => 24,
            Self::MovementType(_) => 31,
            Self::MovementFrequency(_) => 32,
            Self::MovementRoute(_) => 41,
            Self::Trigger(_) => 33,
            Self::Priority(_) => 34,
            Self::PriorityForbidEventOverlap(_) => 35,
            Self::AnimationType(_) => 36,
            Self::InstructionsSize(_) => 51,
            Self::Instructions(_) => 52,
            Self::Unknown { id, .. } => id.0,
        })
    }
}

#[binrw::parser(reader, endian)]
fn parse_instructions() -> Result<Vec<Instruction>, binrw::Error> {
    let mut instructions = Vec::new();
    loop {
        let terminator = reader.read_u32::<LittleEndian>()?;
        if terminator == 0 {
            break;
        }

        reader.seek_relative(-4)?;
        instructions.push(Instruction::read_options(reader, endian, ())?);
    }

    Ok(instructions)
}

#[binrw::writer(writer, endian)]
fn write_instructions(instructions: &Vec<Instruction>) -> Result<(), binrw::Error> {
    for instruction in instructions {
        instruction.write_options(writer, endian, ())?;
    }

    writer.write_u32::<LittleEndian>(0)?;

    Ok(())
}
