//! `BSShader/imagespacemanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x27 during the set-up.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_IMAGE_SPACE: u32 = 0x27;
/// The manager's first-run byte (+0x1e4).
const INITIALIZED: u32 = 0x1e4;
/// The set-up steps, each `thiscall` on the manager without stack words: `00b94520` (engine
/// map unit `imagespacemanager.cpp`), `ImageSpaceManager::InitGeometry` (`00b8c170`),
/// `ImageSpaceManager::SetupEffects` (`00b8b360`) and `00b8ad70` (same unit).
const SET_UP_00B94520: u32 = 0x00b9_4520;
const INIT_GEOMETRY: u32 = 0x00b8_c170;
const SETUP_EFFECTS: u32 = 0x00b8_b360;
const SET_UP_00B8AD70: u32 = 0x00b8_ad70;
/// `ImageSpaceShaderParam::InitConstantMap` (engine map, `00b8ab10`, `thiscall` on the object
/// at `01202410`, `RET 8`).
const INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const CONSTANT_MAP_01202410: u32 = 0x0120_2410;
/// The `float` stored at +0x304 (`01017870`).
const VALUE_01017870: u32 = 0x0101_7870;
/// `00b6e110` (unnamed, `BSShader`; `thiscall` on the object at `011f91a8`, `RET 0x14`), given
/// the word at `011f4748`, 3, 0, 0, 0; its answer goes to +0x318.
const MAKE_00B6E110: u32 = 0x00b6_e110;
const OBJECT_011F91A8: u32 = 0x011f_91a8;
const OBJECT_011F4748: u32 = 0x011f_4748;

// Translated from 00b977d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::Initialize` (Xbox PDB). `thiscall`, no stack words. With the thread's
/// allocation context (+0x2b4 of the thread block) set to 0x27 for the duration (and put back
/// after): when the byte at +0x1e4 is clear, runs `00b94520` and `InitGeometry` (`00b8c170`),
/// sets the byte, runs `SetupEffects` (`00b8b360`), `ImageSpaceShaderParam::InitConstantMap`
/// on `01202410` with (0, 0) and `00b8ad70`; then stores, in the exe's order, 0.0 at +0x2f8,
/// +0x2fc, +0x308 and +0x300, 1.0 at +0x30c and +0x310, the `float` at `01017870` (through
/// the FPU, a signalling NaN quieted) at +0x304, clears +0, stores 0.0 at +0x314, and keeps
/// at +0x318 what `00b6e110` (on `011f91a8`, with the word at `011f4748`, 3, 0, 0, 0) answers.
// Translation incomplete: 00b977d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation context) is not translated.
pub fn image_space_manager_initialize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
    if e.mem.u8(base.wrapping_add(INITIALIZED)) == 0 {
        e.call(SET_UP_00B94520, &args![this]);
        e.call(INIT_GEOMETRY, &args![this]);
        e.mem.set_u8(base.wrapping_add(INITIALIZED), 1);
        e.call(SETUP_EFFECTS, &args![this]);
        e.call(INIT_CONSTANT_MAP, &args![CONSTANT_MAP_01202410, 0u32, 0u32]);
        e.call(SET_UP_00B8AD70, &args![this]);
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);
        let loaded = x87_quiet_bits(e.global::<u32>(VALUE_01017870));
        for offset in [0x2f8u32, 0x2fc, 0x308, 0x300] {
            e.mem.set_u32(base.wrapping_add(offset), zero);
        }
        e.mem.set_u32(base.wrapping_add(0x30c), one);
        e.mem.set_u32(base.wrapping_add(0x310), one);
        e.mem.set_u32(base.wrapping_add(0x304), loaded);
        e.mem.set_u32(base, 0);
        e.mem.set_u32(base.wrapping_add(0x314), zero);
        let object = e.global::<u32>(OBJECT_011F91A8);
        let word = e.global::<u32>(OBJECT_011F4748);
        let made = e
            .call(MAKE_00B6E110, &args![object, word, 3u32, 0u32, 0u32, 0u32])
            .u32();
        e.mem.set_u32(base.wrapping_add(0x318), made);
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
}

