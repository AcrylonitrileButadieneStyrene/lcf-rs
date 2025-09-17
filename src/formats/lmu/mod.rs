use crate::{
    helpers::{Array, Number},
    raw::lmu::{LcfMapUnitChunk, RawLcfMapUnit, event::EventChunk},
};

mod panorama;
mod scroll_type;

pub use panorama::{Panorama, PanoramaBuilder, PanoramaBuilderError};
pub use scroll_type::ScrollType;

#[derive(Clone, Debug, PartialEq, Eq, derive_builder::Builder)]
#[builder(default, setter(strip_option))]
pub struct LcfMapUnit {
    pub chipset: Option<u32>,
    #[builder(default = "20")]
    pub width: u32,
    #[builder(default = "15")]
    pub height: u32,
    pub scroll_type: ScrollType,
    pub panorama: Panorama,
    pub lower: Vec<u16>,
    pub upper: Vec<u16>,
    pub events: Vec<(Number, Array<crate::helpers::Chunk<EventChunk>>)>,
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
    #[error("build error: {0}")]
    Build(#[from] LcfMapUnitBuilderError),
    #[error("panorama build error: {0}")]
    PanoramaBuild(#[from] PanoramaBuilderError),

    #[error("invalid scroll type {0}")]
    InvalidScrollType(u32),
    #[error("contained unknown data. chunk: {0} bytes: {1:?}")]
    UnknownData(u32, Vec<u8>),
}

impl TryFrom<RawLcfMapUnit> for LcfMapUnit {
    type Error = LcfMapUnitReadError;

    fn try_from(value: RawLcfMapUnit) -> Result<Self, Self::Error> {
        let mut builder = LcfMapUnitBuilder::default();
        let mut panorama_builder = PanoramaBuilder::default();

        for chunk in value.0.inner_vec {
            match chunk.data {
                LcfMapUnitChunk::ChipSet(number) => drop(builder.chipset(number.0)),
                LcfMapUnitChunk::Width(number) => drop(builder.width(number.0)),
                LcfMapUnitChunk::Height(number) => drop(builder.height(number.0)),
                LcfMapUnitChunk::ScrollType(number) => {
                    builder.scroll_type(number.0.try_into()?);
                }
                LcfMapUnitChunk::PanoramaEnabled(_)
                | LcfMapUnitChunk::PanoramaFile(_)
                | LcfMapUnitChunk::PanoramaHorizontalLoop(_)
                | LcfMapUnitChunk::PanoramaVerticalLoop(_)
                | LcfMapUnitChunk::PanoramaHorizontalAutoScroll(_)
                | LcfMapUnitChunk::PanoramaHorizontalAutoScrollSpeed(_)
                | LcfMapUnitChunk::PanoramaVerticalAutoScroll(_)
                | LcfMapUnitChunk::PanoramaVerticalAutoScrollSpeed(_) => {
                    panorama_builder.from_chunk(&chunk.data);
                }
                LcfMapUnitChunk::Lower(items) => drop(builder.lower(items)),
                LcfMapUnitChunk::Upper(items) => drop(builder.upper(items)),
                LcfMapUnitChunk::Events { chunks } => drop(builder.events(chunks)),
                LcfMapUnitChunk::SaveTime(number) => drop(builder.save_time(number.0)),
                LcfMapUnitChunk::Unknown { id, bytes } => {
                    Err(LcfMapUnitReadError::UnknownData(id.0, bytes))?;
                }
            }
        }

        builder.panorama(panorama_builder.build()?);
        Ok(builder.build()?)
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
            chunks: val.events.clone(),
        });

        chunks.push(LcfMapUnitChunk::SaveTime(val.save_time.into()));

        Self(Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        })
    }
}
