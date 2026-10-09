//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), part 2: its functions from `0041a6a0` up to
//! (not including) `0041db00` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradatalist`]; anything public there may be used here.
//!
//! Translated so far, in address order: `0041a6a0` to `0041b4b0` (the first
//! 40 functions of the range): the sound, ghost, worn, can-not-wear, seed and
//! package start location setters, the one-line removers of an extra data by
//! type, the starting position and rotation accessors, the starting world or
//! cell accessors and the `ExtraAction` flag and reference setters.
//!
//! Every list operation of the main file is called by its exe address
//! (`GetExtraData` `00410220`, `RemoveExtra` `00410020` and `00410140`,
//! `AddExtra` `0040ff60`, `HasExtra` `0040fe80`), as the functions of another
//! file are. The extra data types are `EXTRA_DATA_TYPE` of the Xbox PDB (the
//! `cEtype` byte of the extra data).
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chains) of the
//! functions that allocate are not translated.

#[allow(unused_imports)]
use super::extradatalist::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees and constants

/// `BaseExtraList::HasExtra(type)`.
const HAS_EXTRA: u32 = 0x0040_fe80;
/// `BaseExtraList::AddExtra(extra)`.
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `BaseExtraList::RemoveExtra(extra, destroy)`.
const REMOVE_EXTRA: u32 = 0x0041_0020;
/// `BaseExtraList::RemoveExtra(type)`: deletes the first extra data of that
/// type.
const REMOVE_EXTRA_BY_TYPE: u32 = 0x0041_0140;
/// `BaseExtraList::GetExtraData(type)`.
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSSoundHandle::~BSSoundHandle` (`00483710`, an empty function).
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// The destructor of `ScriptLocals` (`005a8bc0`; the engine map names it
/// `ScriptLocals::ScriptLocals`, but the body runs two member destructors and
/// frees the variable block at +0x10, so it is the destructor), which the
/// scalar deleting destructor `0041af70` runs.
const SCRIPT_LOCALS_DESTRUCTOR: u32 = 0x005a_8bc0;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// The form id getter (`MOV EAX,[ECX+0x0C]`, `TESForm::iFormID`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The diagnostic logger (`cdecl`, format first; the PC build's body returns
/// 0 and prints nothing).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `"FORMS: Reference %s %08X has no parent save cell"`, the format
/// `SetStartingWorldOrCellForRef` logs when the reference has no child cell.
const MESSAGE_NO_PARENT_SAVE_CELL: u32 = 0x0101_5140;
/// Slot (byte offset in the vtable) of the reference's virtual whose result
/// the message above prints for `%s`.
const REFERENCE_NAME_SLOT: u32 = 0x130;
/// The list's type `0x0E` extra data (`ExtraAction`) or a new one
/// (`004195b0`, main file).
const GET_OR_ADD_ACTION: u32 = 0x0041_95b0;
/// The list's `ExtraStartingPosition` or a new one made from a position
/// (`00418d50`, main file).
const GET_OR_ADD_STARTING_POSITION: u32 = 0x0041_8d50;

const EXTRA_SCRIPT: u8 = 0x0d;
const EXTRA_ACTION: u8 = 0x0e;
const EXTRA_STARTING_POSITION: u8 = 0x0f;
const EXTRA_ANIM: u8 = 0x10;
const EXTRA_CONTAINER_CHANGES: u8 = 0x15;
const EXTRA_WORN: u8 = 0x16;
const EXTRA_WORN_LEFT: u8 = 0x17;
const EXTRA_PACKAGE_START_LOCATION: u8 = 0x18;
const EXTRA_GHOST: u8 = 0x1f;
const EXTRA_OWNERSHIP: u8 = 0x21;
const EXTRA_COUNT: u8 = 0x24;
const EXTRA_HEALTH: u8 = 0x25;
const EXTRA_LIGHT: u8 = 0x29;
const EXTRA_LOCK: u8 = 0x2a;
const EXTRA_TELEPORT: u8 = 0x2b;
const EXTRA_ANIM_SAVE: u8 = 0x2d;
const EXTRA_SCALE: u8 = 0x30;
const EXTRA_SEED: u8 = 0x31;
const EXTRA_CAN_NOT_WEAR: u8 = 0x3e;
const EXTRA_POISON: u8 = 0x3f;
const EXTRA_MAGIC_LIGHT: u8 = 0x40;
const EXTRA_STARTING_WORLD_OR_CELL: u8 = 0x49;
const EXTRA_SOUND: u8 = 0x4f;
const EXTRA_ACTIVATE_LOOP_SOUND: u8 = 0x87;

/// Constructors of the extra data these setters build (`this` = the new
/// block; the other words are the constructor's arguments).
const EXTRA_GHOST_INIT: u32 = 0x0043_22c0;
const EXTRA_WORN_INIT: u32 = 0x0043_22f0;
const EXTRA_WORN_LEFT_INIT: u32 = 0x0043_2320;
const EXTRA_CAN_NOT_WEAR_INIT: u32 = 0x0043_2350;
const EXTRA_SEED_INIT: u32 = 0x0043_25b0;
const EXTRA_PACKAGE_START_LOCATION_INIT: u32 = 0x0043_26c0;
const EXTRA_STARTING_WORLD_OR_CELL_INIT: u32 = 0x0043_08c0;
/// Constructors of the two sound extra data; each takes a `BSSoundHandle`
/// by value (three words).
const EXTRA_SOUND_INIT: u32 = 0x0043_60c0;
const EXTRA_ACTIVATE_LOOP_SOUND_INIT: u32 = 0x0043_6660;

// ---------------------------------------------------------------------------
// Helpers

/// The first extra data of `extra_type` in the list (`GetExtraData`).
fn find_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> Ptr<BSExtraData> {
    e.call(GET_EXTRA_DATA, &args![list, extra_type]).ptr()
}

/// Whether the list has an extra data of `extra_type` (`HasExtra`).
fn has_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> bool {
    e.call(HAS_EXTRA, &args![list, extra_type]).bool()
}

/// Adds `extra` to the list (`AddExtra`).
fn add_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: u32) {
    e.call(ADD_EXTRA, &args![list, extra]);
}

/// Unlinks `extra` and deletes it (`RemoveExtra(extra, true)`).
fn remove_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    e.call(REMOVE_EXTRA, &args![list, extra, true]);
}

/// Deletes the first extra data of `extra_type` (`RemoveExtra(type)`).
fn remove_extra_by_type(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    e.call(REMOVE_EXTRA_BY_TYPE, &args![list, extra_type]);
}

/// `new` and construct: allocates `size` bytes and runs the constructor at
/// `construct` on the block (`this` = the block, then `construct_args`); a
/// failed allocation gives a null extra data. The result is added to the
/// list either way, as the code does. Returns the extra data.
fn add_new_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    size: u32,
    construct: u32,
    construct_args: &[u32],
) -> Ptr<BSExtraData> {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let extra = if block == 0 {
        0
    } else {
        let mut words = vec![block];
        words.extend_from_slice(construct_args);
        e.call(construct, &words).u32()
    };
    add_extra(e, list, extra);
    Ptr::new(extra)
}

/// A `float` loaded and stored through the x87 stack (`FLD`/`FSTP`): the
/// same bits, except that a signalling NaN comes out quiet.
fn x87_float_bits(value: f32) -> u32 {
    let bits = value.to_bits();
    let is_nan = bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0;
    if is_nan {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// The body of the two sound setters. With an extra data of `extra_type` in
/// the list: a sound handle with the same `iSoundID` as an empty handle
/// (`fn_0041a1f0` against a fresh `fn_0041a250` handle) deletes the extra
/// data, any other is copied into it (`fn_00418900`, the `BSSoundHandle` at
/// +0x0C). Without one, a non-empty handle (`fn_0041a220`) builds an extra
/// data of `0x18` bytes with `construct`, which takes the handle by value (a
/// copy made by `fn_00418900` on the stack, three words), and adds it; an
/// empty handle does nothing. The temporary empty handles are destroyed by
/// `00483710`.
fn set_sound_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    sound: Ptr<BSSoundHandle>,
) {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        let is_empty = e.with_stack(0x0c, |e, empty| {
            fn_0041a250(e, empty);
            let equal = fn_0041a1f0(e, sound.cast(), empty);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
            equal
        });
        if is_empty {
            remove_extra(e, list, extra);
        } else {
            fn_00418900(e, Ptr::new(extra.addr() + 0x0c), sound.cast());
        }
        return;
    }
    let differs = e.with_stack(0x0c, |e, empty| {
        fn_0041a250(e, empty);
        let differs = fn_0041a220(e, sound.cast(), empty);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
        differs
    });
    if !differs {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    let new_extra = if block == 0 {
        0
    } else {
        // The handle is passed by value: a copy on the stack, three words.
        let words = e.with_stack(0x0c, |e, copy| {
            fn_00418900(e, copy, sound.cast());
            [
                e.mem.u32(copy.addr()),
                e.mem.u32(copy.addr() + 4),
                e.mem.u32(copy.addr() + 8),
            ]
        });
        e.call(construct, &args![block, words[0], words[1], words[2]])
            .u32()
    };
    add_extra(e, list, new_extra);
}

/// The shape of the flag setters (ghost, can not wear, worn): an extra data
/// of `extra_type` with no payload (`0x0C` bytes, built by `construct`) is
/// added when `wanted` and the list has none, and deleted when not `wanted`
/// and the list has one.
fn set_flag_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    wanted: bool,
) {
    let present = has_extra(e, list, extra_type);
    if wanted && !present {
        add_new_extra(e, list, 0x0c, construct, &[]);
    } else if !wanted && present {
        remove_extra_by_type(e, list, extra_type);
    }
}

/// The list's type `0x0F` extra data (`ExtraStartingPosition`), made by
/// `00418d50` from `position` when the list has none.
fn starting_position_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    position: Ptr,
) -> Ptr<BSExtraData> {
    let extra = find_extra(e, list, EXTRA_STARTING_POSITION);
    if !extra.is_null() {
        return extra;
    }
    e.call(GET_OR_ADD_STARTING_POSITION, &args![list, position])
        .ptr()
}

/// Copies the three words at `source` to `target` (a `NiPoint3`).
fn copy_point(e: &mut Engine, source: u32, target: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
}

/// Stores the three words `x`, `y`, `z` at `target`.
fn store_point(e: &mut Engine, target: u32, x: u32, y: u32, z: u32) {
    e.mem.set_u32(target, x);
    e.mem.set_u32(target + 4, y);
    e.mem.set_u32(target + 8, z);
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 0041a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the sound of the type `0x87` extra data (`EXTRA_ACTIVATE_LOOP_SOUND`
/// in the Xbox enum, built by `00436660`, `0x18` bytes): see
/// `set_sound_extra` for the cases. `sound` is a pointer to the
/// `BSSoundHandle`. The engine map has no name for it (its getter is
/// `ExtraDataList::GetActivateLoopSound`).
pub fn fn_0041a6a0(e: &mut Engine, this: Ptr<ExtraDataList>, sound: Ptr<BSSoundHandle>) {
    set_sound_extra(
        e,
        this,
        EXTRA_ACTIVATE_LOOP_SOUND,
        EXTRA_ACTIVATE_LOOP_SOUND_INIT,
        sound,
    );
}

// Translated from 0041a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetSound` (Xbox PDB): as `fn_0041a6a0`, for the type
/// `0x4F` extra data (`EXTRA_SOUND`, built by `004360c0`).
pub fn extra_data_list_set_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    sound: Ptr<BSSoundHandle>,
) {
    set_sound_extra(e, this, EXTRA_SOUND, EXTRA_SOUND_INIT, sound);
}