// Translated from 00b8b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::SetupEffects` (Xbox PDB), `thiscall`, no stack words: for each of the
/// 0x22 effect slots of the array at +0x8 (re-read for every slot), calls a set effect's
/// vtable slot +0x8 with the manager and 0.
pub fn image_space_manager_setup_effects(e: &mut Engine, this: Ptr) {
    for index in 0..0x22u32 {
        let effects = e.mem.u32(this.addr() + 8);
        let effect = e.mem.u32(effects.wrapping_add(4 * index));
        if effect != 0 {
            e.vcall(effect, 8, &args![this, 0u32]);
        }
    }
}

/// The `float` `00b8ad70` keeps on the x87 stack besides 1.0 and 0.0 (`01016248`).
const HALF_01016248: u32 = 0x0101_6248;

/// `FLD dword` of the `float` at `address` stored by `FSTP`: its bits, a signalling NaN
/// quieted.
fn loaded(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

// Translated from 00b8ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), no arguments: stores the image
/// space defaults in the `float`s from `011ff940` to `011ff9d0`, in the exe's order, from the
/// x87 stack (`FLD`/`FST`/`FXCH` tracked; each `FLD` quiets a signalling NaN): the `float` at
/// `01016248` in `011ff940`, `011ff988`, `011ff98c`, `011ff998` and `011ff9a8`; 1.0 in
/// `011ff94c`, `011ff950`, `011ff954`, `011ff960`, `011ff974`, `011ff948` and `011ff9a4` to
/// `011ff9cc` but `011ff9a8` and `011ff9c0`; 0.0 in `011ff964`, `011ff968`, `011ff9c0`,
/// `011ff9d0`; one constant each in the rest (`01018f98` in `011ff944`, `01022154` in
/// `011ff958`, `0102212c` in `011ff95c`, `01016088` in `011ff96c` and `011ff970`, `01021c9c`
/// in `011ff97c`, `0101df44` in `011ff984`, `01018238` in `011ff980`, `0102d934` in
/// `011ff990`, `010184f4` in `011ff9a0`, `0102d930` in `011ff994`, `0102d92c` in `011ff99c`).
pub fn fn_00b8ad70(e: &mut Engine) {
    let one = 1.0f32.to_bits();
    let zero = 0.0f32.to_bits();
    let half = loaded(e, HALF_01016248);
    e.set_global::<u32>(0x011f_f940, half);
    let value = loaded(e, 0x0101_8f98);
    e.set_global::<u32>(0x011f_f944, value);
    for target in [0x011f_f94cu32, 0x011f_f950, 0x011f_f954] {
        e.set_global::<u32>(target, one);
    }
    let value = loaded(e, 0x0102_2154);
    e.set_global::<u32>(0x011f_f958, value);
    let value = loaded(e, 0x0102_212c);
    e.set_global::<u32>(0x011f_f95c, value);
    e.set_global::<u32>(0x011f_f960, one);
    e.set_global::<u32>(0x011f_f964, zero);
    e.set_global::<u32>(0x011f_f968, zero);
    let value = loaded(e, 0x0101_6088);
    e.set_global::<u32>(0x011f_f96c, value);
    e.set_global::<u32>(0x011f_f970, value);
    e.set_global::<u32>(0x011f_f974, one);
    e.set_global::<u32>(0x011f_f948, one);
    let value = loaded(e, 0x0102_1c9c);
    e.set_global::<u32>(0x011f_f97c, value);
    let value = loaded(e, 0x0101_df44);
    e.set_global::<u32>(0x011f_f984, value);
    let value = loaded(e, 0x0101_8238);
    e.set_global::<u32>(0x011f_f980, value);
    e.set_global::<u32>(0x011f_f988, half);
    e.set_global::<u32>(0x011f_f98c, half);
    let value = loaded(e, 0x0102_d934);
    e.set_global::<u32>(0x011f_f990, value);
    let value = loaded(e, 0x0101_84f4);
    e.set_global::<u32>(0x011f_f9a0, value);
    let value = loaded(e, 0x0102_d930);
    e.set_global::<u32>(0x011f_f994, value);
    e.set_global::<u32>(0x011f_f998, half);
    let value = loaded(e, 0x0102_d92c);
    e.set_global::<u32>(0x011f_f99c, value);
    for target in [
        0x011f_f9b0u32,
        0x011f_f9ac,
        0x011f_f9a4,
        0x011f_f9b4,
        0x011f_f9b8,
        0x011f_f9bc,
        0x011f_f9c4,
        0x011f_f9c8,
        0x011f_f9cc,
    ] {
        e.set_global::<u32>(target, one);
    }
    e.set_global::<u32>(0x011f_f9a8, half);
    e.set_global::<u32>(0x011f_f9c0, zero);
    e.set_global::<u32>(0x011f_f9d0, zero);
}

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InitGeometry`'s callees (engine map names; all `thiscall`):
/// `NiScreenElementsData::NiScreenElementsData` (`00a881e0`, nine words),
/// `NiScreenElements::NiScreenElements` (`00a87810`, the data), `NiScreenElementsData::Insert`
/// (`00a87ae0`, three words), `::SetRectangle` (`00a88020`: index and four `float`s),
/// `::SetTextures` (`00a88130`: two words and four `float`s), `::UpdateBound` (`00a87900`),
/// the name setter `00a5b950` (unnamed: the address of an `NiFixedString`),
/// `NiAVObject::UpdateProperties` (`00a5a040`) and the update `00a59c60` (unnamed: the address
/// of an update block); and the name maker `00a5b690` (`cdecl`, a string).
const SCREEN_ELEMENTS_DATA_CONSTRUCT: u32 = 0x00a8_81e0;
const SCREEN_ELEMENTS_CONSTRUCT: u32 = 0x00a8_7810;
const SCREEN_ELEMENTS_DATA_INSERT: u32 = 0x00a8_7ae0;
const SCREEN_ELEMENTS_DATA_SET_RECTANGLE: u32 = 0x00a8_8020;
const SCREEN_ELEMENTS_DATA_SET_TEXTURES: u32 = 0x00a8_8130;
const SCREEN_ELEMENTS_DATA_UPDATE_BOUND: u32 = 0x00a8_7900;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const MAKE_FIXED_STRING_00A5B690: u32 = 0x00a5_b690;
/// `"Imagespace Effect Geometry"` (`010b04ac`) and the empty name word (`0109b220`).
const GEOMETRY_NAME_010B04AC: u32 = 0x010b_04ac;
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// The rectangle `float`s (`01012054`, `010162c0`, `0104a494`; each through `FLD`).
const RECTANGLE_X_01012054: u32 = 0x0101_2054;
const RECTANGLE_WIDTH_010162C0: u32 = 0x0101_62c0;
const RECTANGLE_HEIGHT_0104A494: u32 = 0x0104_a494;

/// `NiPointer<T>::operator=(T *)` expanded inline on a field of the manager.
fn assign_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00b8c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::InitGeometry` (Xbox PDB), `thiscall`, no stack words. Makes an
/// `NiScreenElementsData` (0x70 bytes, constructed with 0, 0, 1, 1, 1, 4, 1, 2, 1; null when the
/// allocation fails) into the `NiPointer` at +0x1f4 and an `NiScreenElements` on it (0xc4
/// bytes) into the one at +0x1f0. On the elements' data (+0xb8 of the elements, read again
/// for each call): `Insert(4, 0, 0)`, `SetRectangle(0, x, 1.0, width, height)` with the
/// `float`s at `01012054`, `010162c0`, `0104a494`, `SetTextures(0, 0, 0.0, 0.0, 1.0, 1.0)` and
/// `UpdateBound`. Then names the elements `"Imagespace Effect Geometry"` (a name from
/// `00a5b690` passed by address to `00a5b950`, released afterwards unless it is the empty
/// word at `0109b220`), runs `UpdateProperties` and the update `00a59c60` with a stack block of
/// a 0.0 `float` and five clear bytes.
// Translation incomplete: 00b8c170 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the allocations and the name) is not translated.
pub fn image_space_manager_init_geometry(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            SCREEN_ELEMENTS_DATA_CONSTRUCT,
            &args![block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    assign_pointer(e, base + 0x1f4, data);
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let elements = if block != 0 {
        let data = e.mem.u32(base + 0x1f4);
        e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![block, data]).u32()
    } else {
        0
    };
    assign_pointer(e, base + 0x1f0, elements);
    let elements_data = |e: &Engine| e.mem.u32(e.mem.u32(base + 0x1f0) + 0xb8);
    let data = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_INSERT, &args![data, 4u32, 0u32, 0u32]);
    let height = x87_quiet_bits(e.global::<u32>(RECTANGLE_HEIGHT_0104A494));
    let data = elements_data(e);
    let width = x87_quiet_bits(e.global::<u32>(RECTANGLE_WIDTH_010162C0));
    let x = x87_quiet_bits(e.global::<u32>(RECTANGLE_X_01012054));
    e.call(
        SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
        &args![data, 0u32, x, 1.0f32.to_bits(), width, height],
    );
    let data = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_SET_TEXTURES,
        &args![
            data,
            0u32,
            0u32,
            0.0f32.to_bits(),
            0.0f32.to_bits(),
            1.0f32.to_bits(),
            1.0f32.to_bits()
        ],
    );
    let data = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_UPDATE_BOUND, &args![data]);
    e.with_stack(0x10, |e, locals| {
        let name_slot = locals.addr();
        let update = name_slot + 4;
        let name = e
            .call(MAKE_FIXED_STRING_00A5B690, &args![GEOMETRY_NAME_010B04AC])
            .u32();
        e.mem.set_u32(name_slot, name);
        let elements = e.mem.u32(base + 0x1f0);
        e.call(SET_NAME_00A5B950, &args![elements, name_slot]);
        let name = e.mem.u32(name_slot);
        if name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
        let elements = e.mem.u32(base + 0x1f0);
        e.call(UPDATE_PROPERTIES, &args![elements]);
        let elements = e.mem.u32(base + 0x1f0);
        e.mem.set_f32(update, 0.0);
        for offset in 4..9u32 {
            e.mem.set_u8(update + offset, 0);
        }
        e.call(UPDATE_00A59C60, &args![elements, update]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_c170, image_space_manager_init_geometry(Ptr)),
        entry!(0x00b8_ad70, fn_00b8ad70()),
        entry!(0x00b8_b360, image_space_manager_setup_effects(Ptr)),
        entry!(0x00b9_77d0, image_space_manager_initialize(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x6000);
        e.map(0x0101_7000, 0x1000);
        e.set_global(OBJECT_011F91A8, 0x0f00_1000u32);
        e.set_global(OBJECT_011F4748, 0x0f00_2000u32);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global(VALUE_01017870, 0x7f80_0001u32);
        for addr in [
            SET_UP_00B94520,
            INIT_GEOMETRY,
            SETUP_EFFECTS,
            INIT_CONSTANT_MAP,
            SET_UP_00B8AD70,
        ] {
            e.register(addr, |e, _| {
                // Each step runs with the image-space context.
                let tls = e.tls();
                assert_eq!(e.mem.u32(tls + TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
                Ret::default()
            });
        }
        e.register(MAKE_00B6E110, |_, _| 0x0f00_3000u32.into_ret());
        e
    }

    #[test]
    fn image_space_manager_initialize_sets_up_once() {
        let mut e = rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        let this = e.mem.alloc(0x320);
        for offset in (0..0x320).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.mem.set_u8(this + INITIALIZED, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b9_77d0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .skip(1)
            .collect();
        assert_eq!(
            order,
            [
                SET_UP_00B94520,
                INIT_GEOMETRY,
                SETUP_EFFECTS,
                INIT_CONSTANT_MAP,
                SET_UP_00B8AD70,
                MAKE_00B6E110
            ]
        );
        assert_eq!(calls(&e, SET_UP_00B94520), vec![vec![this]]);
        assert_eq!(
            calls(&e, INIT_CONSTANT_MAP),
            vec![vec![CONSTANT_MAP_01202410, 0, 0]]
        );
        assert_eq!(
            calls(&e, MAKE_00B6E110),
            vec![vec![0x0f00_1000, 0x0f00_2000, 3, 0, 0, 0]]
        );
        assert_eq!(e.mem.u8(this + INITIALIZED), 1);
        for offset in [0x2f8u32, 0x2fc, 0x300, 0x308, 0x314] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.f32(this + 0x30c), 1.0);
        assert_eq!(e.mem.f32(this + 0x310), 1.0);
        assert_eq!(e.mem.u32(this + 0x304), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x318), 0x0f00_3000);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);

        // A second run does nothing but set and restore the context.
        e.mem.set_u32(this + 0x318, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00b9_77d0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 0x318), 0x55);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }
}

