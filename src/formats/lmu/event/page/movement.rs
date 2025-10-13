#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Movement {
    pub r#type: u32,
    pub frequency: u32,
    pub speed: u32,
    pub route: super::MoveRoute,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            r#type: 0,
            frequency: 3,
            speed: 3,
            route: super::MoveRoute::default(),
        }
    }
}
