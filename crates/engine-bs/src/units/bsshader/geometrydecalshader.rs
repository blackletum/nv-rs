//! `BSShader/geometrydecalshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The two `NiPointer`s that keep this shader's declarations (`012021e0`, `012021e4`).
const DECLARATION_POINTER_012021E0: u32 = 0x0120_21e0;
const DECLARATION_POINTER_012021E4: u32 = 0x0120_21e4;
/// `GeometryDecalShader::GeometryDecalShader` (`00bcbf00`) and
/// `GeometryDecalShader::LoadShaders` (`00bcb4d0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bc_bf00;
const LOAD_SHADERS: u32 = 0x00bc_b4d0;
/// `00e804d0` (`cdecl`, one word: the address of a word of the shader). The engine map names
/// it `GeometryDecalShader::LoadStagesAndPasses`; the call here does not fit that name, so it
/// is described by address.
const CALL_00E804D0: u32 = 0x00e8_04d0;

/// The `NiPointer` assignment of a global the code inlines: nothing when it already holds
/// `object`; else the old object is released (count at +4 through `InterlockedDecrement`,
/// deleted through vtable slot +4 at 0) when not null, the global set and the new object
/// held (`InterlockedIncrement` on its +4) when not null.
fn assign_pointer(e: &mut Engine, global: u32, object: u32) {
    let old = e.global::<u32>(global);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.set_global(global, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00bcc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x10. No arguments. Makes two declarations
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`: 2 entries, then 4, 1
/// stream each), each assigned at once to its `NiPointer` (`012021e0`, `012021e4`). Through
/// those pointers (read again for each call): the first's entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0)) and slot +0xb4, the second's
/// ((0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0)) and slot +0xb4. Then
/// allocates 0x90 bytes and runs the constructor (`00bcbf00`; a null allocation is used as it
/// is), calls the shader's slot +0x4c, `LoadShaders` (`00bcb4d0`), `00e804d0` on the
/// shader's +0x6c and +0x70, slots +0x13c and +0x11c, then +0xc0 with the first declaration
/// (read from `012021e0`), and answers the shader.
// Translation incomplete: 00bcc0a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn geometry_decal_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    assign_pointer(e, DECLARATION_POINTER_012021E0, first);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    assign_pointer(e, DECLARATION_POINTER_012021E4, second);
    for entry in [[0u32, 0, 0, 2, 0], [1, 3, 3, 2, 0]] {
        let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
        e.vcall(declaration, 0x8c, &entry);
    }
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
    e.vcall(declaration, 0xb4, &args![]);
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 1, 1, 3, 0],
        [2, 2, 2, 4, 0],
        [3, 3, 3, 2, 0],
    ] {
        let declaration = e.global::<u32>(DECLARATION_POINTER_012021E4);
        e.vcall(declaration, 0x8c, &entry);
    }
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E4);
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    for word in [0x6cu32, 0x70] {
        e.call(CALL_00E804D0, &args![shader.wrapping_add(word)]);
    }
    e.vcall(shader, 0x13c, &args![]);
    e.vcall(shader, 0x11c, &args![]);
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bc_c0a0,
        geometry_decal_shader_create_shader() -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    const OLD: u32 = 0x0f90_0100;
    const OLD_DELETE: u32 = 0x0f90_1004;

    /// The rig, with the two pointers mapped (the first holding `OLD`, held once) and the
    /// interlocked imports changing the counts.
    fn decal_rig() -> Engine {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, CALL_00E804D0]);
        e.map(0x0120_2000, 0x1000);
        e.put_vtable(0x0f90_0000, &[0, OLD_DELETE]);
        e.mem.set_u32(OLD, 0x0f90_0000);
        e.mem.set_u32(OLD + 4, 1);
        e.set_global(DECLARATION_POINTER_012021E0, OLD);
        e.register(OLD_DELETE, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e
    }

    #[test]
    fn geometry_decal_shader_create_shader_keeps_both_declarations() {
        let mut e = decal_rig();
        let (shader, log) = run(&mut e, 0x00bc_c0a0);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (IMPORT_INTERLOCKED_DECREMENT, vec![OLD + 4]),
                (OLD_DELETE, vec![OLD]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![a + 4]),
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![b + 4]),
                (declaration_slot(0x8c), vec![a, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![a, 1, 3, 3, 2, 0]),
                (declaration_slot(0xb4), vec![a]),
                (declaration_slot(0x8c), vec![b, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![b, 1, 1, 1, 3, 0]),
                (declaration_slot(0x8c), vec![b, 2, 2, 2, 4, 0]),
                (declaration_slot(0x8c), vec![b, 3, 3, 3, 2, 0]),
                (declaration_slot(0xb4), vec![b]),
                (NI_ALLOCATE, vec![0x90]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (CALL_00E804D0, vec![SHADER + 0x6c]),
                (CALL_00E804D0, vec![SHADER + 0x70]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, a]),
            ]
        );
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E0), a);
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E4), b);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(e.mem.u32(b + 4), 1);
    }

    #[test]
    fn assign_pointer_keeps_the_same_object() {
        let mut e = decal_rig();
        e.call_log = Some(vec![]);
        assign_pointer(&mut e, DECLARATION_POINTER_012021E0, OLD);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.mem.u32(OLD + 4), 1);
        // A null object releases the old one (still held once more here) and holds nothing.
        e.mem.set_u32(OLD + 4, 2);
        assign_pointer(&mut e, DECLARATION_POINTER_012021E0, 0);
        assert_eq!(e.mem.u32(OLD + 4), 1);
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E0), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
