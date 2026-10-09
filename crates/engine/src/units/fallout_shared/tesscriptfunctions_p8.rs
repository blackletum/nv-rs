//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 8: its functions from `005db810` up to
//! (not including) `005e0310` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: the first 40 queue entries of the range (`005db810` to
//! `005dd7d0`) are translated. The next session continues at `005dd7f0`.
//!
//! The command bodies follow the conventions of the main file: `cdecl`, the
//! eight stack words as [`ScriptArgs`], `AL` as the result. Members of
//! classes this unit does not own are read at the PC offsets with a comment.
//! The compiler's exception-unwinding frames are not translated.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees outside this part (by exe address) ----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address first (`cdecl`).
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The logging stub of the unit (`005b5e40`): takes a format and its
/// arguments, does nothing and returns 0 in this build.
const LOG_STUB: u32 = 0x005b_5e40;
/// `_memset` (`dest, value, count`).
const MEMSET: u32 = 0x00ec_61c0;
/// Zero-fills (`array, 0, bytes`); a wrapper of `_memset`.
const ZERO_FILL: u32 = 0x0040_3d30;
/// `sprintf_s` (`dest, size, format, ...`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// Case-insensitive string comparison (`a, b`), 0 when equal; a wrapper of
/// `_stricmp`.
const STRICMP: u32 = 0x0040_4dc0;
/// Form type (`thiscall`, the byte at `this + 4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `*(this + 0x20)`: the base form of a reference (the engine map calls it
/// `BGSSaveFormBuffer::GetForm`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB): `*(this + 0x68)`.
const GET_PROCESS: u32 = 0x008d_8520;
/// `PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB), `thiscall` on the
/// player (`reference`).
const CHECK_QUEST_TARGET_UPDATE: u32 = 0x0095_2c30;
/// `thiscall` on a door's base form (`flag`): sets or clears the minimal-use
/// state (`tesobjectdoor.cpp`).
const DOOR_SET_MINIMAL_USE: u32 = 0x0051_8030;

/// `Script::GetVATSFrontTargetVisibleConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, value, 0, result`).
const GET_VATS_FRONT_TARGET_VISIBLE_CONDITION: u32 = 0x005a_5840;
/// `Script::IsInCritStageConditionFunction` (Xbox PDB), same arguments.
const IS_IN_CRIT_STAGE_CONDITION: u32 = 0x005a_2910;
/// `Script::IsGoreDisabledConditionFunction` (Xbox PDB), same arguments.
const IS_GORE_DISABLED_CONDITION: u32 = 0x005a_5a50;
/// `Script::GetActorsInHighConditionFunction` (Xbox PDB), same arguments.
const GET_ACTORS_IN_HIGH_CONDITION: u32 = 0x005a_5ab0;

/// `thiscall` on an actor (`value`): stores `value` at `+0x10c` and updates
/// the actor (`008a1a70`).
const ACTOR_SET_FIELD_10C: u32 = 0x008a_1a40;

/// Float functions with one `float` argument and the result in `ST0`; their
/// meaning is not named in the engine map. The angle commands combine them
/// with the degree/radian constants.
const FLOAT_FN_004E44B0: u32 = 0x004e_44b0;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004E4470: u32 = 0x004e_4470;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004B5510: u32 = 0x004b_5510;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_0057DD70: u32 = 0x0057_dd70;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004F6E40: u32 = 0x004f_6e40;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004B1460: u32 = 0x004b_1460;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004019B0: u32 = 0x0040_19b0;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_00408840: u32 = 0x0040_8840;
/// The CRT `double` function behind [`fn_005dc460`] (the library match is
/// ambiguous between `cos`, `log`, `sin` and `tan`).
const CRT_DOUBLE_FN_00ECA920: u32 = 0x00ec_a920;

/// `Tile::GetMenuByClass` (Xbox PDB), `cdecl` (`class id`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetMenu` (Xbox PDB), `thiscall`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `ComputersMenu::DisplayPreviousTerminal` (Xbox PDB), `thiscall`.
const DISPLAY_PREVIOUS_TERMINAL: u32 = 0x0075_8a80;
/// `FalloutRadio::PipboyRadioEnable` (Xbox PDB), `cdecl` (`flag`).
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;

/// `this + 0x44` (`thiscall`): the stage list of a quest.
const QUEST_STAGE_LIST: u32 = 0x005d_43c0;
/// Node access of the singly linked lists (`BSSimpleList`: item at +0, next
/// node at +4): returns `this`, the address of the node's item.
const LIST_ITEM_PTR: u32 = 0x0068_15c0;
/// `*(this + 4)`: the next node.
const LIST_NEXT: u32 = 0x0072_6070;
/// `this + 4` (`BaseProcess::GetActorPackageThatIsRunning` in the engine
/// map): the item list embedded in a quest stage.
const PLUS_4: u32 = 0x0071_7e50;
/// `thiscall` on a quest stage: the byte at `this + 0`.
const STAGE_INDEX: u32 = 0x0093_73f0;
/// `BSStringT` constructor (an 8-byte object).
const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor.
const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT<char>::operator=` (`string, text`).
const BS_STRING_ASSIGN: u32 = 0x0043_8390;
/// `BSStringT<char>::operator+=` (`string, text`).
const BS_STRING_APPEND: u32 = 0x0040_4820;
/// `BSStringT` copy constructor (`thiscall` on the new object, `source`).
const BS_STRING_COPY_CONSTRUCT: u32 = 0x0040_47f0;
/// `Interface::PrintLine` (Xbox PDB), `cdecl`: takes the 8-byte `BSStringT` by
/// value (two words).
const PRINT_LINE: u32 = 0x0070_3c80;

/// `TESNPC::InitHead` (Xbox PDB), `thiscall` (`&first, &second`): two output
/// words.
const TES_NPC_INIT_HEAD: u32 = 0x0060_7370;
/// `thiscall` on the base form (`reference, data, first, second`): the
/// head-update step of the face generation command (`00607420`).
const TES_NPC_UPDATE_HEAD: u32 = 0x0060_7420;
/// `thiscall` on the base form: its race, `*(this + 0x110)`.
const TES_NPC_GET_RACE: u32 = 0x004a_c110;
/// `TESRace::GetFaceGenData` (Xbox PDB), `thiscall` on the race (`npc,
/// face gen data, 0, 0`).
const TES_RACE_GET_FACE_GEN_DATA: u32 = 0x0061_41f0;
/// Constructor of a 16-byte object (`thiscall`: `float, byte, byte`).
const FLOAT_AND_FLAGS_CONSTRUCT: u32 = 0x0043_d410;
/// `thiscall` on the node object (`&object`).
const NODE_APPLY: u32 = 0x00a5_9c60;
/// `*(this + 0x18)` (`thiscall`).
const FIELD_18: u32 = 0x0096_11e0;
/// `RecurseAndRemoveObjectsFromPalette` (Xbox PDB), `cdecl` (`object,
/// skeleton`).
const RECURSE_REMOVE_FROM_PALETTE: u32 = 0x00a6_e8e0;
/// `*(this + 0xd8)` as a pointer (`thiscall`, `NiPointer` read).
const POINTER_AT_D8: u32 = 0x0049_6940;
/// `*(this + 0x78)` as a pointer (`thiscall`, `NiPointer` read).
const POINTER_AT_78: u32 = 0x0053_7bd0;
/// `NiPointer` assignment of a raw pointer to a slot (`slot, pointer`);
/// returns the slot.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `NiPointer::operator=` (`to, from`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Stores its byte argument at `011d59e0` (`cdecl`, `flag`): the
/// `ModifyFaceGen` mode byte.
const SET_FACE_GEN_MODE_BYTE: u32 = 0x005d_d860;
/// The name tables of the face generation commands (`cdecl`, `index`): the
/// text of expression, modifier and phoneme number `index`.
const EXPRESSION_NAME: u32 = 0x005d_d830;
/// See [`EXPRESSION_NAME`].
const MODIFIER_NAME: u32 = 0x005d_d840;
/// See [`EXPRESSION_NAME`].
const PHONEME_NAME: u32 = 0x005d_d850;

/// `eh_vector_constructor_iterator` (`array, size, count, constructor,
/// destructor`).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `eh_vector_destructor_iterator` (`array, size, count, destructor`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// Element constructor of the 0x20-byte elements of the face generation
/// data.
const ELEMENT_CONSTRUCT: u32 = 0x0044_9610;
/// Element destructor of the same elements.
const ELEMENT_DESTRUCT: u32 = 0x0044_9680;
/// `thiscall` (`0, 1`): constructors of the four 16-byte arrays at `+0x94`,
/// `+0xa4`, `+0xb4` and `+0xc4` of the face generation data.
const ARRAY_94_CONSTRUCT: u32 = 0x005e_0370;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_A4_CONSTRUCT: u32 = 0x005e_0430;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_B4_CONSTRUCT: u32 = 0x005e_0490;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_C4_CONSTRUCT: u32 = 0x005e_0310;
/// `thiscall`: resets one of the arrays (called on `+0x94`, `+0xa4`, `+0xb4`).
const ARRAY_RESET: u32 = 0x005e_03d0;
/// `thiscall` destructor of the array at `+0x94` (what [`fn_005dd7b0`]
/// calls).
const ARRAY_94_DESTRUCT: u32 = 0x005e_0340;
/// `thiscall` destructor of the array at `+0xa4` (what [`fn_005dd7d0`]
/// calls).
const ARRAY_A4_DESTRUCT: u32 = 0x005e_03a0;
/// Destructor wrapper of the array at `+0xb4` (`005dd7f0`, translated by the
/// next session).
const ARRAY_B4_DESTRUCT_WRAPPER: u32 = 0x005d_d7f0;
/// Destructor wrapper of the array at `+0xc4` (`005dd810`, translated by the
/// next session).
const ARRAY_C4_DESTRUCT_WRAPPER: u32 = 0x005d_d810;
/// Initialises an empty list head (`thiscall`, a two-word object).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Destructor of the list head.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `thiscall` on the list at `+0xe4` of the face generation data, run
/// before the list destructor in [`fn_005dd6f0`].
const LIST_PRE_DESTRUCT: u32 = 0x0047_0470;

/// The no-op "constructor" of the 16-byte vector temporaries
/// (`thiscall`, returns `this`; the same code as [`LIST_ITEM_PTR`]).
const VECTOR_CONSTRUCT_NOOP: u32 = 0x0068_15c0;
/// `thiscall` on a vector (`output`): stores its first lane in all four lanes
/// of the output.
const VECTOR_SPLAT_X: u32 = 0x0056_1240;
/// `cdecl` (`output, vector`): stores the first three lanes of a vector as
/// three separate floats.
const VECTOR_STORE_THREE: u32 = 0x0045_8620;

// ---- Globals and constants ---------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;
/// Counter incremented by [`fn_005dbe10`] (16 bits, never 0).
const COUNTER: u32 = 0x011b_0808;
/// The depth bias toggle of [`script_toggle_depth_bias_function`].
const DEPTH_BIAS_FLAG: u32 = 0x011a_d80f;
/// The value stored by `ModifyFaceGen coord 1 <value>`.
const FACE_GEN_COORD_VALUE: u32 = 0x011d_59f0;
/// Four `float` ones (`1.0f` in every lane).
const VECTOR_ONES: u32 = 0x010e_c7f0;
/// `double` -1.0, the lower bound of the angle commands.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `float` -1.0.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `double` 1.0.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `double` 57.2957763671875, radians to degrees.
const RADIANS_TO_DEGREES: u32 = 0x0102_f248;
/// `double` 0.017453292 (as the exe stores it), degrees to radians.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `float` 2.7182817 (e), the default base of the logarithm command.
const EULER_NUMBER: u32 = 0x0103_d018;
/// `double` 100.0.
const HUNDRED: u32 = 0x0101_7a40;

