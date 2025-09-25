#[repr(u32)]
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::FromRepr,
)]
pub enum Priority {
    #[default]
    BelowCharacters = 0,
    SameAsCharacters = 1,
    AboveCharacters = 2,
}
