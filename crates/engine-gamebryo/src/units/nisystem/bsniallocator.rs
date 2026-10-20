//! `NiSystem/bsniallocator.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa2240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Allocate` (Xbox PDB). Kind 7 reserves the byte alignment
/// in addition to the requested size, adjusts the returned address using
/// byte arithmetic and stores that adjustment immediately before it. Other
/// kinds forward the size and final three context words to `00aa4030`.
/// Both paths reload the size after allocation and zero that many bytes.
#[allow(clippy::too_many_arguments)] // Native receiver plus seven stack words.
pub fn allocate(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_4: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> Ptr {
    let buffer = if kind == 7 {
        let alignment = e.mem.u8(alignment.addr());
        let bytes = e.mem.u32(size.addr()).wrapping_add(alignment as u32);
        let raw = e.call(0x00aa_3e40, &args![0x011f_6238u32, bytes]).u32();
        let adjustment = alignment.wrapping_sub(alignment.wrapping_sub(1) & raw as u8);
        let aligned = raw.wrapping_add(adjustment as u32);
        e.mem.set_u8(aligned.wrapping_sub(1), adjustment);
        aligned
    } else {
        let bytes = e.mem.u32(size.addr());
        e.call(
            0x00aa_4030,
            &args![0x011f_6238u32, bytes, context_1, context_2, context_3],
        )
        .u32()
    };
    let bytes = e.mem.u32(size.addr());
    e.call(0x00ec_61c0, &args![buffer, 0u32, bytes]);
    Ptr::new(buffer)
}

// Translated from 00aa22b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Deallocate` (engine map; thiscall, `RET 0xc`, `this` and the third word
/// unread): frees `memory` through the memory manager (`00aa4060` on `011f6238`); for kind 10
/// the block starts the byte stored just before `memory` earlier (and a null `memory` frees
/// nothing).
pub fn deallocate(e: &mut Engine, _this: Ptr, memory: Ptr, kind: u32, _unused_3: u32) {
    if kind == 10 {
        if memory.addr() == 0 {
            return;
        }
        let adjustment = u32::from(e.mem.u8(memory.addr().wrapping_sub(1)));
        let block = memory.addr().wrapping_sub(adjustment);
        e.call(0x00aa_4060, &args![0x011f_6238u32, block]);
        return;
    }
    e.call(0x00aa_4060, &args![0x011f_6238u32, memory]);
}

/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, one word), aligned
/// reallocate (`00aa4150`, two words) and reallocate (`00aa4200`, five words).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MM_FREE: u32 = 0x00aa_4060;
const MM_REALLOCATE_RAW: u32 = 0x00aa_4150;
const MM_REALLOCATE: u32 = 0x00aa_4200;
/// The vtable the deleting destructor leaves (`010a24d0`).
const BASE_VTABLE: u32 = 0x010a_24d0;

/// The shape of `Deallocate` and `DeallocateExternal`: for the aligned kind `aligned_kind`
/// the block's adjustment byte (`block - 1`) is taken off before freeing and a null block is
/// not freed; any other kind frees the block as it is (null included).
fn free_by_kind(e: &mut Engine, block: u32, kind: u32, aligned_kind: u32) {
    if kind == aligned_kind {
        if block == 0 {
            return;
        }
        let adjustment = e.mem.u8(block.wrapping_sub(1)) as u32;
        e.call(
            MM_FREE,
            &args![MEMORY_MANAGER, block.wrapping_sub(adjustment)],
        );
    } else {
        e.call(MM_FREE, &args![MEMORY_MANAGER, block]);
    }
}

// Translated from 00aa23e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::DeallocateExternal` (engine map; `thiscall`, `ECX` unused, three stack
/// words, `RET 0xc`): as `Deallocate` with the aligned kind 0x10.
pub fn deallocate_external(e: &mut Engine, _this: Ptr, block: u32, kind: u32, _size: u32) {
    free_by_kind(e, block, kind, 0x10);
}

