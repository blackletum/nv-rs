//! `NiMain/nipixelformat.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a7cca0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::NiPixelFormat` (engine map; `thiscall`, no stack words, plain `RET`):
/// stores its default fields in the exe's order: the byte `+1` = 8, the words `+4` = 2,
/// `+0xc` = 0xffffffff, `+0x10` = 0, the byte `+0` = 1, the words `+8` = 0, `+0x14` = 0x10,
/// `+0x18` = 3, the bytes `+0x1c` = 8, `+0x1d` = 1, then at `+0x20`, `+0x2c` and `+0x38`
/// each: the words 0x13 and 5 and the bytes 0 and 1. The bytes in between (`+2`, `+3`,
/// `+0x1e`, ...) are not touched. Answers `this`.
pub fn ni_pixel_format_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let b = this.addr();
    e.mem.set_u8(b + 1, 8);
    e.mem.set_u32(b + 4, 2);
    e.mem.set_u32(b + 0xc, 0xffff_ffff);
    e.mem.set_u32(b + 0x10, 0);
    e.mem.set_u8(b, 1);
    e.mem.set_u32(b + 8, 0);
    e.mem.set_u32(b + 0x14, 0x10);
    e.mem.set_u32(b + 0x18, 3);
    e.mem.set_u8(b + 0x1c, 8);
    e.mem.set_u8(b + 0x1d, 1);
    for entry in [0x20u32, 0x2c, 0x38] {
        e.mem.set_u32(b + entry, 0x13);
        e.mem.set_u32(b + entry + 4, 5);
        e.mem.set_u8(b + entry + 8, 0);
        e.mem.set_u8(b + entry + 9, 1);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_cca0, ni_pixel_format_construct(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pixel_format_gets_its_default_fields() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        for i in 0..0x50u32 {
            e.mem.set_u8(0x1000 + i, 0xaa);
        }
        assert_eq!(e.call(0x00a7_cca0, &args![0x1000u32]).u32(), 0x1000);
        let word = |e: &Engine, o: u32| e.mem.u32(0x1000 + o);
        assert_eq!(e.mem.u8(0x1000), 1);
        assert_eq!(e.mem.u8(0x1001), 8);
        assert_eq!(e.mem.u8(0x1002), 0xaa, "untouched");
        assert_eq!(
            [word(&e, 4), word(&e, 8), word(&e, 0xc), word(&e, 0x10)],
            [2, 0, 0xffff_ffff, 0]
        );
        assert_eq!([word(&e, 0x14), word(&e, 0x18)], [0x10, 3]);
        assert_eq!([e.mem.u8(0x101c), e.mem.u8(0x101d)], [8, 1]);
        assert_eq!(e.mem.u8(0x101e), 0xaa, "untouched");
        for entry in [0x20u32, 0x2c, 0x38] {
            assert_eq!([word(&e, entry), word(&e, entry + 4)], [0x13, 5]);
            assert_eq!(
                [e.mem.u8(0x1000 + entry + 8), e.mem.u8(0x1000 + entry + 9)],
                [0, 1]
            );
        }
        assert_eq!(e.mem.u8(0x1042), 0xaa, "untouched");
    }
}
