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
pub enum Priority {
    #[default]
    BelowCharacters = 0,
    SameAsCharacters = 1,
    AboveCharacters = 2,
}
