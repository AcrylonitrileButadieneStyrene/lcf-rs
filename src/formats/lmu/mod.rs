use crate::{
    enums::ScrollType,
    helpers::{Array, Number},
    lmu::event::Event,
    raw::lmu::{LcfMapUnitChunk, RawLcfMapUnit},
};

pub mod event;
mod generator;
mod panorama;

pub use generator::Generator;
pub use panorama::{Panorama, PanoramaOptions};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LcfMapUnit {
    /// ID of the tileset to use for the upper and lower layers.
    /// - Default: 1.
    pub chipset: u32,
    /// Width of the map in tiles.
    /// - Minimum: 20 (size of screen).
    /// - Maximum: 500.
    ///
    /// Being outside of this range causes no obvious problems.
    pub width: u32,
    /// Height of the map in tiles.
    /// - Minimum: 15 (size of screen).
    /// - Maximum: 500.
    ///
    /// Being outside of this range causes no obvious problems.
    pub height: u32,
    pub scroll_type: ScrollType,
    pub panorama: Panorama,
    pub generator: Generator,
    pub top_level: bool,
    /// Length must match the [`Self::width`] * [`Self::height`]
    pub lower: Vec<u16>,
    /// Length must match the [`Self::width`] * [`Self::height`]
    pub upper: Vec<u16>,
    pub events: Vec<Event>,
    pub save_time: u32,

    pub is_r2k3: bool,
}

