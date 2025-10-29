/// Provides [`ConvertExt::read`] and [`ConvertExt::write`] for the non-raw formats.
///
/// Internally reads/writes a raw format and converts to/from the non-raw format using [`From`] and [`TryFrom`].
pub trait ConvertExt: TryFrom<Self::Raw>
where
    <Self as TryFrom<Self::Raw>>::Error: From<binrw::Error>,
{
    type Raw: binrw::meta::ReadEndian
        + binrw::meta::WriteEndian
        + for<'a> binrw::BinRead<Args<'a> = ()>
        + for<'a> binrw::BinWrite<Args<'a> = ()>
        + for<'a> From<&'a Self>;

    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
    ) -> Result<Self, <Self as TryFrom<Self::Raw>>::Error> {
        <Self::Raw as binrw::BinRead>::read(reader)?.try_into()
    }

    fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        binrw::BinWrite::write(&Into::<Self::Raw>::into(self), writer)
    }
}

impl ConvertExt for crate::Lcf {
    type Raw = crate::raw::RawLcf;
}

impl ConvertExt for crate::ldb::LcfDataBase {
    type Raw = crate::raw::ldb::RawLcfDataBase;
}

impl ConvertExt for crate::lmt::LcfMapTree {
    type Raw = crate::raw::lmt::RawLcfMapTree;
}

impl ConvertExt for crate::lmu::LcfMapUnit {
    type Raw = crate::raw::lmu::RawLcfMapUnit;
}

impl ConvertExt for crate::lsd::LcfSaveData {
    type Raw = crate::raw::lsd::RawLcfSaveData;
}
