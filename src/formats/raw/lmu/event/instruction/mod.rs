use crate::helpers::Number;

mod event_target;

pub use event_target::EventTarget;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[br(import(opcode: Number))]
#[brw(little)]
pub enum Instruction {
    #[br(pre_assert(opcode.0 == 10))]
    End,
    #[br(pre_assert(opcode.0 == 1005))]
    CallCommonEvent, // TODO
    #[br(pre_assert(opcode.0 == 1006))]
    ForceFlee, // TODO
    #[br(pre_assert(opcode.0 == 1007))]
    EnableCombo, // TODO
    #[br(pre_assert(opcode.0 == 1008))]
    ChangeClass, // TODO
    #[br(pre_assert(opcode.0 == 1009))]
    ChangeBattleCommands, // TODO
    #[br(pre_assert(opcode.0 == 5001))]
    OpenLoadMenu, // TODO
    #[br(pre_assert(opcode.0 == 5002))]
    ExitGame, // TODO
    #[br(pre_assert(opcode.0 == 5003))]
    ToggleAtbMode, // TODO
    #[br(pre_assert(opcode.0 == 5004))]
    ToggleFullscreen, // TODO
    #[br(pre_assert(opcode.0 == 5005))]
    OpenVideoOptions, // TODO
    #[br(pre_assert(opcode.0 == 10110))]
    ShowMessage, // TODO
    #[br(pre_assert(opcode.0 == 10120))]
    MessageOptions, // TODO
    #[br(pre_assert(opcode.0 == 10130))]
    ChangeFaceGraphic, // TODO
    #[br(pre_assert(opcode.0 == 10140))]
    ShowChoice, // TODO
    #[br(pre_assert(opcode.0 == 10150))]
    InputNumber, // TODO
    #[br(pre_assert(opcode.0 == 10210))]
    ControlSwitches {
        /// - 0: Single
        /// - 1: Batch
        /// - 2: Variable
        mode: Number,
        start: Number,
        end: Number,
        /// - 0: Set true
        /// - 1: Set false
        /// - 2: Invert
        operation: Number,
    },
    #[br(pre_assert(opcode.0 == 10220))]
    ControlVariables {
        /// - 0: Single
        /// - 1: Batch
        /// - 2: Variable
        mode: Number,
        start: Number,
        end: Number,
        /// - 0: Set
        /// - 1: Addition
        /// - 2: Subtract
        /// - 3: Multiply
        /// - 4: Divide
        /// - 5: Modulus
        operation: Number,
        /// - 0: Constant value
        /// - 1: Variable
        /// - 2: Variable ID
        /// - 3: Random
        /// - 4: Item
        /// - 5: Player
        /// - 6: Event
        /// - 7: Other
        operand: Number,
        /// - Random: minimum value (included in sampled values)
        /// - Other:
        ///   - 0: Money
        ///   - 1: Time remaining
        ///   - 2: Party size
        ///   - 3: Save count
        ///   - 4: Battle count
        ///   - 5: Win count
        ///   - 6: Loss count
        ///   - 7: Escape count
        ///   - 8: MIDI progress?
        value1: Number,
        /// - Random: maximum value (not included in sampled values)
        /// - Item: bool
        /// - Player:
        ///   - 0: Level
        ///   - 1: Experience
        ///   - 2: HP
        ///   - 3: MP
        ///   - 4: Maximum HP
        ///   - 5: Maximum MP
        ///   - 6: Attack
        ///   - 7: Defense
        ///   - 8: Mind
        ///   - 9: Agility
        ///   - 10: Weapon ID
        ///   - 11: Shield ID
        ///   - 12: Body ID
        ///   - 13: Head ID
        ///   - 14: Accessory ID
        /// - Event:
        ///   - 0: Map ID
        ///   - 0: X
        ///   - 0: Y
        ///   - 0: Direction
        ///   - 0: Screen X
        ///   - 0: Screen Y
        value2: Number,
    },
    #[br(pre_assert(opcode.0 == 10230))]
    TimerOperation, // TODO
    #[br(pre_assert(opcode.0 == 10310))]
    ChangeGold, // TODO
    #[br(pre_assert(opcode.0 == 10320))]
    ChangeItems, // TODO
    #[br(pre_assert(opcode.0 == 10330))]
    ChangePartyMembers, // TODO
    #[br(pre_assert(opcode.0 == 10410))]
    ChangeExp, // TODO
    #[br(pre_assert(opcode.0 == 10420))]
    ChangeLevel, // TODO
    #[br(pre_assert(opcode.0 == 10430))]
    ChangeParameters, // TODO
    #[br(pre_assert(opcode.0 == 10440))]
    ChangeSkills, // TODO
    #[br(pre_assert(opcode.0 == 10450))]
    ChangeEquipment, // TODO
    #[br(pre_assert(opcode.0 == 10460))]
    ChangeHP, // TODO
    #[br(pre_assert(opcode.0 == 10470))]
    ChangeSP, // TODO
    #[br(pre_assert(opcode.0 == 10480))]
    ChangeCondition, // TODO
    #[br(pre_assert(opcode.0 == 10490))]
    FullHeal, // TODO
    #[br(pre_assert(opcode.0 == 10500))]
    SimulatedAttack, // TODO
    #[br(pre_assert(opcode.0 == 10610))]
    ChangeHeroName, // TODO
    #[br(pre_assert(opcode.0 == 10620))]
    ChangeHeroTitle, // TODO
    #[br(pre_assert(opcode.0 == 10630))]
    ChangeSpriteAssociation, // TODO
    #[br(pre_assert(opcode.0 == 10640))]
    ChangeActorFace, // TODO
    #[br(pre_assert(opcode.0 == 10650))]
    ChangeVehicleGraphic, // TODO
    #[br(pre_assert(opcode.0 == 10660))]
    ChangeSystemBGM, // TODO
    #[br(pre_assert(opcode.0 == 10670))]
    ChangeSystemSFX, // TODO
    #[br(pre_assert(opcode.0 == 10680))]
    ChangeSystemGraphics, // TODO
    #[br(pre_assert(opcode.0 == 10690))]
    ChangeScreenTransitions {
        /// - 0: Hide player
        /// - 1: Show player
        /// - 2: Hide for battle start
        /// - 3: Show for battle start
        /// - 4: Hide for battle end
        /// - 5: Show for battle end
        mode: Number,
        /// - 0: Fade out/in screen
        /// - 1: Random blocks across screen
        /// - 2: Random blocks from above
        /// - 3: Random blocks from below
        /// - 4: Close/open blinds
        /// - 5: Horizontal blinds
        /// - 6: Vertical blinds
        /// - 7: Inwards
        /// - 8: Center outwards
        /// - 9: Scroll up/down
        /// - 10: Scroll down/up
        /// - 11: Scroll left/right
        /// - 12: Scroll right/left
        /// - 13: Horizontal split/join
        /// - 14: Vertical split/join
        /// - 15: Quadrisection/omnidirectonal
        /// - 16: Zoom in/out
        /// - 17: Mosaic
        /// - 18: Raster scroll
        /// - 19: Cut out/in
        /// - 20: Instantaneous/(not an option)
        value: Number,
    },
    #[br(pre_assert(opcode.0 == 10710))]
    EnemyEncounter, // TODO
    #[br(pre_assert(opcode.0 == 10720))]
    OpenShop, // TODO
    #[br(pre_assert(opcode.0 == 10730))]
    ShowInn, // TODO
    #[br(pre_assert(opcode.0 == 10740))]
    EnterHeroName, // TODO
    #[br(pre_assert(opcode.0 == 10810))]
    TransferPlayer {
        /// If the map ID here is the same as the current map ID, then the map will not be reloaded.
        /// To reload the map, teleport to a different map and immediately teleport back. Deferring not necessary.
        map: Number,
        x: Number,
        y: Number,
    },
    #[br(pre_assert(opcode.0 == 10820))]
    MemorizeLocation, // TODO
    #[br(pre_assert(opcode.0 == 10830))]
    RecallToLocation, // TODO
    #[br(pre_assert(opcode.0 == 10840))]
    EnterExitVehicle, // TODO
    #[br(pre_assert(opcode.0 == 10850))]
    SetVehicleLocation, // TODO
    #[br(pre_assert(opcode.0 == 10860))]
    SetEventLocation {
        source: EventTarget,
        /// - 0: Constant
        /// - 1: Variable
        mode: Number,
        x_pos: Number,
        y_pos: Number,
    },
    #[br(pre_assert(opcode.0 == 10870))]
    TradeEventLocations, // TODO
    #[br(pre_assert(opcode.0 == 10910))]
    StoreTerrainID, // TODO
    #[br(pre_assert(opcode.0 == 10920))]
    GetEventLocation {
        /// - 0: Constant
        /// - 1: Variable
        mode: Number,
        x: Number,
        y: Number,
        output: Number,
    },
    #[br(pre_assert(opcode.0 == 11010))]
    HideScreen {
        /// - [`u32::MAX`][]: Default
        mode: Number,
    },
    #[br(pre_assert(opcode.0 == 11020))]
    ShowScreen {
        /// - [`u32::MAX`][]: Default
        mode: Number,
    },
    #[br(pre_assert(opcode.0 == 11030))]
    TintScreen {
        /// - Type: percentage
        /// - Range: 0-200
        red: Number,
        /// - Type: percentage
        /// - Range: 0-200
        green: Number,
        /// - Type: percentage
        /// - Range: 0-200
        blue: Number,
        /// - Type: percentage
        /// - Range: 0-200
        saturation: Number,
        deciseconds: Number,
        wait_for_completion: Number,
    },
    #[br(pre_assert(opcode.0 == 11040))]
    FlashScreen {
        red: Number,
        green: Number,
        blue: Number,
        value: Number,
        deciseconds: Number,
        wait_for_completion: Number,
    },
    #[br(pre_assert(opcode.0 == 11050))]
    ShakeScreen {
        /// Range: 1-9. 5 = Normal
        power: Number,
        /// Range: 1-9. 5 = Normal
        speed: Number,
        deciseconds: Number,
        wait_for_completion: Number,
    },
    #[br(pre_assert(opcode.0 == 11060))]
    ScrollMap {
        /// 0: Fix
        /// 1: Unfix
        /// 2: Shift position
        /// 3: Return to origin
        mode: Number,
        // 0: Up
        // 1: Left
        // 2: Right
        // 3: Down
        direction: Number,
        distance: Number,
        speed: Number,
        wait_for_completion: Number,
    },
    #[br(pre_assert(opcode.0 == 11070))]
    WeatherEffects, // TODO
    #[br(pre_assert(opcode.0 == 11110))]
    ShowPicture, // TODO
    #[br(pre_assert(opcode.0 == 11120))]
    MovePicture, // TODO
    #[br(pre_assert(opcode.0 == 11130))]
    ErasePicture, // TODO
    #[br(pre_assert(opcode.0 == 11210))]
    ShowBattleAnimation, // TODO
    #[br(pre_assert(opcode.0 == 11310))]
    PlayerVisibility, // TODO
    #[br(pre_assert(opcode.0 == 11320))]
    FlashSprite, // TODO
    #[br(pre_assert(opcode.0 == 11330))]
    MoveEvent {
        // TODO
        target: EventTarget,
        frequency: Number,
        #[br(parse_with = binrw::helpers::until_eof)]
        rest: Vec<Number>,
    },
    #[br(pre_assert(opcode.0 == 11340))]
    WaitForAllMovement, // TODO
    #[br(pre_assert(opcode.0 == 11350))]
    HaltAllMovement, // TODO
    #[br(pre_assert(opcode.0 == 11410))]
    Wait { deciseconds: Number },

