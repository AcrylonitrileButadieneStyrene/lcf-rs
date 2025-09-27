#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum EventPageConditionChunk {
    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for EventPageConditionChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Unknown { id, .. } => *id,
        }
    }
}
