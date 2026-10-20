//! `BSShader/run_00bd5d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00bd6ec0` (`run_00bd5d70`, unnamed; `thiscall`, no stack words), the
/// `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word), the set-up
/// `00bd6ae0` (unnamed; `thiscall` on +0x58, one word) and `00b8bc20` (unit
/// `imagespacemanager.cpp`, unnamed; `thiscall`, no stack words).
const BASE_CONSTRUCT_00BD6EC0: u32 = 0x00bd_6ec0;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const SET_UP_00BD6AE0: u32 = 0x00bd_6ae0;
const CALL_00B8BC20: u32 = 0x00b8_bc20;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// This effect's vtable (`010bc410`; no RTTI locator before it).
const VTABLE_010BC410: u32 = 0x010b_c410;

// Translated from 00bd7120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00bd6ec0`, stores the vtable
/// `010bc410`, clears the bytes +0x7c to +0x7e and the words +0x80 to +0x88, constructs the
/// three 16-byte textures at +0x8c (`eh vector constructor iterator` with `004ebb70` and
/// `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 6 (`00a90180`),
/// stores 0.0 at +0x68 to +0x70, clears +0x74, runs `00bd6ae0` on +0x58 with 4 and `00b8bc20`.
/// Answers `this`.
// Translation incomplete: 00bd7120 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00bd7120(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00BD6EC0, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC410);
    for offset in [0x7cu32, 0x7d, 0x7e] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x80u32, 0x84, 0x88] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x8c,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 6u32]);
    for offset in [0x68u32, 0x6c, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_UP_00BD6AE0, &args![base + 0x58, 4u32]);
    e.call(CALL_00B8BC20, &args![base]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bd_7120, fn_00bd7120(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd7120_builds_the_effect() {
        let mut e = Engine::new();
        for target in [
            BASE_CONSTRUCT_00BD6EC0,
            ARRAY_SET_SIZE_00A90180,
            SET_UP_00BD6AE0,
            CALL_00B8BC20,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        e.mem.write(this, &[0xcc; 0xbc]);
        assert_eq!(e.call(0x00bd_7120, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(e.mem.bytes(this + 0x68, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.bytes(this + 0x7c, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x7f), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x80, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BASE_CONSTRUCT_00BD6EC0, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x8c,
                        0x10,
                        3,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 6]),
                (SET_UP_00BD6AE0, vec![this + 0x58, 4]),
                (CALL_00B8BC20, vec![this]),
            ]
        );
    }
}
