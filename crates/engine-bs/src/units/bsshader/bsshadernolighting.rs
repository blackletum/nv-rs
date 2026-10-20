//! `BSShader/bsshadernolighting.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderNoLighting::BSShaderNoLighting` (`00bc8f80`, `thiscall`, `RET 0x30`: twelve
/// declarations), `::LoadShaders` (`00bc5c40`) and `::LoadStagesAndPasses` (`00bc77e0`; both
/// `thiscall`, no words) (engine map).
const CONSTRUCT: u32 = 0x00bc_8f80;
const LOAD_SHADERS: u32 = 0x00bc_5c40;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bc_77e0;

/// Each declaration's entry count as made (`Create`'s second word) and the entries it gets
/// through vtable slot +0x8c (five words each), in the exe's order.
const DECLARATIONS: [(u32, &[[u32; 5]]); 12] = [
    (2, &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]]),
    (2, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]]),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 3, 2, 0],
            [2, 5, 4, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
        ],
    ),
    (
        6,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
            [5, 7, 5, 1, 0],
        ],
    ),
];

// Translated from 00bc93c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x21. No arguments. Makes twelve declarations, one after the other
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508` read each time, entry
/// counts 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 1 stream each), then in the same order sets
/// each one's entries through its vtable slot +0x8c (the lists of `DECLARATIONS`, read from
/// the pushes) and ends it (slot +0xb4). Allocates 0x1b4 bytes and runs the constructor
/// (`00bc8f80`) with the twelve declarations in order (a null allocation is used as it is),
/// calls the shader's slot +0x4c, `LoadShaders` (`00bc5c40`), `LoadStagesAndPasses`
/// (`00bc77e0`), slot +0x11c, then +0xc0 with the first declaration, and answers the shader.
// Translation incomplete: 00bc93c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_no_lighting_create_shader(e: &mut Engine) -> Ptr {
    let mut made = Vec::with_capacity(DECLARATIONS.len());
    for (count, _) in DECLARATIONS {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        made.push(
            e.call(DECLARATION_CREATE, &args![renderer, count, 1u32])
                .u32(),
        );
    }
    for (declaration, (_, entries)) in made.iter().zip(DECLARATIONS) {
        for entry in entries {
            e.vcall(*declaration, 0x8c, entry);
        }
        e.vcall(*declaration, 0xb4, &args![]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x1b4u32]).u32();
    let shader = if block != 0 {
        let mut words = vec![block];
        words.extend_from_slice(&made);
        e.call(CONSTRUCT, &words).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![made[0]]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bc_93c0,
        bs_shader_no_lighting_create_shader() -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_no_lighting_create_shader_makes_twelve_declarations_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bc_93c0);
        assert_eq!(shader, SHADER);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let mut want: Vec<(u32, Vec<u32>)> = [2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        // The entries as pushed in the exe, declaration by declaration.
        let lists: [&[[u32; 5]]; 12] = [
            &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 3, 2, 0],
                [2, 5, 4, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
                [5, 7, 5, 1, 0],
            ],
        ];
        for (n, list) in lists.iter().enumerate() {
            let declaration = d(n as u32);
            for entry in list.iter() {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                want.push((declaration_slot(0x8c), words));
            }
            want.push((declaration_slot(0xb4), vec![declaration]));
        }
        want.push((NI_ALLOCATE, vec![0x1b4]));
        let mut construct = vec![SHADER];
        construct.extend((0..12).map(d));
        want.push((CONSTRUCT, construct));
        want.push((shader_slot(0x4c), vec![SHADER]));
        want.push((LOAD_SHADERS, vec![SHADER]));
        want.push((LOAD_STAGES_AND_PASSES, vec![SHADER]));
        want.push((shader_slot(0x11c), vec![SHADER]));
        want.push((shader_slot(0xc0), vec![SHADER, d(0)]));
        assert_eq!(log, want);
    }
}
