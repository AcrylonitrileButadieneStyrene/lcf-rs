use crate::{helpers::Number, raw::lmu::LcfMapUnitChunk as Chunk};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Generator {
    pub enabled: bool,
    pub mode: u32,
    pub tiles: u32,
    pub width: u32,
    pub height: u32,
    pub surround: bool,
    pub use_wall_upper: bool,
    pub use_floor_b: bool,
    pub use_floor_c: bool,
    pub use_obstacle_b: bool,
    pub use_obstacle_c: bool,
    pub x: [u32; 9],
    pub y: [u32; 9],
    pub ids: Vec<u16>,
}

impl Generator {
    pub fn write_chunks(&self, chunks: &mut Vec<Chunk>) {
        let mut emit = emitter(chunks);
        emit(self.enabled, Chunk::GeneratorEnabled(Number(1)));
        emit(self.surround, Chunk::GeneratorSurround(Number(1)));
        emit(self.surround, Chunk::GeneratorSurround(Number(1)));
        emit(self.use_wall_upper, Chunk::GeneratorUseWallUpper(Number(1)));
        emit(self.use_floor_b, Chunk::GeneratorUseFloorB(Number(1)));
        emit(self.use_floor_c, Chunk::GeneratorUseFloorC(Number(1)));
        emit(self.use_obstacle_b, Chunk::GeneratorUseObstacleB(Number(1)));
        emit(self.use_obstacle_c, Chunk::GeneratorUseObstacleC(Number(1)));
        drop(emit);

        chunks.push(Chunk::GeneratorMode(Number(self.mode)));
        chunks.push(Chunk::GeneratorTiles(Number(self.tiles)));
        chunks.push(Chunk::GeneratorWidth(Number(self.width)));
        chunks.push(Chunk::GeneratorHeight(Number(self.height)));

        chunks.push(Chunk::GeneratorX(self.x));
        chunks.push(Chunk::GeneratorY(self.y));

        if !self.ids.is_empty() {
            chunks.push(Chunk::GeneratorIDs(self.ids.clone()));
        }
    }
}

fn emitter(chunks: &mut Vec<Chunk>) -> impl FnMut(bool, Chunk) {
    move |condition, value| {
        if condition {
            chunks.push(value);
        }
    }
}