// Translated from 0041a960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetGhost` (Xbox PDB): with `ghost` set, adds the type
/// `0x1F` extra data (`EXTRA_GHOST`, `0x0C` bytes, built by `004322c0`) when
/// the list has none; cleared, deletes it when the list has one.
pub fn extra_data_list_set_ghost(e: &mut Engine, this: Ptr<ExtraDataList>, ghost: bool) {
    set_flag_extra(e, this, EXTRA_GHOST, EXTRA_GHOST_INIT, ghost);
}

// Translated from 0041aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWorn` (Xbox PDB): the flag setter for the worn marker
/// of one hand. With `left` set it is the type `0x17` extra data
/// (`EXTRA_WORN_LEFT`, built by `00432320`), otherwise the type `0x16` one
/// (`EXTRA_WORN`, built by `004322f0`); `worn` adds it when missing and
/// cleared deletes it when present, as `ExtraDataList::SetGhost`.
pub fn extra_data_list_set_worn(e: &mut Engine, this: Ptr<ExtraDataList>, worn: bool, left: bool) {
    if left {
        set_flag_extra(e, this, EXTRA_WORN_LEFT, EXTRA_WORN_LEFT_INIT, worn);
    } else {
        set_flag_extra(e, this, EXTRA_WORN, EXTRA_WORN_INIT, worn);
    }
}

// Translated from 0041ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetCanNotWear` (Xbox PDB): the flag setter for the type
/// `0x3E` extra data (`EXTRA_CANNOTWEAR`, built by `00432350`), as
/// `ExtraDataList::SetGhost`.
pub fn extra_data_list_set_can_not_wear(e: &mut Engine, this: Ptr<ExtraDataList>, wanted: bool) {
    set_flag_extra(e, this, EXTRA_CAN_NOT_WEAR, EXTRA_CAN_NOT_WEAR_INIT, wanted);
}

