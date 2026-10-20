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

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bb230`; no RTTI locator before it).
const NO_LIGHTING_VTABLE_010BB230: u32 = 0x010b_b230;
/// The four words copied (`011a9be0` into `01202010`).
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const COLOR_01202010: u32 = 0x0120_2010;

// Translated from 00bc8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::BSShaderNoLighting` (Xbox PDB), `thiscall` with twelve stack words
/// (`RET 0x30`), the twelve declarations. Runs `BSShader::BSShader` (`00be2200`), stores the
/// vtable `010bb230`, constructs the `NiPointer` arrays at +0x6c (0x1e, destructor
/// `00b79930`), +0xe4 (0x1c) and +0x154 (7; destructor `00b66f20`), clears +0x184 to +0x1b0,
/// sets +0x68 to 0x16, assigns the declarations in order to the `NiPointer`s at +0x184 to
/// +0x1b0, copies the four words at `011a9be0` to `01202010` and clears +0x170. Answers
/// `this`.
// Translation incomplete: 00bc8f80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xf for the base and the arrays) is not translated.
pub fn bs_shader_no_lighting_bs_shader_no_lighting(
    e: &mut Engine,
    this: Ptr,
    declaration_0: Ptr,
    declaration_1: Ptr,
    declaration_2: Ptr,
    declaration_3: Ptr,
    declaration_4: Ptr,
    declaration_5: Ptr,
    declaration_6: Ptr,
    declaration_7: Ptr,
    declaration_8: Ptr,
    declaration_9: Ptr,
    declaration_10: Ptr,
    declaration_11: Ptr,
) -> Ptr {
    let declarations = [
        declaration_0,
        declaration_1,
        declaration_2,
        declaration_3,
        declaration_4,
        declaration_5,
        declaration_6,
        declaration_7,
        declaration_8,
        declaration_9,
        declaration_10,
        declaration_11,
    ];
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NO_LIGHTING_VTABLE_010BB230);
    construct_pointer_array(e, base + 0x6c, 0x1e, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0xe4, 0x1c, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x154, 7, DESTRUCT_00B66F20);
    for index in 0..12u32 {
        e.mem.set_u32(base + 0x184 + 4 * index, 0);
    }
    e.mem.set_u32(base + 0x68, 0x16);
    for (index, declaration) in declarations.iter().enumerate() {
        ni_pointer_assign(e, base + 0x184 + 4 * index as u32, declaration.addr());
    }
    for word in [0u32, 8, 4, 0xc] {
        let value = e.global::<u32>(COLOR_011A9BE0 + word);
        e.set_global::<u32>(COLOR_01202010 + word, value);
    }
    e.mem.set_u32(base + 0x170, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_8f80, bs_shader_no_lighting_bs_shader_no_lighting(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00bc_93c0,
            bs_shader_no_lighting_create_shader() -> Ptr
        ),
    ]
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

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_constructor_keeps_the_twelve_declarations() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BE0 + 4 * index, 0x60 + index);
        }
        let this = e.mem.alloc(0x1b4);
        e.mem.write(this, &[0xcc; 0x1b4]);
        let declarations: Vec<u32> = (0..12).map(|_| object(&mut e, 0)).collect();
        let mut words = vec![this];
        words.extend_from_slice(&declarations);
        assert_eq!(e.call(CONSTRUCT, &words).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NO_LIGHTING_VTABLE_010BB230);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x1e, DESTRUCT_00B79930),
                pointer_array(this + 0xe4, 0x1c, DESTRUCT_00B66F20),
                pointer_array(this + 0x154, 7, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0x16);
        for (index, declaration) in declarations.iter().enumerate() {
            assert_eq!(e.mem.u32(this + 0x184 + 4 * index as u32), *declaration);
            assert_eq!(e.mem.u32(declaration + 4), 1);
        }
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(COLOR_01202010 + 4 * index), 0x60 + index);
        }
        assert_eq!(e.mem.u32(this + 0x170), 0);
    }
}
