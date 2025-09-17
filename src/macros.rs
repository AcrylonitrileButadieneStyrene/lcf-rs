macro_rules! impl_raw {
    ($name: ty) => {
        impl $name {
            pub fn read<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
            ) -> Result<Self, binrw::Error> {
                binrw::BinRead::read(reader)
            }

            pub fn write<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
            ) -> Result<(), binrw::Error> {
                binrw::BinWrite::write(self, writer)
            }
        }
    };
}

macro_rules! impl_rw {
    ($type:ty, $raw:ty, $error:ty) => {
        impl $type {
            pub fn read<R: std::io::Read + std::io::Seek>(reader: &mut R) -> Result<Self, $error> {
                <$raw>::read(reader)?.try_into()
            }

            pub fn write<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
            ) -> Result<(), binrw::Error> {
                Into::<$raw>::into(self).write(writer)
            }
        }
    };
}

macro_rules! impl_endian {
    ($($a:ident $(: $b:ident $(+ $c:ident)* )?),* ; $d:ty, $e:expr) => {
        impl<$($a $(: $b $(+ $c)* )?),*> binrw::meta::ReadEndian for $d {
            const ENDIAN: binrw::meta::EndianKind = $e;
        }

        impl<$($a $(: $b $(+ $c)* )?),*> binrw::meta::WriteEndian for $d {
            const ENDIAN: binrw::meta::EndianKind = $e;
        }
    };
    ($a:ty, $b:expr) => {
        impl binrw::meta::ReadEndian for $a {
            const ENDIAN: binrw::meta::EndianKind = $b;
        }

        impl binrw::meta::WriteEndian for $a {
            const ENDIAN: binrw::meta::EndianKind = $b;
        }
    };
}

impl_raw!(crate::raw::RawLcf);
impl_raw!(crate::raw::ldb::RawLcfDataBase);
impl_raw!(crate::raw::lmt::RawLcfMapTree);
impl_raw!(crate::raw::lmu::RawLcfMapUnit);
impl_raw!(crate::raw::lsd::RawLcfSaveData);
impl_rw!(crate::Lcf, crate::raw::RawLcf, crate::LcfReadError);
impl_rw!(
    crate::ldb::LcfDataBase,
    crate::raw::ldb::RawLcfDataBase,
    crate::ldb::LcfDataBaseReadError
);
impl_rw!(
    crate::lmt::LcfMapTree,
    crate::raw::lmt::RawLcfMapTree,
    crate::lmt::LcfMapTreeReadError
);
impl_rw!(
    crate::lmu::LcfMapUnit,
    crate::raw::lmu::RawLcfMapUnit,
    crate::lmu::LcfMapUnitReadError
);
impl_rw!(
    crate::lsd::LcfSaveData,
    crate::raw::lsd::RawLcfSaveData,
    crate::lsd::LcfSaveDataReadError
);
impl_endian!(T; crate::helpers::Array<T>, binrw::meta::EndianKind::None);
impl_endian!(crate::helpers::Number, binrw::meta::EndianKind::None);
