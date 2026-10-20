//! `NiMain/niavobject_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object at `+0x1c` whose slot `+0x90` takes the update over, and the plain update.
const UPDATE_OWNER: u32 = 0x1c;
const OWNER_UPDATE_SLOT: u32 = 0x90;
const UPDATE_WORLD_TRANSFORM: u32 = 0x00a6_8bf0;

// Translated from 00a68c60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateWorldData` (engine map; `thiscall`, one stack word, `RET 4`): with an
/// object at `+0x1c`, a tail jump to its slot `+0x90` (on it, with the same word); otherwise
/// the world transform from the parent (`00a68bf0`, which takes no words; the word is
/// dropped by this function's `RET 4`).
pub fn ni_av_object_update_world_data(e: &mut Engine, this: Ptr, data: u32) {
    let owner = e.mem.u32(this.addr() + UPDATE_OWNER);
    if owner != 0 {
        e.vcall(owner, OWNER_UPDATE_SLOT, &args![data]);
    } else {
        e.call(UPDATE_WORLD_TRANSFORM, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a6_8c60,
        ni_av_object_update_world_data(Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_owner_takes_the_update_or_the_transform_is_updated() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        e.register(UPDATE_WORLD_TRANSFORM, |e, a| {
            e.mem.set_u32(0x1800, a[0]);
            Ret::default()
        });
        e.call(0x00a6_8c60, &args![0x1000u32, 7u32]);
        assert_eq!(e.mem.u32(0x1800), 0x1000);
        e.put_vtable(0x0300_0000, &[0; 0x25]);
        e.mem.set_u32(0x0300_0000 + OWNER_UPDATE_SLOT, 0x00ab_0000);
        e.mem.set_u32(0x1100, 0x0300_0000);
        e.mem.set_u32(0x1000 + UPDATE_OWNER, 0x1100);
        e.register(0x00ab_0000, |e, a| {
            e.mem.set_u32(0x1804, a[0]);
            e.mem.set_u32(0x1808, a[1]);
            Ret::default()
        });
        e.call(0x00a6_8c60, &args![0x1000u32, 7u32]);
        assert_eq!((e.mem.u32(0x1804), e.mem.u32(0x1808)), (0x1100, 7));
    }
}