/// Virtual slot `0x100` of a reference: whether it is an actor.
const VSLOT_IS_ACTOR: u32 = 0x100;
/// Virtual slot `0x130`: the name (of a script).
const VSLOT_NAME: u32 = 0x130;
/// Virtual slot `0x1b8` of a reference (`0`): the object whose face
/// generation data the `ModifyFaceGen` command changes.
const VSLOT_FACE_GEN_TARGET: u32 = 0x1b8;
/// Virtual slot `0x1b0` of a reference (`0`).
const VSLOT_OBJECT_1B0: u32 = 0x1b0;
/// Virtual slot `0x1ac` of a reference (`0`).
const VSLOT_OBJECT_1AC: u32 = 0x1ac;
/// Virtual slot `0x218` of a reference: a flag.
const VSLOT_FLAG_218: u32 = 0x218;
/// Virtual slot `0x1d0` of a reference: its node object (null when none).
const VSLOT_NODE: u32 = 0x1d0;
/// Virtual slot `0x1e4` of a reference: its animation object.
const VSLOT_ANIMATION: u32 = 0x1e4;
/// Virtual slot `0x1e8` of a reference: the data `TESNPC::BuildObjectArray`
/// takes as `base`.
const VSLOT_BASE_DATA: u32 = 0x1e8;
/// Virtual slot `0x114` of the face generation target (`index, float`):
/// sets one expression.
const VSLOT_SET_EXPRESSION: u32 = 0x114;
/// Virtual slot `0xa4` of the face generation target (`index`): a `float`
/// in `ST0`, the current phoneme value.
const VSLOT_GET_PHONEME: u32 = 0xa4;
/// Virtual slot `0xec` of the face generation target (`array, float`):
/// sets all phonemes.
const VSLOT_SET_PHONEMES: u32 = 0xec;
/// Virtual slot `0xf0` of the face generation target (`array, float`):
/// sets all modifiers.
const VSLOT_SET_MODIFIERS: u32 = 0xf0;
/// Virtual slot `0xb4` of the face generation target (`1.0, 1, 1, 1, 1,
/// 0`): the `reset` request.
const VSLOT_RESET: u32 = 0xb4;
/// Virtual slot `0x79c` of a process (`0`).
const VSLOT_PROCESS_79C: u32 = 0x79c;
/// Virtual slot `0x794` of a process (`0`).
const VSLOT_PROCESS_794: u32 = 0x794;
/// Virtual slot `0x7a4` of a process (`0`).
const VSLOT_PROCESS_7A4: u32 = 0x7a4;
/// Virtual slot `0x58` of a process.
const VSLOT_PROCESS_58: u32 = 0x58;
/// Virtual slot `0xe8` of the node object (`object`).
const VSLOT_NODE_E8: u32 = 0xe8;
/// Virtual slot `0xc` of the node object: its data.
const VSLOT_NODE_DATA: u32 = 0xc;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Depth Bias is now %s!"`
const MSG_DEPTH_BIAS: u32 = 0x0103_d01c;
/// `"OFF"`
const TEXT_OFF: u32 = 0x0103_d034;
/// `"ON"`
const TEXT_ON: u32 = 0x0103_d038;
/// `"SCRIPTS: SetMinimalUse must be called on a door reference."`
const MSG_SET_MINIMAL_USE: u32 = 0x0103_d03c;
/// `")"`
const TEXT_CLOSE_PAREN: u32 = 0x0103_d078;
/// `"Item %d: %d"`
const FORMAT_ITEM: u32 = 0x0103_d07c;
/// `" ("`
const TEXT_OPEN_PAREN: u32 = 0x0103_d088;
/// `"Stage %d: %d "`
const FORMAT_STAGE: u32 = 0x0103_d08c;
/// `", "`
const TEXT_SEPARATOR: u32 = 0x0101_2630;
/// `"reset"`
const KEYWORD_RESET: u32 = 0x0103_d09c;
/// `"coord"`
const KEYWORD_COORD: u32 = 0x0103_d0a4;
/// `"modifier"`
const KEYWORD_MODIFIER: u32 = 0x0103_d0ac;
/// `"phoneme"`
const KEYWORD_PHONEME: u32 = 0x0103_d0b8;
/// `"%d. %s, "`
const FORMAT_NAME_MORE: u32 = 0x0103_d0c0;
/// `"%d. %s"`
const FORMAT_NAME_LAST: u32 = 0x0103_d0cc;
/// `"expression"`
const KEYWORD_EXPRESSION: u32 = 0x0103_d0d4;

// ---- Small helpers -------------------------------------------------------------

/// `Script::ParseParameters` with the given output addresses after the seven
/// fixed words: its `AL`.
fn parse(e: &mut Engine, a: ScriptArgs, outs: &[u32]) -> bool {
    let mut words = args![
        a.param_info,
        a.script_data,
        a.opcode_offset,
        a.this_obj,
        a.containing_obj,
        a.script_obj,
        a.event_list
    ];
    words.extend_from_slice(outs);
    e.call(PARSE_PARAMETERS, &words).bool()
}

/// [`parse`] with `N` word-sized locals (the stack slots the game passes by
/// address) initialised to `init`. `None` when the parameters do not parse,
/// otherwise the values left in the locals.
fn parse_params<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> Option<[u32; N]> {
    let block = e.mem.alloc(4 * N as u32);
    let mut outs = [0u32; N];
    for (i, value) in init.iter().enumerate() {
        outs[i] = block + 4 * i as u32;
        e.mem.set_u32(outs[i], *value);
    }
    let ok = parse(e, a, &outs);
    let mut values = init;
    for (i, value) in values.iter_mut().enumerate() {
        *value = e.mem.u32(outs[i]);
    }
    e.mem.free(block);
    ok.then_some(values)
}

/// Whether the TLS echo flag (commands print their result) is set.
fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// The condition-function call of the commands that hand over their
/// `thisObj` and the result double: `condition(thisObj, argument, 0,
/// result)`, its `AL`.
fn call_condition(e: &mut Engine, condition: u32, a: ScriptArgs, argument: u32) -> bool {
    e.call(condition, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

/// Prints a copy of the string: the game builds a copy on the stack and
/// passes the two words of the object to `Interface::PrintLine`.
fn print_line(e: &mut Engine, string: u32) {
    let (first, second) = e.with_stack(8, |e, copy| {
        e.call(BS_STRING_COPY_CONSTRUCT, &args![copy, string]);
        (e.mem.u32(copy.addr()), e.mem.u32(copy.addr() + 4))
    });
    e.call(PRINT_LINE, &args![first, second]);
}

/// The clamp the angle commands apply to their `float` before the inverse
/// function: `FCOMP` against the `double` -1.0 (only an ordered "less"
/// replaces the value with the `float` -1.0), then against the `double` 1.0
/// (only an ordered "greater" replaces it with 1.0).
fn clamp_to_unit(e: &mut Engine, mut value: f32) -> f32 {
    if (value as f64) < e.global::<f64>(MINUS_ONE_DOUBLE) {
        value = e.global::<f32>(MINUS_ONE_FLOAT);
    }
    if (value as f64) > e.global::<f64>(ONE_DOUBLE) {
        value = 1.0;
    }
    value
}

/// Degrees to radians the way the angle commands do it: the product in
/// `double`, rounded to `float`.
fn to_radians(e: &mut Engine, degrees: f32) -> f32 {
    (degrees as f64 * e.global::<f64>(DEGREES_TO_RADIANS)) as f32
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005db810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSFrontTargetVisibleFunction` (Xbox PDB): parses one value
/// and returns `GetVATSFrontTargetVisibleConditionFunction(thisObj, value,
/// 0, result)`.
pub fn script_get_vats_front_target_visible_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_VATS_FRONT_TARGET_VISIBLE_CONDITION, a, value)
}

// Translated from 005dbe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the 16-bit counter at `011b0808`, skipping 0 (it restarts at
/// 10), and returns it.
pub fn fn_005dbe10(e: &mut Engine) -> u32 {
    let next = e.global::<u32>(COUNTER).wrapping_add(1) & 0xffff;
    e.set_global(COUNTER, next);
    if next == 0 {
        e.set_global(COUNTER, 10u32);
    }
    e.global(COUNTER)
}

// Translated from 005dbe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the flags word at `this + 0x10`.
pub fn fn_005dbe40(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x10) & mask != 0
}

// Translated from 005dbe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates the vectors of `this` ([`fn_005dbec0`]) into a temporary
/// vector and stores its first three lanes through `00458620` into `output`.
pub fn fn_005dbe60(e: &mut Engine, this: Ptr, output: Ptr) {
    e.with_stack(16, |e, result| {
        e.call(VECTOR_CONSTRUCT_NOOP, &args![result]);
        fn_005dbec0(e, this, result);
        e.call(VECTOR_STORE_THREE, &args![output, result]);
    });
}

// Translated from 005dbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Linear interpolation between the vector at `this` and the vector at `this
/// + 0x10` with the weight `float` at `this + 0x40` ([`fn_005dbf70`]); the
/// result goes to `output`.
pub fn fn_005dbec0(e: &mut Engine, this: Ptr, output: Ptr) {
    e.with_stack(16, |e, weight| {
        let t = e.mem.f32(this.addr() + 0x40);
        fn_005dbf20(e, weight, t);
        fn_005dbf70(e, output, this, Ptr::new(this.addr() + 0x10), weight);
    });
}

// Translated from 005dbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the vector `(x, 0, 0, 0)` in `this` and returns `this`.
pub fn fn_005dbf20(e: &mut Engine, this: Ptr, x: f32) -> Ptr {
    e.mem.set_f32(this.addr(), x);
    for lane in 1..4 {
        e.mem.set_f32(this.addr() + 4 * lane, 0.0);
    }
    this
}

// Translated from 005dbf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `output = (1 - t) * a + t * b` lane by lane (single precision, as the
/// `SUBPS`/`MULPS`/`ADDPS` code computes), where `t` is the first lane of
/// `weight` replicated into all four lanes by `00561240`.
pub fn fn_005dbf70(e: &mut Engine, output: Ptr, a: Ptr, b: Ptr, weight: Ptr) {
    e.with_stack(16, |e, t| {
        e.call(VECTOR_SPLAT_X, &args![weight, t]);
        for lane in 0..4u32 {
            let ones: f32 = e.global(VECTOR_ONES + 4 * lane);
            let t_lane = e.mem.f32(t.addr() + 4 * lane);
            let b_lane = e.mem.f32(b.addr() + 4 * lane);
            let a_lane = e.mem.f32(a.addr() + 4 * lane);
            let scaled_b = t_lane * b_lane;
            let scaled_a = (ones - t_lane) * a_lane;
            e.mem.set_f32(output.addr() + 4 * lane, scaled_a + scaled_b);
        }
    });
}

// Translated from 005dc010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer; when `thisObj` is an actor (virtual slot `0x100`),
/// stores it at its `+0x10c` through `008a1a40`. Succeeds when the parameter
/// parsed.
pub fn fn_005dc010(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() && e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        e.call(ACTOR_SET_FIELD_10C, &args![a.this_obj, value]);
    }
    true
}

// Translated from 005dc090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInCritStageFunction` (Xbox PDB): parses one value and returns
/// `IsInCritStageConditionFunction(thisObj, value, 0, result)`.
pub fn script_is_in_crit_stage_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, IS_IN_CRIT_STAGE_CONDITION, a, value)
}

// Translated from 005dc0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angle command: parses a `float` and an integer flag. With the flag
/// clear the result is `004e44b0(value * pi / 180)`; with the flag set it is
/// `004b5510(clamp(value, -1, 1)) * 180 / pi`. Fails when the parameters do
/// not parse.
pub fn fn_005dc0f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let value = clamp_to_unit(e, value);
        let result = e.call(FLOAT_FN_004B5510, &args![value]).f64();
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_004E44B0, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same angle command with the functions `004e4470` (flag clear) and
/// [`fn_005dc270`] (flag set, after the same clamp).
pub fn fn_005dc1b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let value = clamp_to_unit(e, value);
        let result = fn_005dc270(e, value) as f64;
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_004E4470, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to `004f6e40` and returns that function's `ST0`.
pub fn fn_005dc270(e: &mut Engine, x: f32) -> f32 {
    e.call(FLOAT_FN_004F6E40, &args![x]).f32()
}

// Translated from 005dc290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The angle command without clamping: with the flag clear,
/// `0057dd70(value * pi / 180)`; with the flag set, `005dc330(value) * 180 /
/// pi`.
pub fn fn_005dc290(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let result = fn_005dc330(e, value) as f64;
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_0057DD70, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to `004b1460` and returns that function's `ST0`.
pub fn fn_005dc330(e: &mut Engine, x: f32) -> f32 {
    e.call(FLOAT_FN_004B1460, &args![x]).f32()
}

// Translated from 005dc350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one `float` and stores `004019b0(value)` in the result double.
pub fn fn_005dc350(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let result = e
        .call(FLOAT_FN_004019B0, &args![f32::from_bits(value_bits)])
        .f64();
    e.mem.set_f64(a.result.addr(), result);
    true
}

// Translated from 005dc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A logarithm command: parses a `float` and a base (default `e`, the float
/// at `0103d018`); the result is 0 when the base is 1 and
/// `log(value) / log(base)` (through [`fn_005dc440`]) otherwise.
pub fn fn_005dc3b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let euler = e.global::<f32>(EULER_NUMBER);
    let Some([value_bits, base_bits]) = parse_params(e, a, [0.0f32.to_bits(), euler.to_bits()])
    else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    let base = f32::from_bits(base_bits);
    // `FCOMP`: an unordered comparison (NaN) takes the division branch.
    if base as f64 == e.global::<f64>(ONE_DOUBLE) {
        e.mem.set_f64(a.result.addr(), 0.0);
    } else {
        let numerator = fn_005dc440(e, value) as f64;
        let denominator = fn_005dc440(e, base) as f64;
        e.mem.set_f64(a.result.addr(), numerator / denominator);
    }
    true
}

// Translated from 005dc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to [`fn_005dc460`].
pub fn fn_005dc440(e: &mut Engine, x: f32) -> f32 {
    fn_005dc460(e, x)
}

// Translated from 005dc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The single-precision wrapper of the CRT `double` function at `00eca920`
/// (the library match calls it `_logf`): the argument widened to `double`,
/// the result rounded to `float`.
pub fn fn_005dc460(e: &mut Engine, x: f32) -> f32 {
    let wide = e.call(CRT_DOUBLE_FN_00ECA920, &args![x as f64]).f64();
    wide as f32
}

// Translated from 005dc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one `float` and stores `00408840(value)` in the result double.
pub fn fn_005dc480(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let result = e
        .call(FLOAT_FN_00408840, &args![f32::from_bits(value_bits)])
        .f64();
    e.mem.set_f64(a.result.addr(), result);
    true
}

