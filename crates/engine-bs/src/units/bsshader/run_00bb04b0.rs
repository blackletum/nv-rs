//! `BSShader/run_00bb04b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bb04b0` (this unit's first function, unnamed, `thiscall`, no
/// words).
const SHADER_CONSTRUCT_00BB04B0: u32 = 0x00bb_04b0;

// Translated from 00bb0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x1e. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`, 3 entries, 2 streams), sets its entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)), calls slot +0x94 with (1, 0,
/// 6, 0x80000000, 2, 5, 1, 0) and slot +0xb4. Then allocates 0xac bytes and runs the
/// constructor `00bb04b0` (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x148, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bb0660 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bb0660(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 2u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![1u32, 0u32, 6u32, 0x8000_0000u32, 2u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BB04B0, &args![block]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bb_0660, fn_00bb0660() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bb0660_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BB04B0, &[]);
        let (shader, log) = run(&mut e, 0x00bb_0660);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 2]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 1, 0, 6, 0x8000_0000, 2, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xac]),
                (SHADER_CONSTRUCT_00BB04B0, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}
