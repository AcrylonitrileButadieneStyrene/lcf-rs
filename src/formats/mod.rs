use crate::raw::RawLcf;

pub mod ldb;
pub mod lmt;
pub mod lmu;
pub mod lsd;
pub mod raw;

pub enum Lcf {
    DataBase(ldb::LcfDataBase),
    MapTree(lmt::LcfMapTree),
    MapUnit(lmu::LcfMapUnit),
    SaveData(lsd::LcfSaveData),
}

#[derive(Debug, thiserror::Error)]
pub enum LcfReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),
    #[error("database error: {0}")]
    DataBase(#[from] ldb::LcfDataBaseReadError),
    #[error("map tree error: {0}")]
    MapTree(#[from] lmt::LcfMapTreeReadError),
    #[error("map unit error: {0}")]
    MapUnit(#[from] lmu::LcfMapUnitReadError),
    #[error("save data error: {0}")]
    SaveData(#[from] lsd::LcfSaveDataReadError),
}

impl TryFrom<RawLcf> for Lcf {
    type Error = LcfReadError;

    fn try_from(value: RawLcf) -> Result<Self, Self::Error> {
        match value {
            RawLcf::RawDataBase(value) => Ok(Self::DataBase(ldb::LcfDataBase::try_from(value)?)),
            RawLcf::RawMapTree(value) => Ok(Self::MapTree(lmt::LcfMapTree::try_from(value)?)),
            RawLcf::RawMapUnit(value) => Ok(Self::MapUnit(lmu::LcfMapUnit::try_from(value)?)),
            RawLcf::RawSaveData(value) => Ok(Self::SaveData(lsd::LcfSaveData::try_from(value)?)),
        }
    }
}

impl From<&Lcf> for RawLcf {
    fn from(value: &Lcf) -> Self {
        match value {
            Lcf::DataBase(value) => Self::RawDataBase(value.into()),
            Lcf::MapTree(value) => Self::RawMapTree(value.into()),
            Lcf::MapUnit(value) => Self::RawMapUnit(value.into()),
            Lcf::SaveData(value) => Self::RawSaveData(value.into()),
        }
    }
}
