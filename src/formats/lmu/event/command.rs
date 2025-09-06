use std::io::Seek as _;

use crate::{helpers::Number, lmu::event::instruction::Instruction};
use binrw::BinWrite as _;
use byteorder::ReadBytesExt;

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
pub struct Command {
    #[bw(calc = instruction.opcode())]
    opcode: Number,
    pub indent: Number,

    #[bw(calc = Number(string.len() as u32))]
    string_length: Number,

    #[br(count = string_length.0)]
    pub string: Vec<u8>,

    #[bw(ignore)]
    arg_count: Number,

    #[br(parse_with = read_args_bytes, args(arg_count.0))]
    #[bw(ignore)]
    args_bytes: Vec<u8>,

    #[br(calc = {
        let mut cursor = std::io::Cursor::new(&args_bytes);
        let result = Instruction::read_args(&mut cursor, (opcode,))?;
        if cursor.position() < cursor.get_ref().len() as u64 {
            cursor.seek(std::io::SeekFrom::Start(0))?;
            let args = binrw::helpers::until_eof(&mut cursor, binrw::Endian::Little, ())?;
            Instruction::Unknown { opcode, args }
        } else {
            result
        }
    })]
    #[bw(write_with = write_instruction)]
    pub instruction: Instruction,
}

#[binrw::parser(reader)]
fn read_args_bytes(mut count: u32) -> Result<Vec<u8>, binrw::Error> {
    let mut bytes = Vec::with_capacity(count as usize * 5);

    while count > 0 {
        let byte = reader.read_u8()?;
        if byte & 0b1000_0000 == 0 {
            count -= 1;
        }

        bytes.push(byte);
    }

    Ok(bytes)
}

#[binrw::writer(writer, endian)]
fn write_instruction(instruction: &Instruction) -> Result<(), binrw::Error> {
    let mut buf = std::io::Cursor::new(Vec::new());

    instruction.write_options(&mut buf, binrw::Endian::Little, ())?;
    buf.rewind()?;

    let args: Vec<Number> = binrw::helpers::until_eof(&mut buf, binrw::Endian::Little, ())?;
    let length = args.len();

    buf.into_inner().write_options(writer, endian, ())?;
    Number(length as u32).write_options(writer, endian, ())?;

    Ok(())
}
