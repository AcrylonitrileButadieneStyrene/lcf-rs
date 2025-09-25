use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Movement {
    pub r#type: u32,
    pub frequency: u32,
    pub speed: u32,
    // pub route: Array<Chunk<EventMoveRouteChunk>>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            r#type: 0,
            frequency: 3,
            speed: 4,
        }
    }
}
