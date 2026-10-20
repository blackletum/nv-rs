//! `NiXenonRenderer/run_00aa1b60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function pointer `fn_00aa1b90` sets (`011ac224`) and its default `00aa1b60`.
const HANDLER_011AC224: u32 = 0x011a_c224;
const DEFAULT_HANDLER_00AA1B60: u32 = 0x00aa_1b60;

// Translated from 00aa1b90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores the default `00aa1b60` at `011ac224`,
/// then the given function instead when it is not null.
pub fn fn_00aa1b90(e: &mut Engine, handler: u32) {
    e.mem.set_u32(HANDLER_011AC224, DEFAULT_HANDLER_00AA1B60);
    if handler != 0 {
        e.mem.set_u32(HANDLER_011AC224, handler);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_1b90, fn_00aa1b90(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00aa1b90_sets_the_handler_or_the_default() {
        let mut e = Engine::new();
        e.map(HANDLER_011AC224 & !0xfff, 0x1000);
        e.call(0x00aa_1b90, &[0x0040_1234]);
        assert_eq!(e.mem.u32(HANDLER_011AC224), 0x0040_1234);
        e.call(0x00aa_1b90, &[0]);
        assert_eq!(e.mem.u32(HANDLER_011AC224), DEFAULT_HANDLER_00AA1B60);
    }
}
