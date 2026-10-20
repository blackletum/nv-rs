//! `NiMain/nisourcecubemap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSourceCubeMap::ms_RTTI` (the class's `NiRTTI` record).
const NI_SOURCE_CUBE_MAP_RTTI: u32 = 0x011f_4454;

// Translated from 00a60cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_source_cube_map_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SOURCE_CUBE_MAP_RTTI)
}

/// `00a60600`: the jump target of `00a60870`.
const JUMP_TARGET_00A60870: u32 = 0x00a6_0600;

// Translated from 00a60870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::SaveBinary` (Xbox PDB): a 5-byte jump to `00a60600` (engine map: `NiSourceTexture::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiSourceCubeMap@@UAAXAAVNiStream@@@Z`.
pub fn ni_source_cube_map_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A60870, &args![this, stream]);
}

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiSourceTexture::NiSourceTexture` (engine map, `00a5fc10`, `thiscall`, no words).
const NI_SOURCE_TEXTURE_CONSTRUCT: u32 = 0x00a5_fc10;
/// `NiSourceCubeMap`'s vtable (`0109babc`, named by its RTTI locator).
const NI_SOURCE_CUBE_MAP_VTABLE: u32 = 0x0109_babc;
/// `NiSourceCubeMap::LoadFromFiles` (engine map, `00a60cc0`, `thiscall`, seven words: the
/// six file names and the search-path object).
const NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES: u32 = 0x00a6_0cc0;
/// The set-up `00aa24d0` and clean-up `00aa24f0` of the 0x210-byte stack object passed to
/// `LoadFromFiles` (both `thiscall`, no words; they set the vtable whose RTTI locator names
/// `NiSearchPath`).
const SEARCH_PATH_SET_UP_00AA24D0: u32 = 0x00aa_24d0;
const SEARCH_PATH_CLEAN_UP_00AA24F0: u32 = 0x00aa_24f0;
const SEARCH_PATH_BYTES: u32 = 0x210;

// Translated from 00a61120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nisourcecubemap.cpp`): makes an `NiSourceCubeMap`
/// from six files. `cdecl`, nine words: the six file names, the renderer, an unread word and
/// the address of three words of format preferences. Allocates 0x4c bytes (`00aa13e0`); when
/// that succeeds runs `NiSourceTexture::NiSourceTexture` (`00a5fc10`), sets the vtable
/// `0109babc` and clears +0x48 (a null allocation is used as it is). Copies the three
/// preference words to +0x18..+0x20, sets up a stack `NiSearchPath` (`00aa24d0`) and runs
/// `NiSourceCubeMap::LoadFromFiles` (`00a60cc0`) with the six names and it; clears +0x48 when
/// it is set. With a renderer and a non-zero +0x3c, the renderer's vtable slot +0x144 is
/// called with the map; when there is no renderer, or that answers true (`AL`), the map is
/// answered. Otherwise (no word at +0x3c, or the renderer answered false) the map is deleted
/// (slot 0 with 1) and 0 answered. The search path is cleaned up (`00aa24f0`) on both paths.
// Translation incomplete: 00a61120 - the /GS cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a61120(
    e: &mut Engine,
    name_0: Ptr,
    name_1: Ptr,
    name_2: Ptr,
    name_3: Ptr,
    name_4: Ptr,
    name_5: Ptr,
    renderer: Ptr,
    _unused_7: u32,
    preferences: Ptr,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let map = if block != 0 {
        e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_SOURCE_CUBE_MAP_VTABLE);
        e.mem.set_u32(block.wrapping_add(0x48), 0);
        block
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(preferences.addr().wrapping_add(4 * word));
        e.mem.set_u32(map.wrapping_add(0x18 + 4 * word), value);
    }
    e.with_stack(SEARCH_PATH_BYTES, |e, search_path| {
        e.call(SEARCH_PATH_SET_UP_00AA24D0, &args![search_path]);
        e.call(
            NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES,
            &args![
                map,
                name_0,
                name_1,
                name_2,
                name_3,
                name_4,
                name_5,
                search_path
            ],
        );
        if e.mem.u32(map.wrapping_add(0x48)) != 0 {
            e.mem.set_u32(map.wrapping_add(0x48), 0);
        }
        let keep = renderer.is_null()
            || (e.mem.u32(map.wrapping_add(0x3c)) != 0
                && e.vcall(renderer.addr(), 0x144, &args![map]).bool());
        if keep {
            e.call(SEARCH_PATH_CLEAN_UP_00AA24F0, &args![search_path]);
            Ptr::new(map)
        } else {
            e.vcall(map, 0, &args![1u32]);
            e.call(SEARCH_PATH_CLEAN_UP_00AA24F0, &args![search_path]);
            Ptr::NULL
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6_1120,
            fn_00a61120(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(0x00a60cb0, ni_source_cube_map_get_rtti() -> Ptr),
        entry!(0x00a6_0870, ni_source_cube_map_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_source_cube_map_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a60cb0, &args![]).u32(), NI_SOURCE_CUBE_MAP_RTTI);
    }

    #[test]
    fn thunk_00a60870_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_0870;
        const TARGET: u32 = 0x00a6_0600;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const DELETE: u32 = 0x0f00_3000;
    const RENDERER_PREPARE: u32 = 0x0f00_4144;

    /// The map's vtable `0109babc` (slot 0 at `DELETE`); `LoadFromFiles` sets +0x48 and the
    /// word at +0x3c to `loaded`; a renderer whose slot +0x144 answers `accepts`.
    fn cube_rig(loaded: u32, accepts: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.put_vtable(NI_SOURCE_CUBE_MAP_VTABLE, &[DELETE]);
        e.register(DELETE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SOURCE_TEXTURE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x48, 0x99);
            Ret::default()
        });
        e.register(SEARCH_PATH_SET_UP_00AA24D0, |_, _| Ret::default());
        e.register(SEARCH_PATH_CLEAN_UP_00AA24F0, |_, _| Ret::default());
        e.register_double(NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES, move |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0x48), 0);
            e.mem.set_u32(a[0] + 0x48, 1);
            e.mem.set_u32(a[0] + 0x3c, loaded);
            Ret::default()
        });
        let renderer = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x52];
        slots[0x51] = RENDERER_PREPARE;
        e.put_vtable(0x0f00_8000, &slots);
        e.mem.set_u32(renderer, 0x0f00_8000);
        e.register_double(RENDERER_PREPARE, move |_, _| u32::from(accepts).into_ret());
        let preferences = e.mem.alloc(12);
        e.mem.set_u32(preferences, 0xa);
        e.mem.set_u32(preferences + 4, 0xb);
        e.mem.set_u32(preferences + 8, 0xc);
        (e, renderer, preferences)
    }

    fn make(e: &mut Engine, renderer: u32, preferences: u32) -> u32 {
        e.call(
            0x00a6_1120,
            &args![
                0x101u32,
                0x102u32,
                0x103u32,
                0x104u32,
                0x105u32,
                0x106u32,
                renderer,
                0x77u32,
                preferences
            ],
        )
        .u32()
    }

    #[test]
    fn fn_00a61120_keeps_the_map_the_renderer_accepts() {
        let (mut e, renderer, preferences) = cube_rig(1, true);
        e.call_log = Some(vec![]);
        let map = make(&mut e, renderer, preferences);
        assert_ne!(map, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(map), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 0x48), 0);
        assert_eq!(
            [
                e.mem.u32(map + 0x18),
                e.mem.u32(map + 0x1c),
                e.mem.u32(map + 0x20)
            ],
            [0xa, 0xb, 0xc]
        );
        let load = calls(&e, NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES);
        assert_eq!(load.len(), 1);
        assert_eq!(
            &load[0][..7],
            [map, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106]
        );
        let search_path = load[0][7];
        assert_eq!(
            calls(&e, SEARCH_PATH_SET_UP_00AA24D0),
            vec![vec![search_path]]
        );
        assert_eq!(
            calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0),
            vec![vec![search_path]]
        );
        assert_eq!(calls(&e, RENDERER_PREPARE), vec![vec![renderer, map]]);
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn fn_00a61120_without_a_renderer_keeps_the_map() {
        let (mut e, _, preferences) = cube_rig(0, false);
        e.call_log = Some(vec![]);
        let map = make(&mut e, 0, preferences);
        assert_ne!(map, 0);
        assert!(calls(&e, DELETE).is_empty());
        assert_eq!(calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0).len(), 1);
    }

    #[test]
    fn fn_00a61120_deletes_a_map_that_failed_or_was_refused() {
        // Nothing loaded (+0x3c is 0): the renderer is not asked.
        let (mut e, renderer, preferences) = cube_rig(0, true);
        e.call_log = Some(vec![]);
        assert_eq!(make(&mut e, renderer, preferences), 0);
        assert!(calls(&e, RENDERER_PREPARE).is_empty());
        let map = calls(&e, NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES)[0][0];
        assert_eq!(calls(&e, DELETE), vec![vec![map, 1]]);
        assert_eq!(calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0).len(), 1);
        // Loaded but refused by the renderer.
        let (mut e, renderer, preferences) = cube_rig(1, false);
        e.call_log = Some(vec![]);
        assert_eq!(make(&mut e, renderer, preferences), 0);
        assert_eq!(calls(&e, RENDERER_PREPARE).len(), 1);
        assert_eq!(calls(&e, DELETE).len(), 1);
    }
}
