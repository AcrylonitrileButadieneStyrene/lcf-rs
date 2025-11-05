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
pub enum ScrollType {
    #[default]
    None = 0,
    Vertical = 1,
    Horizontal = 2,
    Both = 3,
}
