//! `BSSystem/bstaskletmanager.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b00a80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletManager::CreateTaskGroup` (engine map; `thiscall`, one stack word, `RET 4`: where to
/// put the group). When the manager runs (the byte at `+4`), its virtual slot `+0x10` makes the
/// group into a local and its answer (a byte) is the result, the local copied to `*out`;
/// otherwise `*out` is 0 and the result 0.
pub fn bs_tasklet_manager_create_task_group(e: &mut Engine, this: Ptr, out: u32) -> u8 {
    if e.mem.u8(this.addr() + 4) == 0 {
        e.mem.set_u32(out, 0);
        return 0;
    }
    e.with_stack(4, |e, group| {
        let made = e.vcall(this.addr(), 0x10, &args![group]).u8();
        let value = e.mem.u32(group.addr());
        e.mem.set_u32(out, value);
        made
    })
}

/// The two hook lists the workers run (`BSTaskletManager`'s statics): the counts at `011f8af0` /
/// `011f8af4` and the object arrays at `011f8af8` / `011f8b38`.
const ENTER_COUNT: u32 = 0x011f_8af0;
const LEAVE_COUNT: u32 = 0x011f_8af4;
const ENTER_HOOKS: u32 = 0x011f_8af8;
const LEAVE_HOOKS: u32 = 0x011f_8b38;

/// Slot `+4` of each set object of a hook array, for the first `count` (compared signed, `JGE`).
fn run_hooks(e: &mut Engine, count: u32, hooks: u32) {
    let mut i: i32 = 0;
    while i < e.mem.i32(count) {
        let hook = e.mem.u32(hooks.wrapping_add((i as u32).wrapping_mul(4)));
        if hook != 0 {
            e.vcall(hook, 4, &args![]);
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00b00c90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, plain `RET`; a worker thread runs it as it
/// starts, `00b023f0`): slot `+4` of each set object of the first `[011f8af0]` at `011f8af8`.
pub fn fn_00b00c90(e: &mut Engine, _this: Ptr) {
    run_hooks(e, ENTER_COUNT, ENTER_HOOKS);
}

// Translated from 00b00cf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, plain `RET`; a worker thread runs it as it
/// ends): slot `+4` of each set object of the first `[011f8af4]` at `011f8b38`.
pub fn fn_00b00cf0(e: &mut Engine, _this: Ptr) {
    run_hooks(e, LEAVE_COUNT, LEAVE_HOOKS);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0a80, bs_tasklet_manager_create_task_group(Ptr, u32) -> u8),
        entry!(0x00b0_0c90, fn_00b00c90(Ptr)),
        entry!(0x00b0_0cf0, fn_00b00cf0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_running_manager_makes_the_group_through_slot_0x10() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x00ab_0000]);
        e.mem.set_u32(0x1000, 0x0300_0000);
        e.register(0x00ab_0000, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            1u32.into_ret()
        });
        e.mem.set_u32(0x1080, 0xffff_ffff);
        assert_eq!(
            e.call(0x00b0_0a80, &args![0x1000u32, 0x1080u32]).u32() & 0xff,
            0
        );
        assert_eq!(e.mem.u32(0x1080), 0);
        e.mem.set_u8(0x1004, 1);
        assert_eq!(
            e.call(0x00b0_0a80, &args![0x1000u32, 0x1080u32]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(0x1080), 0x4444);
    }
}

#[cfg(test)]
mod hook_tests {
    use super::*;

    /// The set objects among the first `count` get their slot `+4`; the count is read signed.
    #[test]
    fn the_hooks_run_their_slot_4() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.map(0x0000_1000, 0x100);
        e.put_vtable(0x0300_0000, &[0, 0x00ab_0000]);
        e.mem.set_u32(0x1000, 0x0300_0000);
        e.mem.set_u32(0x1010, 0x0300_0000);
        e.register(0x00ab_0000, |e, a| {
            let n = e.mem.u32(0x1080);
            e.mem.set_u32(0x1084 + 4 * n, a[0]);
            e.mem.set_u32(0x1080, n + 1);
            Ret::default()
        });
        e.mem.set_u32(ENTER_COUNT, 3);
        e.mem.set_u32(ENTER_HOOKS, 0x1000);
        e.mem.set_u32(ENTER_HOOKS + 8, 0x1010);
        e.call(0x00b0_0c90, &args![0u32]);
        assert_eq!(e.mem.u32(0x1080), 2);
        assert_eq!([e.mem.u32(0x1084), e.mem.u32(0x1088)], [0x1000, 0x1010]);
        // A negative count runs nothing.
        e.mem.set_u32(LEAVE_COUNT, 0xffff_ffff);
        e.mem.set_u32(LEAVE_HOOKS, 0x1000);
        e.call(0x00b0_0cf0, &args![0u32]);
        assert_eq!(e.mem.u32(0x1080), 2);
    }
}
