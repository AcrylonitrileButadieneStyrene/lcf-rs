#[repr(u32)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    num_enum::TryFromPrimitive,
)]
pub enum Direction {
    Up = 0,
    Right = 1,
    #[default]
    Down = 2,
    Left = 3,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Up => "Up",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Down => "Down",
        })
    }
}
