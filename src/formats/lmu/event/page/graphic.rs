use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Graphic {
    pub file: Vec<u8>,
    pub index: u32,
    pub direction: u32,
    pub pattern: u32,
    pub transparent: bool,
}
