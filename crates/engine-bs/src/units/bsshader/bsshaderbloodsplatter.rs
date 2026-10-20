//! `BSShader/bsshaderbloodsplatter.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderBloodSplatter::BSShaderBloodSplatter` (`00b80d30`), `::LoadShaders`
/// (`00b80090`) and `::LoadStagesAndPasses` (`00b807d0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_0d30;
const LOAD_SHADERS: u32 = 0x00b8_0090;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_07d0;

// Translated from 00b80e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 7. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 2 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)) and calls slot +0xb4. Then
/// allocates 0x84 bytes and runs the constructor (`00b80d30`; a null allocation is used as it
/// is), calls the shader's slot +0x4c, `LoadShaders` (`00b80090`), `LoadStagesAndPasses`
/// (`00b807d0`), slot +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00b80e70 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_blood_splatter_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x84u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b8_0e70,
        bs_shader_blood_splatter_create_shader() -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_blood_splatter_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_0e70);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x84]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (LOAD_STAGES_AND_PASSES, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}
