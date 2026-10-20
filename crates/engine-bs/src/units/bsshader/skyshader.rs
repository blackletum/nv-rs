//! `BSShader/skyshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `SkyShader::SkyShader` (`00b8a1f0`), `::LoadShaders` (`00b874f0`) and
/// `::LoadStagesAndPasses` (`00b887c0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_a1f0;
const LOAD_SHADERS: u32 = 0x00b8_74f0;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_87c0;

// Translated from 00b8a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0xa. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 3 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 5, 4, 3, 0), (2, 7, 5, 1, 0)) and calls slot
/// +0xb4. Then allocates 0xf0 bytes and runs the constructor (`00b8a1f0`; a null allocation
/// is used as it is), calls the shader's slot +0x4c, `LoadShaders` (`00b874f0`),
/// `LoadStagesAndPasses` (`00b887c0`), slot +0x11c, then +0xc0 with the declaration, and
/// answers the shader.
// Translation incomplete: 00b8a390 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn sky_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 5, 4, 3, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xf0u32]).u32();
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

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The sky shader's vtable (`010afe18`; no RTTI locator before it).
const SKY_SHADER_VTABLE_010AFE18: u32 = 0x010a_fe18;
/// The four words copied (`011a9bd0`) and the seven four-word blocks they go to, in the
/// exe's order of their first words.
const COLOR_011A9BD0: u32 = 0x011a_9bd0;
const COLOR_BLOCKS: [u32; 7] = [
    0x011f_f910,
    0x011f_f920,
    0x011f_f930,
    0x011f_f8e0,
    0x011f_f8f0,
    0x011f_f900,
    0x011f_f8c8,
];

// Translated from 00b8a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::SkyShader` (Xbox PDB), `thiscall`, no stack words. Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010afe18`, constructs the `NiPointer` arrays at +0x6c (0xb,
/// destructor `00b79930`), +0x98 (0xb) and +0xc4 (8; destructor `00b66f20`), sets +0x68 to
/// 0xb, copies the four words at `011a9bd0` into each of the blocks at `011ff910`,
/// `011ff920`, `011ff930`, `011ff8e0`, `011ff8f0`, `011ff900` and `011ff8c8`, and sets the
/// byte +0x24 to 1. Answers `this`.
// Translation incomplete: 00b8a1f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn sky_shader_sky_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SKY_SHADER_VTABLE_010AFE18);
    construct_pointer_array(e, base + 0x6c, 0xb, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x98, 0xb, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xc4, 8, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 0xb);
    let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(COLOR_011A9BD0 + offset));
    for block in COLOR_BLOCKS {
        e.set_global::<u32>(block, color[0]);
    }
    for block in COLOR_BLOCKS {
        for word in 1..4u32 {
            e.set_global::<u32>(block + 4 * word, color[word as usize]);
        }
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_a1f0, sky_shader_sky_shader(Ptr) -> Ptr),
        entry!(0x00b8_a390, sky_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn sky_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_a390);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xf0]),
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

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn sky_shader_constructor_builds_the_shader_and_copies_the_color() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        for word in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BD0 + 4 * word, 0x30 + word);
        }
        let this = e.mem.alloc(0xe4);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SKY_SHADER_VTABLE_010AFE18);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0xb, DESTRUCT_00B79930),
                pointer_array(this + 0x98, 0xb, DESTRUCT_00B66F20),
                pointer_array(this + 0xc4, 8, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0xb);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        for block in [
            0x011f_f8c8u32,
            0x011f_f8e0,
            0x011f_f8f0,
            0x011f_f900,
            0x011f_f910,
            0x011f_f920,
            0x011f_f930,
        ] {
            for word in 0..4u32 {
                assert_eq!(e.global::<u32>(block + 4 * word), 0x30 + word);
            }
        }
        assert_eq!(e.global::<u32>(0x011f_f8d8), 0);
    }
}
