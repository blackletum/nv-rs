//! `(unplaced)/run_00fb56c0` (no source unit: two of the compiler's dynamic initializers, entries
//! of the `_initterm` table `00fdf4a4..010113c0` that Ghidra never made functions of,
//! research/engine-map/ghidra_gaps.tsv; research/engine-map/unit_overrides.tsv), subsystem
//! `compiler: static init/exit`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).
//!
//! Both construct a Gamebryo library's static-data-manager object in place with a tail jump:
//! `MOV ECX, global` then `JMP constructor`; the constructor's answer (`EAX`, the object) is
//! left as theirs. `NiInit` (`00aa20b0`) needs them: the first sets its start-up hook (`NiMain`'s
//! initializer, which makes `NiStream`'s loader map), the second registers one of its start-up
//! functions.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMainSDM`'s constructor (`00a8a5f0`, `thiscall`, no stack words): on its first run it
/// stores `00a89cc0` (and `00a8a280`) as `NiInit`'s start-up hook (`00aa2060`).
const NI_MAIN_SDM_CONSTRUCT: u32 = 0x00a8_a5f0;
/// The object it constructs.
const NI_MAIN_SDM: u32 = 0x011f_4414;
/// The renderer library's registrar constructor (`00e91e30`, `thiscall`, no stack words): on its
/// first run it registers `00e91dc0` (and `00e91c80`) with `NiInit`'s list (`00aa2080`).
const RENDERER_SDM_CONSTRUCT: u32 = 0x00e9_1e30;
/// The object it constructs.
const RENDERER_SDM: u32 = 0x0126_f104;

// Translated from 00fb56c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `0100f820`: `MOV ECX,0x011f4414`, `JMP 0x00a8a5f0`: `NiMainSDM`'s
/// constructor on the global at `011f4414`. Answers what the constructor answers (`EAX`).
pub fn fn_00fb56c0(e: &mut Engine) -> u32 {
    e.call(NI_MAIN_SDM_CONSTRUCT, &args![NI_MAIN_SDM]).u32()
}

// Translated from 00fc72d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `0101130c`: `MOV ECX,0x0126f104`, `JMP 0x00e91e30`: the renderer
/// library's registrar constructor on the global at `0126f104`. Answers what the constructor
/// answers (`EAX`).
pub fn fn_00fc72d0(e: &mut Engine) -> u32 {
    e.call(RENDERER_SDM_CONSTRUCT, &args![RENDERER_SDM]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_56c0, fn_00fb56c0() -> u32),
        entry!(0x00fc_72d0, fn_00fc72d0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each entry runs its constructor once with the global as `this` and answers its `EAX`.
    #[test]
    fn the_entries_construct_their_globals() {
        for (entry, constructor, object) in [
            (0x00fb_56c0u32, NI_MAIN_SDM_CONSTRUCT, NI_MAIN_SDM),
            (0x00fc_72d0, RENDERER_SDM_CONSTRUCT, RENDERER_SDM),
        ] {
            let mut e = Engine::new();
            e.register(constructor, |_, a| (a[0] ^ 0x5a).into_ret());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(entry, &args![]).u32(), object ^ 0x5a);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.iter().filter(|(a, _)| *a == constructor).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].1[0], object);
        }
    }
}
