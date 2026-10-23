//! `BSSystemUtilities/bsdlcsystemutility_xenon.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The PC build asks Steam (`SteamApps`, `ISteamApps` slot +0x1c, the
//! "is the DLC installed" query) which of its listed DLC are there.

#[allow(unused_imports)]
use crate::prelude::*;

/// The singleton (`011f7244`), the memory manager's allocation (`00aa3e40`, `thiscall` on
/// `011f6238`, one word: the size) and the utility's constructor (`00af27c0`, `thiscall`).
const INSTANCE: u32 = 0x011f_7244;
const ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const CONSTRUCT: u32 = 0x00af_27c0;
/// The import slot of `steam_api.dll` `SteamApps` (no arguments; answers the `ISteamApps`
/// object), its slot +0x1c (`thiscall`, one word: the app id; answers `AL`), and the DLC
/// table it walks (`011accb8`, 8 bytes an entry: the app id, -1 at the end, then a word the
/// entry's object is made with).
const STEAM_APPS: u32 = 0x00fd_f494;
const SLOT_DLC_INSTALLED: u32 = 0x1c;
const DLC_TABLE: u32 = 0x011a_ccb8;
/// The utility's own calls: `00af2a60` (`thiscall`, one word), `00b008b0` (`thiscall`, one
/// word: the entry's word; answers the object), the list's tail add (`00af25b0`, `thiscall`,
/// one word: the address of the item word) and `00af2c20` (`thiscall`, no stack words).
const FN_00AF2A60: u32 = 0x00af_2a60;
const MAKE_ENTRY: u32 = 0x00b0_08b0;
const LIST_ADD_TAIL: u32 = 0x00af_25b0;
const FN_00AF2C20: u32 = 0x00af_2c20;
/// The utility's list of found DLC (`+0xc`: a chain of nodes of two words, the item and the
/// next node).
const LIST: u32 = 0xc;

// Translated from 00af26c0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af26c0 - FS:[0] exception frame (handler 00f22f7b, state 0 around the constructor) not translated.
/// `BSDLCSystemUtility::QInstance` (engine map; no arguments): when the singleton (`011f7244`)
/// is null, a 0x24-byte block is allocated (`00aa3e40`) and constructed (`00af27c0`; a null
/// block gives null), and the result kept. Answers the singleton.
pub fn dlc_utility_instance(e: &mut Engine) -> u32 {
    if e.global::<u32>(INSTANCE) == 0 {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x24u32]).u32();
        let object = if block != 0 {
            e.call(CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.set_global::<u32>(INSTANCE, object);
    }
    e.global::<u32>(INSTANCE)
}

// Translated from 00af2940 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af2940 - FS:[0] exception frame (handler 00f22fdd, state 0 around the entry's constructor) not translated.
/// The DLC scan (no label; `thiscall`, two words it does not read, `RET 8`): `00af2a60(1)`;
/// without a Steam apps object (`SteamApps` answering null) 0. Otherwise for each entry of the
/// DLC table until the app id -1: `SteamApps()` again and its "installed" query (slot +0x1c)
/// with the app id; for an installed one the count goes up, an 8-byte object is allocated and
/// made with the entry's second word (`00b008b0`; null for a failed allocation) and added at
/// the tail of the utility's list (`00af25b0` on `+0xc`, through a local word). Answers the
/// count.
pub fn dlc_utility_scan(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    let base = this.addr();
    e.call(FN_00AF2A60, &args![base, 1u32]);
    let mut count: u32 = 0;
    if e.call(STEAM_APPS, &args![]).u32() == 0 {
        return count;
    }
    let mut index: u32 = 0;
    loop {
        let entry = DLC_TABLE.wrapping_add(index.wrapping_mul(8));
        if e.mem.u32(entry) == 0xffff_ffff {
            break;
        }
        let apps = e.call(STEAM_APPS, &args![]).u32();
        let app_id = e.mem.u32(entry);
        if e.vcall(apps, SLOT_DLC_INSTALLED, &args![app_id]).u8() != 0 {
            count = count.wrapping_add(1);
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
            let object = if block != 0 {
                let word = e.mem.u32(entry.wrapping_add(4));
                e.call(MAKE_ENTRY, &args![block, word]).u32()
            } else {
                0
            };
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), object);
                e.call(LIST_ADD_TAIL, &args![base.wrapping_add(LIST), local.addr()]);
            });
        }
        index = index.wrapping_add(1);
    }
    count
}

/// The number of nodes of the list at `list` with a non-null item (following +4 to null).
fn count_items(e: &Engine, list: u32) -> u32 {
    let mut count: u32 = 0;
    let mut node = list;
    while node != 0 {
        if e.mem.u32(node) != 0 {
            count = count.wrapping_add(1);
        }
        node = e.mem.u32(node.wrapping_add(4));
    }
    count
}

