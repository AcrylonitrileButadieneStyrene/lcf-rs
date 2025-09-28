#[repr(u32)]
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::FromRepr,
)]
pub enum Trigger {
    #[default]
    ActionButton = 0,
    PlayerTouch = 1,
    EventTouch = 2,
    Autorun = 3,
    Parallel = 4,
}
