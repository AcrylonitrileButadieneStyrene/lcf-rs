use binrw::BinWrite as _;

use crate::helpers::Number;

pub trait ToChunkID {
    fn id(&self) -> u32;
}

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk<T: ChunkTraitBounds> {
    #[br(temp)]
    #[bw(calc = Number(data.id()))]
    pub id: Number,

    #[br(temp)]
    #[bw(ignore)]
    pub read_length: Number,

    #[br(temp, count = read_length.0)]
    #[bw(ignore)]
    read_bytes: Vec<u8>,

    #[br(calc = {
        let mut cursor = std::io::Cursor::new(&read_bytes);
        let value = T::read_args(&mut cursor, (id.0, read_length.0))?;
        debug_assert_eq!(cursor.position() as u32, read_length.0, "chunk {}", id.0);
        value
    })]
    #[bw(write_with = write_data)]
    pub data: T,
}

#[allow(clippy::ptr_arg)]
fn write_data<W, T>(
    data: &T,
    mut writer: &mut W,
    _endian: binrw::Endian,
    _args: (),
) -> binrw::BinResult<()>
where
    W: std::io::Write + std::io::Seek,
    T: for<'a> binrw::BinWrite<Args<'a>: Default> + binrw::meta::WriteEndian,
{
    let mut cursor = std::io::Cursor::new(Vec::new());
    data.write(&mut cursor)?;
    let bytes = cursor.into_inner();

    Number(bytes.len() as u32).write(&mut writer)?;
    bytes.write(&mut writer)?;

    Ok(())
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
