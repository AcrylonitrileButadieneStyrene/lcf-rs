use serde::{Deserialize, Serialize};

use crate::enums::Direction;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Graphic {
    pub file: Vec<u8>,
    pub index: u32,
    pub direction: Direction,
    pub pattern: u32,
    pub transparent: bool,
}
