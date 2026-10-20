//! `NiXenonRenderer/nid3dshaderprogram.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRefObject`'s vtable (`0101dce4`), its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`, raised through the imported `InterlockedIncrement`), `NiD3DShaderProgram`'s
/// vtable (`010ef73c`, RTTI `.?AVNiD3DShaderProgram@@`) and
/// `NiD3DShaderProgram::SetD3DRenderer` (`00e89760`, `thiscall`, one word).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const SHADER_PROGRAM_VTABLE_010EF73C: u32 = 0x010e_f73c;
const SET_D3D_RENDERER_00E89760: u32 = 0x00e8_9760;

// Translated from 00e897d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::NiD3DShaderProgram` (engine map), `thiscall` with two stack words: the
/// renderer and the program type. The `NiRefObject` part (vtable `0101dce4`, count +4 cleared,
/// `ms_uiObjects` at `011f4400` raised), the program type at +8, the vtable `010ef73c`, the words
/// +0xc to +0x28 cleared, then `SetD3DRenderer` with the renderer. Answers `this`.
pub fn ni_d3d_shader_program_ni_d3d_shader_program(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    program_type: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    e.mem.set_u32(base + 8, program_type);
    e.mem.set_u32(base, SHADER_PROGRAM_VTABLE_010EF73C);
    for offset in (0xcu32..=0x28).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_D3D_RENDERER_00E89760, &args![base, renderer]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e8_97d0,
        ni_d3d_shader_program_ni_d3d_shader_program(Ptr, Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_program_constructor_sets_up_the_program() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(SET_D3D_RENDERER_00E89760, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        let answer = e
            .call(0x00e8_97d0, &args![this, 0x0e00_0000u32, 1u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), SHADER_PROGRAM_VTABLE_010EF73C);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.bytes(this + 0xc, 0x20), vec![0; 0x20]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (
                    IMPORT_INTERLOCKED_INCREMENT,
                    vec![NI_REF_OBJECT_COUNT_011F4400]
                ),
                (SET_D3D_RENDERER_00E89760, vec![this, 0x0e00_0000]),
            ]
        );
    }
}
