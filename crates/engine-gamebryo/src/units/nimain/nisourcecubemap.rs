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

/// The word an empty `NiFixedString` holds (`0109b220`); names equal to it are not counted.
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStandardizeFilePath` (`00a57500`, `cdecl`, the name) and
/// `NiImageConverter::ConvertFilenameToPlatformSpecific_ov2` (`00a76960`, `cdecl`: the name,
/// the converted name) (engine map).
const STANDARDIZE_FILE_PATH: u32 = 0x00a5_7500;
const CONVERT_FILE_NAME: u32 = 0x00a7_6960;
/// The byte that makes the texture load at once (`011a9594`).
const LOAD_AT_ONCE_011A9594: u32 = 0x011a_9594;

// Translated from 00a60f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nisourcecubemap.cpp`): makes an `NiSourceCubeMap`
/// from one file (the cube-map path of `BSShaderManager::GetTexture`). `cdecl`: the address of
/// the name (an `NiFixedString`), the renderer and the address of three words of format
/// preferences. Allocates 0x4c bytes (`00aa13e0`); when that succeeds runs
/// `NiSourceTexture::NiSourceTexture` (`00a5fc10`), sets the vtable `0109babc` and clears +0x48
/// (a null allocation is used as it is). Copies the preferences to +0x18..+0x20, assigns the
/// name to the `NiFixedString` at +0x30 (the new one's count at -8 incremented and the old
/// one's decremented, each unless it is the empty word at `0109b220`), standardizes it
/// (`00a57500`) and converts it into +0x34 (`00a76960`), clears +0x48 when it is set. When the
/// byte at `011a9594` is set: the map's vtable slot +0xa4, then with a renderer its slot +0x144
/// with the map; when that answers a clear byte the map is deleted (slot 0 with 1) and 0
/// answered. Otherwise answers the map.
pub fn fn_00a60f60(e: &mut Engine, name: Ptr, renderer: Ptr, preferences: Ptr) -> Ptr {
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
    let file = map.wrapping_add(0x30);
    let new_name = e.mem.u32(name.addr());
    if e.mem.u32(file) != new_name {
        if new_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![new_name.wrapping_sub(8)],
            );
        }
        let old_name = e.mem.u32(file);
        if old_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_DECREMENT,
                &args![old_name.wrapping_sub(8)],
            );
        }
        e.mem.set_u32(file, new_name);
    }
    e.call(STANDARDIZE_FILE_PATH, &args![file]);
    e.call(CONVERT_FILE_NAME, &args![file, map.wrapping_add(0x34)]);
    if e.mem.u32(map.wrapping_add(0x48)) != 0 {
        e.mem.set_u32(map.wrapping_add(0x48), 0);
    }
    if e.global::<u8>(LOAD_AT_ONCE_011A9594) != 0 {
        e.vcall(map, 0xa4, &args![]);
        if !renderer.is_null() && !e.vcall(renderer.addr(), 0x144, &args![map]).bool() {
            e.vcall(map, 0, &args![1u32]);
            return Ptr::NULL;
        }
    }
    Ptr::new(map)
}
/// Callees of `LoadFromFiles` (engine map names): `NiImageConverter::GetImageConverter`
/// (`00a76640`, no arguments), the allocation `00aa1070` (`cdecl`, size), `_strcpy_s`
/// (`00ec65a6`, `cdecl`), `NiPath::Standardize` (`00aa2a00`, `cdecl`),
/// `NiImageConverter::ConvertFilenameToPlatformSpecific` (`00a768b0`, `cdecl`; answers a new
/// name), `NiSearchPath::SetFilePath` (`00aa2540`, `thiscall`, one word), `NiFile::Access`
/// (`00aa1a30`, `cdecl`: path, mode; answers a byte), the free `00aa10f0` (`cdecl`) and
/// `NiSourceCubeMap::LoadFromPixelData` (`00a60940`, `thiscall`, six words).
const GET_IMAGE_CONVERTER: u32 = 0x00a7_6640;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const STRCPY_S: u32 = 0x00ec_65a6;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const CONVERT_FILE_NAME_TO_PLATFORM: u32 = 0x00a7_68b0;
const SEARCH_PATH_SET_FILE_PATH: u32 = 0x00aa_2540;
const FILE_ACCESS: u32 = 0x00aa_1a30;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const LOAD_FROM_PIXEL_DATA: u32 = 0x00a6_0940;

