use crate::helpers::{Array, Chunk, Number};

pub mod bgm;
pub mod map;
pub mod start;

#[binrw::binrw]
#[brw(magic(b"\x0aLcfMapTree"), little)]
#[derive(Debug)]
pub struct LcfMapTree {
    #[br(temp)]
    #[bw(calc = Number(maps.len() as u32))]
    maps_count: Number,

    #[br(count = maps_count.0)]
    pub maps: Vec<(Number, Array<Chunk<map::MapChunk>>)>,

    #[br(temp)]
    #[bw(calc = Number(order.len() as u32))]
    order_count: Number,

    #[br(count = order_count.0)]
    pub order: Vec<Number>,

    pub active: Number,
    pub start: Array<Chunk<start::StartChunk>>,
}