// Translated from 0041ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the seed byte (`ExtraSeed`, type `0x31`, `0x10` bytes, byte at
/// +0x0C; built by `004325b0`). `0xFF` means no seed: it deletes an existing
/// extra data and does nothing without one. Any other value is stored in the
/// existing extra data or builds one. The engine map has no name for it.
pub fn fn_0041ac30(e: &mut Engine, this: Ptr<ExtraDataList>, seed: u8) {
    let extra = find_extra(e, this, EXTRA_SEED);
    if seed != 0xff {
        if extra.is_null() {
            add_new_extra(e, this, 0x10, EXTRA_SEED_INIT, &[seed as u32]);
        } else {
            e.mem.set_u8(extra.addr() + 0x0c, seed);
        }
    } else if !extra.is_null() {
        remove_extra(e, this, extra);
    }
}

// Translated from 0041ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageStartLocation` (Xbox PDB): the type `0x18` extra
/// data (`ExtraPackageStartLocation`, `0x20` bytes: a `WORLD_LOCATION` at
/// +0x0C, whose `pLocationForm` is `location_form`, or `cell` when that is
/// null, then the position and `fZRot`). Without one, builds it (`004326c0`
/// with the two forms, the position pointer and `z_rot`, which the code
/// moves through the x87 stack) and adds it. With one, stores the form and
/// copies the position; `z_rot` is not stored then.
pub fn extra_data_list_set_package_start_location(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    location_form: u32,
    cell: u32,
    position: Ptr,
    z_rot: f32,
) {
    let extra = find_extra(e, this, EXTRA_PACKAGE_START_LOCATION);
    if extra.is_null() {
        add_new_extra(
            e,
            this,
            0x20,
            EXTRA_PACKAGE_START_LOCATION_INIT,
            &[location_form, cell, position.addr(), x87_float_bits(z_rot)],
        );
        return;
    }
    if location_form != 0 {
        e.mem.set_u32(extra.addr() + 0x0c, location_form);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, cell);
    }
    copy_point(e, position.addr(), extra.addr() + 0x10);
}

// Translated from 0041adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveAnimPtr` (Xbox PDB): deletes the type `0x10` extra
/// data (`EXTRA_ANIM`).
pub fn extra_data_list_remove_anim_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_ANIM);
}

// Translated from 0041ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveAnimSave` (Xbox PDB): deletes the type `0x2D` extra
/// data (`EXTRA_ANIM_SAVE`).
pub fn extra_data_list_remove_anim_save(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_ANIM_SAVE);
}

// Translated from 0041ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x29` extra data (`EXTRA_LIGHT`). The engine map has no
/// name for it.
pub fn fn_0041ae30(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_LIGHT);
}

// Translated from 0041ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x40` extra data (`EXTRA_MAGIC_LIGHT`). The engine map
/// has no name for it.
pub fn fn_0041ae50(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_MAGIC_LIGHT);
}

// Translated from 0041ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveLockPtr` (Xbox PDB): deletes the type `0x2A` extra
/// data (`EXTRA_LOCK`).
pub fn extra_data_list_remove_lock_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_LOCK);
}

// Translated from 0041ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveTeleportPtr` (Xbox PDB): deletes the type `0x2B`
/// extra data (`EXTRA_TELEPORT`).
pub fn extra_data_list_remove_teleport_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_TELEPORT);
}

// Translated from 0041aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x15` extra data (`EXTRA_CONTAINER_CHANGES`). The
/// engine map has no name for it.
pub fn fn_0041aeb0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_CONTAINER_CHANGES);
}

// Translated from 0041aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveOwnership` (Xbox PDB): deletes the type `0x21` extra
/// data (`EXTRA_OWNERSHIP`).
pub fn extra_data_list_remove_ownership(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_OWNERSHIP);
}

// Translated from 0041aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveHealth` (Xbox PDB): deletes the type `0x25` extra
/// data (`EXTRA_HEALTH`).
pub fn extra_data_list_remove_health(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_HEALTH);
}

// Translated from 0041af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveCount` (Xbox PDB): deletes the type `0x24` extra
/// data (`EXTRA_COUNT`).
pub fn extra_data_list_remove_count(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_COUNT);
}

// Translated from 0041af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x3F` extra data (`EXTRA_POISON`). The engine map has
/// no name for it.
pub fn fn_0041af30(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_POISON);
}

// Translated from 0041af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x0D` extra data (`EXTRA_SCRIPT`). The engine map has
/// no name for it.
pub fn fn_0041af50(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_SCRIPT);
}

// Translated from 0041af70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `ScriptLocals` (the variables of a
/// script instance, the `pScriptVars` of an `ExtraScript`): runs the
/// destructor `005a8bc0`, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`. The engine map has no name for it.
pub fn script_locals_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SCRIPT_LOCALS_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0041afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::ClearScriptLocals` (Xbox PDB): sets `pScriptVars` (+0x10)
/// of the type `0x0D` extra data (`ExtraScript`) to null, without freeing
/// it; nothing without one.
pub fn extra_data_list_clear_script_locals(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_SCRIPT);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x10, 0);
    }
}

// Translated from 0041afd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x30` extra data (`EXTRA_SCALE`). The engine map has no
/// name for it.
pub fn fn_0041afd0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_SCALE);
}

// Translated from 0041aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x1F` extra data (`EXTRA_GHOST`). The engine map has no
/// name for it.
pub fn fn_0041aff0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_GHOST);
}

// Translated from 0041b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the worn marker of one hand: the type `0x17` extra data
/// (`EXTRA_WORN_LEFT`) with `left` set, the type `0x16` one (`EXTRA_WORN`)
/// otherwise. The engine map has no name for it.
pub fn fn_0041b010(e: &mut Engine, this: Ptr<ExtraDataList>, left: bool) {
    if left {
        remove_extra_by_type(e, this, EXTRA_WORN_LEFT);
    } else {
        remove_extra_by_type(e, this, EXTRA_WORN);
    }
}

// Translated from 0041b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveCannotWearExtra` (Xbox PDB): deletes the type `0x3E`
/// extra data (`EXTRA_CANNOTWEAR`).
pub fn extra_data_list_remove_cannot_wear_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_CAN_NOT_WEAR);
}

// Translated from 0041b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x18` extra data (`EXTRA_PACKAGESTARTLOC`). The engine
/// map has no name for it.
pub fn fn_0041b060(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_PACKAGE_START_LOCATION);
}

// Translated from 0041b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the rotation (`rot`, a `NiPoint3` at +0x18) of the list's type
/// `0x0F` extra data (`ExtraStartingPosition`) to `out`; the extra data is
/// made from `position` (`00418d50`) when the list has none. Returns `out`.
/// The engine map has no name for it.
pub fn fn_0041b080(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr, position: Ptr) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    copy_point(e, extra.addr() + 0x18, out.addr());
    out
}

// Translated from 0041b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `fn_0041b080`, for the position (`pos`, at +0x0C). Returns `out`. The
/// engine map has no name for it.
pub fn fn_0041b0d0(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr, position: Ptr) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    copy_point(e, extra.addr() + 0x0c, out.addr());
    out
}