#[cfg(test)]
mod setup_effects_tests {
    use super::*;

    #[test]
    fn image_space_manager_setup_effects_calls_each_set_effect() {
        const SETUP: u32 = 0x0e40_0000;
        let mut e = Engine::new();
        e.register(SETUP, |_, _| Ret::default());
        e.put_vtable(0x0e41_0000, &[0, 0, SETUP]);
        let this = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x88);
        e.mem.set_u32(this + 8, effects);
        let mut set = vec![];
        for index in [0u32, 5, 0x21] {
            let effect = e.mem.alloc(4);
            e.mem.set_u32(effect, 0x0e41_0000);
            e.mem.set_u32(effects + 4 * index, effect);
            set.push(effect);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b8_b360, &args![this]);
        let log = e.call_log.take().unwrap();
        let seen: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SETUP)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            seen,
            set.iter().map(|&s| vec![s, this, 0]).collect::<Vec<_>>()
        );
    }
}

#[cfg(test)]
mod defaults_tests {
    use super::*;

    #[test]
    fn fn_00b8ad70_stores_every_default() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x18000);
        e.map(0x011f_f000, 0x1000);
        e.mem.write(0x011f_f940, &[0xee; 0x94]);
        // A signalling NaN is quieted by the load.
        e.set_global::<u32>(HALF_01016248, 0x7f80_0005);
        let sources = [
            0x0101_8f98u32,
            0x0102_2154,
            0x0102_212c,
            0x0101_6088,
            0x0102_1c9c,
            0x0101_df44,
            0x0101_8238,
            0x0102_d934,
            0x0101_84f4,
            0x0102_d930,
            0x0102_d92c,
        ];
        for (index, source) in sources.iter().enumerate() {
            e.set_global::<f32>(*source, 10.0 + index as f32);
        }
        e.call(0x00b8_ad70, &args![]);
        let half = 0x7fc0_0005;
        let one = 0x3f80_0000;
        let value = |index: u32| (10.0 + index as f32).to_bits();
        let expected = [
            (0x940u32, half),
            (0x944, value(0)),
            (0x948, one),
            (0x94c, one),
            (0x950, one),
            (0x954, one),
            (0x958, value(1)),
            (0x95c, value(2)),
            (0x960, one),
            (0x964, 0),
            (0x968, 0),
            (0x96c, value(3)),
            (0x970, value(3)),
            (0x974, one),
            (0x97c, value(4)),
            (0x980, value(6)),
            (0x984, value(5)),
            (0x988, half),
            (0x98c, half),
            (0x990, value(7)),
            (0x994, value(9)),
            (0x998, half),
            (0x99c, value(10)),
            (0x9a0, value(8)),
            (0x9a4, one),
            (0x9a8, half),
            (0x9ac, one),
            (0x9b0, one),
            (0x9b4, one),
            (0x9b8, one),
            (0x9bc, one),
            (0x9c0, 0),
            (0x9c4, one),
            (0x9c8, one),
            (0x9cc, one),
            (0x9d0, 0),
        ];
        for (offset, bits) in expected {
            assert_eq!(e.global::<u32>(0x011f_f000 + offset), bits, "{offset:#x}");
        }
        // 011ff978 is not written.
        assert_eq!(e.global::<u32>(0x011f_f978), 0xeeee_eeee);
    }
}

