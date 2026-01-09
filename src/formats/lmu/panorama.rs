use crate::{helpers::Number, raw::lmu::LcfMapUnitChunk};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Panorama {
    pub enabled: bool,
    pub file: Option<Vec<u8>>,
    pub horizontal: PanoramaOptions,
    pub vertical: PanoramaOptions,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PanoramaOptions {
    /// Fixed to the screen (looping disabled).
    #[default]
    NoLoop,
    /// Can move and loops (autoscroll disabled).
    NoAutoscroll,
    /// Autoscroll is enabled with speed `X`. `X` can be `0`.
    Autoscroll(i32),
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

        match self.horizontal {
            PanoramaOptions::NoLoop => (),
            PanoramaOptions::NoAutoscroll => {
                chunks.push(LcfMapUnitChunk::PanoramaHorizontalLoop(TRUE));
            }
            PanoramaOptions::Autoscroll(speed) => {
                chunks.push(LcfMapUnitChunk::PanoramaHorizontalLoop(TRUE));
                chunks.push(LcfMapUnitChunk::PanoramaHorizontalAutoScroll(TRUE));
                chunks.push(LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(Number(
                    speed as u32,
                )));
            }
        };

        match self.vertical {
            PanoramaOptions::NoLoop => (),
            PanoramaOptions::NoAutoscroll => {
                chunks.push(LcfMapUnitChunk::PanoramaVerticalLoop(TRUE));
            }
            PanoramaOptions::Autoscroll(speed) => {
                chunks.push(LcfMapUnitChunk::PanoramaVerticalLoop(TRUE));
                chunks.push(LcfMapUnitChunk::PanoramaVerticalAutoScroll(TRUE));
                chunks.push(LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(Number(
                    speed as u32,
                )));
            }
        };
    }
}
