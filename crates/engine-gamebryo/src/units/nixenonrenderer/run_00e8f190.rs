//! `NiXenonRenderer/run_00e8f190` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRefObject` virtual table and the global object count it counts up (`011f4400`), the
/// interlocked imports, the bucket allocation `00aa1070`, `memset`, the map set-up `00b601a0`
/// (unnamed; `thiscall`, no stack words) and the allocation `00aa13e0` (`cdecl`, size).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const MAP_00B601A0: u32 = 0x00b6_01a0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// The class virtual tables these constructors store (RTTI `NiDX9IndexBufferManager`, its
/// maps' `NiTMapBase<..., unsigned int, NiDX9IndexBufferManager::NiDX9IBInfo *>` and
/// `NiTPointerMap<...>`, `NiDX9TextureManager`, `NiDX9ErrorShader`, `NiDX9RenderState`).
const NI_DX9_INDEX_BUFFER_MANAGER_VTABLE: u32 = 0x010f_02f0;
const IB_MAP_BASE_VTABLE: u32 = 0x010f_02b0;
const IB_MAP_VTABLE: u32 = 0x010f_02d0;
const NI_DX9_TEXTURE_MANAGER_VTABLE: u32 = 0x010f_086c;
const NI_DX9_ERROR_SHADER_VTABLE: u32 = 0x010f_0764;
const NI_DX9_RENDER_STATE_VTABLE: u32 = 0x010f_088c;
/// The renderer's device word (+0x288).
const RENDERER_DEVICE: u32 = 0x288;

/// The `NiRefObject` start every one of these objects makes: its table, count 0, the global
/// object count up.
fn ref_object_start(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e8fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` constructor (the
/// RTTI of the virtual table it stores), `thiscall` with the device. The `NiRefObject` start,
/// the class's table, three maps at +0x1c, +0x2c, +0x3c (0x25 buckets each, cleared), the
/// device at +8 (counted: its slot 4), +0xc to +0x18 cleared, then `00b601a0` on each map.
/// Answers `this`.
pub fn fn_00e8fb20(e: &mut Engine, this: Ptr, device: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
    for offset in [0x1cu32, 0x2c, 0x3c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, IB_MAP_BASE_VTABLE);
        e.mem.set_u32(map.wrapping_add(4), 0x25);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, IB_MAP_VTABLE);
    }
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    for offset in [0xcu32, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in [0x1cu32, 0x2c, 0x3c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    this
}

// Translated from 00e90a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9TextureManager` constructor (the RTTI
/// of the virtual table it stores), `thiscall` with the renderer. The `NiRefObject` start, the
/// class's table, the renderer at +0xc and its device (+0x288) at +8, counted (its slot 4).
/// Answers `this`.
pub fn fn_00e90a80(e: &mut Engine, this: Ptr, renderer: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_TEXTURE_MANAGER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0xc), renderer);
    let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    this
}

/// The shader base constructor (`00e818a0`, unnamed; `thiscall`, no stack words), the error
/// shader's set-up (`00e90850`, unnamed; `thiscall`, no stack words), the fixed-string maker
/// `00a5b690` (`cdecl`, a C string), the name setter `00a5b950` (unnamed; `thiscall`, the
/// address of a fixed string), the empty fixed string (`0109b220`) and the name
/// `"NiDX9ErrorShader"` (`010f0854`).
const SHADER_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const ERROR_SHADER_SET_UP_00E90850: u32 = 0x00e9_0850;
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
const ERROR_SHADER_NAME_010F0854: u32 = 0x010f_0854;

