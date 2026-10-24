//! `(unplaced)/run_00aa1a70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function-pointer global `00aa1a70` sets.
const CALLBACK_SLOT: u32 = 0x011a_c228;
/// Its default: `00aa1a40` (placed in `NiSystem/nifile.cpp` by the engine
/// map, 45 bytes; not named there).
const DEFAULT_CALLBACK: u32 = 0x00aa_1a40;

// Translated from 00aa1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): stores `callback` in the
/// function-pointer global `011ac228`, or the default `00aa1a40` when
/// `callback` is null (the exe writes the default first, then overwrites it).
pub fn fn_00aa1a70(e: &mut Engine, callback: u32) {
    e.set_global(CALLBACK_SLOT, DEFAULT_CALLBACK);
    if callback != 0 {
        e.set_global(CALLBACK_SLOT, callback);
    }
}

/// The word `00aa1a90` passes as the second argument (`010a2484`, 1 in the exe's data).
const SECOND_WORD: u32 = 0x010a_2484;

// Translated from 00aa1a90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; a vtable slot of several stream classes, `thiscall`,
/// `RET 4`): calls `this`'s own slot `0x14` with its one word and the word at `010a2484`
/// (`this` stays in `ECX`) and answers what that answers.
pub fn fn_00aa1a90(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let second = e.mem.u32(SECOND_WORD);
    e.vcall(this.addr(), 0x14, &args![word, second]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_1a70, fn_00aa1a70(u32)),
        entry!(0x00aa_1a90, fn_00aa1a90(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_callback_or_the_default() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.call(0x00aa_1a70, &args![0x0055_0000u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), 0x0055_0000);
        e.call(0x00aa_1a70, &args![0u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), DEFAULT_CALLBACK);
    }

    #[test]
    fn the_slot_calls_slot_0x14_with_the_word_and_the_global() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.mem.set_u32(SECOND_WORD, 1);
        e.put_vtable(0x0e00_0000, &[0x0e10_0000; 8]);
        e.register(0x0e10_0000, |_, a| (a[1] + a[2] * 0x100).into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0e00_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_1a90, &args![this, 7u32]).u32(), 0x107);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == 0x0e10_0000 && *w == vec![this, 7, 1]));
    }
}
