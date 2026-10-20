//! `(unplaced)/run_00e80390` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, one word), the Ni
/// array allocation `00aa1420` (`cdecl`, the byte count), the compiler's `vector constructor
/// iterator` (`00401050`, `stdcall`: array, element size, count, constructor) and the pass
/// constructor `00e800f0` (unit `nid3dpass.cpp`, unnamed).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const NI_ARRAY_ALLOCATE_00AA1420: u32 = 0x00aa_1420;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const PASS_CONSTRUCT_00E800F0: u32 = 0x00e8_00f0;

// Translated from 00e80390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit): the pass pool's pass array maker, `cdecl` with one
/// word: the count. `operator new[]` of that many 0x68-byte passes after a count word: 4 bytes
/// from the memory manager for a count of 0, else `count * 0x68 + 4` bytes from `00aa1420`
/// (all ones when the product or the sum overflows). Null when that fails; otherwise the count
/// is stored, the passes are built (`vector constructor iterator` with `00e800f0`) and the
/// address after the count is answered.
pub fn fn_00e80390(e: &mut Engine, count: u32) -> u32 {
    let block = if count == 0 {
        e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32()
    } else {
        let product = u64::from(count) * 0x68;
        let bytes = if product > u64::from(u32::MAX) {
            u32::MAX
        } else {
            product as u32
        };
        let total = bytes.checked_add(4).unwrap_or(u32::MAX);
        e.call(NI_ARRAY_ALLOCATE_00AA1420, &args![total]).u32()
    };
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![block + 4, 0x68u32, count, PASS_CONSTRUCT_00E800F0],
    );
    block + 4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_0390, fn_00e80390(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |e, a| {
            if a[0] <= 0x1000 {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(VECTOR_CONSTRUCT_00401050, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e80390_makes_the_counted_pass_array() {
        let mut e = engine();
        let passes = e.call(0x00e8_0390, &args![2u32]).u32();
        assert_eq!(e.mem.u32(passes - 4), 2);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420), vec![vec![0xd4]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![passes, 0x68, 2, PASS_CONSTRUCT_00E800F0]]
        );
        let empty = e.call(0x00e8_0390, &args![0u32]).u32();
        assert_eq!(e.mem.u32(empty - 4), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 4]]);
    }

    #[test]
    fn fn_00e80390_overflowing_sizes_ask_for_all_ones() {
        let mut e = engine();
        assert_eq!(e.call(0x00e8_0390, &args![0x0300_0000u32]).u32(), 0);
        let asked: Vec<u32> = calls(&e, NI_ARRAY_ALLOCATE_00AA1420)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(asked, vec![u32::MAX]);
        assert!(calls(&e, VECTOR_CONSTRUCT_00401050).is_empty());
    }
}
