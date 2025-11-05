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
pub enum AnimationType {
    #[default]
    Standing = 0,
    Walking = 1,
    DirectionFixInanimated = 2,
    DirectionFixAnimated = 3,
    FixedGraphic = 4,
    Spin = 5,
}
