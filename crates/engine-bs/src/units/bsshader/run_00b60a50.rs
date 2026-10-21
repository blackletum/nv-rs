//! `BSShader/run_00b60a50` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The map getter `00b61890` (unnamed; no arguments, plain `RET`: answers the static map at
/// `011f9f1c`, made on first use: vtable +0, bucket count +4, buckets +8, item count +0xc), the
/// clear that runs first, `00b62970` (unnamed; no arguments, plain `RET`), the map step
/// `00e6e630` (`thiscall`, `RET 0xc`: iterator, key and `NiPointer` value addresses), the map's
/// remove-all `00b601a0` (`thiscall`, no stack words) and `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on `011f6238`, one word).
const MAP_00B61890: u32 = 0x00b6_1890;
const CLEAR_00B62970: u32 = 0x00b6_2970;
const MAP_GET_NEXT_00E6E630: u32 = 0x00e6_e630;
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;
const DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
/// The imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT_00B62A20: u32 = 0x00fd_f1c4;

/// An `NiPointer` release as the exe inlines it: the count at +4 down (`InterlockedDecrement`)
/// and, at zero, vtable slot +4 on the object (`thiscall`, no stack words).
fn release_00b62a20(e: &mut Engine, object: u32) {
    if e.call(
        IMPORT_INTERLOCKED_DECREMENT_00B62A20,
        &args![object.wrapping_add(4)],
    )
    .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The release of one held field: when the word at `item + offset` is not null, it is
/// released and the word cleared.
fn release_field_00b62a20(e: &mut Engine, item: u32, offset: u32) {
    let held = e.mem.u32(item.wrapping_add(offset));
    if held != 0 {
        release_00b62a20(e, held);
        e.mem.set_u32(item.wrapping_add(offset), 0);
    }
}

// Translated from 00b62a20 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b60a50`; no arguments, plain `RET`; called from
/// `00b54630` and from `ToggleGrassFunction` at `005b7113`): runs `00b62970`, then empties
/// the map `00b61890` answers. The iterator starts at the first item of the first non-empty
/// bucket (the bucket count and array read once); when the map's item count (+0xc) is not
/// zero, each item is stepped to with `00e6e630` (the map fetched again each time) into an
/// `NiPointer` local, and an item that is not null and whose byte +0x32 is clear has its held
/// objects released: +8, then +0xc deallocated, then +0x14, +0x1c, +0x18 and +0x20, each
/// released word cleared (the deallocated one always). The steps go on while the iterator is
/// not null (none when the first one is null); then the map's remove-all `00b601a0` runs.
/// Last the local releases the item it holds.
// Translation incomplete: 00b62a20 - the C++ exception frame (FS:[0] chain, handler `00f273e8`, unwind state 0 for the NiPointer local) is not translated; the security cookie is pushed but never checked (no 00ec408c call).
pub fn fn_00b62a20(e: &mut Engine) {
    e.call(CLEAR_00B62970, &args![]);
    let map = e.call(MAP_00B61890, &args![]).u32();
    let size = e.mem.u32(map.wrapping_add(4));
    let mut first = 0u32;
    if size > 0 {
        let buckets = e.mem.u32(map.wrapping_add(8));
        let mut i = 0u32;
        while i < size {
            let bucket = e.mem.u32(buckets.wrapping_add(4 * i));
            if bucket != 0 {
                first = bucket;
                break;
            }
            i += 1;
        }
    }
    // The locals: the `NiPointer` item (+0), the iterator (+4) and the key (+8).
    e.with_stack(0xc, |e, frame| {
        let item_slot = frame.addr();
        let iterator = item_slot + 4;
        let key = item_slot + 8;
        e.mem.set_u32(iterator, first);
        e.mem.set_u32(item_slot, 0);
        let mut item = 0u32;
        let map = e.call(MAP_00B61890, &args![]).u32();
        if e.mem.u32(map.wrapping_add(0xc)) != 0 {
            if first != 0 {
                loop {
                    let map = e.call(MAP_00B61890, &args![]).u32();
                    e.call(MAP_GET_NEXT_00E6E630, &args![map, iterator, key, item_slot]);
                    item = e.mem.u32(item_slot);
                    if item != 0 && e.mem.u8(item.wrapping_add(0x32)) == 0 {
                        release_field_00b62a20(e, item, 8);
                        let block = e.mem.u32(item.wrapping_add(0xc));
                        e.call(DEALLOCATE_00AA4060, &args![MEMORY_MANAGER_011F6238, block]);
                        e.mem.set_u32(item.wrapping_add(0xc), 0);
                        release_field_00b62a20(e, item, 0x14);
                        release_field_00b62a20(e, item, 0x1c);
                        release_field_00b62a20(e, item, 0x18);
                        release_field_00b62a20(e, item, 0x20);
                    }
                    if e.mem.u32(iterator) == 0 {
                        break;
                    }
                }
            }
            let map = e.call(MAP_00B61890, &args![]).u32();
            e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
        }
        if item != 0 {
            release_00b62a20(e, item);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_2a20, fn_00b62a20())]
}

#[cfg(test)]
mod empty_map_00b62a20_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const MAP: u32 = 0x0001_0000;
    const BUCKETS: u32 = 0x0001_0100;
    const DELETE: u32 = 0x00ab_0004;
    const VTABLE: u32 = 0x0001_0f00;
    /// The held fields in the order the exe releases them, and the deallocated one.
    const FIELDS: [u32; 5] = [8, 0x14, 0x1c, 0x18, 0x20];
    const BLOCK: u32 = 0xc;

    /// A counted object (vtable `VTABLE`, slot +4 the `DELETE` double).
    fn object(e: &mut Engine, at: u32, count: u32) {
        e.mem.set_u32(at, VTABLE);
        e.mem.set_u32(at + 4, count);
    }

    /// The map (three buckets, the middle one `first`, item count `count`); the step double
    /// hands out `items` in turn and the iterator goes null after the last one.
    fn engine(count: u32, first: u32, items: Vec<u32>) -> Engine {
        let mut e = Engine::new();
        e.map(MAP, 0x4000);
        e.mem.set_u32(MAP + 4, 3);
        e.mem.set_u32(MAP + 8, BUCKETS);
        e.mem.set_u32(MAP + 0xc, count);
        e.mem.set_u32(BUCKETS + 4, first);
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.register(MAP_00B61890, |_, _| MAP.into_ret());
        for target in [
            CLEAR_00B62970,
            MAP_REMOVE_ALL_00B601A0,
            DEALLOCATE_00AA4060,
            DELETE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT_00B62A20, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let next = Rc::new(Cell::new(0usize));
        e.register_double(MAP_GET_NEXT_00E6E630, move |e, a| {
            let i = next.get();
            next.set(i + 1);
            e.mem.set_u32(a[3], items[i]);
            let more = if i + 1 < items.len() { 0x0001_0200 } else { 0 };
            e.mem.set_u32(a[1], more);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The first item holds an object in each field (count 1: each is deleted) and a block;
    /// the second holds nothing. The second stays in the local and is counted down at the end.
    #[test]
    fn each_item_drops_what_it_holds_then_the_map_is_emptied() {
        let (first, second) = (0x0001_1000u32, 0x0001_1100u32);
        let mut e = engine(2, 0x0001_0200, vec![first, second]);
        object(&mut e, first, 5);
        object(&mut e, second, 5);
        let mut held = vec![];
        for (k, offset) in FIELDS.iter().enumerate() {
            let at = 0x0001_2000 + 0x10 * k as u32;
            object(&mut e, at, 1);
            e.mem.set_u32(first + offset, at);
            held.push(vec![at]);
        }
        e.mem.set_u32(first + BLOCK, 0xb10c);
        e.call(0x00b6_2a20, &args![]);
        assert_eq!(calls_to(&e, CLEAR_00B62970).len(), 1);
        let steps = calls_to(&e, MAP_GET_NEXT_00E6E630);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0][0], MAP);
        assert_eq!(
            steps[0][1],
            steps[0][3] + 4,
            "iterator after the item local"
        );
        assert_eq!(steps[0][2], steps[0][3] + 8, "key after the iterator");
        assert_eq!(calls_to(&e, DELETE), held);
        for offset in FIELDS.iter().chain([BLOCK].iter()) {
            assert_eq!(e.mem.u32(first + offset), 0);
        }
        assert_eq!(
            calls_to(&e, DEALLOCATE_00AA4060),
            vec![
                vec![MEMORY_MANAGER_011F6238, 0xb10c],
                vec![MEMORY_MANAGER_011F6238, 0]
            ]
        );
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![MAP]]);
        assert_eq!(e.mem.u32(first + 4), 5);
        assert_eq!(e.mem.u32(second + 4), 4);
    }

    /// An empty item count: no step and no remove-all, only the clear first.
    #[test]
    fn an_empty_map_is_not_stepped() {
        let mut e = engine(0, 0x0001_0200, vec![]);
        e.call(0x00b6_2a20, &args![]);
        assert_eq!(calls_to(&e, CLEAR_00B62970).len(), 1);
        assert!(calls_to(&e, MAP_GET_NEXT_00E6E630).is_empty());
        assert!(calls_to(&e, MAP_REMOVE_ALL_00B601A0).is_empty());
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT_00B62A20).is_empty());
    }

    /// A count but no bucket with an item: no step, but the remove-all runs.
    #[test]
    fn no_first_item_still_removes_all() {
        let mut e = engine(1, 0, vec![]);
        e.call(0x00b6_2a20, &args![]);
        assert!(calls_to(&e, MAP_GET_NEXT_00E6E630).is_empty());
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![MAP]]);
    }

    /// An item whose byte +0x32 is set keeps what it holds (and is still the local's at
    /// the end).
    #[test]
    fn a_marked_item_is_skipped() {
        let item = 0x0001_1000u32;
        let mut e = engine(1, 0x0001_0200, vec![item]);
        object(&mut e, item, 5);
        e.mem.set_u8(item + 0x32, 1);
        object(&mut e, 0x0001_2000, 1);
        e.mem.set_u32(item + 8, 0x0001_2000);
        e.mem.set_u32(item + BLOCK, 0xb10c);
        e.call(0x00b6_2a20, &args![]);
        assert!(calls_to(&e, DELETE).is_empty());
        assert!(calls_to(&e, DEALLOCATE_00AA4060).is_empty());
        assert_eq!(e.mem.u32(item + 8), 0x0001_2000);
        assert_eq!(e.mem.u32(item + BLOCK), 0xb10c);
        assert_eq!(e.mem.u32(item + 4), 4);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![MAP]]);
    }
}
