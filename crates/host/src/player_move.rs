//! The player's movement through the game's own translated code (Build 43).
//!
//! The viewer moves the player each frame. With this hook it first offers
//! the frame's move to the game's code in the host ([`PlayerMovement::step`]):
//! `PlayerMover::Update` (`009e9e50`, the mover's vtable +0x14, as
//! `PlayerCharacter::Update` calls it) → `Actor::Move` (`008a62b0`, player
//! vtable +0x250) → `MobileObject::Move` (`0092f260`) →
//! `bhkCharacterController::Move` (`00c73170`) → the character proxy, the
//! phantom's linear casts, the collision agents and shapes. When that run
//! reaches an address nobody has translated yet (or memory nobody made),
//! the miss is recorded once per address with its count and the nearest
//! caller entered by address ([`Miss`]), and the viewer moves the player
//! with its own `crates/physics` step for that frame, so the game stays
//! playable. [`PlayerMovement::missing_tsv`] is the list of those
//! addresses, for the translators.
//!
//! The game state the move needs is made in the game's memory by the
//! game's own code, in the order the game makes it ([`PlayerMovement::prepare`]):
//!
//! 1. `havok`: `InitHavokSystem` (`00624bf0`, which `WinMain` runs before
//!    `InitTES`).
//! 2. `player`: as `Main::InitTES` (`0086cf20`) makes it: `operator new`
//!    (0xE50 bytes), `PlayerCharacter::PlayerCharacter` (`00938180`), the
//!    pointer at `011dea3c`, form id 0x14 (vtable +0x128, `(0x14, 1)`); then,
//!    once the files are loaded, its base form (`NPC_` 0x7, loaded below) by
//!    `TESObjectREFR::SetObjectReference` (`00575690`), its position (vtable
//!    +0x2A8), its angle (`TESObjectREFR::SetAngleOnReference`, `0086d490`)
//!    and `InitItem` (vtable +0x88). `InitTES`'s `Actor::CastPermanentMagic`
//!    is left out (no movement state).
//! 3. `cell`: the player's cell, loaded as the game loads a form (below),
//!    its temporary references by `TESObjectCELL::LoadTempDataFromFile`
//!    (`00550500`, the file standing at the cell's record), their base forms
//!    first; then its 3D by `TESObjectCELL::Load3D` (`00545cf0`, which makes
//!    the cell's `bhkWorld` in `00552dc0`) and each reference's by
//!    `TESObjectREFR::Load3D` (`0056b2d0`), which adds its collision to the
//!    cell's world.
//! 4. `controller`: the player in the cell: `TESObjectREFR::SetParentCell`
//!    (`00575bb0`), `PlayerCharacter::Load3D` (`0094e1d0`) and
//!    `MobileObject::InitHavok` (`00930c70`, which constructs the
//!    `bhkCharacterController` and its proxy, `00c741e0`).
//!
//! A form is loaded as `TESDataHandler` loads one: `CreateFormOfType`
//! (`00465110`), `TESFile::SetOffset` at its record ([`GameFile::supply`]),
//! `TESDataHandler::LoadForm` (`004601d0`: the form's `Load`, vtable +0x20)
//! and `InitItem` (vtable +0x88). The game runs every form's `InitItem` once
//! all files are loaded; here each runs right after its own load (the order
//! is the bridge's, not the game's).
//!
//! Bridged (stand-ins for exe code, each named):
//! - The file reading below `TESFile` (`BSFile`, platform; `crate::game_file`):
//!   each plugin is opened by the game's `OpenTES` and header reader with
//!   an in-memory `BSFile` already in place, and its masters resolved by
//!   `TESFile::GenIndexTable` against a file list made here (standing in
//!   for the data handler's, [`GameFile::open_with_header`]). Compressed
//!   records are decompressed by the game's own reading (zlib included).
//! - The keys: the game's control handler (`PlayerCharacter::Update`'s
//!   `0093e860`) reads them through `OSInputGlobals` (DirectInput); here the
//!   viewer's keys give the mover's move bits, set with the game's own
//!   `PlayerMover::ForceMoveMode` (`009ea3e0`), and the camera's heading is
//!   set with `SetAngleOnReference`.
//! - Back in step: after a frame the viewer moved itself (or the first
//!   frame), the player is put where the viewer has it (vtable +0x2A8, the
//!   game's `SetPosition`), so the two paths start each frame together.
//! - Jumping is not connected (the jump control's path, player vtable
//!   +0x254, isn't wired): a frame with a jump goes to `crates/physics`.

use crate::game_file::GameFile;
use engine::{Engine, Ret};
use esm::{FormId, LoadOrder};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::rc::Rc;

