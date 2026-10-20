//! `BSShader/speedtreeleafshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bb2a00` and the method `00bb1d40` called directly (both of
/// this unit in the engine map, unnamed, `thiscall`, no words).
const SHADER_CONSTRUCT_00BB2A00: u32 = 0x00bb_2a00;
const SHADER_METHOD_00BB1D40: u32 = 0x00bb_1d40;

// Translated from 00bb2c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 6. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 4 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 7, 5, 1, 0)), calls slot +0x94 with (0, 3, 8, 0x80000000, 3, 2, 0, 0) and
/// slot +0xb4. Then allocates 0x38c bytes and runs the constructor `00bb2a00` (a null
/// allocation is used as it is), calls the shader's slots +0x4c and +0x13c, the method
/// `00bb1d40`, slot +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bb2c00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bb2c00(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 3, 3, 2, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 3u32, 8u32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x38cu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BB2A00, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.vcall(shader, 0x13c, &args![]);
    e.call(SHADER_METHOD_00BB1D40, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The leaf shader's vtable (`010b9190`; no RTTI locator before it).
const LEAF_SHADER_VTABLE_010B9190: u32 = 0x010b_9190;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// The sixteen `float`s the constructor sets (`01200638`..`01200674`): eight 1.0, eight 0.0.
const FLOATS_01200638: u32 = 0x0120_0638;

// Translated from 00bb2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the leaf shader's constructor
/// (the creator `00bb2c00` runs it), `thiscall`, no stack words. Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010b9190`, constructs the `NiPointer` arrays at +0x36c
/// (4) and +0x37c (2, both destructor `00b66f20`) and +0x384 (2, destructor `00b79930`), sets
/// +0x68 to 9, clears 0x300 bytes from +0x6c (`memset`), and stores 1.0 in the eight `float`s
/// from `01200638` and 0.0 in the eight from `01200658`. Answers `this`.
// Translation incomplete: 00bb2a00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bb2a00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, LEAF_SHADER_VTABLE_010B9190);
    construct_pointer_array(e, base + 0x36c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x37c, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x384, 2, DESTRUCT_00B79930);
    e.mem.set_u32(base + 0x68, 9);
    e.call(MEMSET, &args![base + 0x6c, 0u32, 0x300u32]);
    for index in 0..16u32 {
        let value = if index < 8 { 1.0f32 } else { 0.0 };
        e.set_global::<f32>(FLOATS_01200638 + 4 * index, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_2a00, fn_00bb2a00(Ptr) -> Ptr),
        entry!(0x00bb_2c00, fn_00bb2c00() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bb2c00_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BB2A00, &[SHADER_METHOD_00BB1D40]);
        let (shader, log) = run(&mut e, 0x00bb_2c00);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 3, 8, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x38c]),
                (SHADER_CONSTRUCT_00BB2A00, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (SHADER_METHOD_00BB1D40, vec![SHADER]),
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
    fn fn_00bb2a00_builds_the_leaf_shader() {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.map(0x0120_0000, 0x1000);
        e.mem.write(FLOATS_01200638, &[0xff; 0x40]);
        let this = e.mem.alloc(0x390);
        assert_eq!(e.call(0x00bb_2a00, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), LEAF_SHADER_VTABLE_010B9190);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x36c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x37c, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x384, 2, DESTRUCT_00B79930),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 9);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x6c, 0, 0x300]]);
        for index in 0..8u32 {
            assert_eq!(e.global::<u32>(FLOATS_01200638 + 4 * index), 0x3f80_0000);
            assert_eq!(e.global::<u32>(FLOATS_01200638 + 0x20 + 4 * index), 0);
        }
    }
}
