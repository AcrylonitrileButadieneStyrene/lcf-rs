use std::io::Seek as _;

use binrw::BinWrite as _;

use crate::{helpers::Number, raw::lmu::event::instruction::Instruction};

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[brw(little)]
pub struct Command {
    #[br(temp)]
    #[bw(calc = Number(instruction.opcode()))]
    opcode: Number,

    #[br(temp)]
    #[bw(calc = Number(*indent))]
    __indent: Number,

    #[br(calc = __indent.0)]
    #[bw(ignore)]
    pub indent: u32,

    #[bw(calc = Number(string.len() as u32))]
    string_length: Number,

    #[br(count = string_length.0)]
    pub string: Vec<u8>,

    #[br(temp)]
    #[bw(ignore)]
    arg_count: Number,

    #[br(temp, count = arg_count.0)]
    #[bw(ignore)]
    args: Vec<Number>,

    #[br(calc = {
        let mut cursor = std::io::Cursor::new(
            args.iter()
                .flat_map(|arg| arg.0.to_ne_bytes())
                .collect::<Vec<u8>>(),
        );
        if let Ok(instruction) = Instruction::read_args(&mut cursor, (opcode.0,))
            && cursor.position() >= cursor.get_ref().len() as u64
        {
            instruction
        } else {
            cursor.seek(std::io::SeekFrom::Start(0))?;
            Instruction::Unknown {
                opcode: opcode.0,
                args: args.iter().map(|arg| arg.0).collect(),
            }
        }
    })]
    #[bw(write_with = write_instruction)]
    pub instruction: Instruction,
}

#[binrw::writer(writer)]
fn write_instruction(instruction: &Instruction) -> Result<(), binrw::Error> {
    let mut buf = std::io::Cursor::new(Vec::new());
    instruction.write(&mut buf)?;
    let nums: Vec<u32> = buf
        .into_inner()
        .into_iter()
        .array_chunks()
        .map(u32::from_ne_bytes)
        .collect();

    Number(nums.len() as u32).write(writer)?;
    nums.iter()
        .copied()
        .map(Number)
        .collect::<Vec<Number>>()
        .write(writer)?;

    Ok(())
}
