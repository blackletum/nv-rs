//! `NiXenonRenderer/nixenonrenderer.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! On the PC this unit holds `NiDX9Renderer`'s start-up: loading `D3D9.DLL` and its two creation
//! functions, the `IDirect3D9` and the adapter list made from it.

#[allow(unused_imports)]
use crate::prelude::*;

/// `LoadLibraryA` and `GetProcAddress` (KERNEL32 import slots).
const LOAD_LIBRARY_A: u32 = 0x00fd_f0b0;
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// The strings `"D3D9.DLL"`, `"Direct3DCreate9Ex"` and `"Direct3DCreate9"`.
const D3D9_DLL_NAME: u32 = 0x010e_d520;
const CREATE9EX_NAME: u32 = 0x010e_d50c;
const CREATE9_NAME: u32 = 0x010e_d4fc;
/// The module handle and the two procedures `fn_00e69410` keeps.
const D3D9_MODULE_0126F0D4: u32 = 0x0126_f0d4;
const CREATE9_0126F0DC: u32 = 0x0126_f0dc;
const CREATE9EX_0126F0E0: u32 = 0x0126_f0e0;
/// The adapter list `fn_00e69a60` makes once (`0126f0e8`) and the word handed to it
/// (`0126f0f8`).
const ADAPTER_LIST_0126F0E8: u32 = 0x0126_f0e8;
const ADAPTER_LIST_WORD_0126F0F8: u32 = 0x0126_f0f8;
/// `D3D_SDK_VERSION` as the game passes it to `Direct3DCreate9`.
const D3D_SDK_VERSION: u32 = 0x20;
/// The Gamebryo allocator (`00aa13e0`, `cdecl`, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The adapter list's array of adapter descriptions: its vtable (`010eeca4`) and its
/// `SetSize` (`00a90180`, `thiscall`, one word).
const ADAPTER_ARRAY_VTABLE: u32 = 0x010e_eca4;
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// An adapter description's constructor (`00e7b7d0`, `thiscall`: the `IDirect3D9`, the
/// adapter index, the word given to the list; 0x468 bytes).
const ADAPTER_DESC_CONSTRUCT: u32 = 0x00e7_b7d0;
const ADAPTER_DESC_SIZE: u32 = 0x468;

// Translated from 00e69410 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words; answers `AL`): true at once when
/// `Direct3DCreate9` was found before (`0126f0dc`). Otherwise loads `D3D9.DLL` (`LoadLibraryA`,
/// kept at `0126f0d4`; false when that fails), keeps `GetProcAddress(module,
/// "Direct3DCreate9Ex")` at `0126f0e0` and `GetProcAddress(module, "Direct3DCreate9")` at
/// `0126f0dc`, and answers whether the latter was found.
pub fn fn_00e69410(e: &mut Engine) -> bool {
    if e.mem.u32(CREATE9_0126F0DC) != 0 {
        return true;
    }
    let module = e.call(LOAD_LIBRARY_A, &args![D3D9_DLL_NAME]).u32();
    e.mem.set_u32(D3D9_MODULE_0126F0D4, module);
    if module == 0 {
        return false;
    }
    let create_ex = e
        .call(GET_PROC_ADDRESS, &args![module, CREATE9EX_NAME])
        .u32();
    e.mem.set_u32(CREATE9EX_0126F0E0, create_ex);
    let module = e.mem.u32(D3D9_MODULE_0126F0D4);
    let create = e.call(GET_PROC_ADDRESS, &args![module, CREATE9_NAME]).u32();
    e.mem.set_u32(CREATE9_0126F0DC, create);
    create != 0
}

