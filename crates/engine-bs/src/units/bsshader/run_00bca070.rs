//! `BSShader/run_00bca070` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bcaca0` (unnamed, `thiscall`, `RET 8`: two declarations).
const SHADER_CONSTRUCT_00BCACA0: u32 = 0x00bc_aca0;

// Translated from 00bcae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x20. No arguments. Makes two declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`: 2 entries and 3 entries, 1 stream each), sets the first's entries
/// through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 0, 5, 1, 0)) and
/// ends it (slot +0xb4), then the second's ((0, 0, 0, 2, 0), (1, 0, 5, 1, 0), (2, 0, 4, 3,
/// 0)) and ends it. Allocates 0x90 bytes and runs the constructor `00bcaca0` with both (a
/// null allocation is used as it is), calls the shader's slots +0x4c, +0x13c, +0x148,
/// +0x14c, +0x11c, then +0xc0 with the first declaration, and answers the shader.
// Translation incomplete: 00bcae90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcae90(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 0, 5, 1, 0]] {
        e.vcall(first, 0x8c, &entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in [[0u32, 0, 0, 2, 0], [1, 0, 5, 1, 0], [2, 0, 4, 3, 0]] {
        e.vcall(second, 0x8c, &entry);
    }
    e.vcall(second, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BCACA0, &args![block, first, second])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x14c, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_ae90, fn_00bcae90() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bcae90_makes_two_declarations_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BCACA0, &[]);
        let (shader, log) = run(&mut e, 0x00bc_ae90);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (DECLARATION_CREATE, vec![RENDERER, 3, 1]),
                (declaration_slot(0x8c), vec![a, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![a, 1, 0, 5, 1, 0]),
                (declaration_slot(0xb4), vec![a]),
                (declaration_slot(0x8c), vec![b, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![b, 1, 0, 5, 1, 0]),
                (declaration_slot(0x8c), vec![b, 2, 0, 4, 3, 0]),
                (declaration_slot(0xb4), vec![b]),
                (NI_ALLOCATE, vec![0x90]),
                (SHADER_CONSTRUCT_00BCACA0, vec![SHADER, a, b]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x14c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, a]),
            ]
        );
    }
}
