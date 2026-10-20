//! `(unplaced)/run_00662760` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl, the size) and the 4-byte object's constructor `00438170`
/// (`thiscall`, one word; unit `fallout shared/extradataobjects.cpp`), given `0105f3c0`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const CONSTRUCT_00438170: u32 = 0x0043_8170;
const CONSTRUCT_WORD_0105F3C0: u32 = 0x0105_f3c0;
/// Where the object is kept (`011d5b8c`).
const OBJECT_011D5B8C: u32 = 0x011d_5b8c;

// Translated from 00662760 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00662760 - the SEH frame (FS:[0] handler 00f109cb, state 0 around the constructor call) is not translated.
/// Unnamed in the engine map (subsystem `fallout shared/facegen`, no unit; `cdecl`, no words;
/// `0045b020` calls it at start-up): a 4-byte object (`operator new(4)`) built by
/// `00438170(0105f3c0)` (null when the allocation failed) into `011d5b8c`.
pub fn fn_00662760(e: &mut Engine) {
    let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(CONSTRUCT_00438170, &args![block, CONSTRUCT_WORD_0105F3C0])
            .u32()
    };
    e.mem.set_u32(OBJECT_011D5B8C, object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_2760, fn_00662760())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00662760_keeps_the_built_object() {
        let mut e = Engine::new();
        e.map(OBJECT_011D5B8C & !0xfff, 0x1000);
        e.register(OPERATOR_NEW, |_, _| 0x0e00_0000u32.into_ret());
        e.register(CONSTRUCT_00438170, |_, a| {
            assert_eq!(a[1], CONSTRUCT_WORD_0105F3C0);
            a[0].into_ret()
        });
        e.call(0x0066_2760, &[]);
        assert_eq!(e.mem.u32(OBJECT_011D5B8C), 0x0e00_0000);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x0066_2760, &[]);
        assert_eq!(e.mem.u32(OBJECT_011D5B8C), 0);
    }
}