    #[br(pre_assert(opcode.0 == 11510))]
    PlayBGM, // TODO
    #[br(pre_assert(opcode.0 == 11520))]
    FadeOutBGM { seconds: Number },
    #[br(pre_assert(opcode.0 == 11530))]
    MemorizeBGM, // TODO
    #[br(pre_assert(opcode.0 == 11540))]
    PlayMemorizedBGM, // TODO
    #[br(pre_assert(opcode.0 == 11550))]
    PlaySoundEffect {
        /// - Unit: percentage
        /// - Range: 0-100.
        volume: Number,
        /// - Unit: percentage
        /// - Range: 50-150
        tempo: Number,
        /// - Range: 0-100. 0 = left, 50 = middle, 100 = right.
        balance: Number,
    },
    #[br(pre_assert(opcode.0 == 11560))]
    PlayMovie, // TODO
    #[br(pre_assert(opcode.0 == 11610))]
    KeyInputProcessing {
        /// Output variable. Expect this variable to be completely trashed and unusable for other purposes.
        output: Number,
        /// - Type: Bool
        /// If true event processing will pause until the animation is complete
        wait_for_confirmation: Number,
        /// Appears to be unused in r2k.
        unknown: Number,
        /// Space/Z/Enter
        decision: Number,
        /// Escape/X/C/V/B/N/Kp0
        cancel: Number,
        shift: Number,
        down: Number,
        left: Number,
        right: Number,
        up: Number,
    },
    #[br(pre_assert(opcode.0 == 11710))]
    ChangeMapTileset, // TODO
    #[br(pre_assert(opcode.0 == 11720))]
    ChangePBG, // TODO
    #[br(pre_assert(opcode.0 == 11740))]
    ChangeEncounterSteps, // TODO
    #[br(pre_assert(opcode.0 == 11750))]
    ChangeTile {
        is_upper: Number,
        /// 0-based but shows as 1-based in the editor.
        from: Number,
        /// 0-based but shows as 1-based in the editor.
        to: Number,
    },
    #[br(pre_assert(opcode.0 == 11810))]
    TeleportTargets, // TODO
    #[br(pre_assert(opcode.0 == 11820))]
    ChangeTeleportAccess, // TODO
    #[br(pre_assert(opcode.0 == 11830))]
    EscapeTarget, // TODO
    #[br(pre_assert(opcode.0 == 11840))]
    ChangeEscapeAccess, // TODO
    #[br(pre_assert(opcode.0 == 11910))]
    OpenSaveMenu, // TODO
    #[br(pre_assert(opcode.0 == 11930))]
    ChangeSaveAccess, // TODO
    #[br(pre_assert(opcode.0 == 11950))]
    OpenMainMenu, // TODO
    #[br(pre_assert(opcode.0 == 11960))]
    ChangeMainMenuAccess, // TODO
    #[br(pre_assert(opcode.0 == 12010))]
    ConditionalBranch {
        /// 0: Switch
        /// 1: Variable
        mode: Number,
        index: Number,
        /// - Facing:
        ///   - 0: Up
        ///   - 1: Right
        ///   - 2: Down
        ///   - 3: Left
        operator1: Number, // TODO
        value: Number,
        /// - Variable:
        ///   - 0: ==
        ///   - 1: >=
        ///   - 2: <=
        ///   - 3: >
        ///   - 4: <
        ///   - 5: !=
        operator2: Number, // TODO
        has_else: Number,
    },