// Translated from 00e69a60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words; answers `EAX`): the renderer's adapter list,
/// made once (`0126f0e8`). When it isn't made yet and `fn_00e69410` finds `Direct3DCreate9`,
/// creates an `IDirect3D9` (`Direct3DCreate9(0x20)` through `0126f0dc`); with one, allocates the
/// 0x14-byte list (`00aa13e0`), builds it with `fn_00e7bc90(d3d, 0126f0f8)` into `0126f0e8` (0
/// when the allocation failed) and releases the `IDirect3D9` (its slot +8). Answers `0126f0e8`.
pub fn fn_00e69a60(e: &mut Engine) -> u32 {
    if e.mem.u32(ADAPTER_LIST_0126F0E8) != 0 {
        return e.mem.u32(ADAPTER_LIST_0126F0E8);
    }
    if !fn_00e69410(e) {
        return e.mem.u32(ADAPTER_LIST_0126F0E8);
    }
    let create = e.mem.u32(CREATE9_0126F0DC);
    let d3d = e.call(create, &args![D3D_SDK_VERSION]).u32();
    if d3d == 0 {
        return e.mem.u32(ADAPTER_LIST_0126F0E8);
    }
    let block = e.call(NI_ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        e.mem.set_u32(ADAPTER_LIST_0126F0E8, 0);
        e.vcall(d3d, 8, &args![]);
        return e.mem.u32(ADAPTER_LIST_0126F0E8);
    }
    let list = e
        .call(0x00e7_bc90, &args![block, d3d, ADAPTER_LIST_WORD_0126F0F8])
        .u32();
    e.mem.set_u32(ADAPTER_LIST_0126F0E8, list);
    e.vcall(d3d, 8, &args![]);
    e.mem.u32(ADAPTER_LIST_0126F0E8)
}

// Translated from 00e7bc90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`; the adapter list's constructor):
/// clears the adapter count (+0), sets up the array at +4 (vtable `010eeca4`, data +8 null, the
/// 16-bit size +0xc, maximum +0xe and used count +0x10 zero, grow-by +0x12 one), fills it with
/// `fn_00e7bbf0(d3d, word)` and answers `this`.
pub fn fn_00e7bc90(e: &mut Engine, this: u32, d3d: u32, word: u32) -> u32 {
    e.mem.set_u16(this + 0x12, 1);
    e.mem.set_u16(this + 0xe, 0);
    e.mem.set_u16(this + 0xc, 0);
    e.mem.set_u16(this + 0x10, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 4, ADAPTER_ARRAY_VTABLE);
    e.mem.set_u32(this, 0);
    e.call(0x00e7_bbf0, &args![this, d3d, word]);
    this
}

