use crate::helpers::{Array, Array2D, Chunk, Number};

pub mod bgm;
pub mod map;
pub mod start;

#[binrw::binrw]
#[brw(magic(b"\x0aLcfMapTree"), little)]
#[derive(Clone, Debug)]
pub struct RawLcfMapTree {
    pub maps: Array2D<map::MapChunk>,

    #[br(temp)]
    #[bw(calc = Number(order.len() as u32))]
    order_count: Number,

    #[br(count = order_count.0)]
    pub order: Vec<Number>,

    pub active: Number,
    pub start: Array<Chunk<start::StartChunk>>,
}
