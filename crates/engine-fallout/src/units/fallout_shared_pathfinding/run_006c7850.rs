//! `fallout shared/pathfinding/run_006c7850` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's virtual table (`0106c800`) and its base's (`0106c814`).
const VTABLE: u32 = 0x0106_c800;
const BASE_VTABLE: u32 = 0x0106_c814;
/// The constructor of the member at `+0x10` (`0044dee0`, `thiscall`).
const MEMBER_CONSTRUCTOR: u32 = 0x0044_dee0;

// Translated from 006c7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (`thiscall`, no stack words; the engine map has no name for it; the process
/// lists' start-up initializer `00fa9890` reaches it): the base constructor `006c78d0`, then
/// the virtual table `0106c800`, the word at `+8` cleared, the byte at `+0xc` set to 1 and the
/// member at `+0x10` constructed (`0044dee0`). Answers `this`.
// Translation incomplete: 006c7850 - FS:[0] exception frame (handler 00f03d68) not translated.
pub fn fn_006c7850(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006c78d0(e, this);
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0xc, 1);
    e.call(MEMBER_CONSTRUCTOR, &args![base + 0x10]);
    this
}

// Translated from 006c78d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor (`thiscall`): the virtual table `0106c814` and the byte at `+4`
/// cleared. Answers `this`. The engine map has no name for it.
pub fn fn_006c78d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.mem.set_u8(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c7850, fn_006c7850(Ptr) -> Ptr),
        entry!(0x006c78d0, fn_006c78d0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_sets_both_tables_and_its_fields_and_builds_the_member() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x20);
        e.mem.write(0x1000, &[0xaa; 0x20]);
        e.call_log = Some(vec![]);
        e.register(MEMBER_CONSTRUCTOR, |_, a| a[0].into_ret());
        let answer = e.call(0x006c_7850, &args![0x1000u32]).u32();
        assert_eq!(answer, 0x1000);
        assert_eq!(e.mem.u32(0x1000), VTABLE);
        assert_eq!(e.mem.u8(0x1004), 0, "the base cleared its byte");
        assert_eq!(e.mem.u32(0x1008), 0);
        assert_eq!(e.mem.u8(0x100c), 1);
        let log = e.call_log.take().unwrap();
        let member: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MEMBER_CONSTRUCTOR)
            .collect();
        assert_eq!(member.len(), 1);
        assert_eq!(member[0].1, [0x1010]);
    }

    #[test]
    fn the_base_constructor_sets_its_table_and_clears_its_byte() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 8);
        e.mem.write(0x1000, &[0xaa; 8]);
        assert_eq!(e.call(0x006c_78d0, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), BASE_VTABLE);
        assert_eq!(e.mem.u8(0x1004), 0);
        assert_eq!(e.mem.bytes(0x1005, 3), [0xaa; 3]);
    }
}