impl Default for LcfMapUnit {
    fn default() -> Self {
        Self {
            chipset: 1,
            width: 20,
            height: 15,
            scroll_type: ScrollType::default(),
            panorama: Panorama::default(),
            generator: Generator::default(),
            lower: vec![0; 20 * 15],
            upper: vec![10000; 20 * 15],
            events: Vec::new(),
            top_level: false,
            save_time: 2,
            is_r2k3: false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LcfMapUnitReadError {
    #[error("decode error: {0}")]
    Decode(#[from] binrw::Error),

    #[error("invalid direction {0}")]
    InvalidDirection(u32),
    #[error("invalid scroll type {0}")]
    InvalidScrollType(u32),
    #[error("invalid trigger type {0}")]
    InvalidTriggerType(u32),
    #[error("invalid priority type {0}")]
    InvalidPriorityType(u32),
    #[error("invalid animation type {0}")]
    InvalidAnimationType(u32),

    #[error("missing event condition flags")]
    MissingEventConditionFlags,

    #[error("contained unknown event instruction data. chunk: {0} bytes: {1:?}")]
    UnknownEventInstructionData(u32, Vec<u8>),
    #[error("contained unknown event move route data. chunk: {0} bytes: {1:?}")]
    UnknownEventMoveRouteData(u32, Vec<u8>),
    #[error("contained unknown event condition data. chunk: {0} bytes: {1:?}")]
    UnknownEventConditionData(u32, Vec<u8>),
    #[error("contained unknown event page data. chunk: {0} bytes: {1:?}")]
    UnknownEventPageData(u32, Vec<u8>),
    #[error("contained unknown event data. chunk: {0} bytes: {1:?}")]
    UnknownEventData(u32, Vec<u8>),
    #[error("contained unknown data. chunk: {0} bytes: {1:?}")]
    UnknownData(u32, Vec<u8>),
}

impl TryFrom<RawLcfMapUnit> for LcfMapUnit {
    type Error = LcfMapUnitReadError;

    #[expect(clippy::cast_possible_wrap)]
    fn try_from(raw: RawLcfMapUnit) -> Result<Self, Self::Error> {
        let mut value = Self::default();

        for chunk in raw.0.inner_vec {
            match chunk.data {
                LcfMapUnitChunk::ChipSet(number) => value.chipset = number.0,
                LcfMapUnitChunk::Width(number) => value.width = number.0,
                LcfMapUnitChunk::Height(number) => value.height = number.0,
                LcfMapUnitChunk::ScrollType(number) => {
                    value.scroll_type = ScrollType::try_from(number.0)
                        .map_err(|_| LcfMapUnitReadError::InvalidScrollType(number.0))?;
                }
                LcfMapUnitChunk::PanoramaEnabled(number) => value.panorama.enabled = number.0 != 0,
                LcfMapUnitChunk::PanoramaFile(items) => value.panorama.file = Some(items.clone()),
                LcfMapUnitChunk::PanoramaHorizontalLoop(number) => {
                    if number.0 != 0 && matches!(value.panorama.horizontal, PanoramaOptions::NoLoop)
                    {
                        value.panorama.horizontal = PanoramaOptions::NoAutoscroll;
                    }
                }
                LcfMapUnitChunk::PanoramaHorizontalAutoScroll(number) => {
                    if number.0 != 0
                        && matches!(
                            value.panorama.horizontal,
                            PanoramaOptions::NoLoop | PanoramaOptions::NoAutoscroll
                        )
                    {
                        value.panorama.horizontal = PanoramaOptions::Autoscroll(0);
                    }
                }
                LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(number) => {
                    value.panorama.horizontal = PanoramaOptions::Autoscroll(number.0 as i32);
                }
                LcfMapUnitChunk::PanoramaVerticalLoop(number) => {
                    if number.0 != 0 && matches!(value.panorama.vertical, PanoramaOptions::NoLoop) {
                        value.panorama.vertical = PanoramaOptions::NoAutoscroll;
                    }
                }
                LcfMapUnitChunk::PanoramaVerticalAutoScroll(number) => {
                    if number.0 != 0
                        && matches!(
                            value.panorama.vertical,
                            PanoramaOptions::NoLoop | PanoramaOptions::NoAutoscroll
                        )
                    {
                        value.panorama.vertical = PanoramaOptions::Autoscroll(0);
                    }
                }
                LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(number) => {
                    value.panorama.vertical = PanoramaOptions::Autoscroll(number.0 as i32);
                }
                LcfMapUnitChunk::GeneratorEnabled(number) => {
                    value.generator.enabled = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorMode(number) => value.generator.mode = number.0,
                LcfMapUnitChunk::TopLevel(number) => value.top_level = number.0 != 0,
                LcfMapUnitChunk::GeneratorTiles(number) => value.generator.tiles = number.0,
                LcfMapUnitChunk::GeneratorWidth(number) => value.generator.width = number.0,
                LcfMapUnitChunk::GeneratorHeight(number) => {
                    value.generator.height = number.0;
                }
                LcfMapUnitChunk::GeneratorSurround(number) => {
                    value.generator.surround = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorUseWallUpper(number) => {
                    value.generator.use_wall_upper = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorUseFloorB(number) => {
                    value.generator.use_floor_b = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorUseFloorC(number) => {
                    value.generator.use_floor_c = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorUseObstacleB(number) => {
                    value.generator.use_obstacle_b = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorUseObstacleC(number) => {
                    value.generator.use_obstacle_c = number.0 != 0
                }
                LcfMapUnitChunk::GeneratorX(items) => value.generator.x = items,
                LcfMapUnitChunk::GeneratorY(items) => value.generator.y = items,
                LcfMapUnitChunk::GeneratorIDs(items) => value.generator.ids = items,
                LcfMapUnitChunk::Lower(items) => value.lower = items,
                LcfMapUnitChunk::Upper(items) => value.upper = items,
                LcfMapUnitChunk::Events(chunks) => {
                    value.events = chunks
                        .inner_vec
                        .into_iter()
                        .map(|(id, chunks)| {
                            Event {
                                id: id.0,
                                ..Default::default()
                            }
                            .with_chunks(chunks.inner_vec)
                        })
                        .try_collect()?;
                }
                LcfMapUnitChunk::SaveTimeA(number) => {
                    value.is_r2k3 = true;
                    value.save_time = number.0;
                }
                LcfMapUnitChunk::SaveTimeB(number) => value.save_time = number.0,
                LcfMapUnitChunk::Unknown { id, bytes } => {
                    return Err(LcfMapUnitReadError::UnknownData(id, bytes));
                }
            }
        }

        Ok(value)
    }
}

impl From<&LcfMapUnit> for RawLcfMapUnit {
    fn from(val: &LcfMapUnit) -> Self {
        let mut chunks = Vec::new();

        if val.chipset != 1 {
            chunks.push(LcfMapUnitChunk::ChipSet(val.chipset.into()));
        }

        if val.width != 20 {
            chunks.push(LcfMapUnitChunk::Width(val.width.into()));
        }

        if val.height != 15 {
            chunks.push(LcfMapUnitChunk::Height(val.height.into()));
        }

        chunks.push(LcfMapUnitChunk::ScrollType((val.scroll_type as u32).into()));

        val.panorama.write_chunks(&mut chunks);
        val.generator.write_chunks(&mut chunks, val.is_r2k3);

        if val.top_level {
            chunks.push(LcfMapUnitChunk::TopLevel(Number(1)));
        }

        let area = (val.width * val.height) as usize;
        let mut lower = val.lower.clone();
        let mut upper = val.upper.clone();
        lower.resize(area, 0);
        upper.resize(area, 10000);
        chunks.push(LcfMapUnitChunk::Lower(lower));
        chunks.push(LcfMapUnitChunk::Upper(upper));

        chunks.push(LcfMapUnitChunk::Events(
            val.events.iter().map(Event::to_chunks).collect(),
        ));

        if val.is_r2k3 {
            chunks.push(LcfMapUnitChunk::SaveTimeA(val.save_time.into()));
        } else {
            chunks.push(LcfMapUnitChunk::SaveTimeB(val.save_time.into()));
        }

        Self(Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        })
    }
}
