use crate::{
    helpers::{Array, Chunk, Number, ToChunkID},
    raw::lmu::event::{
        commands::Commands, condition::EventPageConditionChunk, move_route::EventMoveRouteChunk,
    },
};

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: Number, length: Number))]
pub enum EventPageChunk {
    #[br(pre_assert(id.0 == 2))]
    Condition(Array<Chunk<EventPageConditionChunk>>),

    /// "If this element is empty event graphic will be upper `ChipSet`."
    /// - Type: string
    #[br(pre_assert(id.0 == 21))]
    GraphicFile(#[br(count = length.0)] Vec<u8>),

    /// * When `CharSet`: 0 to 7
    /// * When Upper `ChipSet`: 0 to 143
    #[br(pre_assert(id.0 == 22))]
    GraphicIndex(Number),

    /// * 0: Up
    /// * 1: Right
    /// * 2: Down
    /// * 3: Left
    #[br(pre_assert(id.0 == 23))]
    GraphicDirection(Number),

    #[br(pre_assert(id.0 == 24))]
    GraphicPattern(Number),

    /// - Type: boolean
    #[br(pre_assert(id.0 == 25))]
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

    #[br(pre_assert(id.0 == 37))]
    MoveSpeed(Number),

    #[br(pre_assert(id.0 == 41))]
    MovementRoute(Array<Chunk<EventMoveRouteChunk>>),

    /// - Type: size in bytes of [`Self::Commands`] chunk. Can be ignored.
    #[br(pre_assert(id.0 == 51))]
    CommandsSize(Number),

    #[br(pre_assert(id.0 == 52))]
    Commands(Commands),

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
            Self::Condition(_) => 2,
            Self::GraphicFile(_) => 21,
            Self::GraphicIndex(_) => 22,
            Self::GraphicDirection(_) => 23,
            Self::GraphicPattern(_) => 24,
            Self::GraphicTransparent(_) => 25,
            Self::MovementType(_) => 31,
            Self::MovementFrequency(_) => 32,
            Self::MovementRoute(_) => 41,
            Self::Trigger(_) => 33,
            Self::Priority(_) => 34,
            Self::PriorityForbidEventOverlap(_) => 35,
            Self::AnimationType(_) => 36,
            Self::MoveSpeed(_) => 37,
            Self::CommandsSize(_) => 51,
            Self::Commands(_) => 52,
            Self::Unknown { id, .. } => id.0,
        })
    }
}
