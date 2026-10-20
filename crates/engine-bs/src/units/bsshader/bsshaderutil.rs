//! `BSShader/bsshaderutil.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB, `00ba91a0`): `cdecl`, three stack words (the
/// object, the flag index, the value).
const SHADER_PROPERTY_SET_FLAG_RECURSE: u32 = 0x00ba_91a0;

// Translated from 00b6b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyWireframe` (Xbox PDB): `cdecl`, two stack words (the
/// object, the value): `BSShaderProperty::SetFlagRecurse` with flag index `0x30`.
pub fn bs_shader_util_recursive_set_property_wireframe(e: &mut Engine, object: Ptr, value: u32) {
    e.call(
        SHADER_PROPERTY_SET_FLAG_RECURSE,
        &args![object, 0x30u32, value],
    );
}

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, the size); the
/// fixed-string maker `00a5b690` (`cdecl`, the text; answers the handle).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const FIXED_STRING: u32 = 0x00a5_b690;
/// The two names and the two globals that keep their 4-byte holders.
const CONVERTED_TEXTURE_NAME: u32 = 0x010a_e658;
const BACKGROUND_CONVERTED_TEXTURE_NAME: u32 = 0x010a_e63c;
const CONVERTED_TEXTURE: u32 = 0x011f_a050;
const BACKGROUND_CONVERTED_TEXTURE: u32 = 0x011f_a054;

// Translated from 00b6ba70 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6ba70 - the SEH frame (FS:[0] handler 00f28786, states 0 and 1 around the two allocations) is not translated.
/// `BSShaderUtil::InitSDM` (engine map; `cdecl`, no words): two 4-byte holders from the memory
/// manager, each holding the fixed string `"ConvertedTexture"` / `"BackgroundConvertedTexture"`
/// (`00a5b690`), kept at `011fa050` and `011fa054` (null where the allocation failed).
pub fn bs_shader_util_init_sdm(e: &mut Engine) {
    for (name, global) in [
        (CONVERTED_TEXTURE_NAME, CONVERTED_TEXTURE),
        (BACKGROUND_CONVERTED_TEXTURE_NAME, BACKGROUND_CONVERTED_TEXTURE),
    ] {
        let holder = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
        if holder != 0 {
            let handle = e.call(FIXED_STRING, &args![name]).u32();
            e.mem.set_u32(holder, handle);
        }
        e.mem.set_u32(global, holder);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_b910,
            bs_shader_util_recursive_set_property_wireframe(Ptr, u32)
        ),
        entry!(0x00b6_ba70, bs_shader_util_init_sdm()),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00b6b910_sets_flag_0x30_recursively() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ba_91a0, |_, _| Ret::default());
        e.call(0x00b6_b910, &args![0x8000u32, 1u32]);
        e.call(0x00b6_b910, &args![0x9000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x00ba_91a0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(sets, vec![&vec![0x8000, 0x30, 1], &vec![0x9000, 0x30, 0]]);
    }
}

#[cfg(test)]
mod init_sdm_tests {
    use super::*;

    #[test]
    fn init_sdm_keeps_two_fixed_string_holders() {
        let mut e = Engine::new();
        e.map(CONVERTED_TEXTURE & !0xfff, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(FIXED_STRING, |_, a| (a[0] + 1).into_ret());
        e.call(0x00b6_ba70, &[]);
        let first = e.mem.u32(CONVERTED_TEXTURE);
        let second = e.mem.u32(BACKGROUND_CONVERTED_TEXTURE);
        assert_eq!(e.mem.u32(first), CONVERTED_TEXTURE_NAME + 1);
        assert_eq!(e.mem.u32(second), BACKGROUND_CONVERTED_TEXTURE_NAME + 1);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00b6_ba70, &[]);
        assert_eq!(e.mem.u32(CONVERTED_TEXTURE), 0);
        assert_eq!(e.mem.u32(BACKGROUND_CONVERTED_TEXTURE), 0);
    }
}
