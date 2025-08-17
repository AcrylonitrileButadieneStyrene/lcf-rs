use crate::helpers::Number;

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
pub struct Instruction {
    pub code: Number,
    pub indent: Number,

    #[bw(calc = Number(string.len() as u32))]
    string_length: Number,
    #[br(count = string_length.0)]
    pub string: Vec<u8>,

    #[bw(calc = Number(args.len() as u32))]
    arg_count: Number,
    #[br(count = arg_count.0)]
    pub args: Vec<Number>,
}
