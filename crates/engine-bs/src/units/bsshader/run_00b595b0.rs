//! `BSShader/run_00b595b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The vtables the two constructors store (`010adc48`, `010adc68`; no RTTI locator before
/// them).
const VTABLE_010ADC48: u32 = 0x010a_dc48;
const VTABLE_010ADC68: u32 = 0x010a_dc68;
/// The `float` constants they copy: 0.5 (`01016248`), 1000.0 (`01013974`), the word at
/// `012023e0` and 600.0 (`0101786c`) (values read in Ghidra).
const HALF_01016248: u32 = 0x0101_6248;
const CONSTANT_01013974: u32 = 0x0101_3974;
const CONSTANT_012023E0: u32 = 0x0120_23e0;
const CONSTANT_0101786C: u32 = 0x0101_786c;
/// The bits of 1.0 (`FLD1`).
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b5a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc48`, sizes
/// the array at +0x28 to 2 (`00a90180`) and copies the `float` at `01016248` (`FLD`/`FST`, a
/// signalling NaN quieted) to +0x6c and +0x70. Answers `this`.
// Translation incomplete: 00b5a410 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b5a410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC48);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.mem.set_u32(base + 0x6c, half);
    e.mem.set_u32(base + 0x70, half);
    this
}

// Translated from 00b5a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc68`, stores
/// 0.0 at +0x5c to +0x98, clears the bytes +0xa4 to +0xa6 and the words +0xa8 to +0xb0,
/// constructs the two 16-byte textures at +0xb4 (`eh vector constructor iterator` with
/// `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 4
/// (`00a90180`), then stores (through the FPU, signalling NaNs quieted) 1.0 at +0x7c, the
/// `float` at `01013974` at +0x80, 0.0 at +0x84 and +0x6c, the word at `012023e0` at +0x88,
/// the `float` at `0101786c` at +0x9c, 1.0 at +0x70 and +0x74, 0.0 at +0x78, and clears +0x58.
/// Answers `this`.
// Translation incomplete: 00b5a590 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00b5a590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC68);
    for offset in (0x5cu32..=0x98).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0xa4u32, 0xa5, 0xa6] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0xa8u32, 0xac, 0xb0] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0xb4,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    let thousand = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
    let copied = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
    let six_hundred = x87_quiet_bits(e.global::<u32>(CONSTANT_0101786C));
    e.mem.set_u32(base + 0x7c, ONE_BITS);
    e.mem.set_u32(base + 0x80, thousand);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x6c, 0);
    e.mem.set_u32(base + 0x88, copied);
    e.mem.set_u32(base + 0x9c, six_hundred);
    e.mem.set_u32(base + 0x70, ONE_BITS);
    e.mem.set_u32(base + 0x74, ONE_BITS);
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x58, 0);
    this
}

/// The map getter `00b58670` (unnamed; no arguments, plain `RET`: answers the static map at
/// `011f9670`, made on first use: vtable +0, bucket count +4, buckets +8, item count +0xc), the
/// clear that runs first, `00b59890` (unnamed; no arguments, plain `RET`), the map step
/// `00e6e630` (`thiscall`, `RET 0xc`: iterator, key and `NiPointer` value addresses), the map's
/// remove-all `00b601a0` (`thiscall`, no stack words) and `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on `011f6238`, one word).
const MAP_00B58670: u32 = 0x00b5_8670;
const CLEAR_00B59890: u32 = 0x00b5_9890;
const MAP_GET_NEXT_00E6E630: u32 = 0x00e6_e630;
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;
const DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
/// The imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT_00B59940: u32 = 0x00fd_f1c4;

