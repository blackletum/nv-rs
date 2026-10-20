//! `NiXenonRenderer/run_00e7bcd0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size),
/// `NiPixelFormat::NiPixelFormat` (engine map, `00a7cca0`; `thiscall`, no stack words) and the
/// format filler `00e7c1e0` (unnamed; `cdecl`: Direct3D format, pixel format).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
const FILL_PIXEL_FORMAT_00E7C1E0: u32 = 0x00e7_c1e0;

// Translated from 00e7c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`), `cdecl`, a Direct3D format: makes an
/// `NiPixelFormat` (0x44 bytes from the memory manager; null when that fails), fills it from
/// the format (`00e7c1e0`, also given null) and answers it.
pub fn fn_00e7c500(e: &mut Engine, format: u32) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
        .u32();
    let pixel_format = if block != 0 {
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(FILL_PIXEL_FORMAT_00E7C1E0, &args![format, pixel_format]);
    pixel_format
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_c500, fn_00e7c500(u32) -> u32)]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e7c500_makes_and_fills_a_pixel_format() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FILL_PIXEL_FORMAT_00E7C1E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00e7_c500, &args![0x16u32]).u32();
        assert_ne!(made, 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(FILL_PIXEL_FORMAT_00E7C1E0, vec![0x16, made])));
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_c500, &args![0x16u32]).u32(), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(FILL_PIXEL_FORMAT_00E7C1E0, vec![0x16, 0])));
    }
}