// Translated from 00aa22f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Reallocate` (engine map; `thiscall`, `ECX` unused, nine stack words,
/// `RET 0x24`: block, size pointer, alignment pointer, kind, two unused words, three context
/// words). Kind 8 (aligned): a null block answers 0; otherwise the raw start (block minus its
/// adjustment byte) is reallocated to size + alignment (`00aa4150`) and aligned again as
/// `Allocate` does, the new adjustment stored before the answer. Other kinds go to
/// `00aa4200` with the block, the size and the three context words.
#[allow(clippy::too_many_arguments)] // Native receiver plus nine stack words.
pub fn reallocate(
    e: &mut Engine,
    _this: Ptr,
    block: u32,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_5: u32,
    _unused_6: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> u32 {
    if kind != 8 {
        let bytes = e.mem.u32(size.addr());
        return e
            .call(
                MM_REALLOCATE,
                &args![
                    MEMORY_MANAGER,
                    block,
                    bytes,
                    context_1,
                    context_2,
                    context_3
                ],
            )
            .u32();
    }
    let bytes = e.mem.u32(size.addr());
    let align = e.mem.u8(alignment.addr());
    if block == 0 {
        return 0;
    }
    let adjustment = e.mem.u8(block.wrapping_sub(1)) as u32;
    let raw = block.wrapping_sub(adjustment);
    let moved = e
        .call(
            MM_REALLOCATE_RAW,
            &args![MEMORY_MANAGER, raw, (align as u32).wrapping_add(bytes)],
        )
        .u32();
    let new_adjustment = align.wrapping_sub(align.wrapping_sub(1) & moved as u8);
    let aligned = moved.wrapping_add(new_adjustment as u32);
    e.mem.set_u8(aligned.wrapping_sub(1), new_adjustment);
    aligned
}

// Translated from 00aa24a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The allocator's scalar deleting destructor (vtable slot 0; `thiscall`, one stack word,
/// `RET 4`): stores the vtable `010a24d0`, and with bit 0 of `flags` frees `this` through the
/// memory manager (`00aa4060`). Answers `this`.
pub fn deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(MM_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa2240,
            allocate(Ptr, Ptr<u32>, Ptr<u8>, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_22b0, deallocate(Ptr, Ptr, u32, u32)),
        entry!(0x00aa_23e0, deallocate_external(Ptr, u32, u32, u32)),
        entry!(
            0x00aa_22f0,
            reallocate(Ptr, u32, Ptr<u32>, Ptr<u8>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00aa_24a0, deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_allocation_forwards_context_and_reloads_size_before_zeroing() {
        let mut e = Engine::new();
        let size = e.mem.alloc(4);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(size, 4);
        e.mem.write(buffer, &[0xcc; 16]);
        e.register_double(0x00aa4030, move |e, args| {
            assert_eq!(args, &[0x011f6238, 4, 12, 13, 14]);
            e.mem.set_u32(size, 8);
            buffer.into_ret()
        });
        // Alignment is unread outside kind 7; null is intentional.
        let result = e.call(0x00aa2240, &[0, size, 0, 5, 11, 12, 13, 14]).u32();
        assert_eq!(result, buffer);
        assert_eq!(e.mem.bytes(buffer, 8), [0; 8]);
        assert_eq!(e.mem.bytes(buffer + 8, 8), [0xcc; 8]);
    }

    #[test]
    fn aligned_allocation_uses_low_alignment_byte_and_preserves_guards() {
        for raw_offset in [0u32, 3, 8] {
            let mut e = Engine::new();
            let size = e.mem.alloc(4);
            let alignment = e.mem.alloc(4);
            let backing = e.mem.alloc(64);
            e.mem.set_u32(size, 4);
            e.mem.set_u32(alignment, 0xaaaaaa10);
            e.mem.write(backing, &[0xcc; 64]);
            let raw = backing + raw_offset;
            e.register_double(0x00aa3e40, move |e, args| {
                assert_eq!(args, &[0x011f6238, 20]);
                // PC caches alignment before this call, but reloads size.
                e.mem.set_u8(alignment, 8);
                e.mem.set_u32(size, 5);
                raw.into_ret()
            });
            let result = e
                .call(0x00aa2240, &[0, size, alignment, 7, 11, 12, 13, 14])
                .u32();
            assert_eq!(result & 15, 0);
            assert!(result > raw);
            let adjustment = result - raw;
            assert_eq!(e.mem.u8(result - 1), adjustment as u8);
            assert_eq!(e.mem.u8(result - 2), 0xcc);
            assert_eq!(e.mem.bytes(result, 5), [0; 5]);
            assert_eq!(e.mem.u8(result + 5), 0xcc);
        }
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn frees(e: &mut Engine, memory: u32, kind: u32) -> Vec<Vec<u32>> {
        e.register(0x00aa_4060, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_22b0, &args![0x1234u32, memory, kind, 0x40u32]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00aa_4060)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn deallocate_frees_through_the_memory_manager() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x20);
        assert_eq!(frees(&mut e, block, 3), vec![vec![0x011f_6238, block]]);
        assert_eq!(frees(&mut e, 0, 3), vec![vec![0x011f_6238, 0]]);
        // Kind 10: the adjustment byte before the memory gives the block's start.
        e.mem.set_u8(block + 5, 6);
        assert_eq!(frees(&mut e, block + 6, 10), vec![vec![0x011f_6238, block]]);
        assert!(frees(&mut e, 0, 10).is_empty());
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        e.register(MM_FREE, |_, _| Ret::default());
        e.register(MM_REALLOCATE_RAW, |_, a| (a[1] + 0x100).into_ret());
        e.register(MM_REALLOCATE, |_, _| 0x1800u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The aligned kinds (10, and 0x10 for the external one) free from the raw start; other
    /// kinds free the block itself, null included; a null aligned block frees nothing.
    #[test]
    fn deallocate_takes_the_adjustment_off_for_the_aligned_kind() {
        let mut e = engine();
        e.mem.set_u8(0x1103, 3);
        e.call(0x00aa_22b0, &args![0u32, 0x1104u32, 10u32, 0u32]);
        e.call(0x00aa_22b0, &args![0u32, 0x1104u32, 1u32, 0u32]);
        e.call(0x00aa_22b0, &args![0u32, 0u32, 10u32, 0u32]);
        e.call(0x00aa_22b0, &args![0u32, 0u32, 1u32, 0u32]);
        e.call(0x00aa_23e0, &args![0u32, 0x1104u32, 0x10u32, 0u32]);
        e.call(0x00aa_23e0, &args![0u32, 0x1104u32, 10u32, 0u32]);
        assert_eq!(
            calls(&e, MM_FREE),
            [
                vec![MEMORY_MANAGER, 0x1101],
                vec![MEMORY_MANAGER, 0x1104],
                vec![MEMORY_MANAGER, 0],
                vec![MEMORY_MANAGER, 0x1101],
                vec![MEMORY_MANAGER, 0x1104],
            ]
        );
    }

    /// Kind 8 reallocates the raw block to size + alignment and aligns the answer again;
    /// other kinds go to `00aa4200` with the size and the three context words.
    #[test]
    fn reallocate_realigns_the_aligned_kind() {
        let mut e = engine();
        e.mem.set_u32(0x1000, 0x40);
        e.mem.set_u8(0x1004, 0x10);
        e.mem.set_u8(0x1103, 4);
        // Raw 0x1100 moves to 0x1200 (the double adds 0x100): 0x10 - (0xf & 0x00) = 0x10.
        let got = e.call(
            0x00aa_22f0,
            &args![0u32, 0x1104u32, 0x1000u32, 0x1004u32, 8u32, 0u32, 0u32, 1u32, 2u32, 3u32],
        );
        assert_eq!(got.u32(), 0x1210);
        assert_eq!(e.mem.u8(0x120f), 0x10);
        assert_eq!(
            calls(&e, MM_REALLOCATE_RAW),
            [vec![MEMORY_MANAGER, 0x1100, 0x50]]
        );
        let null = e.call(
            0x00aa_22f0,
            &args![0u32, 0u32, 0x1000u32, 0x1004u32, 8u32, 0u32, 0u32, 1u32, 2u32, 3u32],
        );
        assert_eq!(null.u32(), 0);
        let other = e.call(
            0x00aa_22f0,
            &args![0u32, 0x1104u32, 0x1000u32, 0x1004u32, 2u32, 0u32, 0u32, 1u32, 2u32, 3u32],
        );
        assert_eq!(other.u32(), 0x1800);
        assert_eq!(
            calls(&e, MM_REALLOCATE),
            [vec![MEMORY_MANAGER, 0x1104, 0x40, 1, 2, 3]]
        );
    }

    #[test]
    fn the_deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        assert_eq!(e.call(0x00aa_24a0, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), BASE_VTABLE);
        assert!(calls(&e, MM_FREE).is_empty());
        e.call(0x00aa_24a0, &args![0x1000u32, 1u32]);
        assert_eq!(calls(&e, MM_FREE), [vec![MEMORY_MANAGER, 0x1000]]);
    }
}
