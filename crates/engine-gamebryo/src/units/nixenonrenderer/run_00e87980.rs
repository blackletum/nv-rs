//! `NiXenonRenderer/run_00e87980` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e88c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with one word: stores its low byte
/// at +0x18.
pub fn fn_00e88c20(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x18), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_8c20, fn_00e88c20(Ptr, u8))]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e88c20_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0xffff_ffff);
        e.call(0x00e8_8c20, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_ff01);
    }
}
