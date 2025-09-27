use crate::{
    enums::ScrollType,
    helpers::Array,
    lmu::event::Event,
    raw::lmu::{LcfMapUnitChunk, RawLcfMapUnit},
};

pub mod event;
mod panorama;

pub use panorama::Panorama;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LcfMapUnit {
    pub chipset: Option<u32>,
    pub width: u32,
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
            chipset: None,
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

    #[error("invalid scroll type {0}")]
    InvalidScrollType(u32),
    #[error("invalid trigger type {0}")]
    InvalidTriggerType(u32),
    #[error("invalid priority type {0}")]
    InvalidPriorityType(u32),
    #[error("invalid animation type {0}")]
    InvalidAnimationType(u32),

    #[error("contained unknown event instruction data. chunk: {0} bytes: {1:?}")]
    UnknownEventInstructionData(u32, Vec<u8>),
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
                LcfMapUnitChunk::ChipSet(number) => value.chipset = Some(number.0),
                LcfMapUnitChunk::Width(number) => value.width = number.0,
                LcfMapUnitChunk::Height(number) => value.height = number.0,
                LcfMapUnitChunk::ScrollType(number) => {
                    value.scroll_type = ScrollType::from_repr(number.0)
                        .ok_or(LcfMapUnitReadError::InvalidScrollType(number.0))?;
                }
                LcfMapUnitChunk::PanoramaEnabled(number) => value.panorama.enabled = number.0 != 0,
                LcfMapUnitChunk::PanoramaFile(items) => value.panorama.file = Some(items.clone()),
                LcfMapUnitChunk::PanoramaHorizontalLoop(number) => {
                    value.panorama.horizontal_loop = number.0 != 0;
                }
                LcfMapUnitChunk::PanoramaVerticalLoop(number) => {
                    value.panorama.vertical_loop = number.0 != 0;
                }
                LcfMapUnitChunk::PanoramaHorizontalAutoScroll(number) => {
                    value.panorama.horizontal_auto_scroll = number.0 != 0;
                }
                LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(number) => {
                    value.panorama.horizontal_auto_scroll_speed = number.0 as i32;
                }
                LcfMapUnitChunk::PanoramaVerticalAutoScroll(number) => {
                    value.panorama.vertical_auto_scroll = number.0 != 0;
                }
                LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(number) => {
                    value.panorama.vertical_auto_scroll_speed = number.0 as i32;
                }
                LcfMapUnitChunk::Lower(items) => value.lower = items,
                LcfMapUnitChunk::Upper(items) => value.upper = items,
                LcfMapUnitChunk::Events { chunks } => {
                    value.events = chunks
                        .into_iter()
                        .map(|(id, chunks)| Event::from_chunks(id.0, chunks.inner_vec))
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

        if let Some(chipset) = val.chipset {
            chunks.push(LcfMapUnitChunk::ChipSet(chipset.into()));
        }

        if val.width != 20 {
            chunks.push(LcfMapUnitChunk::Width(val.width.into()));
        }

        if val.height != 15 {
            chunks.push(LcfMapUnitChunk::Height(val.height.into()));
        }

        chunks.push(LcfMapUnitChunk::ScrollType(
            (val.scroll_type.clone() as u32).into(),
        ));

        val.panorama.write_chunks(&mut chunks);
        chunks.push(LcfMapUnitChunk::Lower(val.lower.clone()));
        chunks.push(LcfMapUnitChunk::Upper(val.upper.clone()));
        chunks.push(LcfMapUnitChunk::Events {
            chunks: val.events.iter().map(Event::to_chunks).collect(),
        });

        chunks.push(LcfMapUnitChunk::SaveTime(val.save_time.into()));

        Self(Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        })
    }
}
