#[repr(u32)]
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::FromRepr,
)]
pub enum Speed {
    Eighth = 1,
    Fourth = 2,
    #[default]
    Half = 3,
    Normal = 4,
    Double = 5,
    Quadruple = 6,
}
