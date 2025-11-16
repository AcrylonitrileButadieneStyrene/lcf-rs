use crate::{
    enums::ScrollType,
    helpers::Array,
    lmu::event::Event,
    raw::lmu::{LcfMapUnitChunk, RawLcfMapUnit},
};

pub mod event;
mod panorama;

pub use panorama::{Panorama, PanoramaOptions};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LcfMapUnit {
    /// ID of the tileset to use for the upper and lower layers. Defaults to 1.
    pub chipset: u32,
    /// Width of the map in tiles. Minimum: 20 (size of screen). Maximum: 500.
    pub width: u32,
    /// Height of the map in tiles. Minimum: 15 (size of screen). Maximum: 500.
    pub height: u32,
    pub scroll_type: ScrollType,
    pub panorama: Panorama,
    pub lower: Vec<u16>,
    pub upper: Vec<u16>,
    pub events: Vec<Event>,
    pub save_time: u32,
}

impl Default for LcfMapUnit {
    fn default() -> Self {
        Self {
            chipset: 1,
            width: 20,
            height: 15,
            scroll_type: ScrollType::default(),
            panorama: Panorama::default(),
            lower: vec![0; 20 * 15],
            upper: vec![10000; 20 * 15],
            events: Vec::new(),
            save_time: 2,
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

                LcfMapUnitChunk::SaveTime(number) => value.save_time = number.0,
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
        assert!(val.width >= 20, "width too small");
        assert!(val.width <= 500, "width too large");
        assert!(val.height >= 15, "height too small");
        assert!(val.height <= 500, "height too large");
        assert_eq!(
            val.upper.len(),
            (val.width * val.height) as usize,
            "incorrect amount of upper tiles"
        );

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
        chunks.push(LcfMapUnitChunk::Lower(val.lower.clone()));
        chunks.push(LcfMapUnitChunk::Upper(val.upper.clone()));
        chunks.push(LcfMapUnitChunk::Events(
            val.events.iter().map(Event::to_chunks).collect(),
        ));

        chunks.push(LcfMapUnitChunk::SaveTime(val.save_time.into()));

        Self(Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        })
    }
}
