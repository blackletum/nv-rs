//! `BSShader/run_00bd3f80` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bd44c0` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BD44C0: u32 = 0x00bd_44c0;

// Translated from 00bd4750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`): the creator `GetShader` (`00b55560`) calls for
/// shader 4. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 6 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 7, 5, 1, 0), (3, 0xe, 0xd, 2, 0),
/// (4, 0xf, 0xe, 2, 0)), calls slot +0x94 with (0, 5, 0xa, 0x80000000, 3, 2, 0, 0) and slot
/// +0xb4. Then allocates 0x12c bytes and runs the constructor `00bd44c0` with the
/// declaration (a null allocation is used as it is), calls the shader's slots +0x4c, +0x144,
/// +0x150, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bd4750 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd4750(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 7, 5, 1, 0],
        [3, 0xe, 0xd, 2, 0],
        [4, 0xf, 0xe, 2, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 5u32, 0xau32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x12cu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD44C0, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bd_4750, fn_00bd4750() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd4750_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD44C0, &[]);
        let (shader, log) = run(&mut e, 0x00bd_4750);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 6, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (declaration_slot(0x8c), vec![d, 3, 0xe, 0xd, 2, 0]),
                (declaration_slot(0x8c), vec![d, 4, 0xf, 0xe, 2, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 5, 0xa, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x12c]),
                (SHADER_CONSTRUCT_00BD44C0, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x144), vec![SHADER]),
                (shader_slot(0x150), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}
