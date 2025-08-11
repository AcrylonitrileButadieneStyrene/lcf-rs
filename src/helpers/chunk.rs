use crate::helpers::Number;

pub trait ToChunkID {
    fn id(&self) -> Number;
}

#[binrw::binrw]
#[derive(Clone, Debug)]
pub struct Chunk<T>
where
    T: for<'a> binrw::BinRead<Args<'a> = (Number, Number)>
        + binrw::meta::ReadEndian
        + binrw::BinWrite
        + ToChunkID,
{
    #[br(temp)]
    #[bw(calc = data.id())]
    pub id: Number,

    #[br(temp)]
    #[bw(calc = Number(bytes.len() as u32))]
    pub length: Number,

    #[br(count = length.0)]
    #[bw(write_with = write_bytes)]
    bytes: Vec<u8>,

    #[br(calc = {
        let mut cursor = std::io::Cursor::new(&bytes);
        T::read_args(&mut cursor, (id, length))?
    })]
    #[bw(ignore)]
    pub data: T,
}

#[allow(clippy::ptr_arg)]
fn write_bytes<W: std::io::Write>(
    bytes: &Vec<u8>,
    writer: &mut W,
    _endian: binrw::Endian,
    _args: (),
) -> binrw::BinResult<()> {
    writer.write_all(bytes)?;
    Ok(())
}
