//! `(unplaced)/run_00453850` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The frame's seconds the frame timer (`0086f260`) keeps for the world step.
const FRAME_SECONDS: u32 = 0x011d_ea30;
/// `iNumHWThreads:General`: `main` sets it to the processor count, at least 2.
const NUM_HW_THREADS: u32 = 0x011c_3ea4;
/// `0043d4d0`: the address of an integer setting's value (`thiscall`).
const SETTING_INT_PTR: u32 = 0x0043_d4d0;
/// The critical section `00453860` takes around the cell's update when the
/// world steps on an AI thread.
const HAVOK_UPDATE_LOCK: u32 = 0x011c_3d84;

// Translated from 00453850 (disassembled, FalloutNV.exe 1.4.0.525)
/// Answers the frame's seconds kept at `011dea30` (`cdecl`, no arguments,
/// the float on the x87 stack). AI linear task thread 2 (`008c7f50`) passes it
/// to `TES::UpdateCellAnimations` (`00453550`).
pub fn fn_00453850(e: &mut Engine) -> f32 {
    e.mem.f32(FRAME_SECONDS)
}

// Translated from 00453860 (disassembled, FalloutNV.exe 1.4.0.525)
/// Takes (`lock` nonzero: `004538a0(lock section, 0)`) or gives back
/// (`004538c0`) the critical section at `011c3d84` around the cell's update,
/// only when `iNumHWThreads` isn't 1 (`thiscall`, `this` unused, `RET 4`).
/// `TES::UpdateCellAnimations` (`00453550`) calls it on both sides of the
/// cell's update.
pub fn fn_00453860(e: &mut Engine, _this: Ptr, lock: u8) {
    let threads = e.call(SETTING_INT_PTR, &args![NUM_HW_THREADS]).u32();
    if e.mem.i32(threads) == 1 {
        return;
    }
    if lock != 0 {
        e.call(0x0045_38a0, &args![HAVOK_UPDATE_LOCK, 0u32]);
    } else {
        e.call(0x0045_38c0, &args![HAVOK_UPDATE_LOCK]);
    }
}

/// `004538e0`: enters the critical section at `this` (`RET 4`, one unused
/// stack word); its own translation is in `run_004538e0.rs`.
const ENTER: u32 = 0x0045_38e0;

// Translated from 004538a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (`RET 4`): calls `004538e0(this, argument)`, which
/// enters the critical section at `this`.
pub fn fn_004538a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ENTER, &args![this, argument]);
}

/// `0082f1f0(this)`: leaves the critical section at `this`.
const LEAVE: u32 = 0x0082_f1f0;

// Translated from 004538c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (no stack words): calls `0082f1f0(this)`, which
/// leaves the critical section at `this`. The engine map names this address
/// `RtlLeaveCriticalSection` after the Xbox import it stands for; on PC it is
/// this ordinary function.
pub fn fn_004538c0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00453850, fn_00453850() -> f32),
        entry!(0x00453860, fn_00453860(Ptr, u8)),
        entry!(0x004538a0, fn_004538a0(Ptr, u32)),
        entry!(0x004538c0, fn_004538c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00453850_answers_the_frame_seconds() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_f32(FRAME_SECONDS, 0.25);
        assert_eq!(e.call(0x0045_3850, &args![]).f32(), 0.25);
    }

    fn lock_engine(threads: i32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(NUM_HW_THREADS, threads as u32);
        e.register(SETTING_INT_PTR, |_, a| a[0].into_ret());
        e.register(0x0045_38e0, |_, _| Ret::default());
        e.register(0x0082_f1f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00453860_locks_and_unlocks_with_more_than_one_thread() {
        let mut e = lock_engine(2);
        e.call(0x0045_3860, &args![0u32, 1u8]);
        e.call(0x0045_3860, &args![0u32, 0u8]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0045_38a0, vec![HAVOK_UPDATE_LOCK, 0])));
        assert!(log.contains(&(0x0045_38c0, vec![HAVOK_UPDATE_LOCK])));
    }

    #[test]
    fn fn_00453860_does_nothing_with_one_thread() {
        let mut e = lock_engine(1);
        e.call(0x0045_3860, &args![0u32, 1u8]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x0045_38a0 || *a == 0x0045_38c0));
    }

    #[test]
    fn fn_004538c0_forwards_this() {
        let mut e = Engine::new();
        e.register(LEAVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LEAVE, vec![0x1000]));
    }

    #[test]
    fn forwards_this_and_the_stack_word() {
        let mut e = Engine::new();
        e.register(ENTER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38a0, &args![0x1000u32, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER, vec![0x1000, 7]));
    }
}
