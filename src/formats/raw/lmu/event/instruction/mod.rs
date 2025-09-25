mod event_target;

pub use event_target::EventTarget;

#[binrw::binrw]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[br(import(opcode: u32))]
#[brw(little)]
pub enum Instruction {
    #[br(pre_assert(opcode == 10))]
    End,
    #[br(pre_assert(opcode == 1005))]
    CallCommonEvent, // TODO
    #[br(pre_assert(opcode == 1006))]
    ForceFlee, // TODO
    #[br(pre_assert(opcode == 1007))]
    EnableCombo, // TODO
    #[br(pre_assert(opcode == 1008))]
    ChangeClass, // TODO
    #[br(pre_assert(opcode == 1009))]
    ChangeBattleCommands, // TODO
    #[br(pre_assert(opcode == 5001))]
    OpenLoadMenu, // TODO
    #[br(pre_assert(opcode == 5002))]
    ExitGame, // TODO
    #[br(pre_assert(opcode == 5003))]
    ToggleAtbMode, // TODO
    #[br(pre_assert(opcode == 5004))]
    ToggleFullscreen, // TODO
    #[br(pre_assert(opcode == 5005))]
    OpenVideoOptions, // TODO
    #[br(pre_assert(opcode == 10110))]
    ShowMessage, // TODO
    #[br(pre_assert(opcode == 10120))]
    MessageOptions, // TODO
    #[br(pre_assert(opcode == 10130))]
    ChangeFaceGraphic, // TODO
    #[br(pre_assert(opcode == 10140))]
    ShowChoice, // TODO
    #[br(pre_assert(opcode == 10150))]
    InputNumber, // TODO
    #[br(pre_assert(opcode == 10210))]
    ControlSwitches {
        /// - 0: Single
        /// - 1: Batch
        /// - 2: Variable
        mode: u32,
        start: u32,
        end: u32,
        /// - 0: Set true
        /// - 1: Set false
        /// - 2: Invert
        operation: u32,
    },
    #[br(pre_assert(opcode == 10220))]
    ControlVariables {
        /// - 0: Single
        /// - 1: Batch
        /// - 2: Variable
        mode: u32,
        start: u32,
        end: u32,
        /// - 0: Set
        /// - 1: Addition
        /// - 2: Subtract
        /// - 3: Multiply
        /// - 4: Divide
        /// - 5: Modulus
        operation: u32,
        /// - 0: Constant value
        /// - 1: Variable
        /// - 2: Variable ID
        /// - 3: Random
        /// - 4: Item
        /// - 5: Player
        /// - 6: Event
        /// - 7: Other
        operand: u32,
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
        value1: u32,
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
        value2: u32,
    },
    #[br(pre_assert(opcode == 10230))]
    TimerOperation, // TODO
    #[br(pre_assert(opcode == 10310))]
    ChangeGold, // TODO
    #[br(pre_assert(opcode == 10320))]
    ChangeItems, // TODO
    #[br(pre_assert(opcode == 10330))]
    ChangePartyMembers, // TODO
    #[br(pre_assert(opcode == 10410))]
    ChangeExp, // TODO
    #[br(pre_assert(opcode == 10420))]
    ChangeLevel, // TODO
    #[br(pre_assert(opcode == 10430))]
    ChangeParameters, // TODO
    #[br(pre_assert(opcode == 10440))]
    ChangeSkills, // TODO
    #[br(pre_assert(opcode == 10450))]
    ChangeEquipment, // TODO
    #[br(pre_assert(opcode == 10460))]
    ChangeHP, // TODO
    #[br(pre_assert(opcode == 10470))]
    ChangeSP, // TODO
    #[br(pre_assert(opcode == 10480))]
    ChangeCondition, // TODO
    #[br(pre_assert(opcode == 10490))]
    FullHeal, // TODO
    #[br(pre_assert(opcode == 10500))]
    SimulatedAttack, // TODO
    #[br(pre_assert(opcode == 10610))]
    ChangeHeroName, // TODO
    #[br(pre_assert(opcode == 10620))]
    ChangeHeroTitle, // TODO
    #[br(pre_assert(opcode == 10630))]
    ChangeSpriteAssociation, // TODO
    #[br(pre_assert(opcode == 10640))]
    ChangeActorFace, // TODO
    #[br(pre_assert(opcode == 10650))]
    ChangeVehicleGraphic, // TODO
    #[br(pre_assert(opcode == 10660))]
    ChangeSystemBGM, // TODO
    #[br(pre_assert(opcode == 10670))]
    ChangeSystemSFX, // TODO
    #[br(pre_assert(opcode == 10680))]
    ChangeSystemGraphics, // TODO
    #[br(pre_assert(opcode == 10690))]
    ChangeScreenTransitions {
        /// - 0: Hide player
        /// - 1: Show player
        /// - 2: Hide for battle start
        /// - 3: Show for battle start
        /// - 4: Hide for battle end
        /// - 5: Show for battle end
        mode: u32,
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
        value: u32,
    },
    #[br(pre_assert(opcode == 10710))]
    EnemyEncounter, // TODO
    #[br(pre_assert(opcode == 10720))]
    OpenShop, // TODO
    #[br(pre_assert(opcode == 10730))]
    ShowInn, // TODO
    #[br(pre_assert(opcode == 10740))]
    EnterHeroName, // TODO
    #[br(pre_assert(opcode == 10810))]
    TransferPlayer {
        /// If the map ID here is the same as the current map ID, then the map will not be reloaded.
        /// To reload the map, teleport to a different map and immediately teleport back. Deferring not necessary.
        map: u32,
        x: u32,
        y: u32,
    },
    #[br(pre_assert(opcode == 10820))]
    MemorizeLocation, // TODO
    #[br(pre_assert(opcode == 10830))]
    RecallToLocation, // TODO
    #[br(pre_assert(opcode == 10840))]
    EnterExitVehicle, // TODO
    #[br(pre_assert(opcode == 10850))]
    SetVehicleLocation, // TODO
    #[br(pre_assert(opcode == 10860))]
    SetEventLocation {
        source: EventTarget,
        /// - 0: Constant
        /// - 1: Variable
        mode: u32,
        x_pos: u32,
        y_pos: u32,
    },
    #[br(pre_assert(opcode == 10870))]
    TradeEventLocations, // TODO
    #[br(pre_assert(opcode == 10910))]
    StoreTerrainID, // TODO
    #[br(pre_assert(opcode == 10920))]
    GetEventLocation {
        /// - 0: Constant
        /// - 1: Variable
        mode: u32,
        x: u32,
        y: u32,
        output: u32,
    },
    #[br(pre_assert(opcode == 11010))]
    HideScreen {
        /// - [`u32::MAX`][]: Default
        mode: u32,
    },
    #[br(pre_assert(opcode == 11020))]
    ShowScreen {
        /// - [`u32::MAX`][]: Default
        mode: u32,
    },
    #[br(pre_assert(opcode == 11030))]
    TintScreen {
        /// - Type: percentage
        /// - Range: 0-200
        red: u32,
        /// - Type: percentage
        /// - Range: 0-200
        green: u32,
        /// - Type: percentage
        /// - Range: 0-200
        blue: u32,
        /// - Type: percentage
        /// - Range: 0-200
        saturation: u32,
        deciseconds: u32,
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11040))]
    FlashScreen {
        red: u32,
        green: u32,
        blue: u32,
        value: u32,
        deciseconds: u32,
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11050))]
    ShakeScreen {
        /// Range: 1-9. 5 = Normal
        power: u32,
        /// Range: 1-9. 5 = Normal
        speed: u32,
        deciseconds: u32,
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11060))]
    ScrollMap {
        /// 0: Fix
        /// 1: Unfix
        /// 2: Shift position
        /// 3: Return to origin
        mode: u32,
        // 0: Up
        // 1: Left
        // 2: Right
        // 3: Down
        direction: u32,
        distance: u32,
        speed: u32,
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11070))]
    WeatherEffects, // TODO
    #[br(pre_assert(opcode == 11110))]
    ShowPicture, // TODO
    #[br(pre_assert(opcode == 11120))]
    MovePicture, // TODO
    #[br(pre_assert(opcode == 11130))]
    ErasePicture, // TODO
    #[br(pre_assert(opcode == 11210))]
    ShowBattleAnimation, // TODO
    #[br(pre_assert(opcode == 11310))]
    PlayerVisibility, // TODO
    #[br(pre_assert(opcode == 11320))]
    FlashSprite, // TODO
    #[br(pre_assert(opcode == 11330))]
    MoveEvent {
        // TODO
        target: EventTarget,
        frequency: u32,
        #[br(parse_with = binrw::helpers::until_eof)]
        rest: Vec<u32>,
    },
    #[br(pre_assert(opcode == 11340))]
    WaitForAllMovement, // TODO
    #[br(pre_assert(opcode == 11350))]
    HaltAllMovement, // TODO
    #[br(pre_assert(opcode == 11410))]
    Wait { deciseconds: u32 },

    #[br(pre_assert(opcode == 11510))]
    PlayBGM, // TODO
    #[br(pre_assert(opcode == 11520))]
    FadeOutBGM { seconds: u32 },
    #[br(pre_assert(opcode == 11530))]
    MemorizeBGM, // TODO
    #[br(pre_assert(opcode == 11540))]
    PlayMemorizedBGM, // TODO
    #[br(pre_assert(opcode == 11550))]
    PlaySoundEffect {
        /// - Unit: percentage
        /// - Range: 0-100.
        volume: u32,
        /// - Unit: percentage
        /// - Range: 50-150
        tempo: u32,
        /// - Range: 0-100. 0 = left, 50 = middle, 100 = right.
        balance: u32,
    },
    #[br(pre_assert(opcode == 11560))]
    PlayMovie, // TODO
    #[br(pre_assert(opcode == 11610))]
    KeyInputProcessing {
        /// Output variable. Expect this variable to be completely trashed and unusable for other purposes.
        output: u32,
        /// - Type: Bool
        /// If true event processing will pause until the animation is complete
        wait_for_confirmation: u32,
        /// Appears to be unused in r2k.
        unknown: u32,
        /// Space/Z/Enter
        decision: u32,
        /// Escape/X/C/V/B/N/Kp0
        cancel: u32,
        shift: u32,
        down: u32,
        left: u32,
        right: u32,
        up: u32,
    },
    #[br(pre_assert(opcode == 11710))]
    ChangeMapTileset, // TODO
    #[br(pre_assert(opcode == 11720))]
    ChangePBG, // TODO
    #[br(pre_assert(opcode == 11740))]
    ChangeEncounterSteps, // TODO
    #[br(pre_assert(opcode == 11750))]
    ChangeTile {
        is_upper: u32,
        /// 0-based but shows as 1-based in the editor.
        from: u32,
        /// 0-based but shows as 1-based in the editor.
        to: u32,
    },
    #[br(pre_assert(opcode == 11810))]
    TeleportTargets, // TODO
    #[br(pre_assert(opcode == 11820))]
    ChangeTeleportAccess, // TODO
    #[br(pre_assert(opcode == 11830))]
    EscapeTarget, // TODO
    #[br(pre_assert(opcode == 11840))]
    ChangeEscapeAccess, // TODO
    #[br(pre_assert(opcode == 11910))]
    OpenSaveMenu, // TODO
    #[br(pre_assert(opcode == 11930))]
    ChangeSaveAccess, // TODO
    #[br(pre_assert(opcode == 11950))]
    OpenMainMenu, // TODO
    #[br(pre_assert(opcode == 11960))]
    ChangeMainMenuAccess, // TODO
    #[br(pre_assert(opcode == 12010))]
    ConditionalBranch {
        /// 0: Switch
        /// 1: Variable
        mode: u32,
        index: u32,
        /// - Facing:
        ///   - 0: Up
        ///   - 1: Right
        ///   - 2: Down
        ///   - 3: Left
        operator1: u32, // TODO
        value: u32,
        /// - Variable:
        ///   - 0: ==
        ///   - 1: >=
        ///   - 2: <=
        ///   - 3: >
        ///   - 4: <
        ///   - 5: !=
        operator2: u32, // TODO
        has_else: u32,
    },

    #[br(pre_assert(opcode == 12110))]
    Label, // TODO
    #[br(pre_assert(opcode == 12120))]
    JumpToLabel, // TODO
    #[br(pre_assert(opcode == 12210))]
    Loop, // TODO
    #[br(pre_assert(opcode == 12220))]
    BreakLoop, // TODO
    #[br(pre_assert(opcode == 12310))]
    EndEventProcessing,
    #[br(pre_assert(opcode == 12320))]
    EraseEvent,
    #[br(pre_assert(opcode == 12330))]
    CallEvent {
        /// 0: Common event
        /// 1: Map event (constant)
        /// 2: Map event (variable)
        mode: u32,
        index: u32,
        page: u32,
    },

    #[br(pre_assert(opcode == 12410))]
    Comment, // TODO
    #[br(pre_assert(opcode == 12420))]
    GameOver, // TODO
    #[br(pre_assert(opcode == 12510))]
    ReturntoTitleScreen, // TODO
    #[br(pre_assert(opcode == 13110))]
    ChangeMonsterHP, // TODO
    #[br(pre_assert(opcode == 13120))]
    ChangeMonsterMP, // TODO
    #[br(pre_assert(opcode == 13130))]
    ChangeMonsterCondition, // TODO
    #[br(pre_assert(opcode == 13150))]
    ShowHiddenMonster, // TODO
    #[br(pre_assert(opcode == 13210))]
    ChangeBattleBG, // TODO
    #[br(pre_assert(opcode == 13260))]
    ShowBattleAnimationB, // TODO
    #[br(pre_assert(opcode == 13310))]
    ConditionalBranchB, // TODO
    #[br(pre_assert(opcode == 13410))]
    TerminateBattle, // TODO
    #[br(pre_assert(opcode == 20110))]
    ShowMessage2, // TODO
    #[br(pre_assert(opcode == 20140))]
    ShowChoiceOption, // TODO
    #[br(pre_assert(opcode == 20141))]
    ShowChoiceEnd, // TODO
    #[br(pre_assert(opcode == 20710))]
    VictoryHandler, // TODO
    #[br(pre_assert(opcode == 20711))]
    EscapeHandler, // TODO
    #[br(pre_assert(opcode == 20712))]
    DefeatHandler, // TODO
    #[br(pre_assert(opcode == 20713))]
    EndBattle, // TODO
    #[br(pre_assert(opcode == 20720))]
    Transaction, // TODO
    #[br(pre_assert(opcode == 20721))]
    NoTransaction, // TODO
    #[br(pre_assert(opcode == 20722))]
    EndShop, // TODO
    #[br(pre_assert(opcode == 20730))]
    Stay, // TODO
    #[br(pre_assert(opcode == 20731))]
    NoStay, // TODO
    #[br(pre_assert(opcode == 20732))]
    EndInn, // TODO
    #[br(pre_assert(opcode == 22010))]
    ElseBranch,
    #[br(pre_assert(opcode == 22011))]
    EndBranch,
    #[br(pre_assert(opcode == 22210))]
    EndLoop, // TODO
    #[br(pre_assert(opcode == 22410))]
    CommentNextLine, // TODO
    #[br(pre_assert(opcode == 23310))]
    ElseBranchB, // TODO
    #[br(pre_assert(opcode == 23311))]
    EndBranchB, // TODO

    Unknown {
        #[br(calc = opcode)]
        #[bw(ignore)]
        opcode: u32,

        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<u32>,
    },
}

impl Instruction {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn opcode(&self) -> u32 {
        match self {
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
            Self::Unknown { opcode, .. } => *opcode,
        }
    }
}
