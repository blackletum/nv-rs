//! `BSShader/watershader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1d while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_WATER: u32 = 0x1d;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `WaterShader::WaterShader` (`00b86bd0`), `WaterShader::LoadShaders` (`00b827c0`) and the
/// method `00b83dd0` of this unit (engine map; all `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_6bd0;
const LOAD_SHADERS: u32 = 0x00b8_27c0;
const WATER_METHOD_00B83DD0: u32 = 0x00b8_3dd0;

// Translated from 00b86de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0x11. No arguments. With the thread's allocation context (+0x2b4 of the thread
/// block) at 0x1d (put back at the end): makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 4 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 4, 0), (3, 7, 5, 1, 0)) and calls slot +0xb4. Then allocates 0x250
/// bytes and runs the constructor (`00b86bd0`; a null allocation is used as it is), calls
/// the shader's slot +0x4c, `LoadShaders` (`00b827c0`), `00b83dd0`, slot +0x11c, then +0xc0
/// with the declaration, and answers the shader.
// Translation incomplete: 00b86de0 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn water_shader_create_shader(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_WATER);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 4, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x250u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(WATER_METHOD_00B83DD0, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The water shader's vtable (`010af9e0`; no RTTI locator before it).
const WATER_SHADER_VTABLE_010AF9E0: u32 = 0x010a_f9e0;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
/// The globals the constructor sets: the shader itself (`011ff370`), four `float`s
/// (`011ff458`..`011ff464`) and the flags and counters at `011ff100`..`011ff11a`.
const WATER_SHADER_011FF370: u32 = 0x011f_f370;
const FLOATS_011FF458: u32 = 0x011f_f458;

// Translated from 00b86bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::WaterShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010af9e0`, constructs the `NiPointer`
/// arrays at +0x6c (0x4a, destructor `00b79930`) and +0x1a0 (0x26, destructor `00b66f20`) with
/// the words +0x194 to +0x19c cleared between them, clears +0x238 to +0x24c, sets +0x68 to 0xd
/// and the byte +0x24 to 1, clears (releasing what they hold) the `NiPointer`s at +0x23c,
/// +0x244, +0x238 and +0x248, keeps the shader in `011ff370`, stores 0.0 in the `float`s
/// `011ff458` to `011ff464`, clears the bytes `011ff104`, `011ff103`, `011ff102` and, unless the
/// shader model setting `011f91c0` is above 2 (signed), `011ff100`; then clears the 16-bit
/// `011ff118`, the byte `011ff11a`, the word `011ff110` and the 16-bit `011ff114`. Answers
/// `this`.
// Translation incomplete: 00b86bd0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xb for the base and the arrays) is not translated.
pub fn water_shader_water_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, WATER_SHADER_VTABLE_010AF9E0);
    construct_pointer_array(e, base + 0x6c, 0x4a, DESTRUCT_00B79930);
    for offset in [0x194u32, 0x198, 0x19c] {
        e.mem.set_u32(base + offset, 0);
    }
    construct_pointer_array(e, base + 0x1a0, 0x26, DESTRUCT_00B66F20);
    for offset in (0x238u32..=0x24c).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0xd);
    e.mem.set_u8(base + 0x24, 1);
    for offset in [0x23cu32, 0x244, 0x238, 0x248] {
        ni_pointer_clear(e, base + offset);
    }
    let above_model_2 = e.global::<i32>(SHADER_MODEL_011F91C0) > 2;
    e.set_global::<u32>(WATER_SHADER_011FF370, base);
    for index in 0..4u32 {
        e.set_global::<f32>(FLOATS_011FF458 + 4 * index, 0.0);
    }
    for flag in [0x011f_f104u32, 0x011f_f103, 0x011f_f102] {
        e.set_global::<u8>(flag, 0);
    }
    if !above_model_2 {
        e.set_global::<u8>(0x011f_f100, 0);
    }
    e.set_global::<u16>(0x011f_f118, 0);
    e.set_global::<u8>(0x011f_f11a, 0);
    e.set_global::<u32>(0x011f_f110, 0);
    e.set_global::<u16>(0x011f_f114, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_6bd0, water_shader_water_shader(Ptr) -> Ptr),
        entry!(0x00b8_6de0, water_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn water_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, WATER_METHOD_00B83DD0]);
        let (shader, log) = run(&mut e, 0x00b8_6de0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 4, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x250]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (WATER_METHOD_00B83DD0, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_WATER);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.mem.write(0x011f_f100, &[0xee; 0x20]);
        e.mem.write(FLOATS_011FF458, &[0xee; 0x10]);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x250);
        e.mem.write(this, &[0xcc; 0x250]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        (e, this)
    }

    #[test]
    fn water_shader_constructor_builds_the_shader_and_resets_the_globals() {
        let (e, this) = construct(2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_VTABLE_010AF9E0);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x4a, DESTRUCT_00B79930),
                pointer_array(this + 0x1a0, 0x26, DESTRUCT_00B66F20),
            ]
        );
        for offset in [
            0x194u32, 0x198, 0x19c, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0xd);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.global::<u32>(WATER_SHADER_011FF370), this);
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(FLOATS_011FF458 + 4 * index), 0);
        }
        for byte in [0x011f_f100u32, 0x011f_f102, 0x011f_f103, 0x011f_f104] {
            assert_eq!(e.global::<u8>(byte), 0, "{byte:#x}");
        }
        assert_eq!(e.global::<u8>(0x011f_f101), 0xee);
        assert_eq!(e.global::<u16>(0x011f_f118), 0);
        assert_eq!(e.global::<u8>(0x011f_f11a), 0);
        assert_eq!(e.global::<u32>(0x011f_f110), 0);
        assert_eq!(e.global::<u16>(0x011f_f114), 0);
        assert_eq!(e.global::<u8>(0x011f_f116), 0xee);
    }

    #[test]
    fn water_shader_constructor_keeps_byte_011ff100_above_model_2() {
        let (e, _) = construct(3);
        assert_eq!(e.global::<u8>(0x011f_f100), 0xee);
        assert_eq!(e.global::<u8>(0x011f_f102), 0);
    }
}