// Translated from 0041b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the rotation (+0x18) of the list's `ExtraStartingPosition` (made from
/// `position` when missing, as `fn_0041b080`) to the three words
/// `x`, `y`, `z`, and stores the same words at `out`. Returns `out`. The
/// engine map has no name for it.
pub fn fn_0041b120(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
    position: Ptr,
    x: u32,
    y: u32,
    z: u32,
) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    store_point(e, extra.addr() + 0x18, x, y, z);
    store_point(e, out.addr(), x, y, z);
    out
}

// Translated from 0041b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `fn_0041b120`, for the position (+0x0C). Returns `out`. The engine map
/// has no name for it.
pub fn fn_0041b180(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
    position: Ptr,
    x: u32,
    y: u32,
    z: u32,
) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    store_point(e, extra.addr() + 0x0c, x, y, z);
    store_point(e, out.addr(), x, y, z);
    out
}

// Translated from 0041b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the form of the type `0x49` extra data (`ExtraStartingWorldOrCell`,
/// `0x10` bytes, `pStartingWorldOrCell` at +0x0C; built by `004308c0`). A
/// null `form` deletes it (`fn_0041b350`); otherwise the extra data is built
/// when the list has none and takes `form`. The engine map has no name for
/// it.
pub fn fn_0041b1e0(e: &mut Engine, this: Ptr<ExtraDataList>, form: u32) {
    if form == 0 {
        fn_0041b350(e, this);
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_STARTING_WORLD_OR_CELL);
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x10, EXTRA_STARTING_WORLD_OR_CELL_INIT, &[]);
    }
    e.mem.set_u32(extra.addr() + 0x0c, form);
}

// Translated from 0041b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetStartingWorldOrCellForRef` (Xbox PDB): sets the starting
/// world or cell from a reference. The reference's base object at +0x18
/// (slot 0 of the vtable stored there) gives a cell; the starting world or
/// cell is that cell's world space (`TESObjectCELL::GetWorldSpace`) when it
/// has one, the cell itself otherwise (`fn_0041b1e0`). When the reference
/// gives no cell, its form id and the result of its virtual `0x130` are
/// logged with the format at `01015140`.
pub fn extra_data_list_set_starting_world_or_cell_for_ref(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: Ptr,
) {
    let cell = e.vcall(reference.addr() + 0x18, 0, &args![]).u32();
    if cell != 0 {
        let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        if world != 0 {
            fn_0041b1e0(e, this, world);
        } else {
            fn_0041b1e0(e, this, cell);
        }
    } else {
        let form_id = e.call(GET_FORM_ID, &args![reference]).u32();
        let name = e
            .vcall(reference.addr(), REFERENCE_NAME_SLOT, &args![])
            .u32();
        e.call(
            LOG_MESSAGE,
            &args![MESSAGE_NO_PARENT_SAVE_CELL, name, form_id],
        );
    }
}

// Translated from 0041b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetStartingWorldOrCell` (Xbox PDB): the
/// `pStartingWorldOrCell` (+0x0C) of the type `0x49` extra data, or null.
pub fn extra_data_list_get_starting_world_or_cell(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_STARTING_WORLD_OR_CELL);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

// Translated from 0041b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x49` extra data (`EXTRA_STARTINGWORLDORCELL`). The
/// engine map has no name for it.
pub fn fn_0041b350(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_STARTING_WORLD_OR_CELL);
}

// Translated from 0041b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `eAction` byte (+0x0C) of the type `0x0E` extra data (`ExtraAction`),
/// or 1 when the list has none. The engine map has no name for it.
pub fn fn_0041b370(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        1
    } else {
        e.mem.u8(extra.addr() + 0x0c) as u32
    }
}

// Translated from 0041b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `eAction` of the list (`fn_0041b370`) has any bit of `mask`.
/// The engine map has no name for it.
pub fn fn_0041b3a0(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) -> bool {
    fn_0041b370(e, this) & mask != 0
}

// Translated from 0041b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `eAction` (+0x0C, the low byte of `action`) of the type `0x0E` extra
/// data (`ExtraAction`). Without one, a value other than 1 (the default
/// `fn_0041b370` reports) gets one from `004195b0`, and 1 does nothing. With
/// one, the value 1 deletes it when its `pActionRef` (+0x10) is null. (The
/// engine map names this `ExtraDataList::SetGlobal`; the body is the
/// `ExtraAction` setter.)
pub fn fn_0041b3d0(e: &mut Engine, this: Ptr<ExtraDataList>, action: u32) {
    let mut extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        if action != 1 {
            extra = e.call(GET_OR_ADD_ACTION, &args![this]).ptr();
        }
    } else if action == 1 && e.mem.u32(extra.addr() + 0x10) == 0 {
        remove_extra(e, this, extra);
        extra = Ptr::NULL;
    }
    if !extra.is_null() {
        e.mem.set_u8(extra.addr() + 0x0c, action as u8);
    }
}

// Translated from 0041b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits of `mask` in `eAction`: `fn_0041b3d0(fn_0041b370 | mask)`.
/// The engine map has no name for it.
pub fn fn_0041b440(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) {
    let action = fn_0041b370(e, this) | mask;
    fn_0041b3d0(e, this, action);
}

// Translated from 0041b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `mask` in `eAction`: `fn_0041b3d0(fn_0041b370 & !mask)`.
/// The engine map has no name for it.
pub fn fn_0041b470(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) {
    let action = fn_0041b370(e, this) & !mask;
    fn_0041b3d0(e, this, action);
}

