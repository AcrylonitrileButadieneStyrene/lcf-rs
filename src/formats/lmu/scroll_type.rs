#[repr(u32)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ScrollType {
    #[default]
    None = 0,
    Vertical = 1,
    Horizontal = 2,
    Both = 3,
}

impl TryFrom<u32> for ScrollType {
    type Error = super::LcfMapUnitReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Vertical),
            2 => Ok(Self::Horizontal),
            3 => Ok(Self::Both),
            x => Err(super::LcfMapUnitReadError::InvalidScrollType(x)),
        }
    }
}
