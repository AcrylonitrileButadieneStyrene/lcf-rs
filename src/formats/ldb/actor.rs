use crate::{
    helpers::{Array, Chunk},
    ldb::LcfDataBaseReadError,
    raw::ldb::actor::ActorChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Actor {
    pub name: Vec<u8>,
    #[doc(alias = "title")]
    pub nickname: Vec<u8>,
    pub charset_name: Vec<u8>,
    pub charset_index: u32,
    pub charset_transparent: bool,
    pub initial_level: u32,
    pub max_level: u32,
    pub critical_hit_enabled: bool,
    pub critical_hit_chance: u32,
    pub faceset_name: Vec<u8>,
    pub faceset_index: u32,
    pub dual_wield: bool,
    pub fixed_equip: bool,
    pub auto_battle: bool,
    pub mighty_guard: bool,
    pub parameter_curves: Vec<u8>,
    pub experience_base: u32,
    pub experience_extra: u32,
    pub experience_acceleration: u32,
    pub starting_equipment: Vec<u8>,
    pub barehand_animation: u32,
    pub class: u32,
    pub battle_x: u32,
    pub battle_y: u32,
    pub battle_animation: u32,
    pub skills: Vec<u8>,
    pub state_rate: Vec<u8>,
    pub element_guard: Vec<u8>,
    pub battle_commands: Vec<u8>,
}

impl Actor {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<ActorChunk>>,
    ) -> Result<Self, LcfDataBaseReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                ActorChunk::Name(items) => self.name = items,
                ActorChunk::Nickname(items) => self.nickname = items,
                ActorChunk::CharSetName(items) => self.charset_name = items,
                ActorChunk::CharSetIndex(number) => self.charset_index = number.0,
                ActorChunk::CharSetTransparent(number) => self.charset_transparent = number.0 != 0,
                ActorChunk::InitialLevel(number) => self.initial_level = number.0,
                ActorChunk::MaxLevel(number) => self.max_level = number.0,
                ActorChunk::CriticalHitEnabled(number) => self.critical_hit_enabled = number.0 != 0,
                ActorChunk::CriticalHitChance(number) => self.critical_hit_chance = number.0,
                ActorChunk::FaceSetName(items) => self.faceset_name = items,
                ActorChunk::FaceSetIndex(number) => self.faceset_index = number.0,
                ActorChunk::DualWield(number) => self.dual_wield = number.0 != 0,
                ActorChunk::FixedEquip(number) => self.fixed_equip = number.0 != 0,
                ActorChunk::AutoBattle(number) => self.auto_battle = number.0 != 0,
                ActorChunk::MightyGuard(number) => self.mighty_guard = number.0 != 0,
                ActorChunk::ParameterCurves(items) => self.parameter_curves = items,
                ActorChunk::ExperienceBase(number) => self.experience_base = number.0,
                ActorChunk::ExperienceExtra(number) => self.experience_extra = number.0,
                ActorChunk::ExperienceAcceleration(number) => {
                    self.experience_acceleration = number.0
                }
                ActorChunk::StartingEquipment(items) => self.starting_equipment = items,
                ActorChunk::BarehandAnimation(number) => self.barehand_animation = number.0,
                ActorChunk::Class(number) => self.class = number.0,
                ActorChunk::BattleX(number) => self.battle_x = number.0,
                ActorChunk::BattleY(number) => self.battle_y = number.0,
                ActorChunk::BattleAnimation(number) => self.battle_animation = number.0,
                ActorChunk::Skills(items) => self.skills = items,
                ActorChunk::StateRate(items) => self.state_rate = items,
                ActorChunk::ElementGuard(items) => self.element_guard = items,
                ActorChunk::BattleCommands(items) => self.battle_commands = items,
                ActorChunk::Unknown { id, bytes } => {
                    return Err(LcfDataBaseReadError::UnknownData(id, bytes));
                }
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<ActorChunk>> {
        let mut chunks = Vec::new();

        chunks.push(ActorChunk::Name(self.name.clone()));
        chunks.push(ActorChunk::Nickname(self.nickname.clone()));
        chunks.push(ActorChunk::CharSetName(self.charset_name.clone()));
        chunks.push(ActorChunk::CharSetIndex(self.charset_index.into()));
        chunks.push(ActorChunk::CharSetTransparent(
            self.charset_transparent.into(),
        ));
        chunks.push(ActorChunk::InitialLevel(self.initial_level.into()));
        chunks.push(ActorChunk::MaxLevel(self.max_level.into()));
        chunks.push(ActorChunk::CriticalHitEnabled(
            self.critical_hit_enabled.into(),
        ));
        chunks.push(ActorChunk::CriticalHitChance(
            self.critical_hit_chance.into(),
        ));
        chunks.push(ActorChunk::FaceSetName(self.faceset_name.clone()));
        chunks.push(ActorChunk::FaceSetIndex(self.faceset_index.into()));
        chunks.push(ActorChunk::DualWield(self.dual_wield.into()));
        chunks.push(ActorChunk::FixedEquip(self.fixed_equip.into()));
        chunks.push(ActorChunk::AutoBattle(self.auto_battle.into()));
        chunks.push(ActorChunk::MightyGuard(self.mighty_guard.into()));
        chunks.push(ActorChunk::ParameterCurves(self.parameter_curves.clone()));
        chunks.push(ActorChunk::ExperienceBase(self.experience_base.into()));
        chunks.push(ActorChunk::ExperienceExtra(self.experience_extra.into()));
        chunks.push(ActorChunk::ExperienceAcceleration(
            self.experience_acceleration.into(),
        ));
        chunks.push(ActorChunk::StartingEquipment(
            self.starting_equipment.clone(),
        ));
        chunks.push(ActorChunk::BarehandAnimation(
            self.barehand_animation.into(),
        ));
        chunks.push(ActorChunk::Class(self.class.into()));
        chunks.push(ActorChunk::BattleX(self.battle_x.into()));
        chunks.push(ActorChunk::BattleY(self.battle_y.into()));
        chunks.push(ActorChunk::BattleAnimation(self.battle_animation.into()));
        chunks.push(ActorChunk::Skills(self.skills.clone()));
        chunks.push(ActorChunk::StateRate(self.state_rate.clone()));
        chunks.push(ActorChunk::ElementGuard(self.element_guard.clone()));
        chunks.push(ActorChunk::BattleCommands(self.battle_commands.clone()));

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
