use crate::{
    helpers::{Array, Chunk, Number},
    raw::shared::MusicChunk,
};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
#[br(import(id: u32, length: u32))]
pub enum SystemChunk {
    #[br(pre_assert(id == 10))]
    ID(Number),
    #[br(pre_assert(id == 11))]
    BoatGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 12))]
    ShipGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 13))]
    AirshipGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 14))]
    BoatGraphicIndex(Number),
    #[br(pre_assert(id == 15))]
    ShipGraphicIndex(Number),
    #[br(pre_assert(id == 16))]
    AirshipGraphicIndex(Number),
    #[br(pre_assert(id == 17))]
    TitleGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 18))]
    GameOverGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 19))]
    WindowGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 20))]
    BattleUISkinGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 21))]
    PartySize(Number),
    #[br(pre_assert(id == 22))]
    Party(#[br(count = length / 2)] Vec<u16>),
    #[br(pre_assert(id == 26))]
    MenuCommandsSize(Number),
    #[br(pre_assert(id == 27))]
    MenuCommands(#[br(count = length / 2)] Vec<u16>),
    #[br(pre_assert(id == 31))]
    TitleScreenMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 32))]
    BattleMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 33))]
    BattleEndMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 34))]
    InnMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 35))]
    BoatMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 36))]
    ShipMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 37))]
    AirshipMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 38))]
    GameOverMusic(Array<Chunk<MusicChunk>>),
    #[br(pre_assert(id == 41))]
    CursorSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 42))]
    SelectSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 43))]
    CancelSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 44))]
    BuzzerSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 45))]
    BattleStartSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 46))]
    EscapeSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 47))]
    EnemyAttackSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 48))]
    EnemyDamageSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 49))]
    ActorDamageSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 50))]
    EvasionSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 51))]
    EnemyCollapseSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 52))]
    UseItemSoundEffect(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 61))]
    TransferPlayerFadeout(Number),
    #[br(pre_assert(id == 62))]
    TransferPlayerFadein(Number),
    #[br(pre_assert(id == 63))]
    BattleStartFadeout(Number),
    #[br(pre_assert(id == 64))]
    BattleStartFadein(Number),
    #[br(pre_assert(id == 65))]
    BattleEndFadeout(Number),
    #[br(pre_assert(id == 66))]
    BattleEndFadein(Number),
    #[br(pre_assert(id == 71))]
    Unknown71(Number),
    #[br(pre_assert(id == 72))]
    Font(Number),
    #[br(pre_assert(id == 81))]
    Unknown81(Number),
    #[br(pre_assert(id == 82))]
    Unknown82(Number),
    #[br(pre_assert(id == 84))]
    Unknown84(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 85))]
    Unknown85(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 91))]
    SaveCount(Number),
    #[br(pre_assert(id == 96))]
    Unknown96(Number),
    #[br(pre_assert(id == 98))]
    Unknown98(Number),
    #[br(pre_assert(id == 99))]
    ScreenFrameEnabled(Number),
    #[br(pre_assert(id == 100))]
    ScreenFrameGraphicFile(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 101))]
    FlipAssets(Number),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for SystemChunk {
    fn id(&self) -> u32 {
        match self {
            Self::ID(_) => 10,
            Self::BoatGraphicFile(_) => 11,
            Self::ShipGraphicFile(_) => 12,
            Self::AirshipGraphicFile(_) => 13,
            Self::BoatGraphicIndex(_) => 14,
            Self::ShipGraphicIndex(_) => 15,
            Self::AirshipGraphicIndex(_) => 16,
            Self::TitleGraphicFile(_) => 17,
            Self::GameOverGraphicFile(_) => 18,
            Self::WindowGraphicFile(_) => 19,
            Self::BattleUISkinGraphicFile(_) => 20,
            Self::PartySize(_) => 21,
            Self::Party(_) => 22,
            Self::MenuCommandsSize(_) => 26,
            Self::MenuCommands(_) => 27,
            Self::TitleScreenMusic(_) => 31,
            Self::BattleMusic(_) => 32,
            Self::BattleEndMusic(_) => 33,
            Self::InnMusic(_) => 34,
            Self::BoatMusic(_) => 35,
            Self::ShipMusic(_) => 36,
            Self::AirshipMusic(_) => 37,
            Self::GameOverMusic(_) => 38,
            Self::CursorSoundEffect(_) => 41,
            Self::SelectSoundEffect(_) => 42,
            Self::CancelSoundEffect(_) => 43,
            Self::BuzzerSoundEffect(_) => 44,
            Self::BattleStartSoundEffect(_) => 45,
            Self::EscapeSoundEffect(_) => 46,
            Self::EnemyAttackSoundEffect(_) => 47,
            Self::EnemyDamageSoundEffect(_) => 48,
            Self::ActorDamageSoundEffect(_) => 49,
            Self::EvasionSoundEffect(_) => 50,
            Self::EnemyCollapseSoundEffect(_) => 51,
            Self::UseItemSoundEffect(_) => 52,
            Self::TransferPlayerFadeout(_) => 61,
            Self::TransferPlayerFadein(_) => 62,
            Self::BattleStartFadeout(_) => 63,
            Self::BattleStartFadein(_) => 64,
            Self::BattleEndFadeout(_) => 65,
            Self::BattleEndFadein(_) => 66,
            Self::Unknown71(_) => 71,
            Self::Font(_) => 72,
            Self::Unknown81(_) => 81,
            Self::Unknown82(_) => 82,
            Self::Unknown84(_) => 84,
            Self::Unknown85(_) => 85,
            Self::SaveCount(_) => 91,
            Self::Unknown96(_) => 96,
            Self::Unknown98(_) => 98,
            Self::ScreenFrameEnabled(_) => 99,
            Self::ScreenFrameGraphicFile(_) => 100,
            Self::FlipAssets(_) => 101,
            Self::Unknown { id, .. } => *id,
        }
    }
}
