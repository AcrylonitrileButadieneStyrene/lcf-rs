use byteorder::{ReadBytesExt as _, WriteBytesExt as _};

use crate::raw::lmu::event::command::Command;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commands(pub Vec<Command>);

impl Commands {
    /// Recalculates the command indentations based off of the control flow instructions
    pub fn normalize_indentation(&mut self) {
        let mut indentation = 0;
        for command in &mut self.0 {
            command.indent = indentation;
            indentation = ((indentation as i32) + command.instruction.indentation_change()) as u32;
        }
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

#[cfg(test)]
mod tests {
    use crate::raw::lmu::event::{command::Command, instruction::Instruction};

    #[test]
    fn normalization() {
        let mut commands = super::Commands(
            vec![
                Instruction::ConditionalBranch {
                    mode: 0,
                    field1: 0,
                    field2: 0,
                    field3: 0,
                    field4: 0,
                    has_else: 0,
                },
                Instruction::WaitForAllMovement,
                Instruction::End,
                Instruction::ElseBranch,
                Instruction::WaitForAllMovement,
                Instruction::End,
                Instruction::EndBranch,
            ]
            .into_iter()
            .map(|instruction| Command {
                indent: 0,
                instruction,
                string: vec![],
            })
            .collect::<Vec<_>>(),
        );
        commands.normalize_indentation();
        let indents = commands
            .0
            .into_iter()
            .map(|command| command.indent)
            .collect::<Vec<u32>>();
        assert_eq!(indents, vec![0, 1, 1, 0, 1, 1, 0]);
    }
}