/// `operator new` (`00401000`).
const NEW: u32 = 0x0040_1000;
/// `InitHavokSystem` (`00624bf0`).
const INIT_HAVOK_SYSTEM: u32 = 0x0062_4bf0;
/// `PlayerCharacter::PlayerCharacter` (Xbox PDB, `00938180`) and its size.
const PLAYER_CONSTRUCT: u32 = 0x0093_8180;
const PLAYER_SIZE: u32 = 0xe50;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The player's base form (`NPC_` 0x7, `InitTES`: `004839c0(7)`).
const PLAYER_BASE: FormId = FormId(7);
/// `TESForm` vtable slots: `SetFormID(id, 1)` (+0x128), `InitItem` (+0x88).
const SET_FORM_ID: u32 = 0x128;
const INIT_ITEM: u32 = 0x88;
/// `TESObjectREFR` vtable slots: `SetPosition(NiPoint3 *)` (+0x2A8), the
/// position (`GetPos`, +0x1F4).
const SET_POSITION: u32 = 0x2a8;
const GET_POSITION: u32 = 0x1f4;
/// `TESObjectREFR::SetObjectReference` (`00575690`), `SetAngleOnReference`
/// (`0086d490`), `SetParentCell` (`00575bb0`), `Load3D` (`0056b2d0`).
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
const SET_ANGLE: u32 = 0x0086_d490;
const SET_PARENT_CELL: u32 = 0x0057_5bb0;
const REFERENCE_LOAD_3D: u32 = 0x0056_b2d0;
/// `PlayerCharacter::Load3D` (`0094e1d0`), `MobileObject::InitHavok`
/// (`00930c70`), `MobileObject::GetCharController` (`009306d0`).
const PLAYER_LOAD_3D: u32 = 0x0094_e1d0;
const INIT_HAVOK: u32 = 0x0093_0c70;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `TESDataHandler::CreateFormOfType` (`00465110`), `TESDataHandler::LoadForm`
/// (`004601d0`), `LookupFormByID` (`004839c0`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
const LOAD_FORM: u32 = 0x0046_01d0;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `TESObjectCELL::LoadTempDataFromFile` (`00550500`), `Load3D` (`00545cf0`).
const CELL_LOAD_TEMP_DATA: u32 = 0x0055_0500;
const CELL_LOAD_3D: u32 = 0x0054_5cf0;
/// `Actor::pActorMover` (Xbox PDB) +0x190; the mover's `Update(seconds)`
/// (vtable +0x14); `PlayerMover::ForceMoveMode` (`009ea3e0`).
const ACTOR_MOVER: u32 = 0x190;
const MOVER_UPDATE: u32 = 0x14;
const FORCE_MOVE_MODE: u32 = 0x009e_a3e0;
/// A record header's size, and a group's.
const HEADER_SIZE: usize = 0x18;
/// `TESObjectREFR` vtable slots: `InitHavok` (+0x1C4, Xbox PDB name; PC
/// `005768b0`: adds the 3D's collision to the cell's `bhkWorld` through
/// `bhkWorld::AddObjects`, world vtable +0xD0) and `Get3D` (+0x1D0).
const REFERENCE_INIT_HAVOK: u32 = 0x1c4;
const GET_3D: u32 = 0x1d0;
/// `bhkWorld::SetDeltaTime(seconds, mode 4, force)` (`00c66760`, as
/// `Main::OnIdle_UpdateTimer` `0086f260` calls it each frame).
const SET_DELTA_TIME: u32 = 0x00c6_6760;
/// `TESObjectCELL::UpdateManagedNodes(time, 0)` (`00551890`): the cell's
/// world step (vtable +0xC4, `bhkWorld::Update`) and the managed references'
/// scene-graph update that brings the bodies' transforms to their nodes.
const UPDATE_MANAGED_NODES: u32 = 0x0055_1890;
/// The time `TES::UpdateCellAnimations` (`00453550`) adds each frame's
/// seconds to and passes to `UpdateManagedNodes` (float, `011c3c08`).
const CELL_ANIMATION_TIME: u32 = 0x011c_3c08;
/// `NiAVObject` world rotation (`NiMatrix3`, rows) and translation, PC
/// offsets (`bhkCollisionObject::hktoNiTransform` `00c65a80` reads them).
const WORLD_ROTATION: u32 = 0x68;
const WORLD_TRANSLATION: u32 = 0x8c;

/// Where the game's world step put a reference's 3D: its root node's world
/// rotation (rows) and translation, game units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyPose {
    pub reference: FormId,
    pub rotation: [[f32; 3]; 3],
    pub translation: [f32; 3],
}

/// The mover's move bits (`PlayerMover::MoveMode`, as `0093e860` sets them).
pub mod move_bits {
    pub const FORWARD: u32 = 0x1;
    pub const BACKWARD: u32 = 0x2;
    pub const LEFT: u32 = 0x4;
    pub const RIGHT: u32 = 0x8;
    pub const WALKING: u32 = 0x100;
    pub const RUNNING: u32 = 0x200;
    pub const SNEAKING: u32 = 0x400;
}

/// A frame's move as the viewer has it.
#[derive(Debug, Clone, Copy, Default)]
pub struct MoveInput {
    /// Where the viewer has the player's feet (game units).
    pub feet: [f32; 3],
    /// The heading (the game's z angle, radians).
    pub heading: f32,
    /// [`move_bits`].
    pub move_bits: u32,
    /// The frame's time step (seconds).
    pub seconds: f32,
    /// A jump asked for this frame.
    pub jump: bool,
}

/// Where the game's code put the player.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MoveOutput {
    pub feet: [f32; 3],
    /// The character controller's `OutVelocity` (+0x4F0), game units/s.
    pub velocity: [f32; 3],
}