#[cfg(test)]
mod init_geometry_tests {
    use super::*;

    const DELETE: u32 = 0x0ef0_0000;
    const VTABLE: u32 = 0x0ef1_0000;
    const NAME: u32 = 0x0ef2_0008;
    const DATA_OF_ELEMENTS: u32 = 0x0ef3_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine(allocate: bool) -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.register_double(NI_ALLOCATE, move |e, a| {
            if allocate {
                let block = e.mem.alloc(a[0]);
                e.mem.set_u32(block, VTABLE);
                block.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(SCREEN_ELEMENTS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, DATA_OF_ELEMENTS);
            a[0].into_ret()
        });
        for addr in [
            SCREEN_ELEMENTS_DATA_INSERT,
            SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
            SCREEN_ELEMENTS_DATA_SET_TEXTURES,
            SCREEN_ELEMENTS_DATA_UPDATE_BOUND,
            SET_NAME_00A5B950,
            UPDATE_PROPERTIES,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(UPDATE_00A59C60, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            assert_eq!(e.mem.bytes(a[1] + 4, 5), vec![0; 5]);
            Ret::default()
        });
        e.register(MAKE_FIXED_STRING_00A5B690, |_, _| NAME.into_ret());
        e.map(0x0101_2000, 0x5000);
        e.map(0x0104_a000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.map(NAME - 8, 0x10);
        e.set_global::<f32>(RECTANGLE_X_01012054, -1.0);
        e.set_global::<f32>(RECTANGLE_WIDTH_010162C0, 2.0);
        e.set_global::<f32>(RECTANGLE_HEIGHT_0104A494, -2.0);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn image_space_manager_init_geometry_builds_the_screen_elements() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x200);
        e.call(INIT_GEOMETRY, &args![this]);
        let data = e.mem.u32(this + 0x1f4);
        let elements = e.mem.u32(this + 0x1f0);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(elements + 4), 1);
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT),
            vec![vec![data, 0, 0, 1, 1, 1, 4, 1, 2, 1]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_CONSTRUCT),
            vec![vec![elements, data]]
        );
        let d = DATA_OF_ELEMENTS;
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![d, 4, 0, 0]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_RECTANGLE),
            vec![vec![
                d,
                0,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                (-2.0f32).to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_TEXTURES),
            vec![vec![d, 0, 0, 0, 0, 1.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, SCREEN_ELEMENTS_DATA_UPDATE_BOUND), vec![vec![d]]);
        assert_eq!(
            calls(&e, MAKE_FIXED_STRING_00A5B690),
            vec![vec![GEOMETRY_NAME_010B04AC]]
        );
        let named = calls(&e, SET_NAME_00A5B950);
        assert_eq!(named.len(), 1);
        assert_eq!(named[0][0], elements);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![NAME - 8]]
        );
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![elements]]);
        assert_eq!(calls(&e, UPDATE_00A59C60).len(), 1);
    }

    #[test]
    fn image_space_manager_init_geometry_failed_allocations_leave_null_pointers() {
        let mut e = engine(false);
        let this = e.mem.alloc(0x200);
        // The elements' data is then read through a null pointer, as the exe does.
        e.map(0, 0x1000);
        e.call(INIT_GEOMETRY, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1f4), 0);
        assert_eq!(e.mem.u32(this + 0x1f0), 0);
        assert!(calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT).is_empty());
        assert!(calls(&e, SCREEN_ELEMENTS_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![0, 4, 0, 0]]
        );
    }
}
