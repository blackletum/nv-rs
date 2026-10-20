//! `NiMain/nifrustum.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a71b70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `NiFrustum::NiFrustum` (engine map; `thiscall`, one stack word, `RET 4`): stores the low
/// byte of `ortho` at `+0x18`; nothing else. Answers `this`.
pub fn ni_frustum_construct(e: &mut Engine, this: Ptr, ortho: u32) -> Ptr {
    e.mem.set_u8(this.addr() + 0x18, ortho as u8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_1b70, ni_frustum_construct(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_byte_at_0x18() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.mem.set_u32(0x1018, 0xffff_ffff);
        assert_eq!(
            e.call(0x00a7_1b70, &args![0x1000u32, 0x101u32]).u32(),
            0x1000
        );
        assert_eq!(e.mem.u32(0x1018), 0xffff_ff01);
    }
}
