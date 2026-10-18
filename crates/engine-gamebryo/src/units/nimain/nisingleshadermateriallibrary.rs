//! `NiMain/nisingleshadermateriallibrary.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// `00a8fe70` (`thiscall`, `RET 4`, engine map unit `nisingleshadermateriallibrary.cpp`,
/// unnamed): called with `this` and the address of a fixed-string handle.
const LIBRARY_SETUP: u32 = 0x00a8_fe70;

// Translated from 00a8fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::NiSingleShaderMaterialLibrary` (Xbox PDB; `thiscall`, no
/// stack word): makes the fixed-string handle for the literal at `0109f7d0`, runs
/// `00a8fe70` with `this` and the address of that handle, releases the handle, clears the
/// words at `+0xc` and `+0x10`, sets the vtable `0109f7b4` and returns `this`.
pub fn ni_single_shader_material_library_ni_single_shader_material_library(
    e: &mut Engine,
    this: Ptr,
) -> Ptr {
    let name = e.call(GET_FIXED_STRING, &args![0x0109_f7d0u32]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), name);
        e.call(LIBRARY_SETUP, &args![this, local]);
        let handle = e.mem.u32(local.addr());
        release_fixed_string(e, handle);
    });
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr(), 0x0109_f7b4);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_fce0, ni_single_shader_material_library_ni_single_shader_material_library(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_up_with_the_string_handle_and_releases_it() {
        let mut e = engine_with_counting();
        let this = e.mem.alloc(0x20);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 1);
        e.register_double(GET_FIXED_STRING, move |_, a| {
            assert_eq!(a, &[0x0109_f7d0]);
            handle.into_ret()
        });
        e.register_double(LIBRARY_SETUP, move |e, a| {
            assert_eq!(a[0], this);
            assert_eq!(e.mem.u32(a[1]), handle);
            e.mem.set_u32(this + 0xc, 0x77);
            Ret::default()
        });
        assert_eq!(e.call(0x00a8_fce0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(handle - 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), 0x0109_f7b4);
    }
}
