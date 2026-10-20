//! `BSShader/run_00b7fa40` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1360` (`cdecl`, two words): compares two C strings, 0 when equal.
const STRING_COMPARE: u32 = 0x00aa_1360;

// Translated from 00b7fce0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a string map's key compare (slot `+8` of `NiStream`'s loader
/// map vtable `0109bf4c`, so `thiscall` with `ECX` unused; two stack words, `RET 8`): the
/// two keys through `00aa1360`, then `NEG`/`SBB`/`INC`: 1 when it answered 0 (equal), else 0.
pub fn fn_00b7fce0(e: &mut Engine, _this: Ptr, first: u32, second: u32) -> u32 {
    let order = e.call(STRING_COMPARE, &args![first, second]).u32();
    u32::from(order == 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b7_fce0, fn_00b7fce0(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_keys_answer_1() {
        let mut e = Engine::new();
        e.register(STRING_COMPARE, |_, a| {
            (if a[0] == a[1] { 0u32 } else { 0xffff_ffff }).into_ret()
        });
        assert_eq!(e.call(0x00b7_fce0, &args![0u32, 7u32, 7u32]).u32(), 1);
        assert_eq!(e.call(0x00b7_fce0, &args![0u32, 7u32, 8u32]).u32(), 0);
    }
}
