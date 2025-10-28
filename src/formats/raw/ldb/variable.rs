#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum VariableChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for VariableChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name { .. } => 1,
            Self::Unknown { id, .. } => *id,
        }
    }
}
