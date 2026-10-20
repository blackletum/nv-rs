//! `NiSystem/nithread_xenon.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of KERNEL32 `RaiseException` (`stdcall`, four words).
const RAISE_EXCEPTION: u32 = 0x00fd_f154;
/// The debugger thread-naming exception code.
const MS_VC_EXCEPTION: u32 = 0x406d_1388;

// Translated from 00aa2740 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa2740 - the FS:[0] exception frame (_except_handler4, scope table
// 01169d88) is not modeled: its __except takes the exception and goes on, which the platform's
// RaiseException stand-in does by coming back.
/// `SetThreadName` (engine map; `cdecl`, two words, plain `RET`): fills a 16-byte
/// `THREADNAME_INFO` on the stack (type 0x1000, the name `name`, the thread id `thread`, flags
/// 0) and raises the debugger thread-naming exception `0x406D1388` with its four words
/// (`RaiseException(code, 0, 4, info)`).
pub fn set_thread_name(e: &mut Engine, thread: u32, name: u32) {
    e.with_stack(0x10, |e, info| {
        let at = info.addr();
        e.mem.set_u32(at, 0x1000);
        e.mem.set_u32(at + 4, name);
        e.mem.set_u32(at + 8, thread);
        e.mem.set_u32(at + 0xc, 0);
        e.call(RAISE_EXCEPTION, &args![MS_VC_EXCEPTION, 0u32, 4u32, info]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_2740, set_thread_name(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raises_the_naming_exception_with_the_info() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.register(RAISE_EXCEPTION, |e, a| {
            let info = a[3];
            for i in 0..4 {
                let word = e.mem.u32(info + 4 * i);
                e.mem.set_u32(0x1010 + 4 * i, word);
            }
            e.mem.set_u32(0x1000, a[0]);
            e.mem.set_u32(0x1004, a[2]);
            Ret::default()
        });
        e.call(0x00aa_2740, &args![7u32, 0x1234u32]);
        assert_eq!((e.mem.u32(0x1000), e.mem.u32(0x1004)), (MS_VC_EXCEPTION, 4));
        assert_eq!(
            [0x1010, 0x1014, 0x1018, 0x101c].map(|a| e.mem.u32(a)),
            [0x1000, 0x1234, 7, 0]
        );
    }
}
