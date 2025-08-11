pub mod ldb;
pub mod lmt;
pub mod lmu;
pub mod lsd;

#[binrw::binrw]
#[brw(little)]
pub enum Lcf {
    DataBase(crate::ldb::LcfDataBase),
    MapTree(crate::lmt::LcfMapTree),
    MapUnit(crate::lmu::LcfMapUnit),
    SaveData(crate::lsd::LcfSaveData),
}
