//! `BSSystemUtilities/run_00af2d60` (the engine map's run of unnamed functions from
//! `00af2d60`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md).
//!
//! Not translated here: `00af2db0` (the network utility's idle step), which calls the
//! `steam_api.dll` imports `SteamAPI_IsSteamRunning` and `SteamUser`, imports with no platform
//! layer.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00af2d60 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00af2d60(_e: &mut Engine, _this: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_2d60, fn_00af2d60(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x55);
        e.call(0x00af_2d60, &args![this]);
        assert_eq!(e.mem.u32(this), 0x55);
    }
}