/// The six face pointers of `LoadFromFiles` released in reverse order (the `NiPointer`
/// destructors), without clearing them.
fn release_faces(e: &mut Engine, faces: u32) {
    for index in (0..6u32).rev() {
        let face = e.mem.u32(faces + 4 * index);
        if face != 0 {
            let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![face + 4]).u32();
            if left == 0 {
                e.vcall(face, 4, &args![]);
            }
        }
    }
}

// Translated from 00a60cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LoadFromFiles` (Xbox PDB), `thiscall` with seven stack words: the six
/// face file names and the search path. With six face `NiPointer`s on the stack (cleared) and
/// the image converter (`GetImageConverter`), for each face in order: copies its name into a
/// new block (`00aa1070` of the length + 1, `_strcpy_s`), standardizes it
/// (`NiPath::Standardize`), converts it (`ConvertFilenameToPlatformSpecific`), gives that to
/// the search path (`SetFilePath`), resets the search path (its vtable slot +4) and asks it
/// for paths (slot +8 with a 0x104-byte buffer and 0x104) until one exists
/// (`NiFile::Access(path, 0)`): then the converter's slot +0xc reads it (with 0, 0) into the
/// face's pointer, and the converted name and the copy are freed (`00aa10f0`). A face left
/// null ends the load (the six pointers released, nothing freed); after the sixth,
/// `LoadFromPixelData` gets the six faces and the pointers are released.
// Translation incomplete: 00a60cc0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_source_cube_map_load_from_files(
    e: &mut Engine,
    this: Ptr,
    name_0: Ptr,
    name_1: Ptr,
    name_2: Ptr,
    name_3: Ptr,
    name_4: Ptr,
    name_5: Ptr,
    search_path: Ptr,
) {
    let names = [name_0, name_1, name_2, name_3, name_4, name_5];
    e.with_stack(0x148, |e, frame| {
        let frame = frame.addr();
        let faces = frame + 4;
        let buffer = frame + 0x40;
        let search = search_path.addr();
        let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
        for index in 0..6u32 {
            let name = names[index as usize].addr();
            let size = e.mem.cstr(name).len() as u32 + 1;
            let copy = e.call(ALLOCATE_00AA1070, &args![size]).u32();
            e.call(STRCPY_S, &args![copy, size, name]);
            e.call(PATH_STANDARDIZE, &args![copy]);
            let converted = e.call(CONVERT_FILE_NAME_TO_PLATFORM, &args![copy]).u32();
            e.call(SEARCH_PATH_SET_FILE_PATH, &args![search, converted]);
            e.vcall(search, 4, &args![]);
            let mut more = e.vcall(search, 8, &args![buffer, 0x104u32]).bool();
            while more {
                if e.call(FILE_ACCESS, &args![buffer, 0u32]).bool() {
                    let texture = e.vcall(converter, 0xc, &args![buffer, 0u32, 0u32]).u32();
                    let slot = faces + 4 * index;
                    let old = e.mem.u32(slot);
                    if old != texture {
                        if old != 0 {
                            let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
                            if left == 0 {
                                e.vcall(old, 4, &args![]);
                            }
                        }
                        e.mem.set_u32(slot, texture);
                        if texture != 0 {
                            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
                        }
                    }
                    e.call(FREE_00AA10F0, &args![converted]);
                    e.call(FREE_00AA10F0, &args![copy]);
                    break;
                }
                more = e.vcall(search, 8, &args![buffer, 0x104u32]).bool();
            }
            if e.mem.u32(faces + 4 * index) == 0 {
                release_faces(e, faces);
                return;
            }
        }
        let face = |e: &Engine, index: u32| e.mem.u32(faces + 4 * index);
        let words = [
            this.addr(),
            face(e, 0),
            face(e, 1),
            face(e, 2),
            face(e, 3),
            face(e, 4),
            face(e, 5),
        ];
        e.call(LOAD_FROM_PIXEL_DATA, &words);
        release_faces(e, faces);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_0f60, fn_00a60f60(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00a6_0cc0,
            ni_source_cube_map_load_from_files(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
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

#[cfg(test)]
mod b000h_tests {
    use super::*;

    const DELETE: u32 = 0x0ea0_0000;
    const MAP_SLOT_A4: u32 = 0x0ea0_0010;
    const RENDERER_SLOT_144: u32 = 0x0ea0_0020;
    const SEARCH_RESET: u32 = 0x0ea0_0030;
    const SEARCH_NEXT: u32 = 0x0ea0_0040;
    const CONVERTER_READ: u32 = 0x0ea0_0050;
    const MAP_VTABLE: u32 = 0x0ea1_0000;
    const OBJECT_VTABLE: u32 = 0x0ea2_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    /// An engine for `00a60f60`: the allocation answers `allocation`, the map's vtable is
    /// `MAP_VTABLE` (the constructor double sets it; the function then stores the real one,
    /// which the test maps to the same slots).
    fn cube_engine(load_at_once: bool, accepted: u32) -> Engine {
        let mut e = Engine::new();
        counting(&mut e);
        e.map(0x0109_b000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.set_global::<u8>(LOAD_AT_ONCE_011A9594, u8::from(load_at_once));
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SOURCE_TEXTURE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STANDARDIZE_FILE_PATH, |_, _| Ret::default());
        e.register(CONVERT_FILE_NAME, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(MAP_SLOT_A4, |_, _| Ret::default());
        e.register_double(RENDERER_SLOT_144, move |_, _| accepted.into_ret());
        let mut slots = vec![DELETE; 0x52];
        slots[0xa4 / 4] = MAP_SLOT_A4;
        slots[0x144 / 4] = RENDERER_SLOT_144;
        e.put_vtable(NI_SOURCE_CUBE_MAP_VTABLE, &slots);
        e.put_vtable(MAP_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    fn name(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, count);
        block + 8
    }

    #[test]
    fn fn_00a60f60_makes_the_map_and_keeps_the_name() {
        let mut e = cube_engine(false, 1);
        let new_name = name(&mut e, 1);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, new_name);
        let preferences = e.mem.alloc(0xc);
        for word in 0..3u32 {
            e.mem.set_u32(preferences + 4 * word, 0x10 + word);
        }
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, MAP_VTABLE);
        let map = e
            .call(0x00a6_0f60, &args![holder, renderer, preferences])
            .u32();
        assert_ne!(map, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(map), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 0x18), 0x10);
        assert_eq!(e.mem.u32(map + 0x20), 0x12);
        assert_eq!(e.mem.u32(map + 0x30), new_name);
        assert_eq!(e.mem.u32(new_name - 8), 2);
        assert_eq!(calls(&e, STANDARDIZE_FILE_PATH), vec![vec![map + 0x30]]);
        assert_eq!(
            calls(&e, CONVERT_FILE_NAME),
            vec![vec![map + 0x30, map + 0x34]]
        );
        // Without the load byte nothing else runs.
        assert!(calls(&e, MAP_SLOT_A4).is_empty());
        assert!(calls(&e, RENDERER_SLOT_144).is_empty());
    }

    #[test]
    fn fn_00a60f60_loads_at_once_and_drops_a_refused_map() {
        let mut e = cube_engine(true, 0x100);
        let holder = e.mem.alloc(4);
        let preferences = e.mem.alloc(0xc);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, MAP_VTABLE);
        let map = e
            .call(0x00a6_0f60, &args![holder, renderer, preferences])
            .u32();
        assert_eq!(map, 0);
        let made = calls(&e, NI_SOURCE_TEXTURE_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, MAP_SLOT_A4), vec![vec![made]]);
        assert_eq!(calls(&e, RENDERER_SLOT_144), vec![vec![renderer, made]]);
        assert_eq!(calls(&e, DELETE), vec![vec![made, 1]]);
        // The empty name (0, the word at 0109b220) is neither counted nor released.
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // Without a renderer the map is kept.
        let mut e = cube_engine(true, 0);
        let holder = e.mem.alloc(4);
        let preferences = e.mem.alloc(0xc);
        let map = e.call(0x00a6_0f60, &args![holder, 0u32, preferences]).u32();
        assert_ne!(map, 0);
        assert!(calls(&e, DELETE).is_empty());
    }

    /// An engine for `LoadFromFiles`: the search path offers `tries` paths per face, the
    /// last of which exists unless `missing_face` is that face; the converter reads a new
    /// object (count 0) for an existing path.
    fn load_engine(tries: u32, missing_face: Option<u32>) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        counting(&mut e);
        e.map(0x0eb0_0000, 0x100);
        e.register(GET_IMAGE_CONVERTER, |e, _| {
            let converter = e.mem.alloc(4);
            e.mem.set_u32(converter, MAP_VTABLE);
            converter.into_ret()
        });
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        e.register(CONVERT_FILE_NAME_TO_PLATFORM, |e, a| {
            let converted = e.mem.alloc(8);
            e.mem.set_u32(converted, a[0]);
            converted.into_ret()
        });
        e.register(SEARCH_PATH_SET_FILE_PATH, |_, _| Ret::default());
        // 0eb00000: the face being searched; 0eb00004: paths offered so far for it.
        e.register(SEARCH_RESET, |e, _| {
            e.mem.set_u32(0x0eb0_0004, 0);
            Ret::default()
        });
        e.register_double(SEARCH_NEXT, move |e, a| {
            let offered = e.mem.u32(0x0eb0_0004) + 1;
            e.mem.set_u32(0x0eb0_0004, offered);
            e.mem.set_u32(a[1], offered);
            u32::from(offered <= tries).into_ret()
        });
        e.register_double(FILE_ACCESS, move |e, a| {
            let face = e.mem.u32(0x0eb0_0000);
            let exists = e.mem.u32(a[0]) == tries && missing_face != Some(face);
            u32::from(exists).into_ret()
        });
        e.register(CONVERTER_READ, |e, _| {
            let face = e.mem.u32(0x0eb0_0000);
            e.mem.set_u32(0x0eb0_0000, face + 1);
            let texture = e.mem.alloc(8);
            e.mem.set_u32(texture, OBJECT_VTABLE);
            texture.into_ret()
        });
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(LOAD_FROM_PIXEL_DATA, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        let mut slots = vec![DELETE; 4];
        slots[1] = SEARCH_RESET;
        slots[2] = SEARCH_NEXT;
        slots[3] = CONVERTER_READ;
        e.put_vtable(MAP_VTABLE, &slots);
        e.put_vtable(OBJECT_VTABLE, &[DELETE, DELETE]);
        let search = e.mem.alloc(4);
        e.mem.set_u32(search, MAP_VTABLE);
        let this = e.mem.alloc(0x4c);
        let names: Vec<u32> = (0..6)
            .map(|index| {
                let text = e.mem.alloc(8);
                e.mem.set_cstr(text, format!("face{index}").as_bytes());
                text
            })
            .collect();
        e.call_log = Some(vec![]);
        (e, this, search, names)
    }

    fn load(e: &mut Engine, this: u32, names: &[u32], search: u32) {
        let mut words = vec![this];
        words.extend_from_slice(names);
        words.push(search);
        e.call(0x00a6_0cc0, &words);
    }

    #[test]
    fn ni_source_cube_map_load_from_files_loads_six_faces() {
        let (mut e, this, search, names) = load_engine(2, None);
        load(&mut e, this, &names, search);
        let copies = calls(&e, STRCPY_S);
        assert_eq!(copies.len(), 6);
        for (index, copy) in copies.iter().enumerate() {
            assert_eq!(copy[1], 6);
            assert_eq!(copy[2], names[index]);
        }
        let loaded = calls(&e, LOAD_FROM_PIXEL_DATA);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0][0], this);
        let faces = &loaded[0][1..];
        for face in faces {
            // Held by the stack pointer only, released at the end: count 0, deleted.
            assert_eq!(e.mem.u32(face + 4), 0);
        }
        assert_eq!(calls(&e, DELETE).len(), 6);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 12);
        assert_eq!(calls(&e, FILE_ACCESS).len(), 12);
    }

    #[test]
    fn ni_source_cube_map_load_from_files_stops_at_a_missing_face() {
        let (mut e, this, search, names) = load_engine(2, Some(2));
        load(&mut e, this, &names, search);
        assert_eq!(calls(&e, STRCPY_S).len(), 3);
        assert!(calls(&e, LOAD_FROM_PIXEL_DATA).is_empty());
        // Two faces were read and released; the missing face's names are not freed.
        assert_eq!(calls(&e, DELETE).len(), 2);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 4);
    }
}
