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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_77d0, image_space_manager_initialize(Ptr))]
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
