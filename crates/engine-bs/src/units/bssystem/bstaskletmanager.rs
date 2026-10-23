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

/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on the manager `011f6238`, one word), the
/// task group's constructor `00b027c0` (`thiscall`), and the KERNEL32 import slots
/// `CreateSemaphoreA` and `InitializeCriticalSection`.
const ALLOCATE_00AA3E40: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
const GROUP_CONSTRUCT_00B027C0: u32 = 0x00b0_27c0;
const IMPORT_CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// The vtables the group's constructor stores, the base's first (`010a48e8`) then its own
/// (`010a48d8`).
const GROUP_BASE_VTABLE_010A48E8: u32 = 0x010a_48e8;
const GROUP_VTABLE_010A48D8: u32 = 0x010a_48d8;

// Translated from 00b02010 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b02010 - the C++ exception frame (FS:[0] chain, handler 00f238fb, the state word around the constructor) is not translated; the security cookie is pushed but never checked.
/// Unnamed in the engine map: the manager's virtual slot `+0x10` that
/// [`bs_tasklet_manager_create_task_group`] calls (`thiscall`, one stack word, `RET 4`: where to
/// put the group; `this` is not read). Allocates 0x3c bytes (`00aa3e40` on `011f6238`) and, when
/// that gives memory, constructs a task group there (`00b027c0`); stores the group (or null) in
/// `*out` and answers whether there is one.
pub fn fn_00b02010(e: &mut Engine, _this: Ptr, out: u32) -> u8 {
    let block = e
        .call(ALLOCATE_00AA3E40, &args![MEMORY_MANAGER_011F6238, 0x3cu32])
        .u32();
    let group = if block == 0 {
        0
    } else {
        e.call(GROUP_CONSTRUCT_00B027C0, &args![block]).u32()
    };
    e.mem.set_u32(out, group);
    u8::from(group != 0)
}

// Translated from 00b027c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a task group's constructor (`thiscall`, no stack words; answers
/// `this`). The base's vtable `010a48e8` and the byte `+4` = 1, then its own vtable
/// `010a48d8`; at `+8` a semaphore block: `+8` 0, `+0x10` 1 and `+0xc` a new semaphore
/// (`CreateSemaphoreA(NULL, 0, 1, NULL)`); the critical section at `+0x14` initialized; the
/// bytes `+0x2c`, `+0x2d`, `+0x2e` and the words `+0x30`, `+0x34`, `+0x38` cleared.
pub fn fn_00b027c0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, GROUP_BASE_VTABLE_010A48E8);
    e.mem.set_u8(t + 4, 1);
    e.mem.set_u32(t, GROUP_VTABLE_010A48D8);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0x10, 1);
    let semaphore = e
        .call(IMPORT_CREATE_SEMAPHORE_A, &args![0u32, 0u32, 1u32, 0u32])
        .u32();
    e.mem.set_u32(t + 0xc, semaphore);
    e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &args![t + 0x14]);
    e.mem.set_u8(t + 0x2c, 0);
    e.mem.set_u8(t + 0x2d, 0);
    e.mem.set_u8(t + 0x2e, 0);
    e.mem.set_u32(t + 0x30, 0);
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0);
    this
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
        entry!(0x00b0_2010, fn_00b02010(Ptr, u32) -> u8),
        entry!(0x00b0_27c0, fn_00b027c0(Ptr) -> Ptr),
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

    /// The group is made in the manager's memory and constructed; without memory `*out` is null.
    #[test]
    fn the_task_group_is_allocated_and_constructed() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x3c);
        let given = std::rc::Rc::new(std::cell::Cell::new(block));
        let g = given.clone();
        e.register_double(ALLOCATE_00AA3E40, move |_, a| {
            assert_eq!(&a[..2], &[MEMORY_MANAGER_011F6238, 0x3c]);
            g.get().into_ret()
        });
        e.register(IMPORT_CREATE_SEMAPHORE_A, |_, a| {
            assert_eq!(a, &[0, 0, 1, 0]);
            0x77u32.into_ret()
        });
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00b0_2010, &args![0u32, out]).u8(), 1);
        assert_eq!(e.mem.u32(out), block);
        assert_eq!(e.mem.u32(block), GROUP_VTABLE_010A48D8);
        assert_eq!(
            (
                e.mem.u8(block + 4),
                e.mem.u32(block + 0xc),
                e.mem.u32(block + 0x10)
            ),
            (1, 0x77, 1)
        );
        given.set(0);
        assert_eq!(e.call(0x00b0_2010, &args![0u32, out]).u8(), 0);
        assert_eq!(e.mem.u32(out), 0);
    }
}
