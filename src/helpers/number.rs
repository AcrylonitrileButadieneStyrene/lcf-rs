use byteorder::ReadBytesExt as _;

const FLAG_CONTINUE: u8 = 0b1000_0000;
const BITS_REST: u8 = 0b0111_1111;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Number(pub u32);

impl std::ops::Deref for Number {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl binrw::BinRead for Number {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut value = 0u64;

        for _ in 0..5 {
            let byte = reader.read_u8()?;

            value <<= 7;
            value |= u64::from(byte & BITS_REST);

            if byte & FLAG_CONTINUE == 0 {
                return u32::try_from(value)
                    .map(Self)
                    .map_err(|_| binrw::Error::Custom {
                        pos: reader.stream_position().unwrap_or_default(),
                        err: Box::new(
                            "Variable sized number had value greater than u32::MAX".to_string(),
                        ),
                    });
            }
        }

        Err(binrw::Error::Custom {
            pos: reader.stream_position().unwrap_or_default(),
            err: Box::new("Variable sized number had more than 5 bytes".to_string()),
        })
    }
}

impl binrw::BinWrite for Number {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let mut value = self.0;
        let mut out = [0xCC_u8; 5];

        let mut index = None;
        for i in (0..5).rev() {
            out[i] = value as u8 & BITS_REST;

            if index.is_some() {
                out[i] |= FLAG_CONTINUE;
            }
            index = Some(i);

            value >>= 7;
            if value == 0 {
                break;
            }
        }

        writer.write_all(&out[index.unwrap_or(5)..])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Seek as _;

    use super::Number;
    use binrw::{BinRead, BinWrite, io::Cursor};

    #[test]
    fn round_trip() {
        for val in vec![
            0,
            1,
            127,
            128,
            16383,
            16384,
            65535,
            65536,
            2097151,
            2097152,
            268435455,
            268435456,
            u32::MAX,
        ]
        .into_iter()
        .map(Number)
        {
            let mut buf = Cursor::new(Vec::new());
            val.write(&mut buf).unwrap();
            buf.rewind().unwrap();

            let out = Number::read(&mut buf).unwrap();
            assert_eq!(out, val);
        }
    }
}
