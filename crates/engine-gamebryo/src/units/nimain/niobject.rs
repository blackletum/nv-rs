//! `NiMain/niobject.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the base part gets first (`0101dce4`) and the one the object ends with
/// (`0109b4f4`).
const BASE_VTABLE: u32 = 0x0101_dce4;
const OBJECT_VTABLE: u32 = 0x0109_b4f4;
/// The count of live objects `InterlockedIncrement` raises (`011f4400`).
const OBJECT_COUNT: u32 = 0x011f_4400;
/// Import slot of KERNEL32 `InterlockedIncrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00a5d3a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niobject.cpp`; `thiscall`, no stack words, plain `RET`;
/// `NiObjectNET`'s constructor `00a5c200` starts with it): stores the vtable `0101dce4` and
/// a reference count of 0 at `+4`, raises the object count at `011f4400`
/// (`InterlockedIncrement`), then stores the vtable `0109b4f4`. Answers `this`.
pub fn fn_00a5d3a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
    e.mem.set_u32(this.addr(), OBJECT_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_d3a0, fn_00a5d3a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_object_gets_its_vtable_a_zero_count_and_is_counted() {
        let mut e = Engine::new();
        e.map(OBJECT_COUNT & !0xfff, 0x1000);
        e.map(0x0000_1000, 0x100);
        e.mem.set_u32(OBJECT_COUNT, 41);
        e.mem.set_u32(0x1004, 0xffff_ffff);
        // The import is bound by `Engine::with_exe` from the exe's import table; here a double.
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_d3a0, &args![0x1000u32]).u32(), 0x1000);
        let log = e.call_log.take().unwrap();
        let counted: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INTERLOCKED_INCREMENT)
            .collect();
        assert_eq!(counted.len(), 1);
        assert_eq!(counted[0].1[0], OBJECT_COUNT);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 42);
        assert_eq!(e.mem.u32(0x1000), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(0x1004), 0);
    }
}
