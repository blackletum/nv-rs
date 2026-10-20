//! `NiMain/niddsreader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual tables the constructor stores (RTTI `NiImageReader`, then `NiDDSReader`).
const NI_IMAGE_READER_VTABLE: u32 = 0x0109_f3d0;
const NI_DDS_READER_VTABLE: u32 = 0x0109_f48c;
/// `InitializeCriticalSection` (import slot) and `NiPixelFormat::NiPixelFormat` (engine map,
/// `00a7cca0`; `thiscall`, no stack words).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
/// The pixel format copied into the reader (0x11 words at `011aa498`) and where it goes.
const PIXEL_FORMAT_SOURCE_011AA498: u32 = 0x011a_a498;
const PIXEL_FORMAT: u32 = 0x110;
const PIXEL_FORMAT_WORDS: u32 = 0x11;

// Translated from 00a8c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`niddsreader.cpp`): the `NiDDSReader` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The `NiImageReader` virtual
/// table, its critical section at +0x80 (`InitializeCriticalSection`), the `NiDDSReader`
/// virtual table, the pixel format at +0x110 (`NiPixelFormat::NiPixelFormat`), +0x100 and
/// +0x104 cleared, then the 0x11 words at `011aa498` copied over the pixel format (`REP
/// MOVSD`). Answers `this`.
pub fn fn_00a8c190(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_IMAGE_READER_VTABLE);
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base, NI_DDS_READER_VTABLE);
    let format = base.wrapping_add(PIXEL_FORMAT);
    e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
    e.mem.set_u32(base.wrapping_add(0x100), 0);
    e.mem.set_u32(base.wrapping_add(0x104), 0);
    for word in 0..PIXEL_FORMAT_WORDS {
        let value = e.mem.u32(PIXEL_FORMAT_SOURCE_011AA498 + 4 * word);
        e.mem.set_u32(format.wrapping_add(4 * word), value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_c190, fn_00a8c190(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    #[test]
    fn fn_00a8c190_builds_the_dds_reader() {
        let mut e = Engine::new();
        e.map(PIXEL_FORMAT_SOURCE_011AA498, 4 * PIXEL_FORMAT_WORDS);
        for word in 0..PIXEL_FORMAT_WORDS {
            e.mem
                .set_u32(PIXEL_FORMAT_SOURCE_011AA498 + 4 * word, 0x100 + word);
        }
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x100, 7);
        assert_eq!(e.call(0x00a8_c190, &args![this]).u32(), this);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        assert!(log.contains(&(NI_PIXEL_FORMAT_CONSTRUCT, vec![this + PIXEL_FORMAT])));
        assert_eq!(e.mem.u32(this), NI_DDS_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        for word in 0..PIXEL_FORMAT_WORDS {
            assert_eq!(e.mem.u32(this + PIXEL_FORMAT + 4 * word), 0x100 + word);
        }
    }
}
