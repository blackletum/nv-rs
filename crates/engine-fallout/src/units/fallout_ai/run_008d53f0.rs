//! `fallout/ai/run_008d53f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `GamePlayFormulas::GetRequiredExperiencePoints` (Xbox PDB, `00648b50`, cdecl, one word).
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;

// Translated from 008d5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the 0x0C-byte object `008d5020` allocates for the player: runs its
/// initializer `008d55b0` on it and answers it (`ECX` in, `EAX` out). The engine map has no
/// name for it.
pub fn fn_008d5590(e: &mut Engine, this: Ptr) -> Ptr {
    fn_008d55b0(e, this);
    this
}

// Translated from 008d55b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes that object: the byte at +0 cleared, the word at +4 set to the experience
/// level 2 needs (`00648b50(2)`), the byte at +8 cleared. The engine map has no name for it.
pub fn fn_008d55b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr(), 0);
    let needed = e.call(REQUIRED_EXPERIENCE, &args![2i32]).u32();
    e.mem.set_u32(this.addr() + 4, needed);
    e.mem.set_u8(this.addr() + 8, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d5590, fn_008d5590(Ptr) -> Ptr),
        entry!(0x008d55b0, fn_008d55b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_clears_the_flags_and_keeps_the_level_two_experience() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x10);
        e.mem.write(0x1000, &[0xff; 12]);
        e.register(REQUIRED_EXPERIENCE, |_, a| {
            assert_eq!(a, [2]);
            200u32.into_ret()
        });
        let p = e.call(0x008d_5590, &args![0x1000u32]).u32();
        assert_eq!(p, 0x1000);
        assert_eq!(e.mem.u8(0x1000), 0);
        assert_eq!(e.mem.u32(0x1004), 200);
        assert_eq!(e.mem.u8(0x1008), 0);
        assert_eq!(e.mem.bytes(0x1009, 3), [0xff; 3], "the rest untouched");
    }
}