// Translated from 00af2b10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The report of the found DLC (no label; `thiscall`, two words: one it does not read and a
/// callback, `RET 8`): `00af2c20`, then the list's items are counted. With a callback, for
/// each node from the first while the node is not null and not empty (item and next both
/// null), the callback (cdecl, four words) gets the node's index, the count less one, 1 and
/// the item; then once more `(-1, -1, 0, 0)` and 0 is answered. Without a callback the items
/// are counted again and the count answered.
pub fn dlc_utility_report(e: &mut Engine, this: Ptr, _unused: u32, callback: u32) -> u32 {
    let base = this.addr();
    e.call(FN_00AF2C20, &args![base]);
    let list = base.wrapping_add(LIST);
    let total = count_items(e, list);
    if callback == 0 {
        return count_items(e, list);
    }
    let mut node = list;
    let mut index: u32 = 0;
    while node != 0 {
        let empty = e.mem.u32(node.wrapping_add(4)) == 0 && e.mem.u32(node) == 0;
        if empty {
            break;
        }
        let item = e.mem.u32(node);
        e.call(callback, &args![index, total.wrapping_sub(1), 1u32, item]);
        index = index.wrapping_add(1);
        node = e.mem.u32(node.wrapping_add(4));
    }
    e.call(callback, &args![0xffff_ffffu32, 0xffff_ffffu32, 0u32, 0u32]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_26c0, dlc_utility_instance() -> u32),
        entry!(0x00af_2940, dlc_utility_scan(Ptr, u32, u32) -> u32),
        entry!(0x00af_2b10, dlc_utility_report(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    const UTILITY: u32 = 0x0800_0000;
    const APPS: u32 = 0x0900_0000;
    const APPS_VTABLE: u32 = 0x0900_0100;
    const INSTALLED: u32 = 0x0a00_0000;
    const CALLBACK: u32 = 0x0a00_0100;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        e.map(UTILITY, 0x100);
        e.map(DLC_TABLE, 0x40);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn instance_is_made_once() {
        let mut e = engine();
        e.register(CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        let first = dlc_utility_instance(&mut e);
        assert_eq!(e.call(0x00af_26c0, &args![]).u32(), first);
        assert_eq!(calls(&e, CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x24]]);
        // A failed allocation keeps null and tries again next time.
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(dlc_utility_instance(&mut e), 0);
        assert_eq!(dlc_utility_instance(&mut e), 0);
        assert!(calls(&e, CONSTRUCT).is_empty());
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
    }

    #[test]
    fn scan_adds_the_installed_dlc_in_table_order() {
        let mut e = engine();
        // Three entries (app ids 10, 11, 12, words 0x100 + i), then the end.
        for i in 0..3u32 {
            e.mem.set_u32(DLC_TABLE + 8 * i, 10 + i);
            e.mem.set_u32(DLC_TABLE + 8 * i + 4, 0x100 + i);
        }
        e.mem.set_u32(DLC_TABLE + 24, 0xffff_ffff);
        e.map(APPS, 4);
        e.put_vtable(APPS_VTABLE, &[0, 0, 0, 0, 0, 0, 0, INSTALLED]);
        e.mem.set_u32(APPS, APPS_VTABLE);
        e.register(STEAM_APPS, |_, _| APPS.into_ret());
        // Apps 10 and 12 installed (the low byte only counts).
        e.register(INSTALLED, |_, a| {
            (if a[1] == 11 { 0x100u32 } else { 1 }).into_ret()
        });
        e.register(FN_00AF2A60, |_, _| Ret::default());
        e.register(MAKE_ENTRY, |_, a| (a[1] + 0x1000).into_ret());
        assert_eq!(dlc_utility_scan(&mut e, Ptr::new(UTILITY), 7, 8), 2);
        assert_eq!(calls(&e, FN_00AF2A60), vec![vec![UTILITY, 1]]);
        assert_eq!(
            calls(&e, INSTALLED)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<u32>>(),
            vec![10, 11, 12]
        );
        assert_eq!(
            calls(&e, MAKE_ENTRY)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<u32>>(),
            vec![0x100, 0x102]
        );
        // The list holds the two objects (the real tail add).
        assert_eq!(e.mem.u32(UTILITY + LIST), 0x1100);
        let second = e.mem.u32(UTILITY + LIST + 4);
        assert_eq!(e.mem.u32(second), 0x1102);
        // Without Steam: 0 and no table walk.
        e.register(STEAM_APPS, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(dlc_utility_scan(&mut e, Ptr::new(UTILITY), 0, 0), 0);
        assert!(calls(&e, INSTALLED).is_empty());
    }

    #[test]
    fn report_calls_back_for_each_item_then_ends() {
        let mut e = engine();
        e.register(FN_00AF2C20, |_, _| Ret::default());
        e.register(CALLBACK, |_, _| Ret::default());
        let second = e.mem.alloc(8);
        e.mem.set_u32(UTILITY + LIST, 0x55);
        e.mem.set_u32(UTILITY + LIST + 4, second);
        e.mem.set_u32(second, 0x66);
        assert_eq!(
            dlc_utility_report(&mut e, Ptr::new(UTILITY), 0, CALLBACK),
            0
        );
        assert_eq!(
            calls(&e, CALLBACK),
            vec![
                vec![0, 1, 1, 0x55],
                vec![1, 1, 1, 0x66],
                vec![0xffff_ffff, 0xffff_ffff, 0, 0]
            ]
        );
        // Without a callback: the count of items.
        assert_eq!(dlc_utility_report(&mut e, Ptr::new(UTILITY), 0, 0), 2);
        // An empty list: only the end call.
        e.mem.set_u32(UTILITY + LIST, 0);
        e.mem.set_u32(UTILITY + LIST + 4, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            dlc_utility_report(&mut e, Ptr::new(UTILITY), 0, CALLBACK),
            0
        );
        assert_eq!(
            calls(&e, CALLBACK),
            vec![vec![0xffff_ffff, 0xffff_ffff, 0, 0]]
        );
    }
}
