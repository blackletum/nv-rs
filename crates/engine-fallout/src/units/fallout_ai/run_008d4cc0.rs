//! `fallout/ai/run_008d4cc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` constant -1.0 the code loads from the exe's data.
const MINUS_ONE: u32 = 0x0101_2054;

// Translated from 008d4e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two armour values of a `Character` (`fArmorRating` at +0x1b8 and
/// `fArmorThreshold` at +0x1bc, Xbox PDB names, same offsets on PC) to -1.0,
/// the constant at `01012054`. The engine map has no name for it.
pub fn fn_008d4e80(e: &mut Engine, this: Ptr) {
    let minus_one: f32 = e.global(MINUS_ONE);
    e.mem.set_f32(this.addr() + 0x1b8, minus_one);
    e.mem.set_f32(this.addr() + 0x1bc, minus_one);
}

/// `operator new` (`00401000`, cdecl, the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The constructor of the 0x0C-byte object (`008d5590`, `ECX` in, `EAX` out).
const CONSTRUCT_008D5590: u32 = 0x008d_5590;

// Translated from 008d5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a 0x0C-byte object (`operator new`), constructs it with `008d5590` when the
/// allocation succeeded, and stores it (or 0) in the word at `this` (`ECX`). The player's
/// constructor reaches it through `008d4360`. The engine map has no name for it.
// Translation incomplete: 008d5020 - FS:[0] exception frame (handler 00eed64b) not translated.
pub fn fn_008d5020(e: &mut Engine, this: Ptr) {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let made = if block == 0 {
        0
    } else {
        e.call(CONSTRUCT_008D5590, &args![block]).u32()
    };
    e.mem.set_u32(this.addr(), made);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d4e80, fn_008d4e80(Ptr)),
        entry!(0x008d5020, fn_008d5020(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_armour_values_get_the_constant() {
        let mut e = Engine::new();
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        let object = e.mem.alloc(0x1d0);
        e.mem.set_f32(object + 0x1b8, 5.0);
        e.mem.set_f32(object + 0x1c0, 6.0);
        e.call(0x008d_4e80, &args![object]);
        assert_eq!(e.mem.f32(object + 0x1b8), -1.0);
        assert_eq!(e.mem.f32(object + 0x1bc), -1.0);
        // Neighbours are untouched.
        assert_eq!(e.mem.f32(object + 0x1c0), 6.0);
    }

    #[test]
    fn the_object_is_allocated_constructed_and_stored() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x10);
        e.mem.set_u32(0x1000, 0xdead_beef);
        e.register(OPERATOR_NEW, |_, a| {
            assert_eq!(a, [0xc]);
            0x2000u32.into_ret()
        });
        e.register(CONSTRUCT_008D5590, |_, a| a[0].into_ret());
        e.call(0x008d_5020, &args![0x1000u32]);
        assert_eq!(e.mem.u32(0x1000), 0x2000);
        // A failed allocation stores 0 without constructing.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.register(CONSTRUCT_008D5590, |_, _| panic!("constructed nothing"));
        e.call(0x008d_5020, &args![0x1000u32]);
        assert_eq!(e.mem.u32(0x1000), 0);
    }
}