/// An `NiPointer` release as the exe inlines it: the count at +4 down (`InterlockedDecrement`)
/// and, at zero, vtable slot +4 on the object (`thiscall`, no stack words).
fn release_00b59940(e: &mut Engine, object: u32) {
    if e.call(
        IMPORT_INTERLOCKED_DECREMENT_00B59940,
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
fn release_field_00b59940(e: &mut Engine, item: u32, offset: u32) {
    let held = e.mem.u32(item.wrapping_add(offset));
    if held != 0 {
        release_00b59940(e, held);
        e.mem.set_u32(item.wrapping_add(offset), 0);
    }
}

// Translated from 00b59940 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; no arguments, plain `RET`; called from
/// `00b54630` and from `00453dc0` at `0045426d`): runs `00b59890`, then empties the map
/// `00b58670` answers. The iterator starts at the first item of the first non-empty bucket
/// (the bucket count and array read once); when the map's item count (+0xc) is not zero,
/// each item is stepped to with `00e6e630` (the map fetched again each time) into an
/// `NiPointer` local, and an item that is not null has its held objects released: +8, then
/// +0x10 deallocated, then +0x18, +0x20, +0x1c and +0x24, each released word cleared (the
/// deallocated one always). The steps go on while the iterator is not null (none when the
/// first one is null); then the map's remove-all `00b601a0` runs. Last the local releases
/// the item it holds.
// Translation incomplete: 00b59940 - the C++ exception frame (FS:[0] chain, handler `00f273e8`, unwind state 0 for the NiPointer local) is not translated; the security cookie is pushed but never checked (no 00ec408c call).
pub fn fn_00b59940(e: &mut Engine) {
    e.call(CLEAR_00B59890, &args![]);
    let map = e.call(MAP_00B58670, &args![]).u32();
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
        let map = e.call(MAP_00B58670, &args![]).u32();
        if e.mem.u32(map.wrapping_add(0xc)) != 0 {
            if first != 0 {
                loop {
                    let map = e.call(MAP_00B58670, &args![]).u32();
                    e.call(MAP_GET_NEXT_00E6E630, &args![map, iterator, key, item_slot]);
                    item = e.mem.u32(item_slot);
                    if item != 0 {
                        release_field_00b59940(e, item, 8);
                        let block = e.mem.u32(item.wrapping_add(0x10));
                        e.call(DEALLOCATE_00AA4060, &args![MEMORY_MANAGER_011F6238, block]);
                        e.mem.set_u32(item.wrapping_add(0x10), 0);
                        release_field_00b59940(e, item, 0x18);
                        release_field_00b59940(e, item, 0x20);
                        release_field_00b59940(e, item, 0x1c);
                        release_field_00b59940(e, item, 0x24);
                    }
                    if e.mem.u32(iterator) == 0 {
                        break;
                    }
                }
            }
            let map = e.call(MAP_00B58670, &args![]).u32();
            e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
        }
        if item != 0 {
            release_00b59940(e, item);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_9940, fn_00b59940()),
        entry!(0x00b5_a410, fn_00b5a410(Ptr) -> Ptr),
        entry!(0x00b5_a590, fn_00b5a590(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the base constructor, the set-size and the vector constructor doubled
    /// and the constants mapped (a signalling NaN at `012023e0`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        for page in [0x0101_6000u32, 0x0101_3000, 0x0120_2000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(CONSTANT_01013974, 1000.0f32);
        e.set_global(CONSTANT_012023E0, 0x7f80_0001u32);
        e.set_global(CONSTANT_0101786C, 600.0f32);
        e.call_log = Some(vec![]);
        e
    }

    fn log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    #[test]
    fn fn_00b5a410_builds_the_effect() {
        let mut e = engine();
        e.set_global(HALF_01016248, 0x7fa0_0000u32);
        let this = e.mem.alloc(0x74);
        assert_eq!(e.call(0x00b5_a410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC48);
        assert_eq!(e.mem.u32(this + 0x6c), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0x70), 0x7fe0_0000);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 2]),
            ]
        );
    }

    #[test]
    fn fn_00b5a590_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0xd4);
        e.mem.write(this, &[0xcc; 0xd4]);
        assert_eq!(e.call(0x00b5_a590, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC68);
        let floats = |e: &Engine, offset: u32| e.mem.u32(this + offset);
        assert_eq!(floats(&e, 0x58), 0);
        assert_eq!(floats(&e, 0x5c), 0);
        assert_eq!(floats(&e, 0x6c), 0);
        assert_eq!(floats(&e, 0x70), ONE_BITS);
        assert_eq!(floats(&e, 0x74), ONE_BITS);
        assert_eq!(floats(&e, 0x78), 0);
        assert_eq!(floats(&e, 0x7c), ONE_BITS);
        assert_eq!(floats(&e, 0x80), 1000.0f32.to_bits());
        assert_eq!(floats(&e, 0x84), 0);
        assert_eq!(floats(&e, 0x88), 0x7fc0_0001);
        assert_eq!(floats(&e, 0x8c), 0);
        assert_eq!(floats(&e, 0x98), 0);
        assert_eq!(floats(&e, 0x9c), 600.0f32.to_bits());
        assert_eq!(floats(&e, 0xa0), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 0xa4, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0xa7), 0xcc);
        assert_eq!(e.mem.bytes(this + 0xa8, 0xc), vec![0; 0xc]);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0xb4,
                        0x10,
                        2,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}

#[cfg(test)]
mod empty_map_00b59940_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const MAP: u32 = 0x0001_0000;
    const BUCKETS: u32 = 0x0001_0100;
    const DELETE: u32 = 0x00ab_0004;
    const VTABLE: u32 = 0x0001_0f00;
    /// The held fields in the order the exe releases them, and the deallocated one.
    const FIELDS: [u32; 5] = [8, 0x18, 0x20, 0x1c, 0x24];
    const BLOCK: u32 = 0x10;

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
        e.register(MAP_00B58670, |_, _| MAP.into_ret());
        for target in [
            CLEAR_00B59890,
            MAP_REMOVE_ALL_00B601A0,
            DEALLOCATE_00AA4060,
            DELETE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT_00B59940, |e, a| {
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
        e.call(0x00b5_9940, &args![]);
        assert_eq!(calls_to(&e, CLEAR_00B59890).len(), 1);
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
        e.call(0x00b5_9940, &args![]);
        assert_eq!(calls_to(&e, CLEAR_00B59890).len(), 1);
        assert!(calls_to(&e, MAP_GET_NEXT_00E6E630).is_empty());
        assert!(calls_to(&e, MAP_REMOVE_ALL_00B601A0).is_empty());
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT_00B59940).is_empty());
    }

    /// A count but no bucket with an item: no step, but the remove-all runs.
    #[test]
    fn no_first_item_still_removes_all() {
        let mut e = engine(1, 0, vec![]);
        e.call(0x00b5_9940, &args![]);
        assert!(calls_to(&e, MAP_GET_NEXT_00E6E630).is_empty());
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![MAP]]);
    }
}