    #[br(pre_assert(opcode.0 == 12110))]
    Label, // TODO
    #[br(pre_assert(opcode.0 == 12120))]
    JumpToLabel, // TODO
    #[br(pre_assert(opcode.0 == 12210))]
    Loop, // TODO
    #[br(pre_assert(opcode.0 == 12220))]
    BreakLoop, // TODO
    #[br(pre_assert(opcode.0 == 12310))]
    EndEventProcessing,
    #[br(pre_assert(opcode.0 == 12320))]
    EraseEvent,
    #[br(pre_assert(opcode.0 == 12330))]
    CallEvent {
        /// 0: Common event
        /// 1: Map event (constant)
        /// 2: Map event (variable)
        mode: Number,
        index: Number,
        page: Number,
    },

    #[br(pre_assert(opcode.0 == 12410))]
    Comment, // TODO
    #[br(pre_assert(opcode.0 == 12420))]
    GameOver, // TODO
    #[br(pre_assert(opcode.0 == 12510))]
    ReturntoTitleScreen, // TODO
    #[br(pre_assert(opcode.0 == 13110))]
    ChangeMonsterHP, // TODO
    #[br(pre_assert(opcode.0 == 13120))]
    ChangeMonsterMP, // TODO
    #[br(pre_assert(opcode.0 == 13130))]
    ChangeMonsterCondition, // TODO
    #[br(pre_assert(opcode.0 == 13150))]
    ShowHiddenMonster, // TODO
    #[br(pre_assert(opcode.0 == 13210))]
    ChangeBattleBG, // TODO
    #[br(pre_assert(opcode.0 == 13260))]
    ShowBattleAnimationB, // TODO
    #[br(pre_assert(opcode.0 == 13310))]
    ConditionalBranchB, // TODO
    #[br(pre_assert(opcode.0 == 13410))]
    TerminateBattle, // TODO
    #[br(pre_assert(opcode.0 == 20110))]
    ShowMessage2, // TODO
    #[br(pre_assert(opcode.0 == 20140))]
    ShowChoiceOption, // TODO
    #[br(pre_assert(opcode.0 == 20141))]
    ShowChoiceEnd, // TODO
    #[br(pre_assert(opcode.0 == 20710))]
    VictoryHandler, // TODO
    #[br(pre_assert(opcode.0 == 20711))]
    EscapeHandler, // TODO
    #[br(pre_assert(opcode.0 == 20712))]
    DefeatHandler, // TODO
    #[br(pre_assert(opcode.0 == 20713))]
    EndBattle, // TODO
    #[br(pre_assert(opcode.0 == 20720))]
    Transaction, // TODO
    #[br(pre_assert(opcode.0 == 20721))]
    NoTransaction, // TODO
    #[br(pre_assert(opcode.0 == 20722))]
    EndShop, // TODO
    #[br(pre_assert(opcode.0 == 20730))]
    Stay, // TODO
    #[br(pre_assert(opcode.0 == 20731))]
    NoStay, // TODO
    #[br(pre_assert(opcode.0 == 20732))]
    EndInn, // TODO
    #[br(pre_assert(opcode.0 == 22010))]
    ElseBranch,
    #[br(pre_assert(opcode.0 == 22011))]
    EndBranch,
    #[br(pre_assert(opcode.0 == 22210))]
    EndLoop, // TODO
    #[br(pre_assert(opcode.0 == 22410))]
    CommentNextLine, // TODO
    #[br(pre_assert(opcode.0 == 23310))]
    ElseBranchB, // TODO
    #[br(pre_assert(opcode.0 == 23311))]
    EndBranchB, // TODO