/// An untranslated address the movement reached.
#[derive(Debug, Clone, Default)]
pub struct Miss {
    /// Times reached (each a stopped stage or frame; in a probe run, every
    /// call).
    pub count: u64,
    /// The nearest caller entered by address, with how often.
    pub callers: BTreeMap<u32, u64>,
    /// The stage that reached it first (`havok`, `player`, `cell`,
    /// `controller`, `frame`).
    pub stage: &'static str,
    /// Its place in the order the addresses were first reached (0 first).
    pub first: usize,
}

/// A stop that isn't an untranslated function (memory nobody made, a
/// refusal of the bridge), by stage and first line.
pub type Stops = BTreeMap<(&'static str, String), u64>;

#[derive(Debug, Clone, PartialEq)]
enum Stage<T> {
    NotTried,
    Done(T),
    Failed(String),
}

impl<T: Copy> Stage<T> {
    fn done(&self) -> Option<T> {
        match self {
            Stage::Done(v) => Some(*v),
            _ => None,
        }
    }
}

/// The player's movement through the game's code: what was made, and what
/// was missing.
pub struct PlayerMovement {
    /// Go on past an untranslated address with a zero result instead of
    /// stopping (a diagnostic run: the addresses reached after the first
    /// miss are only indicative, and the state made is not used to move
    /// the player).
    pub probe: bool,
    pub misses: BTreeMap<u32, Miss>,
    pub stops: Stops,
    /// Frames the game's code moved the player, and frames it didn't.
    pub engine_frames: u64,
    pub fallback_frames: u64,
    havok: Stage<()>,
    player: Stage<u32>,
    /// The cell made, by its form id.
    cell: Option<(FormId, Stage<u32>)>,
    controller: Stage<u32>,
    files: HashMap<u8, GameFile>,
    failed_files: HashMap<u8, String>,
    forms: HashMap<FormId, u32>,
    /// The last frame's result came from the game's code (no resync needed).
    in_step: bool,
    /// The references of the made cell whose 3D loaded, by form id.
    cell_refs: Vec<(FormId, u32)>,
    /// Frames the game's world step ran, and frames it didn't
    /// ([`PlayerMovement::step_world`]).
    pub world_frames: u64,
    pub world_fallback_frames: u64,
    /// The world step stopped once; it isn't tried again in this cell.
    world_failed: Option<String>,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        PlayerMovement {
            probe: false,
            misses: BTreeMap::new(),
            stops: BTreeMap::new(),
            engine_frames: 0,
            fallback_frames: 0,
            havok: Stage::NotTried,
            player: Stage::NotTried,
            cell: None,
            controller: Stage::NotTried,
            files: HashMap::new(),
            failed_files: HashMap::new(),
            forms: HashMap::new(),
            in_step: false,
            cell_refs: Vec::new(),
            world_frames: 0,
            world_fallback_frames: 0,
            world_failed: None,
        }
    }
}

/// What a guarded run reached: untranslated addresses with their nearest
/// caller, in order.
type Reached = Rc<RefCell<Vec<(u32, u32)>>>;

