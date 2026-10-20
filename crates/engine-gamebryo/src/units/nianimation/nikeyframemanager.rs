//! `NiAnimation/nikeyframemanager.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyframeManager::ms_RTTI` (the class's `NiRTTI` record).
const NI_KEYFRAME_MANAGER_RTTI: u32 = 0x011f_3fb4;

// Translated from 00a4da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_keyframe_manager_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_KEYFRAME_MANAGER_RTTI)
}

// Translated from 00a4d6d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nikeyframemanager.cpp` by the map; `thiscall`, one stack
/// word, `RET 4`; a string map's hash, which `NiStream`'s loader map reaches through its
/// `SetAt` `00a642a0`): the hash of the C string `key`, `h = h * 33 + c` over its bytes (each
/// byte sign-extended, `MOVSX`; 32-bit wrap), modulo the table size at `this + 4` (`DIV`, the
/// remainder). A table size of 0 faults (`#DE`), as the `DIV` does.
pub fn fn_00a4d6d0(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let mut hash: u32 = 0;
    let mut at = key.addr();
    loop {
        let c = e.mem.u8(at);
        if c == 0 {
            break;
        }
        hash = (c as i8 as i32 as u32).wrapping_add(hash.wrapping_shl(5).wrapping_add(hash));
        at = at.wrapping_add(1);
    }
    let size = e.mem.u32(this.addr() + 4);
    if size == 0 {
        panic!("divide error (#DE) in 00a4d6d0: table size 0");
    }
    hash % size
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4da80, ni_keyframe_manager_get_rtti() -> Ptr),
        entry!(0x00a4_d6d0, fn_00a4d6d0(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_keyframe_manager_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4da80, &args![]).u32(), NI_KEYFRAME_MANAGER_RTTI);
    }
}

#[cfg(test)]
mod hash_tests {
    use super::*;

    /// `h * 33 + c` over the bytes (sign-extended), modulo the size at `+4`.
    #[test]
    fn hashes_the_string_modulo_the_table_size() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.mem.set_u32(0x1004, 37);
        for (i, b) in b"NiNode\0".iter().enumerate() {
            e.mem.set_u8(0x1010 + i as u32, *b);
        }
        let mut h: u32 = 0;
        for &b in b"NiNode" {
            h = h.wrapping_mul(33).wrapping_add(b as u32);
        }
        assert_eq!(
            e.call(0x00a4_d6d0, &args![0x1000u32, 0x1010u32]).u32(),
            h % 37
        );
        // A byte above 0x7f counts negative; the empty string hashes to 0.
        e.mem.set_u8(0x1020, 0x80);
        e.mem.set_u8(0x1021, 0);
        let expected = (0u32.wrapping_sub(0x80)) % 37;
        assert_eq!(
            e.call(0x00a4_d6d0, &args![0x1000u32, 0x1020u32]).u32(),
            expected
        );
        assert_eq!(e.call(0x00a4_d6d0, &args![0x1000u32, 0x1021u32]).u32(), 0);
    }
}
