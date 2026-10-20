//! `NiMain/nitransform.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The zero point the translation is copied from (`011f426c`..`011f4277`).
const ZERO_POINT: u32 = 0x011f_426c;

// Translated from 00a86bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `NiTransform::MakeIdentity` (engine map; `thiscall`, no stack words, plain `RET`): the
/// rotation (`+0`, nine floats) the identity (`FLD1`/`FLDZ` stores, in the exe's order), the
/// translation (`+0x24`) the three words at `011f426c` (copied through integer registers) and
/// the scale (`+0x30`) 1.0.
pub fn ni_transform_make_identity(e: &mut Engine, this: Ptr) {
    let b = this.addr();
    e.mem.set_f32(b, 1.0);
    for offset in [4u32, 8, 0xc, 0x14, 0x18, 0x1c] {
        e.mem.set_f32(b + offset, 0.0);
    }
    e.mem.set_f32(b + 0x10, 1.0);
    e.mem.set_f32(b + 0x20, 1.0);
    let x = e.mem.u32(ZERO_POINT);
    e.mem.set_u32(b + 0x24, x);
    let y = e.mem.u32(ZERO_POINT + 4);
    e.mem.set_u32(b + 0x28, y);
    let z = e.mem.u32(ZERO_POINT + 8);
    e.mem.set_f32(b + 0x30, 1.0);
    e.mem.set_u32(b + 0x2c, z);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_6bf0, ni_transform_make_identity(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_the_identity_with_the_zero_point() {
        let mut e = Engine::new();
        e.map(ZERO_POINT & !0xfff, 0x1000);
        e.map(0x0000_1000, 0x100);
        for (i, w) in [0x8000_0000u32, 0, 0x7fc0_0001].into_iter().enumerate() {
            e.mem.set_u32(ZERO_POINT + 4 * i as u32, w);
        }
        e.call(0x00a8_6bf0, &args![0x1000u32]);
        let f = |e: &Engine, o: u32| e.mem.f32(0x1000 + o);
        assert_eq!([f(&e, 0), f(&e, 0x10), f(&e, 0x20), f(&e, 0x30)], [1.0; 4]);
        for o in [4u32, 8, 0xc, 0x14, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(0x1000 + o), 0);
        }
        assert_eq!(
            [0x24, 0x28, 0x2c].map(|o| e.mem.u32(0x1000 + o)),
            [0x8000_0000, 0, 0x7fc0_0001]
        );
    }
}
