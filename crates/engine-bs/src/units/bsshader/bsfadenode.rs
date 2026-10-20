//! `BSShader/bsfadenode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator (`00aa13e0`, `cdecl`, the size), the memory manager (`011f6238`) and
/// its allocate (`00aa3e40`, `thiscall`, the size), the fixed-string maker (`00a5b690`, `cdecl`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const FIXED_STRING: u32 = 0x00a5_b690;
/// `NiProperty`'s base constructor (`00a5c200`) and `NiAlphaProperty`'s vtable (`010162dc`).
const NI_PROPERTY_CONSTRUCT: u32 = 0x00a5_c200;
const NI_ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
/// `InterlockedIncrement` and `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The fade nodes' shared alpha property (a smart pointer, `011f9154`), the text
/// `"FadeNode Anim"` (`010a9090`) and the holder of its fixed string (`011f913c`).
const ALPHA_PROPERTY_011F9154: u32 = 0x011f_9154;
const FADE_NODE_ANIM_TEXT: u32 = 0x010a_9090;
const ANIM_NAME_011F913C: u32 = 0x011f_913c;

// Translated from 00b4e270 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b4e270 - the SEH frame (FS:[0] handler 00f26d0a, states 0 and 1 around the two allocations) is not translated.
/// `BSFadeNode::InitSDM` (engine map; `cdecl`, no words; `BSShaderSDM::InitSDM` runs it): a
/// 0x1C-byte `NiAlphaProperty` (`00aa13e0`; `NiProperty`'s constructor, the vtable `010162dc`,
/// the 16-bit flags +0x18 0xEC, the test reference byte +0x1A 0; null when the allocation
/// failed) becomes the shared alpha property at `011f9154` (the smart pointer's assignment:
/// when it differs, the old one released through `InterlockedDecrement` and its vtable slot +4
/// at zero, the new one held through `InterlockedIncrement`); bit 0 of the held property's
/// flags is set (the exe reads the property even when it is null). Then a 4-byte holder from
/// the memory manager with the fixed string `"FadeNode Anim"` into `011f913c` (null when that
/// allocation failed).
pub fn bs_fade_node_init_sdm(e: &mut Engine) {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let property = if block == 0 {
        0
    } else {
        e.call(NI_PROPERTY_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_ALPHA_PROPERTY_VTABLE);
        e.mem.set_u16(block + 0x18, 0xec);
        e.mem.set_u8(block + 0x1a, 0);
        block
    };
    let held = e.mem.u32(ALPHA_PROPERTY_011F9154);
    if held != property {
        if held != 0 && e.call(INTERLOCKED_DECREMENT, &args![held + 4]).i32() == 0 {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(ALPHA_PROPERTY_011F9154, property);
        if property != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![property + 4]);
        }
    }
    let held = e.mem.u32(ALPHA_PROPERTY_011F9154);
    let flags = e.mem.u16(held + 0x18);
    e.mem.set_u16(held + 0x18, flags | 1);
    let holder = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    if holder != 0 {
        let name = e.call(FIXED_STRING, &args![FADE_NODE_ANIM_TEXT]).u32();
        e.mem.set_u32(holder, name);
    }
    e.mem.set_u32(ANIM_NAME_011F913C, holder);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b4_e270, bs_fade_node_init_sdm())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_sdm_makes_the_shared_alpha_property_and_the_anim_name() {
        let mut e = Engine::new();
        e.map(ANIM_NAME_011F913C & !0xfff, 0x1000);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIXED_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], v);
            v.into_ret()
        });
        e.call(0x00b4_e270, &[]);
        let property = e.mem.u32(ALPHA_PROPERTY_011F9154);
        assert_ne!(property, 0);
        assert_eq!(e.mem.u32(property), NI_ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(property + 0x18), 0xed);
        assert_eq!(e.mem.u8(property + 0x1a), 0);
        assert_eq!(e.mem.i32(property + 4), 1);
        let holder = e.mem.u32(ANIM_NAME_011F913C);
        assert_eq!(e.mem.u32(holder), FADE_NODE_ANIM_TEXT + 1);
    }
}