// Translated from 005dc4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ForceTerminalBackFunction` (Xbox PDB): finds the menu of tile
/// class `0x421`, takes its menu object and, if there is one, shows the
/// previous terminal. Always succeeds.
pub fn script_force_terminal_back_function(e: &mut Engine, _a: ScriptArgs) -> bool {
    let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![0x421u32]).u32();
    let menu = if tile != 0 {
        e.call(TILE_GET_MENU, &args![tile]).u32()
    } else {
        0
    };
    if menu != 0 {
        e.call(DISPLAY_PREVIOUS_TERMINAL, &args![menu]);
    }
    true
}

// Translated from 005dc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDepthBiasFunction` (Xbox PDB): flips the byte at
/// `011ad80f` and prints `"Depth Bias is now ON!"` or `"... OFF!"`. Always
/// succeeds.
pub fn script_toggle_depth_bias_function(e: &mut Engine, _a: ScriptArgs) -> bool {
    let enabled = e.global::<u8>(DEPTH_BIAS_FLAG) == 0;
    e.set_global(DEPTH_BIAS_FLAG, enabled as u8);
    let text = if enabled { TEXT_ON } else { TEXT_OFF };
    e.call(CONSOLE_PRINT, &args![MSG_DEPTH_BIAS, text]);
    true
}

// Translated from 005dc580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `FalloutRadio::PipboyRadioEnable(0)`. Always succeeds.
pub fn fn_005dc580(e: &mut Engine, _a: ScriptArgs) -> bool {
    e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
    true
}

// Translated from 005dc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsGoreDisabled` body: returns
/// `IsGoreDisabledConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005dc5a0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_GORE_DISABLED_CONDITION, a, 0)
}

// Translated from 005dc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetActorsInHigh` body: returns
/// `GetActorsInHighConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005dc5c0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_ACTORS_IN_HIGH_CONDITION, a, 0)
}

// Translated from 005dc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetMinimalUseFunction` (Xbox PDB): parses a flag. When `thisObj`
/// has a base form of type `0x1c` (the door), the door's minimal-use state is
/// set to `flag != 0` and the player checks its quest targets; otherwise the
/// logging stub gets the message that the command must be called on a door
/// reference (with the script's name, virtual slot `0x130`). Succeeds when the
/// parameter parsed.
pub fn script_set_minimal_use_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() && e.call(GET_BASE_FORM, &args![a.this_obj]).u32() != 0 {
        let base_form = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if e.call(FORM_TYPE, &args![base_form]).u32() == 0x1c {
            let door = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            e.call(DOOR_SET_MINIMAL_USE, &args![door, (flag != 0) as u32]);
            let player = e.global::<u32>(PLAYER);
            e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, player]);
            return true;
        }
    }
    let name = e.vcall(a.script_obj.addr(), VSLOT_NAME, &args![]).u32();
    e.call(LOG_STUB, &args![MSG_SET_MINIMAL_USE, name]);
    true
}

// Translated from 005dc6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and stores `value != 0` as the byte at the player's
/// `+0x7c7` ([`fn_005dc710`]).
pub fn fn_005dc6a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    fn_005dc710(e, Ptr::new(player), (value != 0) as u8);
    true
}

// Translated from 005dc710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0x7c7` (`PlayerCharacter`, PC offset).
pub fn fn_005dc710(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x7c7, value);
}

// Translated from 005dc730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowQuestStagesFunction` (Xbox PDB): parses a quest. With the TLS
/// echo flag set, prints one line per stage of the quest's stage list
/// (`quest + 0x44`): `"Stage <index>: <flag> "`, followed, when the stage's
/// item list (`stage + 4`) has a second node, by `" (Item <n>: <flag>, Item
/// ...)"` (an item list of one node prints no items). Succeeds when the
/// parameter parsed.
pub fn script_show_quest_stages_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_params(e, a, [0]) else {
        return false;
    };
    let string = e.mem.alloc(8);
    let buffer = e.mem.alloc(0x20);
    e.call(BS_STRING_CONSTRUCT, &args![string]);
    if echo_enabled(e) {
        let mut node = e.call(QUEST_STAGE_LIST, &args![quest]).u32();
        while node != 0 {
            let item_cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
            if e.mem.u32(item_cell) == 0 {
                break;
            }
            let stage_cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
            let stage = e.mem.u32(stage_cell);
            let stage_flag = fn_005dc960(e, Ptr::new(stage)) != 0;
            let index = e.call(STAGE_INDEX, &args![stage]).u8();
            e.call(
                SPRINTF_S,
                &args![
                    buffer,
                    0x1fu32,
                    FORMAT_STAGE,
                    index as u32,
                    stage_flag as u32
                ],
            );
            e.call(BS_STRING_ASSIGN, &args![string, buffer]);
            if e.call(PLUS_4, &args![stage]).u32() != 0 {
                let items = e.call(PLUS_4, &args![stage]).u32();
                if e.call(LIST_NEXT, &args![items]).u32() != 0 {
                    e.call(BS_STRING_APPEND, &args![string, TEXT_OPEN_PAREN]);
                    let mut item_node = e.call(PLUS_4, &args![stage]).u32();
                    while item_node != 0 {
                        let cell = e.call(LIST_ITEM_PTR, &args![item_node]).u32();
                        if e.mem.u32(cell) == 0 {
                            break;
                        }
                        let cell = e.call(LIST_ITEM_PTR, &args![item_node]).u32();
                        let item = e.mem.u32(cell);
                        let has_process = e.call(GET_PROCESS, &args![item]).u32() != 0;
                        let value = fn_005dc980(e, Ptr::new(item));
                        e.call(
                            SPRINTF_S,
                            &args![
                                buffer,
                                0x1fu32,
                                FORMAT_ITEM,
                                value as u32,
                                has_process as u32
                            ],
                        );
                        e.call(BS_STRING_APPEND, &args![string, buffer]);
                        if e.call(LIST_NEXT, &args![item_node]).u32() != 0 {
                            e.call(BS_STRING_APPEND, &args![string, TEXT_SEPARATOR]);
                        }
                        item_node = e.call(LIST_NEXT, &args![item_node]).u32();
                    }
                    e.call(BS_STRING_APPEND, &args![string, TEXT_CLOSE_PAREN]);
                }
            }
            print_line(e, string);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
    e.call(BS_STRING_DESTRUCT, &args![string]);
    e.mem.free(buffer);
    e.mem.free(string);
    true
}

// Translated from 005dc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 1` of a quest stage (read as a flag by
/// [`script_show_quest_stages_function`]).
pub fn fn_005dc960(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 1)
}

// Translated from 005dc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 100` of a quest stage item.
pub fn fn_005dc980(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 100)
}

/// The command's list of names: `"%d. %s, "` for every name but the last
/// (`"%d. %s"`), appended to a string that is then printed.
fn print_name_list(e: &mut Engine, name_function: u32, count: i32) {
    let string = e.mem.alloc(8);
    let buffer = e.mem.alloc(0x200);
    e.call(BS_STRING_CONSTRUCT, &args![string]);
    for index in 0..count {
        let name = e.call(name_function, &args![index]).u32();
        let format = if index == count - 1 {
            FORMAT_NAME_LAST
        } else {
            FORMAT_NAME_MORE
        };
        e.call(SPRINTF_S, &args![buffer, 0x200u32, format, index, name]);
        e.call(BS_STRING_APPEND, &args![string, buffer]);
    }
    print_line(e, string);
    e.call(BS_STRING_DESTRUCT, &args![string]);
    e.mem.free(buffer);
    e.mem.free(string);
}

/// The weight the commands store for a parsed percentage: `value / 100`
/// rounded to `float`.
fn percent(e: &mut Engine, value: i32) -> f32 {
    (value as f64 / e.global::<f64>(HUNDRED)) as f32
}

// Translated from 005dc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModifyFaceGen` (Xbox PDB): parses a keyword (a 0x200-byte
/// string), an index (default -1) and a value (default 0). An empty keyword
/// prints `"expression"`. The object it works on is virtual slot `0x1b8` of
/// `thisObj` (called with 0); without one nothing happens. The keywords
/// (compared without case):
///
/// - `expression`: without an index prints the 15 expression names;
///   otherwise virtual slot `0x114` of the object (`index, value / 100`).
/// - `phoneme`: without an index prints the 16 names; otherwise builds the
///   16 current phoneme values (virtual slot `0xa4`), puts `value / 100` at
///   the index and passes the array to slot `0xec` with 1.0.
/// - `modifier`: without an index prints the 17 names; otherwise stores 0
///   in the face generation mode byte (`005dd860`), builds the 17 modifier
///   values ([`fn_005dd4a0`]) with the same replacement and passes them to
///   slot `0xf0` with 1.0.
/// - `coord`: index -1 attaches the reference to the object of slot `0x1b0`
///   (`+0xe8`, `+0xec` = 0 and the bytes cleared by [`fn_005dd520`] and
///   [`fn_005dd540`]); index 1 does the same with `+0xec` = 1, the bytes set
///   and the value stored at `011d59f0`; index 0 rebuilds the head of an
///   actor `thisObj` (slot `0x218` true) from its race: it removes the
///   head objects (slots `0x1b0` and `0x1ac`) from the palette, clears the
///   head of the base form ([`tes_npc_clear_head`]), initialises it again and
///   regenerates it from a fresh face generation data
///   ([`fn_005dd590`], [`fn_005dd6f0`]).
/// - `reset`: virtual slot `0xb4` of the object (`1.0, 1, 1, 1, 1, 0`), then
///   the mode byte is set to 1.
///
/// Succeeds when the parameters parsed.
pub fn script_modify_face_gen(e: &mut Engine, a: ScriptArgs) -> bool {
    let keyword = e.mem.alloc(0x200);
    e.mem.set_u8(keyword, 0);
    e.call(MEMSET, &args![keyword + 1, 0u32, 0x1ffu32]);
    let numbers = e.mem.alloc(8);
    e.mem.set_i32(numbers, -1);
    e.mem.set_i32(numbers + 4, 0);
    let parsed = parse(e, a, &[keyword, numbers, numbers + 4]);
    let index = e.mem.i32(numbers);
    let value = e.mem.i32(numbers + 4);
    if parsed {
        modify_face_gen(e, a, keyword, index, value);
    }
    e.mem.free(numbers);
    e.mem.free(keyword);
    parsed
}

/// The body of [`script_modify_face_gen`] after the parameters parsed.
fn modify_face_gen(e: &mut Engine, a: ScriptArgs, keyword: u32, index: i32, value: i32) {
    if e.mem.i8(keyword) == 0 {
        e.call(CONSOLE_PRINT, &args![KEYWORD_EXPRESSION]);
        return;
    }
    let target = if a.this_obj.is_null() {
        0
    } else {
        e.vcall(a.this_obj.addr(), VSLOT_FACE_GEN_TARGET, &args![0u32])
            .u32()
    };
    if target == 0 {
        return;
    }
    let is_keyword = |e: &mut Engine, text: u32| e.call(STRICMP, &args![keyword, text]).i32() == 0;
    if is_keyword(e, KEYWORD_EXPRESSION) {
        if index == -1 {
            print_name_list(e, EXPRESSION_NAME, 15);
        } else {
            let weight = percent(e, value);
            e.vcall(target, VSLOT_SET_EXPRESSION, &args![index, weight]);
        }
    } else if is_keyword(e, KEYWORD_PHONEME) {
        if index == -1 {
            print_name_list(e, PHONEME_NAME, 16);
        } else {
            let values = e.mem.alloc(0x40);
            e.call(ZERO_FILL, &args![values, 0u32, 0x40u32]);
            for i in 0..16 {
                let element = if i == index {
                    percent(e, value)
                } else {
                    e.vcall(target, VSLOT_GET_PHONEME, &args![i]).f32()
                };
                e.mem.set_f32(values + 4 * i as u32, element);
            }
            e.vcall(target, VSLOT_SET_PHONEMES, &args![values, 1.0f32]);
            e.mem.free(values);
        }
    } else if is_keyword(e, KEYWORD_MODIFIER) {
        if index == -1 {
            print_name_list(e, MODIFIER_NAME, 17);
        } else {
            e.call(SET_FACE_GEN_MODE_BYTE, &args![0u32]);
            let values = e.mem.alloc(0x44);
            e.call(ZERO_FILL, &args![values, 0u32, 0x44u32]);
            for i in 0..17 {
                let element = if i == index {
                    percent(e, value)
                } else {
                    fn_005dd4a0(e, Ptr::new(target), i)
                };
                e.mem.set_f32(values + 4 * i as u32, element);
            }
            e.vcall(target, VSLOT_SET_MODIFIERS, &args![values, 1.0f32]);
            e.mem.free(values);
        }
    } else if is_keyword(e, KEYWORD_COORD) {
        match index {
            -1 => attach_face_gen_object(e, a.this_obj.addr(), false),
            0 => rebuild_head(e, a.this_obj.addr()),
            1 => {
                attach_face_gen_object(e, a.this_obj.addr(), true);
                e.set_global(FACE_GEN_COORD_VALUE, value);
            }
            _ => {}
        }
    } else if is_keyword(e, KEYWORD_RESET) {
        e.vcall(
            target,
            VSLOT_RESET,
            &args![1.0f32, 1u32, 1u32, 1u32, 1u32, 0u32],
        );
        e.call(SET_FACE_GEN_MODE_BYTE, &args![1u32]);
    }
}