// Translated from 00e909e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The shader base constructor, the
/// bytes +0x24 and +0x25 and the words +0x28 to +0x38 cleared, the class's table, `00e90850`,
/// the name `"NiDX9ErrorShader"` made as a fixed string (in a stack word), set (`00a5b950`) and
/// released (its count at -8 down unless it is the empty string), then +0x20 set to 1.
/// Answers `this`.
pub fn fn_00e909e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base.wrapping_add(0x24), 0);
    e.mem.set_u8(base.wrapping_add(0x25), 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base, NI_DX9_ERROR_SHADER_VTABLE);
    e.call(ERROR_SHADER_SET_UP_00E90850, &args![base]);
    e.with_stack(4, |e, name| {
        let handle = e
            .call(FIXED_STRING_MAKE, &args![ERROR_SHADER_NAME_010F0854])
            .u32();
        e.mem.set_u32(name.addr(), handle);
        e.call(SET_NAME_00A5B950, &args![base, name]);
        let handle = e.mem.u32(name.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    e.mem.set_u8(base.wrapping_add(0x20), 1);
    this
}

/// The render state's base constructor (`00e881a0`, unnamed; `thiscall`, the renderer),
/// `memcpy` (`00ec44d0`), the table of (state, value) word pairs it adjusts (`011bfce8`, ended
/// by state -1, 256 pairs at most), and the object `00e978a0` makes (unnamed; `cdecl`:
/// renderer, caps).
const RENDER_STATE_CONSTRUCT_00E881A0: u32 = 0x00e8_81a0;
const MEMCPY: u32 = 0x00ec_44d0;
const STATE_PAIRS_011BFCE8: u32 = 0x011b_fce8;
const MAKE_00E978A0: u32 = 0x00e9_78a0;

// Translated from 00e91590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): makes the `NiDX9RenderState` (0x1248 bytes; the
/// RTTI of the virtual table it stores), `cdecl`: the renderer, its caps and a byte. A made
/// object gets the base constructor (`00e881a0` with the renderer), its table, the byte
/// +0x1114 cleared and the 0x130 caps bytes copied to +0x1118; null when the allocation fails
/// (its slot 0x104 still runs). Then its virtual slot 0x104; in the pairs at `011bfce8` (until
/// state -1, at most 256), state 7's value becomes 1 when the byte is set, else 0; with the
/// byte set +8 gets bit 1. The object `00e978a0` makes is held at +0x10f0 (`NiPointer`
/// assignment: the old one released, the new one counted). Answers the render state.
pub fn fn_00e91590(e: &mut Engine, renderer: u32, caps: u32, flag: u8) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1248u32]).u32();
    let state = if block != 0 {
        e.call(RENDER_STATE_CONSTRUCT_00E881A0, &args![block, renderer]);
        e.mem.set_u32(block, NI_DX9_RENDER_STATE_VTABLE);
        e.mem.set_u8(block + 0x1114, 0);
        e.call(MEMCPY, &args![block + 0x1118, caps, 0x130u32]);
        block
    } else {
        0
    };
    e.vcall(state, 0x104, &args![]);
    for pair in 0..0x100u32 {
        let at = STATE_PAIRS_011BFCE8 + 8 * pair;
        let id = e.mem.u32(at);
        if id == 0xffff_ffff {
            break;
        }
        if id == 7 {
            e.mem.set_u32(at + 4, u32::from(flag != 0));
        }
    }
    if flag != 0 {
        let flags = e.mem.u32(state.wrapping_add(8)) | 2;
        e.mem.set_u32(state.wrapping_add(8), flags);
    }
    let made = e.call(MAKE_00E978A0, &args![renderer, caps]).u32();
    let slot = state.wrapping_add(0x10f0);
    let held = e.mem.u32(slot);
    if held != made {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(slot, made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    state
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_fb20, fn_00e8fb20(Ptr, u32) -> Ptr),
        entry!(0x00e9_0a80, fn_00e90a80(Ptr, u32) -> Ptr),
        entry!(0x00e9_09e0, fn_00e909e0(Ptr) -> Ptr),
        entry!(0x00e9_1590, fn_00e91590(u32, u32, u8) -> u32),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x6000;
    const ADD_REF: u32 = 0x6100;
    const DELETE: u32 = 0x6104;
    const STATE_SLOT_104: u32 = 0x6200;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose virtual table has `AddRef` at slot 4, delete at slot 4 for released
    /// objects (same slot), and the render state's slot 0x104.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x42];
        slots[1] = ADD_REF;
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(OBJECT_VTABLE, &slots);
        for no_op in [
            ADD_REF,
            DELETE,
            STATE_SLOT_104,
            IMPORT_INTERLOCKED_INCREMENT,
            MEMSET,
            MAP_00B601A0,
            SHADER_CONSTRUCT_00E818A0,
            ERROR_SHADER_SET_UP_00E90850,
            SET_NAME_00A5B950,
            MEMCPY,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, OBJECT_VTABLE);
        e.call_log = Some(vec![]);
        (e, device)
    }

    #[test]
    fn fn_00e8fb20_builds_the_index_buffer_manager() {
        let (mut e, device) = rig();
        let this = e.mem.alloc(0x50);
        e.call(0x00e8_fb20, &args![this, device]);
        assert_eq!(e.mem.u32(this), NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        for offset in [0x1c, 0x2c, 0x3c] {
            assert_eq!(e.mem.u32(this + offset), IB_MAP_VTABLE);
        }
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        assert_eq!(calls(&e, MAP_00B601A0).len(), 3);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
    }

    #[test]
    fn fn_00e90a80_keeps_the_renderer_and_its_device() {
        let (mut e, device) = rig();
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00e9_0a80, &args![this, renderer]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_TEXTURE_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(e.mem.u32(this + 0xc), renderer);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
    }

    #[test]
    fn fn_00e909e0_names_the_error_shader() {
        let (mut e, _) = rig();
        e.map(FIXED_STRING_EMPTY, 4);
        e.register(FIXED_STRING_MAKE, |_, _| 0x7008u32.into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e9_09e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_ERROR_SHADER_VTABLE);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE),
            vec![vec![ERROR_SHADER_NAME_010F0854]]
        );
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![0x7000]]);
        // The empty string is not released.
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x7008);
        e.call_log = Some(vec![]);
        e.call(0x00e9_09e0, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00e91590_makes_the_render_state() {
        let (mut e, _) = rig();
        let mut slots = vec![0u32; 0x42];
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(NI_DX9_RENDER_STATE_VTABLE, &slots);
        e.map(STATE_PAIRS_011BFCE8, 0x20);
        for (i, (id, value)) in [(3u32, 9u32), (7, 9), (0xffff_ffff, 0), (7, 9)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32, *id);
            e.mem
                .set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32 + 4, *value);
        }
        e.register(RENDER_STATE_CONSTRUCT_00E881A0, |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            a[0].into_ret()
        });
        let made = e.mem.alloc(8);
        e.register_double(MAKE_00E978A0, move |_, _| made.into_ret());
        let state = e
            .call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 1u32])
            .u32();
        assert_eq!(e.mem.u32(state), NI_DX9_RENDER_STATE_VTABLE);
        assert_eq!(calls(&e, MEMCPY), vec![vec![state + 0x1118, 0x5000, 0x130]]);
        assert_eq!(calls(&e, STATE_SLOT_104).len(), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0x1c), 9);
        assert_eq!(e.mem.u32(state + 8) & 2, 2);
        assert_eq!(e.mem.u32(state + 0x10f0), made);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT).last(),
            Some(&vec![made + 4])
        );
        // Without the byte, state 7 becomes 0.
        e.call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 0u32]);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 0);
    }
}