impl PlayerMovement {
    /// Runs `f` on the engine with the miss recorder in place; a stop comes
    /// back as its first line. The misses are counted under `stage`.
    fn guarded<R>(
        &mut self,
        e: &mut Engine,
        stage: &'static str,
        f: impl FnOnce(&mut Engine) -> Result<R, String>,
    ) -> Result<R, String> {
        let reached: Reached = Rc::new(RefCell::new(Vec::new()));
        let record = reached.clone();
        let probe = self.probe;
        e.on_open = Some(Rc::new(RefCell::new(
            move |e: &mut Engine, addr: u32, _: &[u32]| {
                let caller = e.call_stack.last().copied().unwrap_or(0);
                record.borrow_mut().push((addr, caller));
                probe.then(Ret::default)
            },
        )));
        e.call_stack.clear();
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e)));
        std::panic::set_hook(hook);
        e.on_open = None;
        // After a panic the stack still holds the functions it unwound
        // through (`Engine::call` pops only on return): the innermost few go
        // with the stop, so a fault inside translated code says where.
        let unwound: Vec<u32> = e.call_stack.iter().rev().take(6).copied().collect();
        e.call_stack.clear();
        for &(addr, caller) in reached.borrow().iter() {
            let first = self.misses.len();
            let miss = self.misses.entry(addr).or_insert_with(|| Miss {
                stage,
                first,
                ..Miss::default()
            });
            miss.count += 1;
            *miss.callers.entry(caller).or_default() += 1;
        }
        let out = match out {
            Ok(result) => result,
            Err(payload) => {
                let text = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                let first = text.lines().next().unwrap_or("");
                if unwound.is_empty() {
                    Err(first.to_string())
                } else {
                    let path: Vec<String> = unwound.iter().map(|a| format!("{a:08x}")).collect();
                    Err(format!("{first} (in {})", path.join(" < ")))
                }
            }
        };
        if let Err(why) = &out {
            if !why.starts_with("open function") {
                *self.stops.entry((stage, why.clone())).or_default() += 1;
            }
        }
        // In a probe run a miss doesn't stop the run, so the stages after it
        // run too; the state made is not the game's, and `step` never moves
        // the player with it.
        out
    }

    /// Makes what the move needs that isn't made yet (the module's stages),
    /// for the player in `cell` at `feet`. Each stage runs once (per cell for
    /// the cell's); one that stopped isn't run again. True when everything is
    /// made.
    pub fn prepare(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> bool {
        if self.havok == Stage::NotTried {
            self.havok = match self.guarded(e, "havok", |e| {
                e.call(INIT_HAVOK_SYSTEM, &[]);
                Ok(())
            }) {
                Ok(()) => Stage::Done(()),
                Err(why) => Stage::Failed(why),
            };
        }
        if self.player == Stage::NotTried {
            let made = self.make_player(e, order, input);
            self.player = match made {
                Ok(p) => Stage::Done(p),
                Err(why) => Stage::Failed(why),
            };
        }
        let Some(cell) = cell else {
            return false;
        };
        if self.cell.as_ref().map(|(id, _)| *id) != Some(cell) {
            // A new cell: the player's controller is made again in it.
            self.cell_refs.clear();
            self.world_failed = None;
            let stage = if self.havok.done().is_some() {
                match self.make_cell(e, order, cell) {
                    Ok(c) => Stage::Done(c),
                    Err(why) => Stage::Failed(why),
                }
            } else {
                Stage::Failed("needs the havok stage".into())
            };
            self.cell = Some((cell, stage));
            self.controller = Stage::NotTried;
        }
        let made_cell = self.cell.as_ref().and_then(|(_, s)| s.done());
        if self.controller == Stage::NotTried {
            self.controller = match (self.player.done(), made_cell) {
                (Some(player), Some(cell)) => match self.guarded(e, "controller", |e| {
                    e.call(SET_PARENT_CELL, &[player, cell]);
                    set_position(e, player, input.feet);
                    e.call(PLAYER_LOAD_3D, &[player, 0]);
                    e.call(INIT_HAVOK, &[player]);
                    let controller = e.call(GET_CHAR_CONTROLLER, &[player]).u32();
                    if controller == 0 {
                        return Err("MobileObject::InitHavok made no character controller".into());
                    }
                    Ok(controller)
                }) {
                    Ok(c) => Stage::Done(c),
                    Err(why) => Stage::Failed(why),
                },
                _ => Stage::Failed("needs the player and cell stages".into()),
            };
        }
        self.controller.done().is_some()
    }

    /// The player as `Main::InitTES` makes it (the module, stage 2).
    fn make_player(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        input: &MoveInput,
    ) -> Result<u32, String> {
        let player = self.guarded(e, "player", |e| {
            let block = e.call(NEW, &[PLAYER_SIZE]).u32();
            let player = e.call(PLAYER_CONSTRUCT, &[block]).u32();
            e.mem.set_u32(PLAYER, player);
            e.vcall(player, SET_FORM_ID, &[0x14, 1]);
            Ok(player)
        })?;
        let base = self.load_form(e, order, PLAYER_BASE)?;
        self.guarded(e, "player", |e| {
            e.call(SET_OBJECT_REFERENCE, &[player, base]);
            set_position(e, player, input.feet);
            set_heading(e, player, input.heading);
            e.vcall(player, INIT_ITEM, &[]);
            Ok(player)
        })
    }

    /// The plugin's file in the game's memory, made on first need.
    /// Opened by the game's `OpenTES` and header reader
    /// ([`GameFile::open_with_header`]), its masters first; then its master
    /// table by `TESFile::GenIndexTable` ([`crate::game_file::gen_index_table`])
    /// against every file opened so far.
    fn file(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        plugin: &esm::LoadedPlugin,
    ) -> Result<GameFile, String> {
        if let Some(file) = self.files.get(&plugin.load_index) {
            return Ok(*file);
        }
        if let Some(why) = self.failed_files.get(&plugin.load_index) {
            return Err(why.clone());
        }
        let made = self.open_file(e, order, plugin);
        if let Err(why) = &made {
            self.failed_files.insert(plugin.load_index, why.clone());
        }
        made
    }

    fn open_file(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        plugin: &esm::LoadedPlugin,
    ) -> Result<GameFile, String> {
        for master in &plugin.plugin.header().masters {
            let Some(m) = order
                .plugins()
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(master))
            else {
                return Err(format!("{}: master {master} not loaded", plugin.name));
            };
            self.file(e, order, m)?;
        }
        let directory = plugin
            .path
            .as_ref()
            .and_then(|p| p.parent())
            .map(|d| format!("{}\\", d.display()))
            .unwrap_or_else(|| "Data\\".into());
        let file = self.guarded(e, "files", |e| {
            GameFile::open_with_header(e, plugin, &directory)
        })?;
        self.files.insert(plugin.load_index, file);
        let mut opened: Vec<(u8, GameFile)> = self.files.iter().map(|(i, f)| (*i, *f)).collect();
        opened.sort_by_key(|(i, _)| *i);
        let opened: Vec<GameFile> = opened.into_iter().map(|(_, f)| f).collect();
        self.guarded(e, "files", |e| {
            if crate::game_file::gen_index_table(e, &file, &opened) {
                Ok(())
            } else {
                Err(format!(
                    "{}: TESFile::GenIndexTable missed a master",
                    plugin.name
                ))
            }
        })?;
        Ok(file)
    }

    /// Loads the form `id` as `TESDataHandler` does (see the module), once.
    fn load_form(&mut self, e: &mut Engine, order: &LoadOrder, id: FormId) -> Result<u32, String> {
        if let Some(form) = self.forms.get(&id) {
            return Ok(*form);
        }
        let record = order
            .get(id)
            .ok_or_else(|| format!("form {id}: not in the load order"))?;
        let kind = record.entry.header.kind;
        let form_type = world::more_functions::form_type_number(kind)
            .ok_or_else(|| format!("form {id}: record type {kind} has no form type"))?;
        let file = self.file(e, order, record.plugin)?;
        let plugin = record.plugin;
        let entry = record.entry;
        let form = self.guarded(e, "forms", |e| {
            let form = e.call(CREATE_FORM_OF_TYPE, &[u32::from(form_type)]).u32();
            if form == 0 {
                return Err(format!(
                    "form {id}: CreateFormOfType({form_type:#x}) gave none"
                ));
            }
            file.supply_any(e, plugin, entry)?;
            if !e.call(LOAD_FORM, &[form, file.file]).bool() {
                return Err(format!("form {id}: {kind} Load failed"));
            }
            e.vcall(form, INIT_ITEM, &[]);
            Ok(form)
        })?;
        self.forms.insert(id, form);
        Ok(form)
    }

    /// The player's cell, its temporary references and their 3D (the
    /// module, stage 3).
    fn make_cell(&mut self, e: &mut Engine, order: &LoadOrder, id: FormId) -> Result<u32, String> {
        let record = order
            .get(id)
            .ok_or_else(|| format!("cell {id}: not in the load order"))?;
        let plugin = record.plugin;
        let local = record.entry.header.form_id;
        // The cell's placed records (its children groups) and their base
        // forms.
        let children: Vec<&esm::RecordEntry> = plugin
            .plugin
            .records()
            .iter()
            .filter(|r| r.cell == Some(local))
            .collect();
        let mut bases = Vec::new();
        for child in &children {
            let kind = child.header.kind.as_bytes();
            if kind != b"REFR" && kind != b"ACHR" && kind != b"ACRE" {
                continue;
            }
            let name = plugin
                .plugin
                .subrecord_of(child, esm::FourCC(*b"NAME"))
                .map_err(|err| err.to_string())?;
            if let Some(name) = name.filter(|n| n.len() >= 4) {
                let base = FormId(u32::from_le_bytes([name[0], name[1], name[2], name[3]]));
                bases.push(plugin.to_global(base));
            }
        }
        bases.sort();
        bases.dedup();
        let mut unloaded = 0;
        for base in bases {
            if self.load_form(e, order, base).is_err() {
                unloaded += 1;
            }
        }
        if unloaded > 0 {
            *self
                .stops
                .entry((
                    "cell",
                    format!("{unloaded} base forms of the cell's references didn't load"),
                ))
                .or_default() += 1;
        }
        let cell = self.load_form(e, order, id)?;
        let file = self.file(e, order, plugin)?;
        // The cell's record and its children groups, as one range of the
        // file (the groups follow the record), and the 0x18 bytes after it.
        let start = record.entry.offset;
        let end = children
            .iter()
            .map(|r| r.offset + plugin.plugin.record_bytes(r).len())
            .chain(std::iter::once(
                start + plugin.plugin.record_bytes(record.entry).len(),
            ))
            .max()
            .unwrap_or(start)
            + HEADER_SIZE;
        let bytes = plugin
            .plugin
            .file_bytes(start..end.min(plugin.plugin.file_size()));
        let refs: Vec<FormId> = children
            .iter()
            .filter(|r| r.header.kind.as_bytes() == b"REFR")
            .map(|r| plugin.to_global(r.header.form_id))
            .collect();
        self.guarded(e, "cell", |e| {
            engine::units::platform::supply_file_bytes(e, file.bs_file, start as u32, bytes);
            if !e.call(0x0047_23a0, &[file.file, start as u32]).bool() {
                return Err(format!("cell {id}: TESFile::SetOffset failed"));
            }
            if !e.call(CELL_LOAD_TEMP_DATA, &[cell, file.file]).bool() {
                return Err(format!("cell {id}: LoadTempDataFromFile failed"));
            }
            e.call(CELL_LOAD_3D, &[cell]);
            Ok(())
        })?;
        for reference in refs {
            let made =
                self.guarded(e, "cell", |e| Ok(e.call(LOOKUP_FORM, &[reference.0]).u32()))?;
            if made == 0 {
                continue;
            }
            if self
                .guarded(e, "cell", |e| {
                    e.call(REFERENCE_LOAD_3D, &[made, 0]);
                    Ok(())
                })
                .is_ok()
            {
                self.cell_refs.push((reference, made));
                // Bridge: the game adds a reference's collision to the
                // world when its queued 3D task is unpacked on the main
                // thread (`TaskUnpackFunc` `0087b990`, the reference's vtable
                // +0x1C4 at `0087cbf3`); here right after its load.
                let _ = self.guarded(e, "cell", |e| {
                    e.vcall(made, REFERENCE_INIT_HAVOK, &[]);
                    Ok(())
                });
            }
        }
        Ok(cell)
    }

    /// The frame's move through the game's code; `None` when it couldn't run
    /// (the misses are recorded) and the viewer moves the player itself.
    pub fn step(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> Option<MoveOutput> {
        let ready = self.prepare(e, order, cell, input);
        let (player, controller) = (self.player.done(), self.controller.done());
        // A probe run goes through the frame with whatever was made (and
        // never moves the player).
        let runs = if self.probe {
            player.is_some()
        } else {
            ready && player.is_some() && controller.is_some()
        };
        if !runs || input.jump {
            self.fallback_frames += 1;
            self.in_step = false;
            return None;
        }
        let (player, controller) = (player.unwrap_or(0), controller.unwrap_or(0));
        let resync = !self.in_step;
        let out = self.guarded(e, "frame", |e| {
            if resync {
                set_position(e, player, input.feet);
            }
            set_heading(e, player, input.heading);
            let mover = e.mem.u32(player + ACTOR_MOVER);
            e.call(FORCE_MOVE_MODE, &[mover, input.move_bits]);
            e.vcall(mover, MOVER_UPDATE, &[input.seconds.to_bits()]);
            let position = e.vcall(player, GET_POSITION, &[]).u32();
            let feet = [0, 4, 8].map(|o| e.mem.f32(position + o));
            let velocity = [0, 4, 8].map(|o| e.mem.f32(controller + 0x4f0 + o));
            Ok(MoveOutput { feet, velocity })
        });
        match out {
            Ok(out) if !self.probe => {
                self.engine_frames += 1;
                self.in_step = true;
                Some(out)
            }
            _ => {
                self.fallback_frames += 1;
                self.in_step = false;
                None
            }
        }
    }

    /// The frame's Havok world through the game's code (`--physics engine`):
    /// the frame timer's `bhkWorld::SetDeltaTime` (as `0086f260` calls it),
    /// then, as `TES::UpdateCellAnimations` (`00453550`) does for an
    /// interior, the frame's seconds added to its time (`011c3c08`) and
    /// `TESObjectCELL::UpdateManagedNodes` (`00551890`) on the cell: the
    /// world's step (`bhkWorld::Update`, batched adds, `hkpWorld::stepDeltaTime`
    /// until the frame's marker) and the scene-graph update that copies each
    /// moving body's transform to its node (`bhkCollisionObject::UpdateWorldData`).
    /// Returns every loaded reference's root node where it is now, or `None`
    /// when the world isn't made or the step stopped (the misses are
    /// recorded under `world`; a stopped step isn't tried again in the cell)
    /// and the viewer steps its own bodies.
    ///
    /// Bridged: the rest of `0086f260` and `00453550` (the wind, the sky,
    /// `LockHavokUpdateMT`, which is a critical section only with more than
    /// one thread) doesn't run; the cell is the one the stages made.
    pub fn step_world(&mut self, e: &mut Engine, seconds: f32) -> Option<Vec<BodyPose>> {
        let cell = match (&self.havok, &self.cell, &self.world_failed) {
            (Stage::Done(()), Some((_, Stage::Done(cell))), None) => *cell,
            _ => {
                self.world_fallback_frames += 1;
                return None;
            }
        };
        let refs = self.cell_refs.clone();
        let out = self.guarded(e, "world", |e| {
            e.call(SET_DELTA_TIME, &[seconds.to_bits(), 0, 0]);
            let time = e.mem.f32(CELL_ANIMATION_TIME) + seconds;
            e.mem.set_f32(CELL_ANIMATION_TIME, time);
            e.call(UPDATE_MANAGED_NODES, &[cell, time.to_bits(), 0]);
            let mut poses = Vec::with_capacity(refs.len());
            for &(reference, made) in &refs {
                let root = e.vcall(made, GET_3D, &[]).u32();
                if root == 0 {
                    continue;
                }
                let r = |i: u32| e.mem.f32(root + WORLD_ROTATION + 4 * i);
                let rotation = [[r(0), r(1), r(2)], [r(3), r(4), r(5)], [r(6), r(7), r(8)]];
                let translation = [0, 4, 8].map(|o| e.mem.f32(root + WORLD_TRANSLATION + o));
                poses.push(BodyPose {
                    reference,
                    rotation,
                    translation,
                });
            }
            Ok(poses)
        });
        match out {
            Ok(poses) if !self.probe => {
                self.world_frames += 1;
                Some(poses)
            }
            Ok(_) => {
                self.world_fallback_frames += 1;
                None
            }
            Err(why) => {
                self.world_fallback_frames += 1;
                self.world_failed = Some(why);
                None
            }
        }
    }

    /// Whether the stages are made, as a line.
    pub fn summary(&self) -> String {
        fn word<T>(s: &Stage<T>) -> String {
            match s {
                Stage::NotTried => "not tried".into(),
                Stage::Done(_) => "made".into(),
                Stage::Failed(why) => format!("stopped ({why})"),
            }
        }
        let cell = match &self.cell {
            Some((id, s)) => format!("{id}: {}", word(s)),
            None => "none".into(),
        };
        let mut line = format!(
            "Game's movement: {} frames moved by the game's code, {} by crates/physics; \
             havok {}; player {}; cell {cell}; controller {}; {} untranslated addresses reached.",
            self.engine_frames,
            self.fallback_frames,
            word(&self.havok),
            word(&self.player),
            word(&self.controller),
            self.misses.len()
        );
        if self.world_frames + self.world_fallback_frames > 0 {
            let _ = write!(
                line,
                " Game's physics: {} world steps by the game's code, {} by crates/physics{}.",
                self.world_frames,
                self.world_fallback_frames,
                match &self.world_failed {
                    Some(why) => format!(" (stopped: {why})"),
                    None => String::new(),
                }
            );
        }
        line
    }

    /// The other stops (memory nobody made, a refusal of the bridge), most
    /// first: count, stage, first line.
    pub fn stops_tsv(&self) -> String {
        let mut rows: Vec<_> = self.stops.iter().collect();
        rows.sort_by(|a, b| b.1.cmp(a.1));
        let mut out = String::from("count\tstage\tstop\n");
        for ((stage, why), n) in rows {
            let _ = writeln!(out, "{n}\t{stage}\t{why}");
        }
        out
    }

    /// The untranslated addresses reached, most reached first: address,
    /// count, stage, callers (`caller×count`), the order they were first
    /// reached in, and the ledger's unit and name when `names` has them.
    pub fn missing_tsv(&self, names: &dyn Fn(u32) -> Option<(String, String)>) -> String {
        let mut rows: Vec<(&u32, &Miss)> = self.misses.iter().collect();
        rows.sort_by(|a, b| b.1.count.cmp(&a.1.count).then(a.0.cmp(b.0)));
        let mut out = String::from("address\thits\tstage\tcallers\tfirst\tunit\tname\n");
        for (addr, miss) in rows {
            let callers = miss
                .callers
                .iter()
                .map(|(c, n)| format!("{c:08x}x{n}"))
                .collect::<Vec<_>>()
                .join(",");
            let (unit, name) = names(*addr).unwrap_or_default();
            let _ = writeln!(
                out,
                "{addr:08x}\t{}\t{}\t{callers}\t{}\t{unit}\t{name}",
                miss.count, miss.stage, miss.first
            );
        }
        out
    }
}

