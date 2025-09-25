#[repr(u32)]
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::FromRepr,
)]
pub enum ScrollType {
    #[default]
    None = 0,
    Vertical = 1,
    Horizontal = 2,
    Both = 3,
}
