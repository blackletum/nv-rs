//! `(unplaced)/run_00647b70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00647b70 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (subsystem `fallout shared`, no unit; `cdecl`, two words): the
/// larger of two signed integers (`CMP`/`JLE`: the first when it is greater, else the second).
pub fn fn_00647b70(_e: &mut Engine, first: i32, second: i32) -> i32 {
    if first > second {
        first
    } else {
        second
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_7b70, fn_00647b70(i32, i32) -> i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00647b70_answers_the_larger() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0064_7b70, &args![3i32, -4i32]).i32(), 3);
        assert_eq!(e.call(0x0064_7b70, &args![-4i32, 3i32]).i32(), 3);
        assert_eq!(e.call(0x0064_7b70, &args![5i32, 5i32]).i32(), 5);
    }
}