impl crate::Host {
    /// The frame's move of the player through the game's code
    /// ([`PlayerMovement::step`]); `None`: the caller moves the player itself.
    pub fn move_player(
        &mut self,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> Option<MoveOutput> {
        self.movement.step(&mut self.engine, order, cell, input)
    }

    /// The frame's Havok world step through the game's code
    /// ([`PlayerMovement::step_world`]); `None`: the caller steps its own.
    pub fn step_world(&mut self, seconds: f32) -> Option<Vec<BodyPose>> {
        self.movement.step_world(&mut self.engine, seconds)
    }

    /// The static initializers start-up skipped (`static_init` in
    /// [`crate::Host::start`]), one row each: the entry, why (`open`: not
    /// translated; `callee`: a callee isn't; `stopped`: stopped part-way),
    /// the untranslated callee, and the global the entry makes (its table
    /// row's object or register global; empty for others). The movement
    /// path reads globals these make (the process lists, `011e0e80`, among
    /// them), so a fault in translated code there can come from a skipped
    /// one.
    pub fn static_skips_tsv(&self) -> String {
        use engine::static_init::Row;
        let rows = engine::static_init::table();
        let global = |entry: u32| match rows.get(&entry) {
            Some(Row::Object { this, .. }) => format!("{this:08x}"),
            Some(Row::Reg { global, .. }) => format!("{global:08x}"),
            _ => String::new(),
        };
        let report = &self.static_init;
        let mut out = String::from("entry\twhy\tcallee\tglobal\tstop\n");
        for &entry in &report.open {
            let _ = writeln!(out, "{entry:08x}\topen\t\t{}\t", global(entry));
        }
        for &(entry, callee) in &report.callee_open {
            let _ = writeln!(
                out,
                "{entry:08x}\tcallee\t{callee:08x}\t{}\t",
                global(entry)
            );
        }
        for (entry, stop) in &report.stopped {
            let stop = stop.lines().next().unwrap_or("");
            let _ = writeln!(out, "{entry:08x}\tstopped\t\t{}\t{stop}", global(*entry));
        }
        out
    }
}

/// The game's `SetPosition` (vtable +0x2A8) with a point on the game's stack.
fn set_position(e: &mut Engine, reference: u32, feet: [f32; 3]) {
    e.with_stack(12, |e, p| {
        for (i, v) in feet.iter().enumerate() {
            e.mem.set_f32(p.addr() + 4 * i as u32, *v);
        }
        e.vcall(reference, SET_POSITION, &[p.addr()]);
    });
}

/// `TESObjectREFR::SetAngleOnReference` (`0086d490`) with (0, 0, heading).
fn set_heading(e: &mut Engine, reference: u32, heading: f32) {
    e.with_stack(12, |e, p| {
        e.mem.set_f32(p.addr(), 0.0);
        e.mem.set_f32(p.addr() + 4, 0.0);
        e.mem.set_f32(p.addr() + 8, heading);
        e.call(SET_ANGLE, &[reference, p.addr()]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A guarded run records each untranslated address with its nearest
    /// caller and the stage, stops at the first, and comes back as `Err`;
    /// the list puts the most reached first.
    #[test]
    fn misses_are_recorded_with_their_caller_and_counted() {
        let mut e = Engine::new();
        e.register(0x00aa_0000, |e, _| e.call(0x00ab_cdef, &[]));
        let mut m = PlayerMovement::default();
        for _ in 0..3 {
            let out = m.guarded(&mut e, "frame", |e| {
                e.call(0x00aa_0000, &[]);
                Ok(())
            });
            assert!(out.unwrap_err().starts_with("open function 00abcdef"));
        }
        let _ = m.guarded(&mut e, "cell", |e| {
            e.call(0x00ab_0001, &[]);
            Ok(())
        });
        let miss = &m.misses[&0x00ab_cdef];
        assert_eq!(miss.count, 3);
        assert_eq!(miss.stage, "frame");
        assert_eq!(miss.callers[&0x00aa_0000], 3);
        assert!(m.stops.is_empty());
        let tsv = m.missing_tsv(&|_| None);
        let lines: Vec<&str> = tsv.lines().collect();
        assert_eq!(lines[1], "00abcdef\t3\tframe\t00aa0000x3\t0\t\t");
        assert_eq!(lines[2], "00ab0001\t1\tcell\t00000000x1\t1\t\t");
        assert!(e.on_open.is_none() && e.call_stack.is_empty());
    }

    /// A probe run goes on past a miss with a zero result, records every
    /// miss; `step` never moves the player in a probe run.
    #[test]
    fn a_probe_goes_on_past_misses() {
        let mut e = Engine::new();
        let mut m = PlayerMovement {
            probe: true,
            ..PlayerMovement::default()
        };
        let out = m.guarded(&mut e, "frame", |e| {
            let a = e.call(0x00ab_0001, &[]).u32();
            let b = e.call(0x00ab_0002, &[]).u32();
            Ok(a + b)
        });
        assert_eq!(out, Ok(0));
        assert_eq!(m.misses.len(), 2);
    }

    /// Without the havok and cell stages the world step doesn't run and the
    /// frame is counted as the viewer's.
    #[test]
    fn the_world_step_waits_for_its_stages() {
        let mut e = Engine::new();
        let mut m = PlayerMovement::default();
        assert_eq!(m.step_world(&mut e, 0.016), None);
        assert_eq!((m.world_frames, m.world_fallback_frames), (0, 1));
        assert!(m
            .summary()
            .contains("0 world steps by the game's code, 1 by crates/physics"));
    }

    /// The world step runs the frame timer and the cell's managed-node
    /// update with the frame's time added, and reads each reference's root
    /// node; a stop is recorded under `world` and not tried again.
    #[test]
    fn the_world_step_runs_the_cells_update_and_reads_the_nodes() {
        let mut e = Engine::new();
        e.mem.map(CELL_ANIMATION_TIME & !0xfff, 0x1000);
        e.mem.set_f32(CELL_ANIMATION_TIME, 1.0);
        e.register(SET_DELTA_TIME, |e, a| {
            e.mem.set_u32(0x0000_2000, a[0]);
            Ret::default()
        });
        e.register(UPDATE_MANAGED_NODES, |e, a| {
            e.mem.set_u32(0x0000_2004, a[1]);
            Ret::default()
        });
        e.mem.map(0x2000, 0x1000);
        // A reference whose Get3D (+0x1D0) answers a root node at 0x2400.
        let vtable = 0x2100;
        e.mem.set_u32(0x2200, vtable);
        e.mem.set_u32(vtable + GET_3D, 0x00ab_0000);
        e.register(0x00ab_0000, |_, _| engine::abi::RetVal::into_ret(0x2400u32));
        for i in 0..9 {
            e.mem.set_f32(
                0x2400 + WORLD_ROTATION + 4 * i,
                if i % 4 == 0 { 1.0 } else { 0.0 },
            );
        }
        e.mem.set_f32(0x2400 + WORLD_TRANSLATION, 5.0);
        let mut m = PlayerMovement {
            havok: Stage::Done(()),
            cell: Some((FormId(0x1234), Stage::Done(0x3000))),
            cell_refs: vec![(FormId(0x99), 0x2200)],
            ..PlayerMovement::default()
        };
        let poses = m.step_world(&mut e, 0.5).expect("ran");
        assert_eq!(e.mem.f32(0x2000), 0.5);
        assert_eq!(e.mem.f32(0x2004), 1.5);
        assert_eq!(e.mem.f32(CELL_ANIMATION_TIME), 1.5);
        assert_eq!(poses.len(), 1);
        assert_eq!(poses[0].reference, FormId(0x99));
        assert_eq!(poses[0].translation, [5.0, 0.0, 0.0]);
        assert_eq!(poses[0].rotation[1], [0.0, 1.0, 0.0]);
        assert_eq!(m.world_frames, 1);
        // A stop: recorded, and the step isn't tried again.
        e.register(UPDATE_MANAGED_NODES, |e, _| e.call(0x00ab_cdef, &[]));
        assert_eq!(m.step_world(&mut e, 0.5), None);
        assert_eq!(m.misses[&0x00ab_cdef].stage, "world");
        assert_eq!(m.step_world(&mut e, 0.5), None);
        assert_eq!(m.misses[&0x00ab_cdef].count, 1);
        assert_eq!(m.world_fallback_frames, 2);
    }

    /// A stop that isn't a missing function (memory nobody made) is kept
    /// with its stage.
    #[test]
    fn other_stops_are_kept_by_stage() {
        let mut e = Engine::new();
        let mut m = PlayerMovement::default();
        let _ = m.guarded(&mut e, "frame", |e| Ok(e.mem.u32(0x0000_1000)));
        assert_eq!(m.stops.len(), 1);
        let ((stage, why), n) = m.stops.iter().next().unwrap();
        assert_eq!((*stage, *n), ("frame", 1));
        assert!(why.contains("00001000"), "{why}");
    }
}
