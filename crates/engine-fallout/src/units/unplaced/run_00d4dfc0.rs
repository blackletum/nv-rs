//! `(unplaced)/run_00d4dfc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's virtual table (`010d0a1c`).
const VTABLE: u32 = 0x010d_0a1c;
/// `hkpBridgeConstraintAtom::init` (Xbox PDB, `00d21ce0`, `thiscall`, one word).
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;

// Translated from 00d4dfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constraint data constructor with the "finish" flag (`thiscall`, `RET 4`; the engine map has
/// no name for it; `00dced70` builds one on its stack with the flag 0 to read the virtual
/// table, which a Havok type info's start-up initializer `00fc00d0` needs): stores the virtual
/// table `010d0a1c`; when `finish` is not 0 the bridge atom at `+0xc` is set up with the word at
/// `+0x14` (`hkpBridgeConstraintAtom::init`), and, the flag tested again, with `this`. Answers
/// `this`.
pub fn fn_00d4dfc0(e: &mut Engine, this: Ptr, finish: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    let atom = base + 0xc;
    if finish != 0 {
        let word = e.mem.u32(atom + 8);
        e.call(BRIDGE_ATOM_INIT, &args![atom, word]);
        if finish != 0 {
            e.call(BRIDGE_ATOM_INIT, &args![atom, base]);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4dfc0, fn_00d4dfc0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_the_flag_only_the_table_is_stored() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x20);
        e.register(BRIDGE_ATOM_INIT, |_, _| {
            panic!("no atom set-up without the flag")
        });
        assert_eq!(e.call(0x00d4_dfc0, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), VTABLE);
    }

    #[test]
    fn with_the_flag_the_bridge_atom_is_set_up_twice() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x20);
        e.mem.set_u32(0x1014, 0x5555);
        e.register(BRIDGE_ATOM_INIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d4_dfc0, &args![0x1000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        let inits: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == BRIDGE_ATOM_INIT)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(inits, [vec![0x100c, 0x5555], vec![0x100c, 0x1000]]);
    }
}
