use serde::{Deserialize, Serialize};

use crate::enums::Direction;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Graphic {
    pub file: Vec<u8>,
    pub index: u32,
    pub direction: Direction,
    pub pattern: u32,
    pub transparent: bool,
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            file: Vec::new(),
            index: 0,
            direction: Direction::default(),
            pattern: 1,
            transparent: false,
        }
    }
}
