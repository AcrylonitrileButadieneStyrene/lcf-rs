pub mod ldb;
pub mod lmt;
pub mod lmu;
pub mod lsd;

#[binrw::binrw]
#[brw(little)]
pub enum RawLcf {
    RawDataBase(ldb::RawLcfDataBase),
    RawMapTree(lmt::RawLcfMapTree),
    RawMapUnit(lmu::RawLcfMapUnit),
    RawSaveData(lsd::RawLcfSaveData),
}
