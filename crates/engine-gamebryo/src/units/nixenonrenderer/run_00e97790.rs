//! `NiXenonRenderer/run_00e97790` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, size) and the constructor `00e97790` (unnamed;
/// `thiscall`, two words).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const CONSTRUCT_00E97790: u32 = 0x00e9_7790;

// Translated from 00e978a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e97790`), `cdecl`: two words. Makes a 0x8c-byte object
/// with `00e97790` and the two words; answers what the constructor answers, or null when the
/// allocation fails.
pub fn fn_00e978a0(e: &mut Engine, first: u32, second: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x8cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CONSTRUCT_00E97790, &args![block, first, second])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_78a0, fn_00e978a0(u32, u32) -> u32)]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e978a0_makes_the_object() {
        let mut e = Engine::new();
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_00E97790, |_, a| (a[0] + a[1] + a[2]).into_ret());
        e.call_log = Some(vec![]);
        let made = e.call(0x00e9_78a0, &args![1u32, 2u32]).u32();
        let log = e.call_log.clone().unwrap();
        let block = log
            .iter()
            .find(|(a, _)| *a == CONSTRUCT_00E97790)
            .unwrap()
            .1[0];
        assert_eq!(made, block + 3);
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e9_78a0, &args![1u32, 2u32]).u32(), 0);
    }
}
