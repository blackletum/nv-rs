//! `NiMain/run_00a68bf0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// An `NiAVObject`'s parent (`+0x18`), flags (`+0x30`), local transform (`+0x34`) and world
/// transform (`+0x68`); a transform is 0x34 bytes (13 words).
const PARENT: u32 = 0x18;
const FLAGS: u32 = 0x30;
const LOCAL: u32 = 0x34;
const WORLD: u32 = 0x68;
const TRANSFORM_WORDS: u32 = 13;
/// `0062c250` (`thiscall` on a transform, two stack words: the result, the other transform):
/// the product of two transforms; answers the result.
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00a68bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; `NiAVObject::
/// UpdateWorldData` `00a68c60` calls it): the world transform from the parent's. Without a
/// parent it is the local transform; with one, when flag bit 9 is set it is the parent's world
/// transform as it is, else the parent's world transform times the local one (`0062c250` into a
/// 0x34-byte stack buffer); 13 words copied (`REP MOVSD`) into `+0x68`.
pub fn fn_00a68bf0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let parent = e.mem.u32(base + PARENT);
    if parent == 0 {
        copy_words(e, base + WORLD, base + LOCAL, TRANSFORM_WORDS);
        return;
    }
    if (e.mem.u32(base + FLAGS) >> 9) & 1 != 0 {
        copy_words(e, base + WORLD, parent + WORLD, TRANSFORM_WORDS);
        return;
    }
    e.with_stack(0x34, |e, buffer| {
        let product = e
            .call(
                TRANSFORM_PRODUCT,
                &args![parent + WORLD, buffer, base + LOCAL],
            )
            .u32();
        copy_words(e, base + WORLD, product, TRANSFORM_WORDS);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_8bf0, fn_00a68bf0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_world_transform_comes_from_the_local_or_the_parent() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        let (child, parent) = (0x1000u32, 0x1200u32);
        for i in 0..TRANSFORM_WORDS {
            e.mem.set_u32(child + LOCAL + 4 * i, 0x100 + i);
        }
        // No parent: the local transform.
        e.call(0x00a6_8bf0, &args![child]);
        assert_eq!(e.mem.u32(child + WORLD), 0x100);
        assert_eq!(e.mem.u32(child + WORLD + 48), 0x10c);
        // A parent, bit 9: the parent's world transform.
        e.mem.set_u32(child + PARENT, parent);
        for i in 0..TRANSFORM_WORDS {
            e.mem.set_u32(parent + WORLD + 4 * i, 0x200 + i);
        }
        e.mem.set_u32(child + FLAGS, 1 << 9);
        e.call(0x00a6_8bf0, &args![child]);
        assert_eq!(e.mem.u32(child + WORLD + 4), 0x201);
        // Otherwise the product.
        e.mem.set_u32(child + FLAGS, 0);
        e.register(TRANSFORM_PRODUCT, |e, a| {
            assert_eq!(a[0], 0x1200 + WORLD);
            assert_eq!(a[2], 0x1000 + LOCAL);
            for i in 0..TRANSFORM_WORDS {
                e.mem.set_u32(a[1] + 4 * i, 0x300 + i);
            }
            a[1].into_ret()
        });
        e.call(0x00a6_8bf0, &args![child]);
        assert_eq!(e.mem.u32(child + WORLD + 8), 0x302);
    }
}
