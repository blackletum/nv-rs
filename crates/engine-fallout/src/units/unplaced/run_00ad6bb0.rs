//! `(unplaced)/run_00ad6bb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the base audio object gets (`010a323c`).
const AUDIO_BASE_VTABLE: u32 = 0x010a_323c;
/// `00aea0a0` (`thiscall`, no stack words): the timer part at `+0x14`.
const TIMER_CONSTRUCT: u32 = 0x00ae_a0a0;
/// The immediate `00ad6f40` tests (`MOV EDX,0x011f6dfc`; `TEST EDX,EDX`), the byte it then
/// reads (`011f6e00`) and the one the other branch clears and reads (`01202800`).
const SETTING_HOLDER: u32 = 0x011f_6dfc;
const SETTING_BYTE: u32 = 0x011f_6e00;
const FALLBACK_BYTE: u32 = 0x0120_2800;

// Translated from 00ad6f40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; the base constructor of
/// `BSWin32Audio`'s `00aea1b0`): the vtable `010a323c`, the timer at `+0x14` (`00aea0a0`), the
/// byte `+4` cleared, the byte `+6` from the byte at `011f6e00` (the immediate `011f6dfc` it tests
/// is never 0; the other branch, kept as the exe has it, clears `01202800` and reads that), the
/// byte `+5`, the word `+8`, the byte `+0xc` and the words `+0x20` to `+0x34` and `+0x10`
/// cleared, in the exe's order. Answers `this`.
pub fn fn_00ad6f40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, AUDIO_BASE_VTABLE);
    e.call(TIMER_CONSTRUCT, &args![base + 0x14]);
    e.mem.set_u8(base + 4, 0);
    let holder = std::hint::black_box(SETTING_HOLDER);
    let source = if holder != 0 {
        SETTING_BYTE
    } else {
        e.mem.set_u8(FALLBACK_BYTE, 0);
        FALLBACK_BYTE
    };
    let byte = e.mem.u8(source);
    e.mem.set_u8(base + 6, byte);
    e.mem.set_u8(base + 5, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0xc, 0);
    for offset in [0x20u32, 0x24, 0x28, 0x2c, 0x30, 0x34] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ad_6f40, fn_00ad6f40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_base_takes_its_vtable_the_timer_and_the_setting_byte() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.map(SETTING_BYTE & !0xfff, 0x1000);
        e.mem.set_u8(SETTING_BYTE, 0x5a);
        for i in 0..0x10u32 {
            e.mem.set_u32(0x1000 + 4 * i, 0xffff_ffff);
        }
        e.register(TIMER_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_6f40, &args![0x1000u32]).u32(), 0x1000);
        let log = e.call_log.take().unwrap();
        let timer: Vec<_> = log.iter().filter(|(a, _)| *a == TIMER_CONSTRUCT).collect();
        assert_eq!(timer[0].1, [0x1014]);
        assert_eq!(e.mem.u32(0x1000), AUDIO_BASE_VTABLE);
        assert_eq!([4, 5, 6].map(|o| e.mem.u8(0x1000 + o)), [0, 0, 0x5a]);
        assert_eq!(e.mem.u8(0x1007), 0xff, "untouched");
        assert_eq!(e.mem.u32(0x1008), 0);
        assert_eq!(e.mem.u8(0x100c), 0);
        assert_eq!(e.mem.u32(0x1010), 0);
        assert_eq!(
            [0x20, 0x24, 0x28, 0x2c, 0x30, 0x34].map(|o| e.mem.u32(0x1000 + o)),
            [0; 6]
        );
    }
}