    Unknown {
        #[br(calc = opcode)]
        #[bw(ignore)]
        opcode: Number,

        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<Number>,
    },
}

impl Instruction {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn opcode(&self) -> Number {
        Number(match self {
            Self::End => 10,
            Self::CallCommonEvent => 1005,
            Self::ForceFlee => 1006,
            Self::EnableCombo => 1007,
            Self::ChangeClass => 1008,
            Self::ChangeBattleCommands => 1009,
            Self::OpenLoadMenu => 5001,
            Self::ExitGame => 5002,
            Self::ToggleAtbMode => 5003,
            Self::ToggleFullscreen => 5004,
            Self::OpenVideoOptions => 5005,
            Self::ShowMessage => 10110,
            Self::MessageOptions => 10120,
            Self::ChangeFaceGraphic => 10130,
            Self::ShowChoice => 10140,
            Self::InputNumber => 10150,
            Self::ControlSwitches { .. } => 10210,
            Self::ControlVariables { .. } => 10220,
            Self::TimerOperation => 10230,
            Self::ChangeGold => 10310,
            Self::ChangeItems => 10320,
            Self::ChangePartyMembers => 10330,
            Self::ChangeExp => 10410,
            Self::ChangeLevel => 10420,
            Self::ChangeParameters => 10430,
            Self::ChangeSkills => 10440,
            Self::ChangeEquipment => 10450,
            Self::ChangeHP => 10460,
            Self::ChangeSP => 10470,
            Self::ChangeCondition => 10480,
            Self::FullHeal => 10490,
            Self::SimulatedAttack => 10500,
            Self::ChangeHeroName => 10610,
            Self::ChangeHeroTitle => 10620,
            Self::ChangeSpriteAssociation => 10630,
            Self::ChangeActorFace => 10640,
            Self::ChangeVehicleGraphic => 10650,
            Self::ChangeSystemBGM => 10660,
            Self::ChangeSystemSFX => 10670,
            Self::ChangeSystemGraphics => 10680,
            Self::ChangeScreenTransitions { .. } => 10690,
            Self::EnemyEncounter => 10710,
            Self::OpenShop => 10720,
            Self::ShowInn => 10730,
            Self::EnterHeroName => 10740,
            Self::TransferPlayer { .. } => 10810,
            Self::MemorizeLocation => 10820,
            Self::RecallToLocation => 10830,
            Self::EnterExitVehicle => 10840,
            Self::SetVehicleLocation => 10850,
            Self::SetEventLocation { .. } => 10860,
            Self::TradeEventLocations => 10870,
            Self::StoreTerrainID => 10910,
            Self::GetEventLocation { .. } => 10920,
            Self::HideScreen { .. } => 11010,
            Self::ShowScreen { .. } => 11020,
            Self::TintScreen { .. } => 11030,
            Self::FlashScreen { .. } => 11040,
            Self::ShakeScreen { .. } => 11050,
            Self::ScrollMap { .. } => 11060,
            Self::WeatherEffects => 11070,
            Self::ShowPicture => 11110,
            Self::MovePicture => 11120,
            Self::ErasePicture => 11130,
            Self::ShowBattleAnimation => 11210,
            Self::PlayerVisibility => 11310,
            Self::FlashSprite => 11320,
            Self::MoveEvent { .. } => 11330,
            Self::WaitForAllMovement => 11340,
            Self::HaltAllMovement => 11350,
            Self::Wait { .. } => 11410,
            Self::PlayBGM => 11510,
            Self::FadeOutBGM { .. } => 11520,
            Self::MemorizeBGM => 11530,
            Self::PlayMemorizedBGM => 11540,
            Self::PlaySoundEffect { .. } => 11550,
            Self::PlayMovie => 11560,
            Self::KeyInputProcessing { .. } => 11610,
            Self::ChangeMapTileset => 11710,
            Self::ChangePBG => 11720,
            Self::ChangeEncounterSteps => 11740,
            Self::ChangeTile { .. } => 11750,
            Self::TeleportTargets => 11810,
            Self::ChangeTeleportAccess => 11820,
            Self::EscapeTarget => 11830,
            Self::ChangeEscapeAccess => 11840,
            Self::OpenSaveMenu => 11910,
            Self::ChangeSaveAccess => 11930,
            Self::OpenMainMenu => 11950,
            Self::ChangeMainMenuAccess => 11960,
            Self::ConditionalBranch { .. } => 12010,
            Self::Label => 12110,
            Self::JumpToLabel => 12120,
            Self::Loop => 12210,
            Self::BreakLoop => 12220,
            Self::EndEventProcessing => 12310,
            Self::EraseEvent => 12320,
            Self::CallEvent { .. } => 12330,
            Self::Comment => 12410,
            Self::GameOver => 12420,
            Self::ReturntoTitleScreen => 12510,
            Self::ChangeMonsterHP => 13110,
            Self::ChangeMonsterMP => 13120,
            Self::ChangeMonsterCondition => 13130,
            Self::ShowHiddenMonster => 13150,
            Self::ChangeBattleBG => 13210,
            Self::ShowBattleAnimationB => 13260,
            Self::ConditionalBranchB => 13310,
            Self::TerminateBattle => 13410,
            Self::ShowMessage2 => 20110,
            Self::ShowChoiceOption => 20140,
            Self::ShowChoiceEnd => 20141,
            Self::VictoryHandler => 20710,
            Self::EscapeHandler => 20711,
            Self::DefeatHandler => 20712,
            Self::EndBattle => 20713,
            Self::Transaction => 20720,
            Self::NoTransaction => 20721,
            Self::EndShop => 20722,
            Self::Stay => 20730,
            Self::NoStay => 20731,
            Self::EndInn => 20732,
            Self::ElseBranch => 22010,
            Self::EndBranch => 22011,
            Self::EndLoop => 22210,
            Self::CommentNextLine => 22410,
            Self::ElseBranchB => 23310,
            Self::EndBranchB => 23311,
            Self::Unknown { opcode, .. } => opcode.0,
        })
    }
}
