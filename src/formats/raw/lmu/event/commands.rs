use byteorder::{ReadBytesExt as _, WriteBytesExt as _};

use crate::raw::lmu::event::command::Command;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commands(pub Vec<Command>);

impl Commands {
    /// Recalculates the command indentations based off of the control flow instructions
    pub fn normalize_indentation() {
        todo!("will add later")
    }
}

impl binrw::BinRead for Commands {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut instructions = Vec::new();
        loop {
            let terminator = reader.read_u32::<byteorder::NativeEndian>()?;
            if terminator == 0 {
                break;
            }

            reader.seek_relative(-4)?;
            instructions.push(Command::read_options(reader, endian, args)?);
        }

        Ok(Self(instructions))
    }
}

impl binrw::BinWrite for Commands {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        for instruction in &self.0 {
            instruction.write_options(writer, endian, args)?;
        }

        writer.write_u32::<byteorder::NativeEndian>(0)?;

        Ok(())
    }
}