// Translated from 0041b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pActionRef` (+0x10) of the type `0x0E` extra data (`ExtraAction`).
/// Without one, a non-null `reference` gets one from `004195b0` and null does
/// nothing. With one, a null `reference` deletes it when its `eAction` is 1.
/// The engine map has no name for it.
pub fn fn_0041b4b0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let mut extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        if reference != 0 {
            extra = e.call(GET_OR_ADD_ACTION, &args![this]).ptr();
        }
    } else if e.mem.u8(extra.addr() + 0x0c) == 1 && reference == 0 {
        remove_extra(e, this, extra);
        extra = Ptr::NULL;
    }
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x10, reference);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0041a6a0,
            fn_0041a6a0(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a800,
            extra_data_list_set_sound(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a960,
            extra_data_list_set_ghost(Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x0041aa20,
            extra_data_list_set_worn(Ptr<ExtraDataList>, bool, bool)
        ),
        entry!(
            0x0041ab70,
            extra_data_list_set_can_not_wear(Ptr<ExtraDataList>, bool)
        ),
        entry!(0x0041ac30, fn_0041ac30(Ptr<ExtraDataList>, u8)),
        entry!(
            0x0041ad00,
            extra_data_list_set_package_start_location(Ptr<ExtraDataList>, u32, u32, Ptr, f32)
        ),
        entry!(
            0x0041adf0,
            extra_data_list_remove_anim_ptr(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041ae10,
            extra_data_list_remove_anim_save(Ptr<ExtraDataList>)
        ),
        entry!(0x0041ae30, fn_0041ae30(Ptr<ExtraDataList>)),
        entry!(0x0041ae50, fn_0041ae50(Ptr<ExtraDataList>)),
        entry!(
            0x0041ae70,
            extra_data_list_remove_lock_ptr(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041ae90,
            extra_data_list_remove_teleport_ptr(Ptr<ExtraDataList>)
        ),
        entry!(0x0041aeb0, fn_0041aeb0(Ptr<ExtraDataList>)),
        entry!(
            0x0041aed0,
            extra_data_list_remove_ownership(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041aef0,
            extra_data_list_remove_health(Ptr<ExtraDataList>)
        ),
        entry!(0x0041af10, extra_data_list_remove_count(Ptr<ExtraDataList>)),
        entry!(0x0041af30, fn_0041af30(Ptr<ExtraDataList>)),
        entry!(0x0041af50, fn_0041af50(Ptr<ExtraDataList>)),
        entry!(
            0x0041af70,
            script_locals_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0041afa0,
            extra_data_list_clear_script_locals(Ptr<ExtraDataList>)
        ),
        entry!(0x0041afd0, fn_0041afd0(Ptr<ExtraDataList>)),
        entry!(0x0041aff0, fn_0041aff0(Ptr<ExtraDataList>)),
        entry!(0x0041b010, fn_0041b010(Ptr<ExtraDataList>, bool)),
        entry!(
            0x0041b040,
            extra_data_list_remove_cannot_wear_extra(Ptr<ExtraDataList>)
        ),
        entry!(0x0041b060, fn_0041b060(Ptr<ExtraDataList>)),
        entry!(0x0041b080, fn_0041b080(Ptr<ExtraDataList>, Ptr, Ptr) -> Ptr),
        entry!(0x0041b0d0, fn_0041b0d0(Ptr<ExtraDataList>, Ptr, Ptr) -> Ptr),
        entry!(
            0x0041b120,
            fn_0041b120(Ptr<ExtraDataList>, Ptr, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0041b180,
            fn_0041b180(Ptr<ExtraDataList>, Ptr, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x0041b1e0, fn_0041b1e0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041b2a0,
            extra_data_list_set_starting_world_or_cell_for_ref(Ptr<ExtraDataList>, Ptr)
        ),
        entry!(
            0x0041b320,
            extra_data_list_get_starting_world_or_cell(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041b350, fn_0041b350(Ptr<ExtraDataList>)),
        entry!(0x0041b370, fn_0041b370(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041b3a0, fn_0041b3a0(Ptr<ExtraDataList>, u32) -> bool),
        entry!(0x0041b3d0, fn_0041b3d0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b440, fn_0041b440(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b470, fn_0041b470(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b4b0, fn_0041b4b0(Ptr<ExtraDataList>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Test vtable of the extra data: slot 0 the scalar deleting destructor.
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;
    /// Vtable at +0 of a test reference (slot `0x130 / 4` gives its name) and
    /// the one at +0x18 (slot 0 gives its cell).
    const REFERENCE_VTABLE: u32 = 0x0200_2000;
    const CHILD_CELL_VTABLE: u32 = 0x0200_3000;
    const REFERENCE_NAME: u32 = 0x0200_1010;
    const REFERENCE_CELL: u32 = 0x0200_1014;

    /// The callees of the list operations of the main file (lock, the type
    /// and next accessors, the cache clearing) and of the sound handles.
    const LOCK: u32 = 0x0040_fbf0;
    const UNLOCK: u32 = 0x0040_fba0;
    const GET_TYPE: u32 = 0x004f_1540;
    const GET_NEXT: u32 = 0x0044_ddc0;
    const SET_NEXT: u32 = 0x0040_3550;
    const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
    const READ_WORD: u32 = 0x0055_9450;
    const MEMSET: u32 = 0x0040_3d30;

    /// Every extra data type of this file with the constructor that makes it.
    const CONSTRUCTORS: &[(u32, u8)] = &[
        (EXTRA_GHOST_INIT, EXTRA_GHOST),
        (EXTRA_WORN_INIT, EXTRA_WORN),
        (EXTRA_WORN_LEFT_INIT, EXTRA_WORN_LEFT),
        (EXTRA_CAN_NOT_WEAR_INIT, EXTRA_CAN_NOT_WEAR),
        (EXTRA_SEED_INIT, EXTRA_SEED),
        (
            EXTRA_PACKAGE_START_LOCATION_INIT,
            EXTRA_PACKAGE_START_LOCATION,
        ),
        (
            EXTRA_STARTING_WORLD_OR_CELL_INIT,
            EXTRA_STARTING_WORLD_OR_CELL,
        ),
        (EXTRA_SOUND_INIT, EXTRA_SOUND),
        (EXTRA_ACTIVATE_LOOP_SOUND_INIT, EXTRA_ACTIVATE_LOOP_SOUND),
        // ExtraAction's constructor, which `004195b0` (main file) calls.
        (0x0043_1780, EXTRA_ACTION),
    ];

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// An engine with working doubles for the callees of the list
    /// operations, `operator new`, and the constructors of this file's
    /// extra data (they set the vtable and the type, and keep their
    /// argument words from +0x0C).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.put_vtable(VTABLE, &[DESTRUCTOR]);
        e.register(DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_NEXT, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SIMPLE_LIST_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        for address in [
            LOCK,
            UNLOCK,
            OPERATOR_DELETE,
            SOUND_HANDLE_DESTRUCTOR,
            SCRIPT_LOCALS_DESTRUCTOR,
            LOG_MESSAGE,
        ] {
            stub(&mut e, address);
        }
        for &(address, extra_type) in CONSTRUCTORS {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                for (index, word) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 0x0c + 4 * index as u32, *word);
                }
                returns(a[0])
            });
        }
        e
    }

    /// A list holding extra data of the given `(type, word at +0x0C)`, in
    /// order, linked through `AddExtra` (so the type bitmap is set).
    fn list_with(
        e: &mut Engine,
        entries: &[(u8, u32)],
    ) -> (Ptr<ExtraDataList>, Vec<Ptr<BSExtraData>>) {
        let list: Ptr<ExtraDataList> = e.new_object();
        let mut extras = vec![];
        for &(extra_type, word) in entries {
            let extra: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x40));
            e.mem.set_u32(extra.addr(), VTABLE);
            e.set(extra, BSExtraData::cEtype, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, word);
            e.call(ADD_EXTRA, &args![list, extra]);
            extras.push(extra);
        }
        (list, extras)
    }

    /// The types in the list's chain, sorted (some types are added at the
    /// head of the chain, the others at the end).
    fn sorted_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = vec![];
        let mut current: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        while !current.is_null() {
            types.push(e.get(current, BSExtraData::cEtype));
            current = e.get(current, BSExtraData::pNext).cast();
        }
        types.sort();
        types
    }

    /// Runs `call` on the engine with the call log on and returns the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The objects the log shows deleted (the destructor called).
    fn deleted(log: &Log) -> Vec<u32> {
        calls_to(log, DESTRUCTOR).iter().map(|a| a[0]).collect()
    }

    /// A sound handle in memory.
    fn sound_handle(e: &mut Engine, id: u32, assume_success: u8, state: u32) -> Ptr<BSSoundHandle> {
        let handle: Ptr<BSSoundHandle> = e.new_object();
        e.set(handle, BSSoundHandle::iSoundID, id);
        e.set(handle, BSSoundHandle::bAssumeSuccess, assume_success);
        e.set(handle, BSSoundHandle::eState, state);
        handle
    }

    /// A type that is not `extra_type`, for the extra data a function must
    /// leave alone.
    fn other_type(extra_type: u8) -> u8 {
        if extra_type == 0x01 {
            0x02
        } else {
            0x01
        }
    }

    /// Checks a sound setter of `address` for `extra_type` built by
    /// `construct`.
    fn check_sound_setter(address: u32, extra_type: u8, construct: u32) {
        // No extra data, a sound: one is built from a copy of the handle.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let sound = sound_handle(&mut e, 5, 1, 2);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, sound]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x18]]);
        let built = calls_to(&log, construct);
        assert_eq!(built.len(), 1);
        assert_eq!((built[0][1], built[0][2] & 0xff), (5, 1));
        assert_eq!(built[0][3], 2);
        assert_eq!(sorted_types(&e, list), vec![extra_type]);
        // No extra data, an empty sound: nothing.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let empty = sound_handle(&mut e, 0xffff_ffff, 0, 0);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, empty]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(sorted_types(&e, list).is_empty());
        // An extra data and an empty sound: it is deleted.
        let (list, extras) = list_with(&mut e, &[(other_type(extra_type), 0), (extra_type, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, empty]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert_eq!(sorted_types(&e, list), vec![other_type(extra_type)]);
        // An extra data and a sound: the handle is copied into it.
        let (list, extras) = list_with(&mut e, &[(extra_type, 0)]);
        let sound = sound_handle(&mut e, 9, 1, 4);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, sound]);
        });
        assert!(deleted(&log).is_empty());
        let handle: Ptr<BSSoundHandle> = Ptr::new(extras[0].addr() + 0x0c);
        assert_eq!(e.get(handle, BSSoundHandle::iSoundID), 9);
        assert_eq!(e.get(handle, BSSoundHandle::bAssumeSuccess), 1);
        assert_eq!(e.get(handle, BSSoundHandle::eState), 4);
    }

    /// Checks a flag setter of `address` (`flag_args` are the words after
    /// `this` for "wanted" and not "wanted") for `extra_type` built by
    /// `construct`.
    fn check_flag_setter(address: u32, extra_type: u8, construct: u32, other_args: &[u32]) {
        let call = |e: &mut Engine, list: Ptr<ExtraDataList>, wanted: bool| {
            let mut words = vec![list.addr(), wanted as u32];
            words.extend_from_slice(other_args);
            e.call(address, &words);
        };
        // Wanted and missing: built (0x0C bytes) and added.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(other_type(extra_type), 0)]);
        let log = logged(&mut e, |e| call(e, list, true));
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x0c]]);
        assert_eq!(calls_to(&log, construct).len(), 1);
        let mut expected = vec![other_type(extra_type), extra_type];
        expected.sort();
        assert_eq!(sorted_types(&e, list), expected);
        // Wanted and present: nothing.
        let log = logged(&mut e, |e| call(e, list, true));
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
        // Not wanted and present: deleted by type.
        let log = logged(&mut e, |e| call(e, list, false));
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), extra_type as u32]]
        );
        assert_eq!(sorted_types(&e, list), vec![other_type(extra_type)]);
        // Not wanted and missing: nothing.
        let log = logged(&mut e, |e| call(e, list, false));
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
    }

    /// Checks a one-line remover of `address` for `extra_type`: the first
    /// extra data of that type goes, the others stay, and an empty list is
    /// fine.
    fn check_remover(address: u32, extra_type: u8) {
        let mut e = engine();
        let decoy = other_type(extra_type);
        let (list, extras) = list_with(&mut e, &[(decoy, 0), (extra_type, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert_eq!(sorted_types(&e, list), vec![decoy]);
        let (empty, _) = list_with(&mut e, &[]);
        e.call(address, &args![empty]);
        assert!(sorted_types(&e, empty).is_empty());
    }

    #[test]
    fn activate_loop_sound_setter_covers_the_four_cases() {
        check_sound_setter(
            0x0041_a6a0,
            EXTRA_ACTIVATE_LOOP_SOUND,
            EXTRA_ACTIVATE_LOOP_SOUND_INIT,
        );
    }

    #[test]
    fn sound_setter_covers_the_four_cases() {
        check_sound_setter(0x0041_a800, EXTRA_SOUND, EXTRA_SOUND_INIT);
    }

    #[test]
    fn ghost_setter_adds_and_removes_the_flag() {
        check_flag_setter(0x0041_a960, EXTRA_GHOST, EXTRA_GHOST_INIT, &[]);
    }

    #[test]
    fn worn_setter_picks_the_hand() {
        // The right hand (`left` false) is the type 0x16, the left one 0x17.
        check_flag_setter(0x0041_aa20, EXTRA_WORN, EXTRA_WORN_INIT, &[0]);
        check_flag_setter(0x0041_aa20, EXTRA_WORN_LEFT, EXTRA_WORN_LEFT_INIT, &[1]);
    }

    #[test]
    fn can_not_wear_setter_adds_and_removes_the_flag() {
        check_flag_setter(
            0x0041_ab70,
            EXTRA_CAN_NOT_WEAR,
            EXTRA_CAN_NOT_WEAR_INIT,
            &[],
        );
    }

    #[test]
    fn seed_setter_stores_builds_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // 0xFF without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0xffu8]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Another value without one: built (0x10 bytes) with the seed.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0x42u8]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_SEED_INIT)[0][1], 0x42);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_SEED]);
        // With one: the byte is stored, nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0x07u8]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let extra = find_extra(&mut e, list, EXTRA_SEED);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0x07);
        // 0xFF with one: deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0xffu8]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn package_start_location_builds_then_updates() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let position: Ptr = Ptr::new(e.mem.alloc(0x0c));
        for (index, word) in [0x1111u32, 0x2222, 0x3333].iter().enumerate() {
            e.mem.set_u32(position.addr() + 4 * index as u32, *word);
        }
        // Built from the forms, the position pointer and the rotation (a
        // signalling NaN comes out quiet from the x87 stack).
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_ad00,
                &args![
                    list,
                    0xaau32,
                    0xbbu32,
                    position,
                    f32::from_bits(0x7f80_0001)
                ],
            );
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        let built = calls_to(&log, EXTRA_PACKAGE_START_LOCATION_INIT);
        assert_eq!(built[0][1..], [0xaa, 0xbb, position.addr(), 0x7fc0_0001]);
        // Updated: the first form wins, the position is copied.
        let extra = find_extra(&mut e, list, EXTRA_PACKAGE_START_LOCATION);
        e.mem.set_u32(position.addr() + 4, 0x9999);
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_ad00,
                &args![list, 0xccu32, 0xddu32, position, 1.0f32],
            );
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xcc);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x1111);
        assert_eq!(e.mem.u32(extra.addr() + 0x14), 0x9999);
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 0x3333);
        // The second form stands in for a null first one.
        e.call(0x0041_ad00, &args![list, 0u32, 0xddu32, position, 1.0f32]);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xdd);
    }

    #[test]
    fn anim_ptr_remover() {
        check_remover(0x0041_adf0, EXTRA_ANIM);
    }

    #[test]
    fn anim_save_remover() {
        check_remover(0x0041_ae10, EXTRA_ANIM_SAVE);
    }

    #[test]
    fn light_remover() {
        check_remover(0x0041_ae30, EXTRA_LIGHT);
    }

    #[test]
    fn magic_light_remover() {
        check_remover(0x0041_ae50, EXTRA_MAGIC_LIGHT);
    }

    #[test]
    fn lock_ptr_remover() {
        check_remover(0x0041_ae70, EXTRA_LOCK);
    }

    #[test]
    fn teleport_ptr_remover() {
        check_remover(0x0041_ae90, EXTRA_TELEPORT);
    }

    #[test]
    fn container_changes_remover() {
        check_remover(0x0041_aeb0, EXTRA_CONTAINER_CHANGES);
    }

    #[test]
    fn ownership_remover() {
        check_remover(0x0041_aed0, EXTRA_OWNERSHIP);
    }

    #[test]
    fn health_remover() {
        check_remover(0x0041_aef0, EXTRA_HEALTH);
    }

    #[test]
    fn count_remover() {
        check_remover(0x0041_af10, EXTRA_COUNT);
    }

    #[test]
    fn poison_remover() {
        check_remover(0x0041_af30, EXTRA_POISON);
    }

    #[test]
    fn script_remover() {
        check_remover(0x0041_af50, EXTRA_SCRIPT);
    }

    #[test]
    fn script_locals_destructor_deletes_only_with_the_flag() {
        let mut e = engine();
        let locals: Ptr = Ptr::new(e.mem.alloc(0x20));
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_af70, &args![locals, 0u32]).ptr::<()>();
            assert_eq!(result, locals);
        });
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_DESTRUCTOR),
            vec![vec![locals.addr()]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_af70, &args![locals, 3u32]).ptr::<()>();
            assert_eq!(result, locals);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![locals.addr()]]);
    }

    #[test]
    fn clear_script_locals_nulls_the_variables_pointer() {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(other_type(EXTRA_SCRIPT), 0), (EXTRA_SCRIPT, 0)]);
        e.mem.set_u32(extras[1].addr() + 0x10, 0x1234);
        e.mem.set_u32(extras[0].addr() + 0x10, 0x5678);
        e.call(0x0041_afa0, &args![list]);
        assert_eq!(e.mem.u32(extras[1].addr() + 0x10), 0);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x10), 0x5678);
        // Without the extra data: nothing happens.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_afa0, &args![empty]);
    }

    #[test]
    fn scale_remover() {
        check_remover(0x0041_afd0, EXTRA_SCALE);
    }

    #[test]
    fn ghost_remover() {
        check_remover(0x0041_aff0, EXTRA_GHOST);
    }

    #[test]
    fn worn_remover_picks_the_hand() {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(EXTRA_WORN, 0), (EXTRA_WORN_LEFT, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b010, &args![list, true]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b010, &args![list, false]);
        });
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn cannot_wear_remover() {
        check_remover(0x0041_b040, EXTRA_CAN_NOT_WEAR);
    }

    #[test]
    fn package_start_location_remover() {
        check_remover(0x0041_b060, EXTRA_PACKAGE_START_LOCATION);
    }

    /// A list with an `ExtraStartingPosition` whose position is (1, 2, 3)
    /// and rotation (4, 5, 6).
    fn list_with_starting_position(e: &mut Engine) -> (Ptr<ExtraDataList>, Ptr<BSExtraData>) {
        let (list, extras) = list_with(
            e,
            &[
                (other_type(EXTRA_STARTING_POSITION), 0),
                (EXTRA_STARTING_POSITION, 1),
            ],
        );
        for (index, word) in [1u32, 2, 3, 4, 5, 6].iter().enumerate() {
            e.mem
                .set_u32(extras[1].addr() + 0x0c + 4 * index as u32, *word);
        }
        (list, extras[1])
    }

    /// The case where the list has no starting position: `00418d50` (main
    /// file) is a double that hands out `made`.
    fn make_starting_position_double(e: &mut Engine, made: Ptr<BSExtraData>) {
        e.register_double(GET_OR_ADD_STARTING_POSITION, move |_, _| {
            returns(made.addr())
        });
    }

    #[test]
    fn starting_rotation_getter_copies_plus_0x18() {
        let mut e = engine();
        let (list, _) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_b080, &args![list, out, 0u32]).ptr::<()>();
            assert_eq!(result, out);
        });
        assert!(calls_to(&log, GET_OR_ADD_STARTING_POSITION).is_empty());
        assert_eq!(
            e.mem.bytes(out.addr(), 12),
            [4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]
        );
        // Missing: the extra data `00418d50` makes from the position is read.
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(made.addr() + 0x18, 0x77);
        make_starting_position_double(&mut e, made);
        let position: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let log = logged(&mut e, |e| {
            e.call(0x0041_b080, &args![empty, out, position]);
        });
        assert_eq!(
            calls_to(&log, GET_OR_ADD_STARTING_POSITION),
            vec![vec![empty.addr(), position.addr()]]
        );
        assert_eq!(e.mem.u32(out.addr()), 0x77);
    }

    #[test]
    fn starting_position_getter_copies_plus_0x0c() {
        let mut e = engine();
        let (list, _) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e.call(0x0041_b0d0, &args![list, out, 0u32]).ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(
            e.mem.bytes(out.addr(), 12),
            [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
        );
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(made.addr() + 0x0c, 0x88);
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b0d0, &args![empty, out, 0u32]);
        assert_eq!(e.mem.u32(out.addr()), 0x88);
    }

    #[test]
    fn starting_rotation_setter_stores_plus_0x18_and_the_out_copy() {
        let mut e = engine();
        let (list, extra) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e
            .call(0x0041_b120, &args![list, out, 0u32, 7u32, 8u32, 9u32])
            .ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 7);
        assert_eq!(e.mem.u32(extra.addr() + 0x20), 9);
        // The position stays.
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 1);
        assert_eq!(e.mem.u32(out.addr() + 4), 8);
        // Missing: written into the extra data `00418d50` returns.
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b120, &args![empty, out, 0u32, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(made.addr() + 0x1c), 2);
    }

    #[test]
    fn starting_position_setter_stores_plus_0x0c_and_the_out_copy() {
        let mut e = engine();
        let (list, extra) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e
            .call(0x0041_b180, &args![list, out, 0u32, 7u32, 8u32, 9u32])
            .ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 7);
        assert_eq!(e.mem.u32(extra.addr() + 0x14), 9);
        // The rotation stays.
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 4);
        assert_eq!(e.mem.u32(out.addr() + 8), 9);
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b180, &args![empty, out, 0u32, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(made.addr() + 0x10), 2);
    }

    #[test]
    fn starting_world_or_cell_setter_builds_updates_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // A null form without an extra data: nothing to remove.
        e.call(0x0041_b1e0, &args![list, 0u32]);
        assert!(sorted_types(&e, list).is_empty());
        // A form: built (0x10 bytes, no arguments) and stored.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0x1234u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_STARTING_WORLD_OR_CELL_INIT).len(), 1);
        let extra = find_extra(&mut e, list, EXTRA_STARTING_WORLD_OR_CELL);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x1234);
        // Another form: stored in the same extra data.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0x5678u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x5678);
        // Null: removes it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x49]]
        );
        assert!(sorted_types(&e, list).is_empty());
    }

    /// A reference with a child cell base at +0x18 whose slot 0 gives
    /// `cell`, a form id of `form_id` and a name from slot `0x130`.
    fn reference(e: &mut Engine, cell: u32, form_id: u32) -> Ptr {
        let mut slots = vec![0u32; 0x130 / 4 + 1];
        slots[0x130 / 4] = REFERENCE_NAME;
        e.put_vtable(REFERENCE_VTABLE, &slots);
        e.put_vtable(CHILD_CELL_VTABLE, &[REFERENCE_CELL]);
        e.register(REFERENCE_NAME, |_, _| returns(0x00c0_ffee));
        e.register_double(REFERENCE_CELL, move |_, _| returns(cell));
        e.register(GET_FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        let reference: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(reference.addr(), REFERENCE_VTABLE);
        e.mem.set_u32(reference.addr() + 0x0c, form_id);
        e.mem.set_u32(reference.addr() + 0x18, CHILD_CELL_VTABLE);
        reference
    }

    #[test]
    fn starting_world_or_cell_for_ref_prefers_the_world_space() {
        let mut e = engine();
        e.register(CELL_GET_WORLD_SPACE, |_, a| {
            returns(if a[0] == 0xce11 { 0xf00d } else { 0 })
        });
        // A cell with a world space: the world space is stored.
        let (list, _) = list_with(&mut e, &[]);
        let with_world = reference(&mut e, 0xce11, 0x1);
        e.call(0x0041_b2a0, &args![list, with_world]);
        assert_eq!(
            extra_data_list_get_starting_world_or_cell(&mut e, list),
            0xf00d
        );
        // A cell without one: the cell is stored.
        let (list, _) = list_with(&mut e, &[]);
        let without_world = reference(&mut e, 0xce22, 0x2);
        e.call(0x0041_b2a0, &args![list, without_world]);
        assert_eq!(
            extra_data_list_get_starting_world_or_cell(&mut e, list),
            0xce22
        );
    }

    #[test]
    fn starting_world_or_cell_for_ref_logs_when_there_is_no_cell() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let orphan = reference(&mut e, 0, 0x0001_4abc);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b2a0, &args![list, orphan]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_NO_PARENT_SAVE_CELL, 0x00c0_ffee, 0x0001_4abc]]
        );
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn starting_world_or_cell_getter() {
        let mut e = engine();
        let (list, _) = list_with(
            &mut e,
            &[
                (other_type(EXTRA_STARTING_WORLD_OR_CELL), 1),
                (EXTRA_STARTING_WORLD_OR_CELL, 0x4242),
            ],
        );
        assert_eq!(e.call(0x0041_b320, &args![list]).u32(), 0x4242);
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b320, &args![empty]).u32(), 0);
    }

    #[test]
    fn starting_world_or_cell_remover() {
        check_remover(0x0041_b350, EXTRA_STARTING_WORLD_OR_CELL);
    }

    #[test]
    fn action_getter_is_a_byte_and_defaults_to_one() {
        let mut e = engine();
        let (list, _) = list_with(
            &mut e,
            &[(other_type(EXTRA_ACTION), 0), (EXTRA_ACTION, 0x1234_5603)],
        );
        assert_eq!(e.call(0x0041_b370, &args![list]).u32(), 3);
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b370, &args![empty]).u32(), 1);
    }

    #[test]
    fn action_mask_test() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x06)]);
        assert!(e.call(0x0041_b3a0, &args![list, 0x04u32]).bool());
        assert!(!e.call(0x0041_b3a0, &args![list, 0x09u32]).bool());
        // The default 1 when there is no extra data.
        let (empty, _) = list_with(&mut e, &[]);
        assert!(e.call(0x0041_b3a0, &args![empty, 1u32]).bool());
        assert!(!e.call(0x0041_b3a0, &args![empty, 2u32]).bool());
    }

    #[test]
    fn action_setter_builds_stores_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // 1 is the default: nothing is built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 1u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Another value: `004195b0` builds the extra data (0x14 bytes).
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 5u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let extra = find_extra(&mut e, list, EXTRA_ACTION);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 5);
        // A reference keeps the extra data when the value goes back to 1.
        e.mem.set_u32(extra.addr() + 0x10, 0xabc);
        e.call(0x0041_b3d0, &args![list, 1u32]);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 1);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_ACTION]);
        // Without a reference the value 1 deletes it.
        e.mem.set_u32(extra.addr() + 0x10, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 1u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn action_bits_are_set_with_an_or() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x04)]);
        e.call(0x0041_b440, &args![list, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, list), 0x06);
        // Without the extra data the default 1 is the base.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_b440, &args![empty, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, empty), 0x03);
    }

    #[test]
    fn action_bits_are_cleared_with_an_and_not() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x07)]);
        e.call(0x0041_b470, &args![list, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, list), 0x05);
        // Clearing bit 0 of the default 1 builds the extra data with 0.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_b470, &args![empty, 0x01u32]);
        assert_eq!(sorted_types(&e, empty), vec![EXTRA_ACTION]);
        assert_eq!(fn_0041b370(&mut e, empty), 0);
    }

    #[test]
    fn action_reference_setter_builds_stores_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Null without an extra data: nothing.
        e.call(0x0041_b4b0, &args![list, 0u32]);
        assert!(sorted_types(&e, list).is_empty());
        // A reference: `004195b0` builds the extra data, the reference is stored.
        e.call(0x0041_b4b0, &args![list, 0xbeefu32]);
        let extra = find_extra(&mut e, list, EXTRA_ACTION);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0xbeef);
        // Null with eAction not 1: stored, the extra data stays.
        e.mem.set_u8(extra.addr() + 0x0c, 4);
        e.call(0x0041_b4b0, &args![list, 0u32]);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_ACTION]);
        // Null with eAction 1: deleted.
        e.mem.set_u8(extra.addr() + 0x0c, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b4b0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }
}
