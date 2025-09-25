use crate::{helpers::Number, raw::lmu::LcfMapUnitChunk};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Panorama {
    pub enabled: bool,
    pub file: Option<Vec<u8>>,
    pub horizontal_loop: bool,
    pub vertical_loop: bool,
    pub horizontal_auto_scroll: bool,
    pub horizontal_auto_scroll_speed: i32,
    pub vertical_auto_scroll: bool,
    pub vertical_auto_scroll_speed: i32,
}

impl Panorama {
    #[expect(clippy::cast_sign_loss)]
    pub fn write_chunks(&self, chunks: &mut Vec<LcfMapUnitChunk>) {
        const TRUE: Number = Number(1);

        if self.enabled {
            chunks.push(LcfMapUnitChunk::PanoramaEnabled(TRUE));
        }

        if let Some(bytes) = &self.file {
            chunks.push(LcfMapUnitChunk::PanoramaFile(bytes.clone()));
        }

        if self.horizontal_loop {
            chunks.push(LcfMapUnitChunk::PanoramaHorizontalLoop(TRUE));
        }

        if self.vertical_loop {
            chunks.push(LcfMapUnitChunk::PanoramaVerticalLoop(TRUE));
        }

        if self.horizontal_auto_scroll {
            chunks.push(LcfMapUnitChunk::PanoramaHorizontalAutoScroll(TRUE));
        }

        if self.horizontal_auto_scroll_speed != 0 {
            chunks.push(LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(Number(
                self.horizontal_auto_scroll_speed as u32,
            )));
        }

        if self.vertical_auto_scroll {
            chunks.push(LcfMapUnitChunk::PanoramaVerticalAutoScroll(TRUE));
        }

        if self.vertical_auto_scroll_speed != 0 {
            chunks.push(LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(Number(
                self.vertical_auto_scroll_speed as u32,
            )));
        }
    }
}
