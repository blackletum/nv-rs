//! `fallout/interface/run_00700480` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007004e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00700500`, which stores the vtable `0106e8dc` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00700500`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_007004e0(e: &mut Engine, this: Ptr) {
    e.call(0x0070_0500, &args![this]);
}

/// The three 0x1C-byte records at `011d8830` `fn_007005d0` clears.
const RECORDS_011D8830: u32 = 0x011d_8830;
/// The smart-pointer assignment `0066b0d0` (`thiscall`, one word: the new object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;

// Translated from 007005d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; `main` makes the 8-byte object at
/// `0086afdc` and keeps it at `011d8804`): clears the object's two words, then for each of
/// the three 0x1C-byte records at `011d8830` clears its byte +0 and word +8, stores 0.0 at +0xC
/// (`FLDZ`/`FSTP`) and assigns null to its pointer at +0x10 (`0066b0d0`). Answers `this`.
pub fn fn_007005d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    for index in 0..3u32 {
        let record = RECORDS_011D8830 + index * 0x1c;
        e.mem.set_u8(record, 0);
        e.mem.set_u32(record + 8, 0);
        e.mem.set_u32(record + 0xc, 0.0f32.to_bits());
        e.call(POINTER_ASSIGN, &args![record + 0x10, 0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_04e0, fn_007004e0(Ptr)),
        entry!(0x0070_05d0, fn_007005d0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_007004e0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0070_0500, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_04e0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0070_0500), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod records_tests {
    use super::*;

    #[test]
    fn fn_007005d0_clears_the_object_and_the_three_records() {
        let mut e = Engine::new();
        e.map(RECORDS_011D8830 & !0xfff, 0x1000);
        for offset in 0..0x54u32 {
            e.mem.set_u8(RECORDS_011D8830 + offset, 0xaa);
        }
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 7);
        e.mem.set_u32(this + 4, 9);
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_05d0, &args![this]).u32(), this);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0, 0));
        let assigned: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == POINTER_ASSIGN)
            .map(|c| c.1)
            .collect();
        let mut expected = vec![];
        for index in 0..3u32 {
            let record = RECORDS_011D8830 + index * 0x1c;
            assert_eq!(e.mem.u8(record), 0);
            assert_eq!(e.mem.u8(record + 1), 0xaa);
            assert_eq!(e.mem.u32(record + 8), 0);
            assert_eq!(e.mem.u32(record + 0xc), 0);
            expected.push(vec![record + 0x10, 0]);
        }
        assert_eq!(assigned, expected);
    }
}
