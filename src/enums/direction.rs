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
    Left = 1,
    Right = 2,
    #[default]
    Down = 3,
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
