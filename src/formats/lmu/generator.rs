use crate::{helpers::Number, raw::lmu::LcfMapUnitChunk};

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
    #[expect(clippy::cast_sign_loss)]
    pub fn write_chunks(&self, chunks: &mut Vec<LcfMapUnitChunk>, is_r2k3: bool) {
        if !is_r2k3 {
            return;
        }

        chunks.push(LcfMapUnitChunk::GeneratorEnabled(Number(
            self.enabled as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorMode(Number(self.mode)));
        chunks.push(LcfMapUnitChunk::GeneratorTiles(Number(self.tiles)));
        chunks.push(LcfMapUnitChunk::GeneratorWidth(Number(self.width)));
        chunks.push(LcfMapUnitChunk::GeneratorHeight(Number(self.height)));
        chunks.push(LcfMapUnitChunk::GeneratorSurround(Number(
            self.surround as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorUseWallUpper(Number(
            self.use_wall_upper as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorUseFloorB(Number(
            self.use_floor_b as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorUseFloorC(Number(
            self.use_floor_c as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorUseObstacleB(Number(
            self.use_obstacle_b as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorUseObstacleC(Number(
            self.use_obstacle_c as u32,
        )));
        chunks.push(LcfMapUnitChunk::GeneratorX(self.x));
        chunks.push(LcfMapUnitChunk::GeneratorY(self.y));
        chunks.push(LcfMapUnitChunk::GeneratorIDs(self.ids.clone()));
    }
}