/// The `coord` cases -1 and 1: the object of virtual slot `0x1b0` of
/// `reference` is told about it (`+0xe8`) and gets its flag byte (`+0xec`) and
/// the two bytes at `+0xd4` / `+0xe3` set to `on`.
fn attach_face_gen_object(e: &mut Engine, reference: u32, on: bool) {
    let object = e.vcall(reference, VSLOT_OBJECT_1B0, &args![0u32]).u32();
    e.mem.set_u32(object + 0xe8, reference);
    e.mem.set_u8(object + 0xec, on as u8);
    fn_005dd520(e, Ptr::new(object), on as u8);
    fn_005dd540(e, Ptr::new(object), on as u8);
}

/// The `coord 0` case: rebuilds the head of an actor.
fn rebuild_head(e: &mut Engine, reference: u32) {
    if reference == 0 || !e.vcall(reference, VSLOT_FLAG_218, &args![]).bool() {
        return;
    }
    let base_form = e.call(GET_BASE_FORM, &args![reference]).u32();
    let node = e.vcall(reference, VSLOT_NODE, &args![]).u32();
    let node_data = if node != 0 {
        e.vcall(node, VSLOT_NODE_DATA, &args![]).u32()
    } else {
        0
    };
    if node_data != 0 {
        let mut skeleton = 0;
        if e.vcall(reference, VSLOT_ANIMATION, &args![]).u32() != 0 {
            let animation = e.vcall(reference, VSLOT_ANIMATION, &args![]).u32();
            if e.call(POINTER_AT_D8, &args![animation]).u32() != 0 {
                let animation = e.vcall(reference, VSLOT_ANIMATION, &args![]).u32();
                let inner = e.call(POINTER_AT_D8, &args![animation]).u32();
                skeleton = e.call(POINTER_AT_78, &args![inner]).u32();
            }
        }
        let object = e.vcall(reference, VSLOT_OBJECT_1B0, &args![0u32]).u32();
        if object != 0 && e.call(FIELD_18, &args![object]).u32() != 0 {
            e.call(RECURSE_REMOVE_FROM_PALETTE, &args![object, skeleton]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_79C, &args![0u32]);
            let owner = e.call(FIELD_18, &args![object]).u32();
            e.vcall(owner, VSLOT_NODE_E8, &args![object]);
        }
        let object = e.vcall(reference, VSLOT_OBJECT_1AC, &args![0u32]).u32();
        if object != 0 && e.call(FIELD_18, &args![object]).u32() != 0 {
            e.call(RECURSE_REMOVE_FROM_PALETTE, &args![object, skeleton]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_794, &args![0u32]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_7A4, &args![0u32]);
            let owner = e.call(FIELD_18, &args![object]).u32();
            e.vcall(owner, VSLOT_NODE_E8, &args![object]);
        }
    }
    let process = e.call(GET_PROCESS, &args![reference]).u32();
    e.vcall(process, VSLOT_PROCESS_58, &args![]);
    tes_npc_clear_head(e, Ptr::new(base_form));
    e.with_stack(8, |e, outputs| {
        e.call(
            TES_NPC_INIT_HEAD,
            &args![base_form, outputs, outputs.addr() + 4],
        );
        let first = e.mem.u32(outputs.addr());
        let second = e.mem.u32(outputs.addr() + 4);
        let data = e.vcall(reference, VSLOT_BASE_DATA, &args![]).u32();
        e.call(
            TES_NPC_UPDATE_HEAD,
            &args![base_form, reference, data, first, second],
        );
    });
    e.with_stack(0xf0, |e, face_gen| {
        fn_005dd590(e, face_gen);
        let race = e.call(TES_NPC_GET_RACE, &args![base_form]).u32();
        e.call(
            TES_RACE_GET_FACE_GEN_DATA,
            &args![race, base_form, face_gen, 0u32, 0u32],
        );
        e.with_stack(16, |e, scratch| {
            e.call(
                FLOAT_AND_FLAGS_CONSTRUCT,
                &args![scratch, 0.0f32, 0u32, 0u32],
            );
            e.call(NODE_APPLY, &args![node, scratch]);
        });
        fn_005dd6f0(e, face_gen);
    });
}

// Translated from 005dd4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The modifier value number `index` of the face generation target: `0.0` for
/// an index of 17 or more (signed comparison), else the value of the float
/// array at `this + 0xa8` ([`fn_005dd4e0`]).
pub fn fn_005dd4a0(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    if index < 0x11 {
        fn_005dd4e0(e, Ptr::new(this.addr() + 0xa8), index as u32)
    } else {
        0.0
    }
}

// Translated from 005dd4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of a `float` array (pointer at `this + 0xc`, count at
/// `this + 0x10`), or `0.0` when the index is out of range (unsigned).
pub fn fn_005dd4e0(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    if index < e.mem.u32(this.addr() + 0x10) {
        let data = e.mem.u32(this.addr() + 0xc);
        e.mem.f32(data.wrapping_add(index.wrapping_mul(4)))
    } else {
        0.0
    }
}

// Translated from 005dd520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0xd4`.
pub fn fn_005dd520(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xd4, value);
}

// Translated from 005dd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0xe3`.
pub fn fn_005dd540(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xe3, value);
}

// Translated from 005dd560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::ClearHead` (Xbox PDB): empties the `NiPointer` at `this + 0x1c4`
/// and assigns it to the one at `this + 0x1c8`.
pub fn tes_npc_clear_head(e: &mut Engine, this: Ptr) {
    let cleared = e
        .call(NI_POINTER_SET, &args![this.addr() + 0x1c4, 0u32])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x1c8, cleared]);
}

// Translated from 005dd590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the face generation data (0xec bytes): four 0x20-byte
/// elements, four 16-byte arrays (`+0x94`, `+0xa4`, `+0xb4`, `+0xc4`), a
/// list head at `+0xe4`; the fields `+0x80..+0x90` are cleared, the arrays
/// reset, the bytes at `+0xd4` / `+0xd5` cleared and `+0xe0` set to -1.
/// Returns `this`.
pub fn fn_005dd590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        VECTOR_CONSTRUCT,
        &args![base, 0x20u32, 4u32, ELEMENT_CONSTRUCT, ELEMENT_DESTRUCT],
    );
    e.call(ARRAY_94_CONSTRUCT, &args![base + 0x94, 0u32, 1u32]);
    e.call(ARRAY_A4_CONSTRUCT, &args![base + 0xa4, 0u32, 1u32]);
    e.call(ARRAY_B4_CONSTRUCT, &args![base + 0xb4, 0u32, 1u32]);
    e.call(ARRAY_C4_CONSTRUCT, &args![base + 0xc4, 0u32, 1u32]);
    e.call(LIST_CONSTRUCT, &args![base + 0xe4]);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_f32(base + 0x88, 0.0);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.call(ARRAY_RESET, &args![base + 0x94]);
    e.call(ARRAY_RESET, &args![base + 0xa4]);
    e.call(ARRAY_RESET, &args![base + 0xb4]);
    e.mem.set_u8(base + 0xd4, 0);
    e.mem.set_u8(base + 0xd5, 0);
    e.mem.set_u32(base + 0xe0, 0xffff_ffff);
    this
}

// Translated from 005dd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the face generation data: the list at `+0xe4`, the arrays at
/// `+0xc4`, `+0xb4`, `+0xa4` and `+0x94`, then the four 0x20-byte elements.
pub fn fn_005dd6f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(LIST_PRE_DESTRUCT, &args![base + 0xe4]);
    e.call(LIST_DESTRUCT, &args![base + 0xe4]);
    e.call(ARRAY_C4_DESTRUCT_WRAPPER, &args![base + 0xc4]);
    e.call(ARRAY_B4_DESTRUCT_WRAPPER, &args![base + 0xb4]);
    fn_005dd7d0(e, Ptr::new(base + 0xa4));
    fn_005dd7b0(e, Ptr::new(base + 0x94));
    e.call(
        VECTOR_DESTRUCT,
        &args![base, 0x20u32, 4u32, ELEMENT_DESTRUCT],
    );
}

// Translated from 005dd7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the array at `+0x94` of the face generation data: calls
/// `005e0340`.
pub fn fn_005dd7b0(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_94_DESTRUCT, &args![this]);
}