// Translated from 00e7bbf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`): the adapter count from the
/// `IDirect3D9` (slot +0x10, `GetAdapterCount`) into +0, the array at +4 sized to it
/// (`00a90180`), then for each adapter a 0x468-byte description (`00aa13e0`, built by
/// `00e7b7d0(d3d, index, word)`, 0 when the allocation failed) stored at its index: at or past
/// the array's maximum (+0xe) the maximum becomes index + 1 and a description raises the used
/// count (+0x10); below it the used count rises when a description replaces null and falls
/// when null replaces one (16-bit, wrapping).
pub fn fn_00e7bbf0(e: &mut Engine, this: u32, d3d: u32, word: u32) {
    let count = e.vcall(d3d, 0x10, &args![]).u32();
    let array = this + 4;
    e.mem.set_u32(this, count);
    e.call(ARRAY_SET_SIZE, &args![array, count]);
    let mut index = 0u32;
    while index < e.mem.u32(this) {
        let block = e.call(NI_ALLOCATE, &args![ADAPTER_DESC_SIZE]).u32();
        let desc = if block == 0 {
            0
        } else {
            e.call(ADAPTER_DESC_CONSTRUCT, &args![block, d3d, index, word])
                .u32()
        };
        let maximum = u32::from(e.mem.u16(array + 0xa));
        let used = array + 0xc;
        if index >= maximum {
            e.mem.set_u16(array + 0xa, (index + 1) as u16);
            if desc != 0 {
                e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
            }
        } else {
            let held = e.mem.u32(e.mem.u32(array + 4) + index * 4);
            if desc != 0 {
                if held == 0 {
                    e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
                }
            } else if held != 0 {
                e.mem.set_u16(used, e.mem.u16(used).wrapping_add(0xffff));
            }
        }
        let slot = e.mem.u32(array + 4) + index * 4;
        e.mem.set_u32(slot, desc);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_9410, fn_00e69410() -> bool),
        entry!(0x00e6_9a60, fn_00e69a60() -> u32),
        entry!(0x00e7_bc90, fn_00e7bc90(u32, u32, u32) -> u32),
        entry!(0x00e7_bbf0, fn_00e7bbf0(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(LOAD_LIBRARY_A, |_, _| 0x7e11_0000u32.into_ret());
        e.register(GET_PROC_ADDRESS, |_, a| {
            (if a[1] == CREATE9_NAME {
                0x0e00_0000u32
            } else {
                0x0e00_0004
            })
            .into_ret()
        });
        e
    }

    #[test]
    fn fn_00e69410_loads_the_dll_and_both_procedures_once() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9410, &[]).bool());
        assert_eq!(e.mem.u32(D3D9_MODULE_0126F0D4), 0x7e11_0000);
        assert_eq!(e.mem.u32(CREATE9_0126F0DC), 0x0e00_0000);
        assert_eq!(e.mem.u32(CREATE9EX_0126F0E0), 0x0e00_0004);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOAD_LIBRARY_A), vec![vec![D3D9_DLL_NAME]]);
        assert_eq!(
            calls(&log, GET_PROC_ADDRESS),
            vec![
                vec![0x7e11_0000, CREATE9EX_NAME],
                vec![0x7e11_0000, CREATE9_NAME]
            ]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9410, &[]).bool());
        assert!(calls(&e.call_log.take().unwrap(), LOAD_LIBRARY_A).is_empty());
    }

    #[test]
    fn fn_00e69410_without_the_dll_is_false() {
        let mut e = engine();
        e.register(LOAD_LIBRARY_A, |_, _| Ret::default());
        assert!(!e.call(0x00e6_9410, &[]).bool());
        assert_eq!(e.mem.u32(CREATE9_0126F0DC), 0);
    }

    #[test]
    fn fn_00e69a60_makes_the_list_once_and_releases_the_d3d_object() {
        let mut e = engine();
        let d3d = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(d3d, vtable);
        e.mem.set_u32(vtable + 8, 0x0e00_0100);
        e.register_double(0x0e00_0000, move |_, a| {
            assert_eq!(a[0], D3D_SDK_VERSION);
            d3d.into_ret()
        });
        e.register(0x0e00_0100, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00e7_bc90, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let list = e.call(0x00e6_9a60, &[]).u32();
        assert_ne!(list, 0);
        assert_eq!(e.mem.u32(ADAPTER_LIST_0126F0E8), list);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x00e7_bc90),
            vec![vec![list, d3d, ADAPTER_LIST_WORD_0126F0F8]]
        );
        assert_eq!(calls(&log, 0x0e00_0100), vec![vec![d3d]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9a60, &[]).u32(), list);
        assert!(calls(&e.call_log.take().unwrap(), 0x0e00_0000).is_empty());
    }

    #[test]
    fn fn_00e7bc90_fills_one_description_per_adapter() {
        let mut e = Engine::new();
        let d3d = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(d3d, vtable);
        e.mem.set_u32(vtable + 0x10, 0x0e00_0200);
        e.register(0x0e00_0200, |_, _| 2u32.into_ret());
        e.register(ARRAY_SET_SIZE, |e, a| {
            let data = e.mem.alloc(4 * a[1].max(1));
            e.mem.set_u32(a[0] + 4, data);
            Ret::default()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ADAPTER_DESC_CONSTRUCT, |_, a| a[0].into_ret());
        let list = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_bc90, &args![list, d3d, 7u32]).u32(), list);
        assert_eq!(e.mem.u32(list), 2);
        assert_eq!(e.mem.u32(list + 4), ADAPTER_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(list + 0xe), 2, "the maximum");
        assert_eq!(e.mem.u16(list + 0x10), 2, "used");
        assert_eq!(e.mem.u16(list + 0x12), 1, "grow-by");
        let log = e.call_log.take().unwrap();
        let made = calls(&log, ADAPTER_DESC_CONSTRUCT);
        assert_eq!(made.len(), 2);
        assert_eq!(made[1][1..], [d3d, 1, 7]);
        let data = e.mem.u32(list + 8);
        assert_eq!(e.mem.u32(data + 4), made[1][0]);
    }
}
