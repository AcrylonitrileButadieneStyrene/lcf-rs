use byteorder::ReadBytesExt as _;

#[derive(Clone, Debug)]
pub struct Array<T> {
    pub inner_vec: Vec<T>,
    /// Decides if a null byte should be added after all of the data. r2k seems inconsistent with this.
    pub null_terminated: bool,
}

impl<T> std::ops::Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_vec
    }
}

impl<T> binrw::BinRead for Array<T>
where
    for<'a> T: binrw::BinRead<Args<'a>: Clone>,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut items = Vec::new();

        loop {
            match reader.read_u8().map_err(|err| (err.kind(), err)) {
                Ok(0) => {
                    return Ok(Self {
                        inner_vec: items,
                        null_terminated: true,
                    });
                }
                Err((std::io::ErrorKind::UnexpectedEof, _)) => {
                    return Ok(Self {
                        inner_vec: items,
                        null_terminated: false,
                    });
                }
                Err((_, err)) => Err(err)?,
                Ok(_) => {
                    reader.seek_relative(-1)?;
                    items.push(T::read_options(reader, endian, args.clone())?);
                }
            }
        }
    }
}

impl<T> binrw::BinWrite for Array<T>
where
    for<'a> T: binrw::BinWrite<Args<'a> = ()>,
{
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        for item in &self.inner_vec {
            item.write_options(writer, endian, args)?;
        }

        if self.null_terminated {
            writer.write_all(&[0])?;
        }

        Ok(())
    }
}
