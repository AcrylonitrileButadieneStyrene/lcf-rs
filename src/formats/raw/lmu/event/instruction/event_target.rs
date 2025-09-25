use byteorder::{ReadBytesExt as _, WriteBytesExt as _};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EventTarget {
    Player,
    Boat,
    Ship,
    Airship,
    Itself,
    Map(u32),
}

impl binrw::BinRead for EventTarget {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(match reader.read_u32::<byteorder::NativeEndian>()? {
            10001 => Self::Player,
            10002 => Self::Boat,
            10003 => Self::Ship,
            10004 => Self::Airship,
            10005 => Self::Itself,
            x => Self::Map(x),
        })
    }
}

impl binrw::BinWrite for EventTarget {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        Ok(writer.write_u32::<byteorder::NativeEndian>(match self {
            Self::Player => 10001,
            Self::Boat => 10002,
            Self::Ship => 10003,
            Self::Airship => 10004,
            Self::Itself => 10005,
            Self::Map(x) => *x,
        })?)
    }
}