// Translated from 005dd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the array at `+0xa4` of the face generation data: calls
/// `005e03a0`.
pub fn fn_005dd7d0(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_A4_DESTRUCT, &args![this]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005db810, script_get_vats_front_target_visible_function(ScriptArgs) -> bool),
        entry!(0x005dbe10, fn_005dbe10() -> u32),
        entry!(0x005dbe40, fn_005dbe40(Ptr, u32) -> bool),
        entry!(0x005dbe60, fn_005dbe60(Ptr, Ptr)),
        entry!(0x005dbec0, fn_005dbec0(Ptr, Ptr)),
        entry!(0x005dbf20, fn_005dbf20(Ptr, f32) -> Ptr),
        entry!(0x005dbf70, fn_005dbf70(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x005dc010, fn_005dc010(ScriptArgs) -> bool),
        entry!(0x005dc090, script_is_in_crit_stage_function(ScriptArgs) -> bool),
        entry!(0x005dc0f0, fn_005dc0f0(ScriptArgs) -> bool),
        entry!(0x005dc1b0, fn_005dc1b0(ScriptArgs) -> bool),
        entry!(0x005dc270, fn_005dc270(f32) -> f32),
        entry!(0x005dc290, fn_005dc290(ScriptArgs) -> bool),
        entry!(0x005dc330, fn_005dc330(f32) -> f32),
        entry!(0x005dc350, fn_005dc350(ScriptArgs) -> bool),
        entry!(0x005dc3b0, fn_005dc3b0(ScriptArgs) -> bool),
        entry!(0x005dc440, fn_005dc440(f32) -> f32),
        entry!(0x005dc460, fn_005dc460(f32) -> f32),
        entry!(0x005dc480, fn_005dc480(ScriptArgs) -> bool),
        entry!(0x005dc4e0, script_force_terminal_back_function(ScriptArgs) -> bool),
        entry!(0x005dc530, script_toggle_depth_bias_function(ScriptArgs) -> bool),
        entry!(0x005dc580, fn_005dc580(ScriptArgs) -> bool),
        entry!(0x005dc5a0, fn_005dc5a0(ScriptArgs) -> bool),
        entry!(0x005dc5c0, fn_005dc5c0(ScriptArgs) -> bool),
        entry!(0x005dc5e0, script_set_minimal_use_function(ScriptArgs) -> bool),
        entry!(0x005dc6a0, fn_005dc6a0(ScriptArgs) -> bool),
        entry!(0x005dc710, fn_005dc710(Ptr, u8)),
        entry!(0x005dc730, script_show_quest_stages_function(ScriptArgs) -> bool),
        entry!(0x005dc960, fn_005dc960(Ptr) -> u8),
        entry!(0x005dc980, fn_005dc980(Ptr) -> u8),
        entry!(0x005dc9a0, script_modify_face_gen(ScriptArgs) -> bool),
        entry!(0x005dd4a0, fn_005dd4a0(Ptr, i32) -> f32),
        entry!(0x005dd4e0, fn_005dd4e0(Ptr, u32) -> f32),
        entry!(0x005dd520, fn_005dd520(Ptr, u8)),
        entry!(0x005dd540, fn_005dd540(Ptr, u8)),
        entry!(0x005dd560, tes_npc_clear_head(Ptr)),
        entry!(0x005dd590, fn_005dd590(Ptr) -> Ptr),
        entry!(0x005dd6f0, fn_005dd6f0(Ptr)),
        entry!(0x005dd7b0, fn_005dd7b0(Ptr)),
        entry!(0x005dd7d0, fn_005dd7d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_NAME: u32 = 0x0900_0130;
    const V_IS_ACTOR: u32 = 0x0900_0100;

    /// An engine with the pages of the globals these commands read mapped and
    /// set to the exe's values.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_7000,
            0x0101_a000,
            0x0102_3000,
            0x0102_f000,
            0x0103_d000,
            0x010e_c000,
            0x011a_d000,
            0x011b_0000,
            0x011d_5000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(RADIANS_TO_DEGREES, 57.2957763671875f64);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(EULER_NUMBER, 2.7182817f32);
        e.set_global(HUNDRED, 100.0f64);
        for lane in 0..4 {
            e.set_global(VECTOR_ONES + 4 * lane, 1.0f32);
        }
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        e
    }

    /// A zeroed object of 0x800 bytes with a vtable that has the given
    /// `(byte offset, function)` slots.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x800);
        e.mem.set_u32(object, vtable);
        object
    }

    fn object(e: &mut Engine) -> u32 {
        object_with(e, &[])
    }

    /// The standard eight words with `this_obj` set and a result double.
    fn command(e: &mut Engine, this_obj: u32) -> ScriptArgs {
        let result = e.mem.alloc(8);
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::new(this_obj),
            containing_obj: Ptr::NULL,
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::new(result),
            opcode_offset: 8,
        }
    }

    /// `ParseParameters` double: returns `ok` and stores `outs` through its
    /// output pointers (after the seven fixed words).
    fn parse_gives(e: &mut Engine, ok: bool, outs: &[u32]) {
        let outs = outs.to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in outs.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            ok.into_ret()
        });
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of all logged calls, in order.
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    fn set_echo(e: &mut Engine, on: bool) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, on as u8);
    }

    fn set_player(e: &mut Engine) -> u32 {
        let player = object(e);
        e.set_global(PLAYER, player);
        player
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    /// The test of a "parse one value, hand it to a condition function"
    /// command: the condition function gets `(thisObj, value, 0, result)`.
    fn check_parse_then_condition(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0x77]);
        e.register(condition, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, this_obj, 0, 5, 6]
        );
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0x77, 0, a.result.addr()]]
        );
        // The condition function's AL is the command's.
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, condition).is_empty());
    }

    /// The test of a "no arguments, return a condition function" command.
    fn check_condition_only(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
    }

    #[test]
    fn get_vats_front_target_visible_passes_the_parsed_value_to_the_condition_function() {
        check_parse_then_condition(0x005d_b810, GET_VATS_FRONT_TARGET_VISIBLE_CONDITION);
    }

    #[test]
    fn fn_005dbe10_counts_up_and_skips_zero() {
        let mut e = engine();
        e.set_global(COUNTER, 5u32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 6);
        assert_eq!(e.global::<u32>(COUNTER), 6);
        // The 16-bit counter wraps to 0, which is replaced by 10.
        e.set_global(COUNTER, 0xffffu32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 10);
        assert_eq!(e.global::<u32>(COUNTER), 10);
        // Bits above the low 16 are dropped.
        e.set_global(COUNTER, 0x1_2345u32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 0x2346);
    }

    #[test]
    fn fn_005dbe40_tests_the_flags_word_against_the_mask() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x10, 0b1010);
        assert!(e.call(0x005d_be40, &args![this, 0b0010u32]).bool());
        assert!(!e.call(0x005d_be40, &args![this, 0b0101u32]).bool());
        assert!(!e.call(0x005d_be40, &args![this, 0u32]).bool());
    }

    /// Doubles for the vector helpers: `00561240` replicates lane 0.
    fn vector_engine() -> Engine {
        let mut e = engine();
        e.register(VECTOR_SPLAT_X, |e, a| {
            let x = e.mem.f32(a[0]);
            for lane in 0..4 {
                e.mem.set_f32(a[1] + 4 * lane, x);
            }
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCT_NOOP, |_, a| a[0].into_ret());
        e
    }

    fn put_vector(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (i, lane) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *lane);
        }
    }

    fn get_vector(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
    }

    #[test]
    fn fn_005dbf20_builds_a_vector_with_only_the_first_lane() {
        let mut e = engine();
        let vector = e.mem.alloc(16);
        put_vector(&mut e, vector, [9.0; 4]);
        let returned = e.call(0x005d_bf20, &args![vector, 2.5f32]).u32();
        assert_eq!(returned, vector);
        assert_eq!(get_vector(&e, vector), [2.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_005dbf70_interpolates_lane_by_lane() {
        let mut e = vector_engine();
        let (output, a, b, weight) = (
            e.mem.alloc(16),
            e.mem.alloc(16),
            e.mem.alloc(16),
            e.mem.alloc(16),
        );
        put_vector(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        put_vector(&mut e, b, [10.0, 20.0, 30.0, 40.0]);
        put_vector(&mut e, weight, [0.25, 9.0, 9.0, 9.0]);
        start_log(&mut e);
        e.call(0x005d_bf70, &args![output, a, b, weight]);
        // `t * b + (1 - t) * a`.
        assert_eq!(get_vector(&e, output), [3.25, 6.5, 9.75, 13.0]);
        // The first lane of the weight vector is replicated by `00561240`.
        assert_eq!(calls(&e, VECTOR_SPLAT_X).len(), 1);
        assert_eq!(calls(&e, VECTOR_SPLAT_X)[0][0], weight);
    }

    #[test]
    fn fn_005dbec0_interpolates_the_two_vectors_with_the_weight_at_0x40() {
        let mut e = vector_engine();
        let this = e.mem.alloc(0x50);
        let output = e.mem.alloc(16);
        put_vector(&mut e, this, [0.0, 4.0, 8.0, 12.0]);
        put_vector(&mut e, this + 0x10, [4.0, 8.0, 12.0, 16.0]);
        e.mem.set_f32(this + 0x40, 0.5);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [2.0, 6.0, 10.0, 14.0]);
        // A weight of 0 gives the first vector, 1 the second.
        e.mem.set_f32(this + 0x40, 0.0);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [0.0, 4.0, 8.0, 12.0]);
        e.mem.set_f32(this + 0x40, 1.0);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [4.0, 8.0, 12.0, 16.0]);
    }

    #[test]
    fn fn_005dbe60_interpolates_into_a_temporary_and_stores_it_through_00458620() {
        let mut e = vector_engine();
        // `00458620 (output, vector)` copies three lanes.
        e.register(VECTOR_STORE_THREE, |e, a| {
            for lane in 0..3 {
                let value = e.mem.f32(a[1] + 4 * lane);
                e.mem.set_f32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let output = e.mem.alloc(16);
        put_vector(&mut e, this, [0.0, 4.0, 8.0, 12.0]);
        put_vector(&mut e, this + 0x10, [4.0, 8.0, 12.0, 16.0]);
        e.mem.set_f32(this + 0x40, 0.5);
        put_vector(&mut e, output, [7.0; 4]);
        start_log(&mut e);
        e.call(0x005d_be60, &args![this, output]);
        // Three lanes are stored, the fourth is untouched.
        assert_eq!(get_vector(&e, output), [2.0, 6.0, 10.0, 7.0]);
        let order = call_order(&e);
        assert_eq!(order[1], VECTOR_CONSTRUCT_NOOP);
        assert_eq!(*order.last().unwrap(), VECTOR_STORE_THREE);
        assert_eq!(calls(&e, VECTOR_STORE_THREE)[0][0], output);
    }

    #[test]
    fn fn_005dc010_stores_the_value_on_actors_only() {
        let mut e = engine();
        let actor = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register(ACTOR_SET_FIELD_10C, |_, _| Ret::default());
        parse_gives(&mut e, true, &[7]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_c010, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_SET_FIELD_10C), vec![vec![actor, 7]]);
        // Not an actor: the value is dropped, the command still succeeds.
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c010, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_FIELD_10C).is_empty());
        // No reference.
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_c010, &args![none]).bool());
        assert!(calls(&e, ACTOR_SET_FIELD_10C).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c010, &args![a]).bool());
    }

    #[test]
    fn is_in_crit_stage_passes_the_parsed_value_to_the_condition_function() {
        check_parse_then_condition(0x005d_c090, IS_IN_CRIT_STAGE_CONDITION);
    }

    /// Parse double for the angle commands: a `float` and an integer flag.
    fn parse_angle(e: &mut Engine, value: f32, flag: u32) {
        parse_gives(e, true, &[value.to_bits(), flag]);
    }

    fn result_double(e: &Engine, a: ScriptArgs) -> f64 {
        e.mem.f64(a.result.addr())
    }

    fn radians_of(degrees: f32) -> f32 {
        (degrees as f64 * f64::from_bits(0x3f91_df46_a000_0000)) as f32
    }

    #[test]
    fn fn_005dc0f0_converts_between_degrees_and_the_function_values() {
        let mut e = engine();
        // The two float functions: `+ 1` and `* 0.5`.
        e.register(FLOAT_FN_004E44B0, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004B5510, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        // Flag clear: degrees to radians (rounded to float), then 004e44b0.
        parse_angle(&mut e, 90.0, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c0f0, &args![a]).bool());
        let radians = radians_of(90.0);
        assert_eq!(calls(&e, FLOAT_FN_004E44B0), vec![vec![radians.to_bits()]]);
        assert_eq!(result_double(&e, a), (radians + 1.0) as f64);
        // Flag set: clamped to [-1, 1], then 004b5510, times 180 / pi.
        for (value, clamped) in [(2.0f32, 1.0f32), (-5.0, -1.0), (0.25, 0.25)] {
            parse_angle(&mut e, value, 1);
            start_log(&mut e);
            assert!(e.call(0x005d_c0f0, &args![a]).bool());
            assert_eq!(calls(&e, FLOAT_FN_004B5510), vec![vec![clamped.to_bits()]]);
            assert_eq!(
                result_double(&e, a),
                (clamped * 0.5) as f64 * 57.2957763671875
            );
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c0f0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc1b0_converts_between_degrees_and_the_function_values() {
        let mut e = engine();
        e.register(FLOAT_FN_004E4470, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004F6E40, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        parse_angle(&mut e, 45.0, 0);
        assert!(e.call(0x005d_c1b0, &args![a]).bool());
        assert_eq!(result_double(&e, a), (radians_of(45.0) + 1.0) as f64);
        for (value, clamped) in [(1.5f32, 1.0f32), (-1.5, -1.0), (-0.5, -0.5)] {
            parse_angle(&mut e, value, 1);
            start_log(&mut e);
            assert!(e.call(0x005d_c1b0, &args![a]).bool());
            assert_eq!(calls(&e, FLOAT_FN_004F6E40), vec![vec![clamped.to_bits()]]);
            assert_eq!(
                result_double(&e, a),
                (clamped * 0.5) as f64 * 57.2957763671875
            );
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c1b0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc270_forwards_to_004f6e40() {
        let mut e = engine();
        e.register(FLOAT_FN_004F6E40, |_, a| (f(a[0]) + 0.5).into_ret());
        start_log(&mut e);
        assert_eq!(e.call(0x005d_c270, &args![2.0f32]).f32(), 2.5);
        assert_eq!(calls(&e, FLOAT_FN_004F6E40), vec![vec![2.0f32.to_bits()]]);
    }

    #[test]
    fn fn_005dc290_does_not_clamp() {
        let mut e = engine();
        e.register(FLOAT_FN_0057DD70, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004B1460, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        parse_angle(&mut e, 30.0, 0);
        assert!(e.call(0x005d_c290, &args![a]).bool());
        assert_eq!(result_double(&e, a), (radians_of(30.0) + 1.0) as f64);
        // Flag set: the value 5.0 reaches 004b1460 unchanged.
        parse_angle(&mut e, 5.0, 1);
        start_log(&mut e);
        assert!(e.call(0x005d_c290, &args![a]).bool());
        assert_eq!(calls(&e, FLOAT_FN_004B1460), vec![vec![5.0f32.to_bits()]]);
        assert_eq!(result_double(&e, a), 2.5 * 57.2957763671875);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c290, &args![a]).bool());
    }

    #[test]
    fn fn_005dc330_forwards_to_004b1460() {
        let mut e = engine();
        e.register(FLOAT_FN_004B1460, |_, a| (f(a[0]) + 0.5).into_ret());
        assert_eq!(e.call(0x005d_c330, &args![2.0f32]).f32(), 2.5);
    }

    /// The test of the "parse a float, store the result of a float function"
    /// commands.
    fn check_float_command(command_address: u32, function: u32) {
        let mut e = engine();
        e.register(function, |_, a| (f(a[0]) * 3.0).into_ret());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[1.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(calls(&e, function), vec![vec![1.5f32.to_bits()]]);
        assert_eq!(result_double(&e, a), 4.5);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, function).is_empty());
    }

    #[test]
    fn fn_005dc350_stores_the_result_of_004019b0() {
        check_float_command(0x005d_c350, FLOAT_FN_004019B0);
    }

    #[test]
    fn fn_005dc480_stores_the_result_of_00408840() {
        check_float_command(0x005d_c480, FLOAT_FN_00408840);
    }

    /// `00eca920` as the natural logarithm.
    fn ln_engine() -> Engine {
        let mut e = engine();
        e.register(CRT_DOUBLE_FN_00ECA920, |_, a| {
            let x = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            Ret {
                st0: x.ln(),
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn fn_005dc460_widens_the_argument_and_rounds_the_result_to_float() {
        let mut e = ln_engine();
        start_log(&mut e);
        let result = e.call(0x005d_c460, &args![8.0f32]).f32();
        assert_eq!(result, 8.0f64.ln() as f32);
        // The CRT function got the argument as a double (two words).
        assert_eq!(
            calls(&e, CRT_DOUBLE_FN_00ECA920),
            vec![vec![
                8.0f64.to_bits() as u32,
                (8.0f64.to_bits() >> 32) as u32
            ]]
        );
    }

    #[test]
    fn fn_005dc440_forwards_to_005dc460() {
        let mut e = ln_engine();
        assert_eq!(
            e.call(0x005d_c440, &args![2.0f32]).f32(),
            2.0f64.ln() as f32
        );
    }

    #[test]
    fn fn_005dc3b0_divides_the_logarithms_of_the_value_and_the_base() {
        let mut e = ln_engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[8.0f32.to_bits(), 2.0f32.to_bits()]);
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        let expected = (8.0f64.ln() as f32) as f64 / (2.0f64.ln() as f32) as f64;
        assert_eq!(result_double(&e, a), expected);
        // A base of 1 gives 0 without taking logarithms.
        parse_gives(&mut e, true, &[8.0f32.to_bits(), 1.0f32.to_bits()]);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        assert_eq!(result_double(&e, a), 0.0);
        assert!(calls(&e, CRT_DOUBLE_FN_00ECA920).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c3b0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc3b0_starts_with_value_0_and_base_e() {
        let mut e = ln_engine();
        let a = command(&mut e, 0);
        // The locals the command passes to `ParseParameters` start as 0.0
        // and e.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 0.0f32.to_bits());
            assert_eq!(e.mem.u32(a[8]), 2.7182817f32.to_bits());
            true.into_ret()
        });
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        // log(0) / log(e): minus infinity.
        assert_eq!(result_double(&e, a), f64::NEG_INFINITY);
    }

    #[test]
    fn force_terminal_back_shows_the_previous_terminal_only_with_a_menu() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x7700.into_ret());
        e.register(DISPLAY_PREVIOUS_TERMINAL, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x421]]);
        assert!(calls(&e, TILE_GET_MENU).is_empty());
        assert!(calls(&e, DISPLAY_PREVIOUS_TERMINAL).is_empty());
        // A tile whose menu is null.
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x6600.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, TILE_GET_MENU), vec![vec![0x6600]]);
        assert!(calls(&e, DISPLAY_PREVIOUS_TERMINAL).is_empty());
        // Both present.
        e.register(TILE_GET_MENU, |_, _| 0x7700.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, DISPLAY_PREVIOUS_TERMINAL), vec![vec![0x7700]]);
    }

    #[test]
    fn toggle_depth_bias_flips_the_flag_and_prints_it() {
        let mut e = engine();
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 1);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![MSG_DEPTH_BIAS, TEXT_ON],
                vec![MSG_DEPTH_BIAS, TEXT_OFF]
            ]
        );
        // Any non-zero byte counts as on and is cleared.
        e.set_global(DEPTH_BIAS_FLAG, 7u8);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 0);
    }

    #[test]
    fn fn_005dc580_disables_the_pipboy_radio() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(PIPBOY_RADIO_ENABLE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_c580, &args![a]).bool());
        assert_eq!(calls(&e, PIPBOY_RADIO_ENABLE), vec![vec![0]]);
    }

    #[test]
    fn fn_005dc5a0_returns_the_gore_condition_function_result() {
        check_condition_only(0x005d_c5a0, IS_GORE_DISABLED_CONDITION);
    }

    #[test]
    fn fn_005dc5c0_returns_the_actors_in_high_condition_function_result() {
        check_condition_only(0x005d_c5c0, GET_ACTORS_IN_HIGH_CONDITION);
    }

    /// A reference whose base form (at `+0x20`) has the given type byte.
    fn reference_with_base(e: &mut Engine, form_type: u8) -> (u32, u32) {
        let base = object(e);
        e.mem.set_u8(base + 4, form_type);
        let reference = object(e);
        e.mem.set_u32(reference + 0x20, base);
        (reference, base)
    }

    fn minimal_use_engine() -> Engine {
        let mut e = engine();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(DOOR_SET_MINIMAL_USE, |_, _| Ret::default());
        e.register(CHECK_QUEST_TARGET_UPDATE, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        e.register(V_NAME, |_, _| 0x4e41.into_ret());
        e
    }

    #[test]
    fn set_minimal_use_sets_the_door_state_and_updates_quest_targets() {
        let mut e = minimal_use_engine();
        let player = set_player(&mut e);
        let (reference, base) = reference_with_base(&mut e, 0x1c);
        let script = object_with(&mut e, &[(0x130, V_NAME)]);
        let mut a = command(&mut e, reference);
        a.script_obj = Ptr::new(script);
        for (parsed, state) in [(1u32, 1u32), (0, 0), (5, 1)] {
            parse_gives(&mut e, true, &[parsed]);
            start_log(&mut e);
            assert!(e.call(0x005d_c5e0, &args![a]).bool());
            assert_eq!(calls(&e, DOOR_SET_MINIMAL_USE), vec![vec![base, state]]);
            assert_eq!(
                calls(&e, CHECK_QUEST_TARGET_UPDATE),
                vec![vec![player, player]]
            );
            assert!(calls(&e, LOG_STUB).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_c5e0, &args![a]).bool());
        assert!(calls(&e, DOOR_SET_MINIMAL_USE).is_empty());
    }

    #[test]
    fn set_minimal_use_logs_when_the_reference_is_not_a_door() {
        let mut e = minimal_use_engine();
        set_player(&mut e);
        let script = object_with(&mut e, &[(0x130, V_NAME)]);
        parse_gives(&mut e, true, &[1]);
        // Another form type, no base form, no reference.
        let (reference, _) = reference_with_base(&mut e, 0x1b);
        let no_base = object(&mut e);
        for this_obj in [reference, no_base, 0] {
            let mut a = command(&mut e, this_obj);
            a.script_obj = Ptr::new(script);
            start_log(&mut e);
            assert!(e.call(0x005d_c5e0, &args![a]).bool());
            assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_SET_MINIMAL_USE, 0x4e41]]);
            assert!(calls(&e, DOOR_SET_MINIMAL_USE).is_empty());
            assert!(calls(&e, CHECK_QUEST_TARGET_UPDATE).is_empty());
        }
    }

    #[test]
    fn fn_005dc6a0_stores_the_flag_on_the_player() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[3]);
        assert!(e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 1);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 0);
        e.mem.set_u8(player + 0x7c7, 9);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 9);
    }

    #[test]
    fn fn_005dc710_stores_the_byte_at_0x7c7() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x005d_c710, &args![this, 0x105u32]);
        assert_eq!(e.mem.u8(this + 0x7c7), 5);
        assert_eq!(e.mem.u8(this + 0x7c8), 0);
    }

    #[test]
    fn fn_005dc960_and_fn_005dc980_read_single_bytes() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u8(this + 1, 0x11);
        e.mem.set_u8(this + 100, 0x64);
        assert_eq!(e.call(0x005d_c960, &args![this]).u8(), 0x11);
        assert_eq!(e.call(0x005d_c980, &args![this]).u8(), 0x64);
    }

    // ---- ShowQuestStages ----------------------------------------------------

    /// Doubles for the list accessors and string helpers of the quest stage
    /// printout; the string operations are only logged.
    fn stages_engine() -> Engine {
        let mut e = engine();
        e.register(QUEST_STAGE_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        e.register(STAGE_INDEX, |e, a| (e.mem.u8(a[0]) as u32).into_ret());
        e.register(GET_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(BS_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_ASSIGN, |_, _| Ret::default());
        e.register(BS_STRING_APPEND, |_, _| Ret::default());
        e.register(BS_STRING_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xc0c0);
            e.mem.set_u32(a[0] + 4, 0x0505);
            a[0].into_ret()
        });
        e.register(PRINT_LINE, |_, _| Ret::default());
        e
    }

    /// A stage with the given index, flag and `(value, has process)` items.
    /// The embedded list head sits at `stage + 4` (item at +4, next at +8).
    fn make_stage(e: &mut Engine, index: u8, flag: u8, items: &[(u8, bool)]) -> u32 {
        let stage = e.mem.alloc(0x40);
        e.mem.set_u8(stage, index);
        e.mem.set_u8(stage + 1, flag);
        let mut previous_next_cell = stage + 8;
        for (position, (value, has_process)) in items.iter().enumerate() {
            let item = e.mem.alloc(0x80);
            e.mem.set_u8(item + 100, *value);
            e.mem
                .set_u32(item + 0x68, if *has_process { 0x9999 } else { 0 });
            if position == 0 {
                e.mem.set_u32(stage + 4, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(previous_next_cell, node);
                previous_next_cell = node + 4;
            }
        }
        stage
    }

    /// A quest whose stage list (at `+0x44`) holds the given stages.
    fn make_quest(e: &mut Engine, stages: &[u32]) -> u32 {
        let quest = e.mem.alloc(0x80);
        let mut previous_next_cell = quest + 0x48;
        for (position, stage) in stages.iter().enumerate() {
            if position == 0 {
                e.mem.set_u32(quest + 0x44, *stage);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, *stage);
                e.mem.set_u32(previous_next_cell, node);
                previous_next_cell = node + 4;
            }
        }
        quest
    }

    /// The logged string operations as `(function, words after the first)`;
    /// the print gets both of its words.
    fn string_operations(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| {
                [SPRINTF_S, BS_STRING_ASSIGN, BS_STRING_APPEND, PRINT_LINE].contains(a)
            })
            .map(|(a, words)| {
                let keep = if *a == PRINT_LINE {
                    words.clone()
                } else {
                    words[1..].to_vec()
                };
                (*a, keep)
            })
            .collect()
    }

    #[test]
    fn show_quest_stages_prints_one_line_per_stage_with_its_items() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        let stage_one = make_stage(&mut e, 10, 1, &[(7, true), (8, false)]);
        let stage_two = make_stage(&mut e, 20, 0, &[]);
        let quest = make_quest(&mut e, &[stage_one, stage_two]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        let buffer = calls(&e, SPRINTF_S)[0][0];
        assert_eq!(
            string_operations(&e),
            vec![
                (SPRINTF_S, vec![0x1f, FORMAT_STAGE, 10, 1]),
                (BS_STRING_ASSIGN, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_OPEN_PAREN]),
                (SPRINTF_S, vec![0x1f, FORMAT_ITEM, 7, 1]),
                (BS_STRING_APPEND, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_SEPARATOR]),
                (SPRINTF_S, vec![0x1f, FORMAT_ITEM, 8, 0]),
                (BS_STRING_APPEND, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_CLOSE_PAREN]),
                (PRINT_LINE, vec![0xc0c0, 0x0505]),
                (SPRINTF_S, vec![0x1f, FORMAT_STAGE, 20, 0]),
                (BS_STRING_ASSIGN, vec![buffer]),
                (PRINT_LINE, vec![0xc0c0, 0x0505]),
            ]
        );
        // The string is constructed once and destroyed once.
        assert_eq!(calls(&e, BS_STRING_CONSTRUCT).len(), 1);
        assert_eq!(
            calls(&e, BS_STRING_CONSTRUCT),
            calls(&e, BS_STRING_DESTRUCT)
        );
    }

    #[test]
    fn show_quest_stages_lists_items_only_when_the_item_list_has_a_second_node() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        // One item: the list head has no next node, so no items are printed.
        let stage = make_stage(&mut e, 5, 0, &[(7, true)]);
        let quest = make_quest(&mut e, &[stage]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, BS_STRING_APPEND).is_empty());
        assert_eq!(calls(&e, PRINT_LINE).len(), 1);
    }

    #[test]
    fn show_quest_stages_prints_nothing_without_the_echo_flag() {
        let mut e = stages_engine();
        set_echo(&mut e, false);
        let stage = make_stage(&mut e, 10, 1, &[]);
        let quest = make_quest(&mut e, &[stage]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, PRINT_LINE).is_empty());
        assert!(calls(&e, SPRINTF_S).is_empty());
        // The string is still built and destroyed.
        assert_eq!(calls(&e, BS_STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, BS_STRING_DESTRUCT).len(), 1);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, BS_STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn show_quest_stages_stops_at_a_node_without_a_stage() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        let stage = make_stage(&mut e, 10, 1, &[]);
        // The head holds a stage, the second node holds null.
        let quest = make_quest(&mut e, &[stage, 0]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert_eq!(calls(&e, PRINT_LINE).len(), 1);
        // An empty list (head item null) prints nothing.
        let empty = make_quest(&mut e, &[]);
        parse_gives(&mut e, true, &[empty]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    // ---- ModifyFaceGen ----------------------------------------------------------

    const V_TARGET: u32 = 0x0900_01b8;
    const V_OBJECT_1B0: u32 = 0x0900_01b0;
    const V_OBJECT_1AC: u32 = 0x0900_01ac;
    const V_FLAG_218: u32 = 0x0900_0218;
    const V_NODE: u32 = 0x0900_01d0;
    const V_ANIMATION: u32 = 0x0900_01e4;
    const V_BASE_DATA: u32 = 0x0900_01e8;
    const V_SET_EXPRESSION: u32 = 0x0900_0114;
    const V_GET_PHONEME: u32 = 0x0900_00a4;
    const V_SET_PHONEMES: u32 = 0x0900_00ec;
    const V_SET_MODIFIERS: u32 = 0x0900_00f0;
    const V_RESET: u32 = 0x0900_00b4;
    const V_NODE_DATA: u32 = 0x0900_000c;
    const V_NODE_E8: u32 = 0x0900_00e8;
    const V_PROCESS_79C: u32 = 0x0900_079c;
    const V_PROCESS_794: u32 = 0x0900_0794;
    const V_PROCESS_7A4: u32 = 0x0900_07a4;
    const V_PROCESS_58: u32 = 0x0900_0058;

    /// Where the fake reference keeps what its virtual functions return.
    const REF_FLAG_218: u32 = 0x300;
    const REF_NODE: u32 = 0x304;
    const REF_ANIMATION: u32 = 0x308;
    const REF_TARGET: u32 = 0x30c;
    const REF_OBJECT_1B0: u32 = 0x310;
    const REF_OBJECT_1AC: u32 = 0x314;
    /// Where the fake target copies the arrays it is given (phonemes,
    /// modifiers).
    const TARGET_PHONEMES: u32 = 0x400;
    const TARGET_MODIFIERS: u32 = 0x500;

    fn put_keywords(e: &mut Engine) {
        e.mem.set_cstr(KEYWORD_EXPRESSION, b"expression");
        e.mem.set_cstr(KEYWORD_PHONEME, b"phoneme");
        e.mem.set_cstr(KEYWORD_MODIFIER, b"modifier");
        e.mem.set_cstr(KEYWORD_COORD, b"coord");
        e.mem.set_cstr(KEYWORD_RESET, b"reset");
    }

    /// `ParseParameters` double for `ModifyFaceGen`: the keyword, the index
    /// and the value.
    fn parse_face_gen(e: &mut Engine, ok: bool, keyword: &str, index: i32, value: i32) {
        let keyword = keyword.as_bytes().to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], &keyword);
            e.mem.set_i32(a[8], index);
            e.mem.set_i32(a[9], value);
            ok.into_ret()
        });
    }

    /// The reference, the face generation target and the holder objects
    /// `ModifyFaceGen` works on.
    struct FaceGen {
        a: ScriptArgs,
        reference: u32,
        target: u32,
        holder_1b0: u32,
        holder_1ac: u32,
    }

    fn face_gen_setup() -> (Engine, FaceGen) {
        let mut e = engine();
        put_keywords(&mut e);
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(ZERO_FILL, |_, _| Ret::default());
        e.register(STRICMP, |e, a| {
            let first = e.mem.cstr(a[0]).to_ascii_lowercase();
            let second = e.mem.cstr(a[1]).to_ascii_lowercase();
            (first.cmp(&second) as i32 as u32).into_ret()
        });
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(BS_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_APPEND, |_, _| Ret::default());
        e.register(BS_STRING_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xc0c0);
            e.mem.set_u32(a[0] + 4, 0x0505);
            a[0].into_ret()
        });
        e.register(PRINT_LINE, |_, _| Ret::default());
        e.register(EXPRESSION_NAME, |_, a| (0x5000 + a[0]).into_ret());
        e.register(PHONEME_NAME, |_, a| (0x6000 + a[0]).into_ret());
        e.register(MODIFIER_NAME, |_, a| (0x7000 + a[0]).into_ret());
        e.register(SET_FACE_GEN_MODE_BYTE, |_, _| Ret::default());
        // Reference.
        e.register(V_TARGET, |e, a| e.mem.u32(a[0] + REF_TARGET).into_ret());
        e.register(V_OBJECT_1B0, |e, a| {
            e.mem.u32(a[0] + REF_OBJECT_1B0).into_ret()
        });
        e.register(V_OBJECT_1AC, |e, a| {
            e.mem.u32(a[0] + REF_OBJECT_1AC).into_ret()
        });
        e.register(V_FLAG_218, |e, a| e.mem.u32(a[0] + REF_FLAG_218).into_ret());
        e.register(V_NODE, |e, a| e.mem.u32(a[0] + REF_NODE).into_ret());
        e.register(V_ANIMATION, |e, a| {
            e.mem.u32(a[0] + REF_ANIMATION).into_ret()
        });
        e.register(V_BASE_DATA, |_, _| 0xda7a.into_ret());
        // Target.
        e.register(V_SET_EXPRESSION, |_, _| Ret::default());
        e.register(V_GET_PHONEME, |_, a| (a[1] as f32 / 10.0).into_ret());
        e.register(V_SET_PHONEMES, |e, a| {
            for i in 0..16 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + TARGET_PHONEMES + 4 * i, word);
            }
            Ret::default()
        });
        e.register(V_SET_MODIFIERS, |e, a| {
            for i in 0..17 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + TARGET_MODIFIERS + 4 * i, word);
            }
            Ret::default()
        });
        e.register(V_RESET, |_, _| Ret::default());
        let target = object_with(
            &mut e,
            &[
                (0x114, V_SET_EXPRESSION),
                (0xa4, V_GET_PHONEME),
                (0xec, V_SET_PHONEMES),
                (0xf0, V_SET_MODIFIERS),
                (0xb4, V_RESET),
            ],
        );
        // The modifier values: 17 floats `0.5 * index` behind `target + 0xa8`.
        let values = e.mem.alloc(17 * 4);
        for i in 0..17 {
            e.mem.set_f32(values + 4 * i, 0.5 * i as f32);
        }
        e.mem.set_u32(target + 0xa8 + 0xc, values);
        e.mem.set_u32(target + 0xa8 + 0x10, 17);
        let holder_1b0 = object(&mut e);
        let holder_1ac = object(&mut e);
        let reference = object_with(
            &mut e,
            &[
                (0x1b8, V_TARGET),
                (0x1b0, V_OBJECT_1B0),
                (0x1ac, V_OBJECT_1AC),
                (0x218, V_FLAG_218),
                (0x1d0, V_NODE),
                (0x1e4, V_ANIMATION),
                (0x1e8, V_BASE_DATA),
            ],
        );
        e.mem.set_u32(reference + REF_TARGET, target);
        e.mem.set_u32(reference + REF_OBJECT_1B0, holder_1b0);
        e.mem.set_u32(reference + REF_OBJECT_1AC, holder_1ac);
        let a = command(&mut e, reference);
        (
            e,
            FaceGen {
                a,
                reference,
                target,
                holder_1b0,
                holder_1ac,
            },
        )
    }

    fn run_face_gen(e: &mut Engine, g: &FaceGen) -> bool {
        start_log(e);
        e.call(0x005d_c9a0, &args![g.a]).bool()
    }

    #[test]
    fn modify_face_gen_fails_when_the_parameters_do_not_parse() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, false, "expression", 1, 2);
        assert!(!run_face_gen(&mut e, &g));
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // The keyword buffer got a leading NUL and `memset` cleared the rest.
        assert_eq!(calls(&e, MEMSET)[0][1..], [0, 0x1ff]);
        // The parameters: the buffer, then the index and the value cells.
        let parse = &calls(&e, PARSE_PARAMETERS)[0];
        assert_eq!(parse.len(), 10);
        assert_eq!(parse[7] + 1, calls(&e, MEMSET)[0][0]);
    }

    #[test]
    fn modify_face_gen_without_a_keyword_prints_expression() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "", -1, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![KEYWORD_EXPRESSION]]);
        assert!(calls(&e, V_TARGET).is_empty());
    }

    #[test]
    fn modify_face_gen_needs_a_target_object() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "expression", 3, 50);
        // No reference.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c9a0, &args![none]).bool());
        assert!(calls(&e, V_TARGET).is_empty());
        // The reference has no target.
        e.mem.set_u32(g.reference + REF_TARGET, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, V_TARGET), vec![vec![g.reference, 0]]);
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
    }

    /// The names of a printed list: `(format, index, name)` of every
    /// `sprintf_s` call and the number of `PrintLine` calls.
    fn printed_names(e: &Engine) -> (Vec<(u32, u32, u32)>, usize) {
        (
            calls(e, SPRINTF_S)
                .iter()
                .map(|w| {
                    assert_eq!(w[1], 0x200);
                    (w[2], w[3], w[4])
                })
                .collect(),
            calls(e, PRINT_LINE).len(),
        )
    }

    fn check_name_list(keyword: &str, count: u32, name_base: u32) {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, keyword, -1, 0);
        assert!(run_face_gen(&mut e, &g));
        let (names, printed) = printed_names(&e);
        assert_eq!(names.len(), count as usize);
        for (i, (format, index, name)) in names.iter().enumerate() {
            let last = i as u32 == count - 1;
            assert_eq!(
                *format,
                if last {
                    FORMAT_NAME_LAST
                } else {
                    FORMAT_NAME_MORE
                }
            );
            assert_eq!(*index, i as u32);
            assert_eq!(*name, name_base + i as u32);
        }
        assert_eq!(printed, 1);
        // One append per name, then the string is printed and destroyed.
        assert_eq!(calls(&e, BS_STRING_APPEND).len(), count as usize);
        assert_eq!(
            calls(&e, BS_STRING_CONSTRUCT),
            calls(&e, BS_STRING_DESTRUCT)
        );
        // No virtual function of the target was called.
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
        assert!(calls(&e, V_SET_PHONEMES).is_empty());
        assert!(calls(&e, V_SET_MODIFIERS).is_empty());
    }

    #[test]
    fn modify_face_gen_lists_the_expressions() {
        check_name_list("expression", 15, 0x5000);
    }

    #[test]
    fn modify_face_gen_lists_the_phonemes() {
        check_name_list("phoneme", 16, 0x6000);
    }

    #[test]
    fn modify_face_gen_lists_the_modifiers() {
        check_name_list("MODIFIER", 17, 0x7000);
    }

    #[test]
    fn modify_face_gen_sets_one_expression() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "Expression", 3, 50);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(
            calls(&e, V_SET_EXPRESSION),
            vec![vec![g.target, 3, 0.5f32.to_bits()]]
        );
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    fn phoneme_array(e: &Engine, g: &FaceGen) -> Vec<f32> {
        (0..16)
            .map(|i| e.mem.f32(g.target + TARGET_PHONEMES + 4 * i))
            .collect()
    }

    #[test]
    fn modify_face_gen_replaces_one_phoneme_and_keeps_the_others() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "phoneme", 2, 25);
        assert!(run_face_gen(&mut e, &g));
        let mut expected: Vec<f32> = (0..16).map(|i| i as f32 / 10.0).collect();
        expected[2] = 0.25;
        assert_eq!(phoneme_array(&e, &g), expected);
        // The array is zeroed first, the current values are asked for every
        // other phoneme and the array goes to slot 0xec with 1.0.
        assert_eq!(calls(&e, ZERO_FILL)[0][1..], [0, 0x40]);
        assert_eq!(calls(&e, V_GET_PHONEME).len(), 15);
        let set = &calls(&e, V_SET_PHONEMES)[0];
        assert_eq!((set[0], set[2]), (g.target, 1.0f32.to_bits()));
        assert_eq!(set[1], calls(&e, ZERO_FILL)[0][0]);
        // The mode byte is not touched.
        assert!(calls(&e, SET_FACE_GEN_MODE_BYTE).is_empty());
    }

    #[test]
    fn modify_face_gen_ignores_an_out_of_range_index() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "phoneme", 40, 25);
        assert!(run_face_gen(&mut e, &g));
        let expected: Vec<f32> = (0..16).map(|i| i as f32 / 10.0).collect();
        assert_eq!(phoneme_array(&e, &g), expected);
        assert_eq!(calls(&e, V_GET_PHONEME).len(), 16);
    }

    #[test]
    fn modify_face_gen_replaces_one_modifier_after_clearing_the_mode_byte() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "modifier", 5, 200);
        assert!(run_face_gen(&mut e, &g));
        let mut expected: Vec<f32> = (0..17).map(|i| 0.5 * i as f32).collect();
        expected[5] = 2.0;
        let modifiers: Vec<f32> = (0..17)
            .map(|i| e.mem.f32(g.target + TARGET_MODIFIERS + 4 * i))
            .collect();
        assert_eq!(modifiers, expected);
        // The mode byte is cleared before the array is built.
        let order = call_order(&e);
        let mode = order
            .iter()
            .position(|a| *a == SET_FACE_GEN_MODE_BYTE)
            .unwrap();
        let zero = order.iter().position(|a| *a == ZERO_FILL).unwrap();
        assert!(mode < zero);
        assert_eq!(calls(&e, SET_FACE_GEN_MODE_BYTE), vec![vec![0]]);
        assert_eq!(calls(&e, ZERO_FILL)[0][1..], [0, 0x44]);
        let set = &calls(&e, V_SET_MODIFIERS)[0];
        assert_eq!((set[0], set[2]), (g.target, 1.0f32.to_bits()));
    }

    fn fill_holder(e: &mut Engine, holder: u32) {
        e.mem.set_u8(holder + 0xec, 0xaa);
        e.mem.set_u8(holder + 0xd4, 0xaa);
        e.mem.set_u8(holder + 0xe3, 0xaa);
    }

    #[test]
    fn modify_face_gen_coord_minus_one_attaches_the_object_and_clears_the_bytes() {
        let (mut e, g) = face_gen_setup();
        fill_holder(&mut e, g.holder_1b0);
        parse_face_gen(&mut e, true, "coord", -1, 77);
        e.set_global(FACE_GEN_COORD_VALUE, 5u32);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, V_OBJECT_1B0), vec![vec![g.reference, 0]]);
        assert_eq!(e.mem.u32(g.holder_1b0 + 0xe8), g.reference);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xec), 0);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xd4), 0);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xe3), 0);
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 5);
    }

    #[test]
    fn modify_face_gen_coord_one_sets_the_bytes_and_stores_the_value() {
        let (mut e, g) = face_gen_setup();
        fill_holder(&mut e, g.holder_1b0);
        parse_face_gen(&mut e, true, "coord", 1, 77);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(e.mem.u32(g.holder_1b0 + 0xe8), g.reference);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xec), 1);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xd4), 1);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xe3), 1);
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 77);
        // Other indices do nothing.
        parse_face_gen(&mut e, true, "coord", 2, 99);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 77);
        assert!(calls(&e, V_OBJECT_1B0).is_empty());
    }

    /// Everything the head rebuild (`coord 0`) calls, with a recording
    /// double each; the reference is an actor with a node and an animation.
    fn rebuild_setup() -> (Engine, FaceGen, u32, u32) {
        let (mut e, g) = face_gen_setup();
        let base_form = object(&mut e);
        e.mem.set_u32(g.reference + 0x20, base_form);
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.mem.set_u32(g.reference + REF_FLAG_218, 1);
        let node = object_with(&mut e, &[(0xc, V_NODE_DATA)]);
        e.register(V_NODE_DATA, |_, _| 0x1111.into_ret());
        e.mem.set_u32(g.reference + REF_NODE, node);
        e.mem.set_u32(g.reference + REF_ANIMATION, 0xa11);
        e.register(POINTER_AT_D8, |_, _| 0xd8d8.into_ret());
        e.register(POINTER_AT_78, |_, _| 0x7878.into_ret());
        // The holders answer `+0x18` with an owner object.
        let owner = object_with(&mut e, &[(0xe8, V_NODE_E8)]);
        e.register(V_NODE_E8, |_, _| Ret::default());
        e.register_double(FIELD_18, move |_, _| owner.into_ret());
        e.register(RECURSE_REMOVE_FROM_PALETTE, |_, _| Ret::default());
        let process = object_with(
            &mut e,
            &[
                (0x79c, V_PROCESS_79C),
                (0x794, V_PROCESS_794),
                (0x7a4, V_PROCESS_7A4),
                (0x58, V_PROCESS_58),
            ],
        );
        e.register(V_PROCESS_79C, |_, _| Ret::default());
        e.register(V_PROCESS_794, |_, _| Ret::default());
        e.register(V_PROCESS_7A4, |_, _| Ret::default());
        e.register(V_PROCESS_58, |_, _| Ret::default());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(NI_POINTER_SET, |_, a| a[0].into_ret());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(TES_NPC_INIT_HEAD, |e, a| {
            e.mem.set_u32(a[1], 11);
            e.mem.set_u32(a[2], 22);
            Ret::default()
        });
        e.register(TES_NPC_UPDATE_HEAD, |_, _| Ret::default());
        e.register(TES_NPC_GET_RACE, |_, _| 0x4ace.into_ret());
        e.register(TES_RACE_GET_FACE_GEN_DATA, |_, _| Ret::default());
        e.register(FLOAT_AND_FLAGS_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_APPLY, |_, _| Ret::default());
        face_gen_data_doubles(&mut e);
        (e, g, base_form, node)
    }

    /// Doubles for what the face generation data's constructor and destructor
    /// call.
    fn face_gen_data_doubles(e: &mut Engine) {
        for address in [
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            ARRAY_94_CONSTRUCT,
            ARRAY_A4_CONSTRUCT,
            ARRAY_B4_CONSTRUCT,
            ARRAY_C4_CONSTRUCT,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            LIST_PRE_DESTRUCT,
            ARRAY_RESET,
            ARRAY_94_DESTRUCT,
            ARRAY_A4_DESTRUCT,
            ARRAY_B4_DESTRUCT_WRAPPER,
            ARRAY_C4_DESTRUCT_WRAPPER,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn modify_face_gen_coord_zero_rebuilds_the_head() {
        let (mut e, g, base_form, node) = rebuild_setup();
        // The animation's pointer chain: `00496940` then `00537bd0`.
        parse_face_gen(&mut e, true, "coord", 0, 0);
        assert!(run_face_gen(&mut e, &g));
        // The palette is cleaned of both head objects with the skeleton
        // `00537bd0` found.
        assert_eq!(
            calls(&e, RECURSE_REMOVE_FROM_PALETTE),
            vec![vec![g.holder_1b0, 0x7878], vec![g.holder_1ac, 0x7878],]
        );
        assert_eq!(calls(&e, POINTER_AT_78), vec![vec![0xd8d8]]);
        assert_eq!(calls(&e, V_OBJECT_1B0), vec![vec![g.reference, 0]]);
        assert_eq!(calls(&e, V_OBJECT_1AC), vec![vec![g.reference, 0]]);
        assert_eq!(calls(&e, V_PROCESS_79C).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_794).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_7A4).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_58).len(), 1);
        assert_eq!(calls(&e, V_NODE_E8).len(), 2);
        // The head: cleared, initialised with two outputs and updated with
        // the data of slot 0x1e8 and the two values `InitHead` produced.
        assert_eq!(calls(&e, NI_POINTER_SET), vec![vec![base_form + 0x1c4, 0]]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![base_form + 0x1c8, base_form + 0x1c4]]
        );
        let init = &calls(&e, TES_NPC_INIT_HEAD)[0];
        assert_eq!(init[0], base_form);
        assert_eq!(init[2], init[1] + 4);
        assert_eq!(
            calls(&e, TES_NPC_UPDATE_HEAD),
            vec![vec![base_form, g.reference, 0xda7a, 11, 22]]
        );
        // The face generation data is built, filled by the race and torn
        // down; the node is told about the 16-byte scratch object.
        let construct = &calls(&e, ARRAY_94_CONSTRUCT)[0];
        let face_gen = construct[0] - 0x94;
        assert_eq!(
            calls(&e, TES_RACE_GET_FACE_GEN_DATA),
            vec![vec![0x4ace, base_form, face_gen, 0, 0]]
        );
        assert_eq!(calls(&e, TES_NPC_GET_RACE), vec![vec![base_form]]);
        let scratch = &calls(&e, FLOAT_AND_FLAGS_CONSTRUCT)[0];
        assert_eq!(scratch[1..], [0.0f32.to_bits(), 0, 0]);
        assert_eq!(calls(&e, NODE_APPLY), vec![vec![node, scratch[0]]]);
        assert_eq!(calls(&e, ARRAY_94_DESTRUCT), vec![vec![face_gen + 0x94]]);
        // Order: constructed, filled, applied, destroyed.
        let order = call_order(&e);
        let position = |address: u32| order.iter().position(|a| *a == address).unwrap();
        assert!(position(ARRAY_94_CONSTRUCT) < position(TES_RACE_GET_FACE_GEN_DATA));
        assert!(position(TES_RACE_GET_FACE_GEN_DATA) < position(NODE_APPLY));
        assert!(position(NODE_APPLY) < position(ARRAY_94_DESTRUCT));
        assert!(position(TES_NPC_UPDATE_HEAD) < position(ARRAY_94_CONSTRUCT));
    }

    #[test]
    fn modify_face_gen_coord_zero_skips_the_palette_without_a_node_and_needs_the_flag() {
        let (mut e, g, base_form, _) = rebuild_setup();
        parse_face_gen(&mut e, true, "coord", 0, 0);
        // No node: the palette is left alone but the head is rebuilt.
        e.mem.set_u32(g.reference + REF_NODE, 0);
        assert!(run_face_gen(&mut e, &g));
        assert!(calls(&e, RECURSE_REMOVE_FROM_PALETTE).is_empty());
        assert!(calls(&e, V_ANIMATION).is_empty());
        assert_eq!(calls(&e, V_PROCESS_58).len(), 1);
        assert_eq!(calls(&e, TES_NPC_UPDATE_HEAD)[0][0], base_form);
        assert_eq!(calls(&e, NODE_APPLY)[0][0], 0);
        // Without the flag of slot 0x218 nothing happens.
        e.mem.set_u32(g.reference + REF_FLAG_218, 0);
        assert!(run_face_gen(&mut e, &g));
        assert!(calls(&e, V_NODE).is_empty());
        assert!(calls(&e, TES_NPC_INIT_HEAD).is_empty());
    }

    #[test]
    fn modify_face_gen_reset_asks_the_target_and_sets_the_mode_byte() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "reset", -1, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(
            calls(&e, V_RESET),
            vec![vec![g.target, 1.0f32.to_bits(), 1, 1, 1, 1, 0]]
        );
        assert_eq!(calls(&e, SET_FACE_GEN_MODE_BYTE), vec![vec![1]]);
    }

    #[test]
    fn modify_face_gen_ignores_unknown_keywords() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "nonsense", 1, 1);
        assert!(run_face_gen(&mut e, &g));
        for slot in [V_SET_EXPRESSION, V_SET_PHONEMES, V_SET_MODIFIERS, V_RESET] {
            assert!(calls(&e, slot).is_empty());
        }
        assert!(calls(&e, SET_FACE_GEN_MODE_BYTE).is_empty());
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    // ---- The face generation data ------------------------------------------------

    #[test]
    fn fn_005dd4e0_reads_a_float_array_with_an_unsigned_range_check() {
        let mut e = engine();
        let this = object(&mut e);
        let data = e.mem.alloc(16);
        for i in 0..3 {
            e.mem.set_f32(data + 4 * i, 1.5 + i as f32);
        }
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 3);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 0u32]).f32(), 1.5);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 2u32]).f32(), 3.5);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 3u32]).f32(), 0.0);
        // A negative index is a huge unsigned one.
        assert_eq!(e.call(0x005d_d4e0, &args![this, -1i32]).f32(), 0.0);
    }

    #[test]
    fn fn_005dd4a0_reads_the_array_at_0xa8_for_indices_below_17() {
        let mut e = engine();
        let this = object(&mut e);
        let data = e.mem.alloc(80);
        for i in 0..20 {
            e.mem.set_f32(data + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(this + 0xa8 + 0xc, data);
        e.mem.set_u32(this + 0xa8 + 0x10, 20);
        assert_eq!(e.call(0x005d_d4a0, &args![this, 0i32]).f32(), 10.0);
        assert_eq!(e.call(0x005d_d4a0, &args![this, 16i32]).f32(), 26.0);
        // 17 and above are zero even though the array has more.
        assert_eq!(e.call(0x005d_d4a0, &args![this, 17i32]).f32(), 0.0);
        // The signed comparison lets a negative index through to the
        // unsigned range check.
        assert_eq!(e.call(0x005d_d4a0, &args![this, -1i32]).f32(), 0.0);
    }

    #[test]
    fn fn_005dd520_and_fn_005dd540_store_single_bytes() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x005d_d520, &args![this, 1u32]);
        e.call(0x005d_d540, &args![this, 2u32]);
        assert_eq!(e.mem.u8(this + 0xd4), 1);
        assert_eq!(e.mem.u8(this + 0xe3), 2);
        assert_eq!(e.mem.u8(this + 0xd5), 0);
    }

    #[test]
    fn tes_npc_clear_head_empties_one_pointer_and_assigns_it_to_the_other() {
        let mut e = engine();
        e.register(NI_POINTER_SET, |_, a| a[0].into_ret());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_d560, &args![0x1000u32]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (NI_POINTER_SET, vec![0x11c4, 0]),
                (NI_POINTER_ASSIGN, vec![0x11c8, 0x11c4]),
            ]
        );
    }

    #[test]
    fn fn_005dd590_constructs_the_face_generation_data() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        let this = e.mem.alloc(0x100);
        for offset in (0..0x100).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        let returned = e.call(0x005d_d590, &args![this]).u32();
        assert_eq!(returned, this);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (
                    VECTOR_CONSTRUCT,
                    vec![this, 0x20, 4, ELEMENT_CONSTRUCT, ELEMENT_DESTRUCT]
                ),
                (ARRAY_94_CONSTRUCT, vec![this + 0x94, 0, 1]),
                (ARRAY_A4_CONSTRUCT, vec![this + 0xa4, 0, 1]),
                (ARRAY_B4_CONSTRUCT, vec![this + 0xb4, 0, 1]),
                (ARRAY_C4_CONSTRUCT, vec![this + 0xc4, 0, 1]),
                (LIST_CONSTRUCT, vec![this + 0xe4]),
                (ARRAY_RESET, vec![this + 0x94]),
                (ARRAY_RESET, vec![this + 0xa4]),
                (ARRAY_RESET, vec![this + 0xb4]),
            ]
        );
        for offset in [0x80, 0x84, 0x88, 0x8c, 0x90] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0xd4), 0);
        assert_eq!(e.mem.u8(this + 0xd5), 0);
        assert_eq!(e.mem.u32(this + 0xe0), 0xffff_ffff);
        // Fields the constructor does not touch keep their bytes.
        assert_eq!(e.mem.u32(this + 0x94), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0xd8), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_005dd6f0_destroys_the_parts_in_reverse_order() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        let this = 0x2000u32;
        start_log(&mut e);
        e.call(0x005d_d6f0, &args![this]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (LIST_PRE_DESTRUCT, vec![this + 0xe4]),
                (LIST_DESTRUCT, vec![this + 0xe4]),
                (ARRAY_C4_DESTRUCT_WRAPPER, vec![this + 0xc4]),
                (ARRAY_B4_DESTRUCT_WRAPPER, vec![this + 0xb4]),
                (ARRAY_A4_DESTRUCT, vec![this + 0xa4]),
                (ARRAY_94_DESTRUCT, vec![this + 0x94]),
                (VECTOR_DESTRUCT, vec![this, 0x20, 4, ELEMENT_DESTRUCT]),
            ]
        );
    }

    #[test]
    fn fn_005dd7b0_and_fn_005dd7d0_call_the_array_destructors() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        start_log(&mut e);
        e.call(0x005d_d7b0, &args![0x3000u32]);
        e.call(0x005d_d7d0, &args![0x4000u32]);
        assert_eq!(calls(&e, ARRAY_94_DESTRUCT), vec![vec![0x3000]]);
        assert_eq!(calls(&e, ARRAY_A4_DESTRUCT), vec![vec![0x4000]]);
    }
}
