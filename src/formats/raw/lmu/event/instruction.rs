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
    MessageOptions {
        /// - 0: Normal
        /// - 1: Transparent
        opacity: u32,
        /// - 0: Top
        /// - 1: Middle
        /// - 2: Bottom
        position: u32,
        /// - Type: bool
        avoid_covering_player: u32,
        /// - Type: bool
        allow_event_processing: u32,
    },
    #[br(pre_assert(opcode == 10130))]
    ChangeFaceset {
        /// - Range: 0 - 15
        pattern: u32,
        /// - 0: Left
        /// - 1: Right
        display_position: u32,
        /// - Type: bool
        flip_horizontally: u32,
    },
    #[br(pre_assert(opcode == 10140))]
    ShowChoice {
        /// - 0: Disallow (cannot cancel)
        /// - 1: Option 1
        /// - 2: Option 2
        /// - 3: Option 3
        /// - 4: Option 4
        /// - 5: Branch (else block)
        cancel_option: u32,
    },
    #[br(pre_assert(opcode == 10150))]
    InputNumber {
        /// - Range: 0 - 6
        digits: u32,
        variable: u32,
    },
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
    ControlTimer {
        /// - 0: Set Time
        /// - 1: Start
        /// - 2: Stop
        operation: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        seconds: u32,
        /// - Type: bool
        show_timer: u32,
        /// - Type: bool
        continue_in_battles: u32,
    },
    #[br(pre_assert(opcode == 10310))]
    ChangeGold {
        /// - 0: increase
        /// - 1: decrease
        operation: u32,
        /// - 0: constant
        /// - 1: variable
        operand: u32,
        value: u32,
    },
    #[br(pre_assert(opcode == 10320))]
    ChangeItems {
        /// - 0: Increase
        /// - 1: Decrease
        operation: u32,
        /// - 0: Fixed
        /// - 1: Variable
        item_target: u32,
        item: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        value: u32,
    },
    #[br(pre_assert(opcode == 10330))]
    ChangePartyMembers {
        /// - 0: Add
        /// - 1: Remove
        operation: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        actor: u32,
    },
    #[br(pre_assert(opcode == 10410))]
    ChangeExp, // TODO
    #[br(pre_assert(opcode == 10420))]
    ChangeLevel {
        /// - 0: Entire party
        /// - 1: Fixed
        /// - 2: Variable
        actor_operand: u32,
        actor: u32,
        /// - 0: Increase
        /// - 1: Decrease
        operation: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        value: u32,
        /// - Type: bool
        show_level_up_message: u32,
    },
    #[br(pre_assert(opcode == 10430))]
    ChangeParameters {
        /// - 0: Entire Party
        /// - 1: Constant
        /// - 2: Variable
        actor_operand: u32,
        actor: u32,
        /// - 0: Increase
        /// - 1: Decrease
        operation: u32,
        /// - 0: Maximum HP
        /// - 1: Maximum MP
        /// - 2: Attack
        /// - 3: Defense
        /// - 4: Mind
        /// - 5: Agility
        parameter: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        value: u32,
    },
    #[br(pre_assert(opcode == 10440))]
    ChangeSkills {
        /// - 0: Entire party
        /// - 1: Fixed
        /// - 2: Variable
        actor_operand: u32,
        actor: u32,
        /// - 0: Learn
        /// - 1: Forget
        operation: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        value: u32,
    },
    #[br(pre_assert(opcode == 10450))]
    ChangeEquipment {
        /// - 0: Entire party
        /// - 1: Fixed
        /// - 2: Variable
        actor_operand: u32,
        actor: u32,
        /// - 0: Change
        /// - 1: Remove
        operation: u32,
        operand_or_type: u32,
        value: u32,
    },
    #[br(pre_assert(opcode == 10460))]
    ChangeHP, // TODO
    #[br(pre_assert(opcode == 10470))]
    ChangeSP, // TODO
    #[br(pre_assert(opcode == 10480))]
    ChangeCondition, // TODO
    #[br(pre_assert(opcode == 10490))]
    RecoverAll {
        /// - 0: Entire Party
        /// - 1: Constant
        /// - 2: Variable
        operand: u32,
        value: u32,
    }, // TODO
    #[br(pre_assert(opcode == 10500))]
    SimulatedAttack, // TODO
    #[br(pre_assert(opcode == 10610))]
    ChangeActorName { actor: u32 },
    #[br(pre_assert(opcode == 10620))]
    ChangeActorNickname { actor: u32 },
    #[br(pre_assert(opcode == 10630))]
    ChangeActorGraphic {
        actor: u32,
        /// - Range: 0 - 15
        pattern: u32,
        /// - Type: bool
        transparent: u32,
    },
    #[br(pre_assert(opcode == 10640))]
    ChangeActorFaceset { actor: u32, pattern: u32 },
    #[br(pre_assert(opcode == 10650))]
    ChangeVehicleGraphic, // TODO
    #[br(pre_assert(opcode == 10660))]
    ChangeSystemBGM, // TODO
    #[br(pre_assert(opcode == 10670))]
    ChangeSystemSE {
        /// - 0: Cursor
        /// - 1: Select
        /// - 2: Cancel
        /// - 3: Buzzer
        /// - 4: Battle Start
        /// - 5: Enemy Attack
        /// - 6: Enemy Damage
        /// - 7: Actor Damage
        /// - 8: Evasion
        /// - 9: Enemy Collapse
        /// - 10: Use Item
        r#type: u32,
        volume: u32,
        temp: u32,
        balance: u32,
    },
    #[br(pre_assert(opcode == 10680))]
    ChangeSystemGraphics {
        /// - 0: Stretch to Fit
        /// - 1: Tiled Pattern
        window_background: u32,
        /// - 0: RPG2000
        /// - 1: RPG2000G
        font: u32,
    },
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
    OpenShop {
        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<u32>,
    }, // TODO
    #[br(pre_assert(opcode == 10730))]
    ShowInn, // TODO
    #[br(pre_assert(opcode == 10740))]
    NameInputProcessing {
        actor: u32,
        /// - 0: Letters
        /// - 1: Symbols
        mode: u32,
        /// - Type: bool
        show_default_name: u32,
    },
    #[br(pre_assert(opcode == 10810))]
    TransferPlayer {
        /// If the map ID here is the same as the current map ID, then the map will not be reloaded.
        /// To reload the map, teleport to a different map and immediately teleport back. Deferring not necessary.
        map: u32,
        x: u32,
        y: u32,
        /// Facing direction after teleporting.
        /// Trying to view an instruction with this field present causes the Steam English version of the r2k editor to freeze for a minute.
        /// - 0: Preserve
        /// - 1: Up
        /// - 2: Right
        /// - 3: Down
        /// - 4: Left
        #[br(try)]
        direction: Option<u32>,
    },
    #[br(pre_assert(opcode == 10820))]
    GetPlayerLocation {
        output_map: u32,
        output_x: u32,
        output_y: u32,
    },
    #[br(pre_assert(opcode == 10830))]
    MoveToVariableLocation {
        map_variable: u32,
        x_variable: u32,
        y_variable: u32,
    },
    #[br(pre_assert(opcode == 10840))]
    EnterExitVehicle, // TODO
    #[br(pre_assert(opcode == 10850))]
    SetVehicleLocation {
        /// - -1: Unknown
        /// - 0: Boat
        /// - 1: Ship
        /// - 2: Airship
        vehicle: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        map: u32,
        x: u32,
        y: u32,
    },
    #[br(pre_assert(opcode == 10860))]
    SetEventLocation {
        source: u32,
        /// - 0: Constant
        /// - 1: Variable
        mode: u32,
        x_pos: u32,
        y_pos: u32,
        #[br(try)]
        direction: Option<u32>,
    },
    #[br(pre_assert(opcode == 10870))]
    SwapEventLocation { left: u32, right: u32 },
    #[br(pre_assert(opcode == 10910))]
    StoreTerrainID {
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        x: u32,
        y: u32,
        output: u32,
    },
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
        #[br(try)]
        mode: Option<u32>,
    },
    #[br(pre_assert(opcode == 11050))]
    ShakeScreen {
        /// Range: 1-9. 5 = Normal
        power: u32,
        /// Range: 1-9. 5 = Normal
        speed: u32,
        deciseconds: u32,
        wait_for_completion: u32,
        /// - 0: Temporary
        /// - 1: Start
        /// - 2: Stop
        #[br(try)]
        mode: Option<u32>,
    },
    #[br(pre_assert(opcode == 11060))]
    ScrollMap {
        /// - 0: Fix
        /// - 1: Unfix
        /// - 2: Shift position
        /// - 3: Return to origin
        mode: u32,
        /// - 0: Up
        /// - 1: Left
        /// - 2: Right
        /// - 3: Down
        direction: u32,
        distance: u32,
        speed: u32,
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11070))]
    WeatherEffects {
        /// - 0: None
        /// - 1: Rain
        /// - 2: Snow
        r#type: u32,
        /// - 0: Weak
        /// - 1: Medium
        /// - 2: Strong
        power: u32,
    },
    #[br(pre_assert(opcode == 11110))]
    ShowPicture {
        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<u32>,
    },
    #[br(pre_assert(opcode == 11120))]
    MovePicture {
        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<u32>,
    },
    #[br(pre_assert(opcode == 11130))]
    ErasePicture {
        #[br(parse_with = binrw::helpers::until_eof)]
        args: Vec<u32>,
    },
    #[br(pre_assert(opcode == 11210))]
    ShowAnimation {
        animation: u32,
        character: u32,
        /// - Type: bool
        wait_for_completion: u32,
        /// - Type: bool
        show_entire_map: u32,
    },
    #[br(pre_assert(opcode == 11310))]
    ShowHidePlayer {
        /// - 0: show player
        /// - 1: hide player
        state: u32,
    },
    #[br(pre_assert(opcode == 11320))]
    FlashEvent {
        target: u32,
        /// - Range: 0 - 31
        red: u32,
        /// - Range: 0 - 31
        green: u32,
        /// - Range: 0 - 31
        blue: u32,
        /// - Range: 0 - 31
        value: u32,
        /// - Unit: deciseconds
        time: u32,
        /// - Type: bool
        wait_for_completion: u32,
    },
    #[br(pre_assert(opcode == 11330))]
    MoveEvent {
        target: u32,
        frequency: u32,
        #[br(parse_with = binrw::helpers::until_eof)]
        rest: Vec<u32>, // TODO
    },
    #[br(pre_assert(opcode == 11340))]
    WaitForAllMovement, // TODO
    #[br(pre_assert(opcode == 11350))]
    HaltAllMovement, // TODO
    #[br(pre_assert(opcode == 11410))]
    Wait {
        deciseconds: u32,
        #[br(try)]
        unknown: Option<u32>,
    },
    #[br(pre_assert(opcode == 11510))]
    PlayBGM {
        /// - Unit: milliseconds
        /// - Range: 0 - 10,000, step size 1000
        fade_in: u32,
        /// - Unit: percentage
        /// - Range: 0 - 100
        volume: u32,
        /// - Unit: percentage
        /// - Range: 50 - 150
        tempo: u32,
        /// - Range: 0 - 100
        /// - 0: Left
        /// - 50: Center
        /// - 100: Right
        balance: u32,
    },
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
    PlayMovie {
        width: u32,
        height: u32,
        /// - 0: Constant
        /// - 1: Variable
        operand: u32,
        x: u32,
        y: u32,
    },
    #[br(pre_assert(opcode == 11610))]
    KeyInputProcessing {
        /// There are 3 different versions of this instruction.
        /// In old r2k it has 5 args, in new r2k it has 10, and in r2k3 it has 14.
        /// # Common fields:
        /// 1. output variable (expect this variable to be completely trashed and unusable for other purposes.)
        /// 2. wait for key press (bool)
        /// 3. decision key (Space/Z/Enter)
        /// 4. cancel key (Escape/X/C/V/B/N/Kp0)
        /// # Old r2k:
        /// 5. check all directions
        /// # New r2k:
        /// 5. unused/unknown (always 0)
        /// 6. shift
        /// 7. down
        /// 8. left
        /// 9. right
        /// 10. up
        /// # r2k3:
        /// 5. unused/unknown
        /// 6.
        /// 7.
        /// 8.
        /// 9.
        /// 10. shift
        /// 11. down
        /// 12. left
        /// 13. right
        /// 14. up
        #[br(parse_with(binrw::helpers::until_eof))]
        args: Vec<u32>,
    },
    #[br(pre_assert(opcode == 11710))]
    ChangeMapTileset { tileset: u32 },
    #[br(pre_assert(opcode == 11720))]
    ChangeParallaxBackground {
        /// - Type: bool
        horizontal_loop: u32,
        /// - Type: bool
        horizontal_auto_scroll: u32,
        /// - Range: -8 - 8
        horizontal_auto_scroll_speed: u32,
        /// - Type: bool
        vertical_loop: u32,
        /// - Type: bool
        vertical_auto_scroll: u32,
        /// - Range: -8 - 8
        vertical_auto_scroll_speed: u32,
    },
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
    SetTeleportPoint {
        /// - 0: Add
        /// - 1: Remove
        operation: u32,
        map: u32,
        x: u32,
        y: u32,
        switch_enabled: u32,
        switch: u32,
    },
    #[br(pre_assert(opcode == 11820))]
    ChangeTeleportAccess, // TODO
    #[br(pre_assert(opcode == 11830))]
    SetEscapeLocation {
        map: u32,
        x: u32,
        y: u32,
        /// - Type: bool
        switch_enabled: u32,
        switch: u32,
    },
    #[br(pre_assert(opcode == 11840))]
    ChangeEscapeAccess {
        /// - Type: bool
        enabled: u32,
    }, // TODO
    #[br(pre_assert(opcode == 11910))]
    OpenSaveMenu,
    #[br(pre_assert(opcode == 11930))]
    ChangeSaveAccess {
        /// - Type: bool
        state: u32,
    }, // TODO
    #[br(pre_assert(opcode == 11950))]
    OpenMainMenu, // TODO
    #[br(pre_assert(opcode == 11960))]
    ChangeMenuAccess {
        /// - Type: bool
        enabled: u32,
    },
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
    Label {
        /// Range: 1 - 1000
        value: u32,
    },
    #[br(pre_assert(opcode == 12120))]
    JumpToLabel {
        /// Range: 1 - 1000
        value: u32,
    },
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
    ShowChoiceOption { index: u32 },
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
            Self::MessageOptions { .. } => 10120,
            Self::ChangeFaceset { .. } => 10130,
            Self::ShowChoice { .. } => 10140,
            Self::InputNumber { .. } => 10150,
            Self::ControlSwitches { .. } => 10210,
            Self::ControlVariables { .. } => 10220,
            Self::ControlTimer { .. } => 10230,
            Self::ChangeGold { .. } => 10310,
            Self::ChangeItems { .. } => 10320,
            Self::ChangePartyMembers { .. } => 10330,
            Self::ChangeExp => 10410,
            Self::ChangeLevel { .. } => 10420,
            Self::ChangeParameters { .. } => 10430,
            Self::ChangeSkills { .. } => 10440,
            Self::ChangeEquipment { .. } => 10450,
            Self::ChangeHP => 10460,
            Self::ChangeSP => 10470,
            Self::ChangeCondition => 10480,
            Self::RecoverAll { .. } => 10490,
            Self::SimulatedAttack => 10500,
            Self::ChangeActorName { .. } => 10610,
            Self::ChangeActorNickname { .. } => 10620,
            Self::ChangeActorGraphic { .. } => 10630,
            Self::ChangeActorFaceset { .. } => 10640,
            Self::ChangeVehicleGraphic => 10650,
            Self::ChangeSystemBGM => 10660,
            Self::ChangeSystemSE { .. } => 10670,
            Self::ChangeSystemGraphics { .. } => 10680,
            Self::ChangeScreenTransitions { .. } => 10690,
            Self::EnemyEncounter => 10710,
            Self::OpenShop { .. } => 10720,
            Self::ShowInn => 10730,
            Self::NameInputProcessing { .. } => 10740,
            Self::TransferPlayer { .. } => 10810,
            Self::GetPlayerLocation { .. } => 10820,
            Self::MoveToVariableLocation { .. } => 10830,
            Self::EnterExitVehicle => 10840,
            Self::SetVehicleLocation { .. } => 10850,
            Self::SetEventLocation { .. } => 10860,
            Self::SwapEventLocation { .. } => 10870,
            Self::StoreTerrainID { .. } => 10910,
            Self::GetEventLocation { .. } => 10920,
            Self::HideScreen { .. } => 11010,
            Self::ShowScreen { .. } => 11020,
            Self::TintScreen { .. } => 11030,
            Self::FlashScreen { .. } => 11040,
            Self::ShakeScreen { .. } => 11050,
            Self::ScrollMap { .. } => 11060,
            Self::WeatherEffects { .. } => 11070,
            Self::ShowPicture { .. } => 11110,
            Self::MovePicture { .. } => 11120,
            Self::ErasePicture { .. } => 11130,
            Self::ShowAnimation { .. } => 11210,
            Self::ShowHidePlayer { .. } => 11310,
            Self::FlashEvent { .. } => 11320,
            Self::MoveEvent { .. } => 11330,
            Self::WaitForAllMovement => 11340,
            Self::HaltAllMovement => 11350,
            Self::Wait { .. } => 11410,
            Self::PlayBGM { .. } => 11510,
            Self::FadeOutBGM { .. } => 11520,
            Self::MemorizeBGM => 11530,
            Self::PlayMemorizedBGM => 11540,
            Self::PlaySoundEffect { .. } => 11550,
            Self::PlayMovie { .. } => 11560,
            Self::KeyInputProcessing { .. } => 11610,
            Self::ChangeMapTileset { .. } => 11710,
            Self::ChangeParallaxBackground { .. } => 11720,
            Self::ChangeEncounterSteps => 11740,
            Self::ChangeTile { .. } => 11750,
            Self::SetTeleportPoint { .. } => 11810,
            Self::ChangeTeleportAccess => 11820,
            Self::SetEscapeLocation { .. } => 11830,
            Self::ChangeEscapeAccess { .. } => 11840,
            Self::OpenSaveMenu => 11910,
            Self::ChangeSaveAccess { .. } => 11930,
            Self::OpenMainMenu => 11950,
            Self::ChangeMenuAccess { .. } => 11960,
            Self::ConditionalBranch { .. } => 12010,
            Self::Label { .. } => 12110,
            Self::JumpToLabel { .. } => 12120,
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
            Self::ShowChoiceOption { .. } => 20140,
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
