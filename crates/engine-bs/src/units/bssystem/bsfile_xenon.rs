//! `BSSystem/bsfile_xenon.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md). `BSFile` itself is platform
//! (`engine-core`'s `platform.rs`, `open_memory_file`).
//!
//! Not translated here: `00b003c0`, which calls the KERNEL32 import
//! `SystemTimeToTzSpecificLocalTime`, an import with no platform layer.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b004c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, two words it does not read, `RET 8`): answers 1 (`AL`).
pub fn fn_00b004c0(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) -> u8 {
    1
}

// Translated from 00b004d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00b004d0(_e: &mut Engine, _this: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_04c0, fn_00b004c0(u32, u32, u32) -> u8),
        entry!(0x00b0_04d0, fn_00b004d0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_one_and_does_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b0_04c0, &args![1u32, 2u32, 3u32]).u8(), 1);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 9);
        e.call(0x00b0_04d0, &args![this]);
        assert_eq!(e.mem.u32(this), 9);
    }
}
