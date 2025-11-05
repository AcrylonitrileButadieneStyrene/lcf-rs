use crate::helpers::Number;

pub trait ChunkTraitBounds:
    for<'a> binrw::BinRead<Args<'a> = (u32, u32)>
    + binrw::meta::ReadEndian
    + for<'a> binrw::BinWrite<Args<'a>: Default>
    + binrw::meta::WriteEndian
    + ToChunkID
{
}

impl<T> ChunkTraitBounds for T where
    T: for<'a> binrw::BinRead<Args<'a> = (u32, u32)>
        + binrw::meta::ReadEndian
        + for<'a> binrw::BinWrite<Args<'a>: Default>
        + binrw::meta::WriteEndian
        + ToChunkID
{
}

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawChunk<T>
where
    T: Clone + From<u32> + Into<u32>,
{
    #[br(temp)]
    #[bw(calc = Number(id.clone().into()))]
    tag: Number,

    #[br(calc = tag.0.into())]
    #[bw(ignore)]
    pub id: T,

    #[br(temp)]
    #[bw(calc = Number(bytes.len() as u32))]
    length: Number,

    #[br(count = length.0)]
    pub bytes: Vec<u8>,
}

pub trait ToChunkID {
    fn id(&self) -> u32;
}

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk<T: ChunkTraitBounds> {
    #[br(temp)]
    #[bw(calc = {
        RawChunk { id: data.id(), bytes: {
            let mut cursor = std::io::Cursor::new(Vec::new());
            data.write(&mut cursor)?;
            cursor.into_inner()
        } }
    })]
    inner: RawChunk<u32>,

    #[br(calc = {
        let mut cursor = std::io::Cursor::new(&inner.bytes);
        let value = T::read_args(&mut cursor, (inner.id, inner.bytes.len() as u32))?;
        debug_assert_eq!(cursor.position(), inner.bytes.len() as u64, "chunk {}", inner.id);
        value
    })]
    #[bw(ignore)]
    pub data: T,
}

impl<T: ChunkTraitBounds> From<T> for Chunk<T> {
    fn from(value: T) -> Self {
        Self { data: value }
    }
}

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum UnknownChunk {
    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl ToChunkID for UnknownChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Unknown { id, .. } => *id,
        }
    }
}
