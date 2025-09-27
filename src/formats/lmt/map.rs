use serde::{Deserialize, Serialize};

use crate::{
    helpers::{Array, Chunk, Number},
    lmt::{BGM, LcfMapTreeReadError},
    raw::lmt::{bgm::MapBGMChunk, map::MapChunk},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
    pub name: Vec<u8>,
    pub parent: u32,
    pub indentation: u32,
    pub r#type: MapType,
    pub horizontal_scroll_bar: u32,
    pub vertical_scroll_bar: u32,
    pub expanded: bool,
    pub bgm: Setting,
    pub bgm_data: BGM,
    pub background: Setting,
    pub background_file: Vec<u8>,
    pub teleport: Setting,
    pub escape: Setting,
    pub save: Setting,
    pub enemies: Vec<u8>,
    pub encounter_rate: u32,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            name: Vec::new(),
            parent: 0,
            indentation: 0,
            r#type: MapType::default(),
            horizontal_scroll_bar: 0,
            vertical_scroll_bar: 0,
            expanded: false,
            bgm: Setting::default(),
            bgm_data: BGM::default(),
            background: Setting::default(),
            background_file: Vec::new(),
            teleport: Setting::default(),
            escape: Setting::default(),
            save: Setting::default(),
            enemies: Vec::default(),
            encounter_rate: 25,
        }
    }
}

impl Map {
    pub fn from_chunks(id: u32, chunks: Vec<Chunk<MapChunk>>) -> Result<Self, LcfMapTreeReadError> {
        let mut map = Self::default();

        for chunk in chunks {
            match chunk.data {
                MapChunk::Name(bytes) => map.name = bytes,
                MapChunk::Parent(val) => map.parent = val.0,
                MapChunk::Indentation(val) => map.indentation = val.0,
                MapChunk::Type(val) => {
                    map.r#type = match val.0 {
                        0 => MapType::Game,
                        1 => MapType::Map,
                        2 => continue,
                        x => return Err(LcfMapTreeReadError::InvalidMapType(x, id)),
                    }
                }
                MapChunk::HorizontalScrollBar(val) => {
                    map.horizontal_scroll_bar = val.0;
                }
                MapChunk::VerticalScrollBar(val) => map.vertical_scroll_bar = val.0,
                MapChunk::Expanded(val) => map.expanded = val.0 != 0,
                MapChunk::BGM(val) => map.bgm = (val, "BGM", id).try_into()?,
                MapChunk::BGMData(chunks) => {
                    let mut bgm = BGM::default();
                    for chunk in chunks.inner_vec {
                        match chunk.data {
                            MapBGMChunk::FileName(bytes) => bgm.file = bytes,
                            MapBGMChunk::FadeInTime(val) => bgm.fade_in_time = val.0,
                            MapBGMChunk::Volume(val) => bgm.volume = val.0,
                            MapBGMChunk::Tempo(val) => bgm.tempo = val.0,
                            MapBGMChunk::Balance(val) => bgm.balance = val.0,
                            MapBGMChunk::Unknown { id, bytes } => {
                                return Err(LcfMapTreeReadError::UnknownBGMData(id, bytes));
                            }
                        }
                    }

                    map.bgm_data = bgm;
                }
                MapChunk::Background(val) => {
                    map.background = (val, "background", id).try_into()?;
                }
                MapChunk::BackgroundFile(bytes) => map.background_file = bytes,
                MapChunk::Teleport(val) => {
                    map.teleport = (val, "teleport", id).try_into()?;
                }
                MapChunk::Escape(val) => map.escape = (val, "escape", id).try_into()?,
                MapChunk::Save(val) => map.save = (val, "save", id).try_into()?,
                MapChunk::EncounterEnemyGroup(bytes) => map.enemies = bytes,
                MapChunk::EnemyAppearStep(number) => map.encounter_rate = number.0,
                MapChunk::AreaRange {
                    begin_x,
                    begin_y,
                    end_x,
                    end_y,
                } => {
                    if begin_x != 0 || begin_y != 0 && end_x != 0 && end_y != 0 {
                        map.r#type = MapType::Area {
                            begin_x,
                            begin_y,
                            end_x,
                            end_y,
                        };
                    }
                }
                MapChunk::Unknown { id, bytes } => {
                    return Err(LcfMapTreeReadError::UnknownData(id, bytes));
                }
            }
        }

        Ok(map)
    }

    pub fn to_chunks(&self) -> Array<Chunk<MapChunk>> {
        let mut chunks = Vec::new();
        chunks.push(MapChunk::Name(self.name.clone()));

        if self.parent != 0 {
            chunks.push(MapChunk::Parent(Number(self.parent)));
        }

        if self.indentation != 0 {
            chunks.push(MapChunk::Indentation(Number(self.indentation)));
        }

        chunks.push(MapChunk::Type(Number(match self.r#type {
            MapType::Game => 0,
            MapType::Map => 1,
            MapType::Area { .. } => 2,
        })));

        if self.horizontal_scroll_bar != 0 {
            chunks.push(MapChunk::HorizontalScrollBar(Number(
                self.horizontal_scroll_bar,
            )));
        }

        if self.vertical_scroll_bar != 0 {
            chunks.push(MapChunk::VerticalScrollBar(Number(
                self.vertical_scroll_bar,
            )));
        }

        if self.expanded {
            chunks.push(MapChunk::Expanded(Number(1)));
        }

        chunks.push(MapChunk::BGM(Number(self.bgm as u32)));
        chunks.push(MapChunk::BGMData(self.bgm_data.to_chunks()));
        chunks.push(MapChunk::Background(Number(self.background as u32)));
        chunks.push(MapChunk::Teleport(Number(self.teleport as u32)));
        chunks.push(MapChunk::Escape(Number(self.escape as u32)));
        chunks.push(MapChunk::Save(Number(self.save as u32)));
        chunks.push(MapChunk::EncounterEnemyGroup(self.enemies.clone()));

        if self.encounter_rate != 25 || matches!(self.r#type, MapType::Game) {
            chunks.push(MapChunk::EnemyAppearStep(Number(self.encounter_rate)));
        }

        chunks.push(match self.r#type {
            MapType::Game | MapType::Map => MapChunk::AreaRange {
                begin_x: 0,
                begin_y: 0,
                end_x: 0,
                end_y: 0,
            },
            MapType::Area {
                begin_x,
                begin_y,
                end_x,
                end_y,
            } => MapChunk::AreaRange {
                begin_x,
                begin_y,
                end_x,
                end_y,
            },
        });

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Setting {
    #[default]
    Inherit = 0,
    SetByEvent = 1,
    Specified = 2,
}

impl TryFrom<(Number, &'static str, u32)> for Setting {
    type Error = LcfMapTreeReadError;

    fn try_from(value: (Number, &'static str, u32)) -> Result<Self, Self::Error> {
        Ok(match value.0.0 {
            0 => Self::Inherit,
            1 => Self::SetByEvent,
            2 => Self::Specified,
            x => return Err(LcfMapTreeReadError::InvalidSetting(x, value.1, value.2)),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapType {
    Game,
    #[default]
    Map,
    Area {
        begin_x: u32,
        begin_y: u32,
        end_x: u32,
        end_y: u32,
    },
}
