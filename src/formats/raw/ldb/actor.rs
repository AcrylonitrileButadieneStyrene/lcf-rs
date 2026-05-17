use crate::helpers::Number;

#[binrw::binrw]
#[br(import(id: u32, length: u32))]
#[derive(Clone, Debug)]
#[brw(little)]
pub enum ActorChunk {
    #[br(pre_assert(id == 1))]
    Name(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 2))]
    Nickname(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 3))]
    CharSetName(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 4))]
    CharSetIndex(Number),
    #[br(pre_assert(id == 5))]
    CharSetTransparent(Number),
    #[br(pre_assert(id == 7))]
    InitialLevel(Number),
    #[br(pre_assert(id == 8))]
    MaxLevel(Number),
    #[br(pre_assert(id == 9))]
    CriticalHitEnabled(Number),
    #[br(pre_assert(id == 10))]
    CriticalHitChance(Number),
    #[br(pre_assert(id == 15))]
    FaceSetName(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 16))]
    FaceSetIndex(Number),
    #[br(pre_assert(id == 21))]
    DualWield(Number),
    #[br(pre_assert(id == 22))]
    FixedEquip(Number),
    #[br(pre_assert(id == 23))]
    AutoBattle(Number),
    #[br(pre_assert(id == 24))]
    MightyGuard(Number),
    #[br(pre_assert(id == 31))]
    ParameterCurves(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 41))]
    ExperienceBase(Number),
    #[br(pre_assert(id == 42))]
    ExperienceExtra(Number),
    #[br(pre_assert(id == 43))]
    ExperienceAcceleration(Number),
    #[br(pre_assert(id == 51))]
    StartingEquipment(#[br(count = 10)] Vec<u8>),
    #[br(pre_assert(id == 56))]
    BarehandAnimation(Number),
    #[br(pre_assert(id == 57))]
    Class(Number),
    #[br(pre_assert(id == 59))]
    BattleX(Number),
    #[br(pre_assert(id == 60))]
    BattleY(Number),
    #[br(pre_assert(id == 62))]
    BattleAnimation(Number),
    #[br(pre_assert(id == 63))]
    Skills(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 72))]
    StateRate(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 74))]
    ElementGuard(#[br(count = length)] Vec<u8>),
    #[br(pre_assert(id == 80))]
    BattleCommands(#[br(count = length)] Vec<u8>),

    Unknown {
        #[br(calc = id)]
        #[bw(ignore)]
        id: u32,

        #[br(count = length)]
        bytes: Vec<u8>,
    },
}

impl crate::helpers::ToChunkID for ActorChunk {
    fn id(&self) -> u32 {
        match self {
            Self::Name(_) => 1,
            Self::Nickname(_) => 2,
            Self::CharSetName(_) => 3,
            Self::CharSetIndex(_) => 4,
            Self::CharSetTransparent(_) => 5,
            Self::InitialLevel(_) => 7,
            Self::MaxLevel(_) => 8,
            Self::CriticalHitEnabled(_) => 9,
            Self::CriticalHitChance(_) => 10,
            Self::FaceSetName(_) => 15,
            Self::FaceSetIndex(_) => 16,
            Self::DualWield(_) => 21,
            Self::FixedEquip(_) => 22,
            Self::AutoBattle(_) => 23,
            Self::MightyGuard(_) => 24,
            Self::ParameterCurves(_) => 31,
            Self::ExperienceBase(_) => 41,
            Self::ExperienceExtra(_) => 42,
            Self::ExperienceAcceleration(_) => 43,
            Self::StartingEquipment(_) => 51,
            Self::BarehandAnimation(_) => 56,
            Self::Class(_) => 57,
            Self::BattleX(_) => 59,
            Self::BattleY(_) => 60,
            Self::BattleAnimation(_) => 62,
            Self::Skills(_) => 63,
            Self::StateRate(_) => 72,
            Self::ElementGuard(_) => 74,
            Self::BattleCommands(_) => 80,
            Self::Unknown { id, .. } => *id,
        }
    }
}
