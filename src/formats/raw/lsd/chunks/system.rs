use crate::helpers::Number;

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum SaveSystemChunk {
    #[br(pre_assert(id == 31))]
    SwitchesSize(Number),
    #[br(pre_assert(id == 32))]
    Switches(#[br(parse_with = binrw::helpers::until_eof)] Vec<Number>),
    #[br(pre_assert(id == 33))]
    VariablesSize(Number),
    #[br(pre_assert(id == 34))]
    Variables(#[br(parse_with = binrw::helpers::until_eof)] Vec<i32>),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for SaveSystemChunk {
    fn id(&self) -> u32 {
        match self {
            Self::SwitchesSize(_) => 31,
            Self::Switches(_) => 32,
            Self::VariablesSize(_) => 33,
            Self::Variables(_) => 34,
            Self::Unknown { id, .. } => *id,
        }
    }
}
