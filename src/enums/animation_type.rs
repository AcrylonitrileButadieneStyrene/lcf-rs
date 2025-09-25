#[repr(u32)]
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::FromRepr,
)]
pub enum AnimationType {
    #[default]
    Standing = 0,
    Walking = 1,
    DirectionFixInanimated = 2,
    DirectionFixAnimated = 3,
    FixedGraphic = 4,
    Spin = 5,
}
