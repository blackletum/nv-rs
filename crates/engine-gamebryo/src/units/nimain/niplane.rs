//! `NiMain/niplane.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three words `NiPlane::NiPlane` copies into the normal (`011f426c`..`011f4277`).
const DEFAULT_NORMAL: u32 = 0x011f_426c;

// Translated from 00a69940 (disassembled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane` (engine map; `thiscall`, no stack words, plain `RET`): the normal at
/// `+0` from the three words at `011f426c` (copied through integer registers, unchanged),
/// the constant at `+0xc` set to `0.0` (`FLDZ`/`FSTP`), in the exe's store order (`+0`, `+4`,
/// `+0xc`, `+8`). Answers `this`.
pub fn ni_plane_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let x = e.mem.u32(DEFAULT_NORMAL);
    e.mem.set_u32(base, x);
    let y = e.mem.u32(DEFAULT_NORMAL + 4);
    e.mem.set_u32(base + 4, y);
    let z = e.mem.u32(DEFAULT_NORMAL + 8);
    e.mem.set_f32(base + 0xc, 0.0);
    e.mem.set_u32(base + 8, z);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_9940, ni_plane_construct(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_plane_takes_the_default_normal_and_a_zero_constant() {
        let mut e = Engine::new();
        e.map(DEFAULT_NORMAL & !0xfff, 0x1000);
        e.map(0x0000_1000, 0x100);
        for (i, bits) in [0x3f80_0000u32, 0xffc0_0001, 0x8000_0000]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(DEFAULT_NORMAL + 4 * i as u32, bits);
        }
        e.mem.set_u32(0x100c, 0xdead_beef);
        assert_eq!(e.call(0x00a6_9940, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x3f80_0000);
        assert_eq!(e.mem.u32(0x1004), 0xffc0_0001, "copied unchanged");
        assert_eq!(e.mem.u32(0x1008), 0x8000_0000);
        assert_eq!(e.mem.u32(0x100c), 0);
    }
}
