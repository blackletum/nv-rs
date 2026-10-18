//! `fallout shared/gameplayformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The setting's value getter (`0043d4d0`, `this` = the setting, answers the address of its
/// value).
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// The `iXPBase` setting (default 200, made by the start-up initializer `00f6a6c0`).
const I_XP_BASE: u32 = 0x011d_03e8;
/// The `iXPBumpBase` setting (default 100, made by `00f6a6f0`).
const I_XP_BUMP_BASE: u32 = 0x011d_0ae4;

// Translated from 00648b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetRequiredExperiencePoints` (Xbox PDB, cdecl): the experience a
/// character needs for `level`. 0 up to level 1 (`JG` on `level > 1`, signed); otherwise
/// `iXPBase`, and for each level from 2 to `level - 1` the step grows by `iXPBumpBase` and is
/// added (the loop's counter runs from 1 while below `level - 1`, signed; the sums wrap as
/// the exe's `ADD`s do).
pub fn game_play_formulas_get_required_experience_points(e: &mut Engine, level: i32) -> i32 {
    if level <= 1 {
        return 0;
    }
    let base_at: Ptr = e.call_as(SETTING_VALUE, &args![I_XP_BASE]);
    let mut total = e.mem.i32(base_at.addr());
    let mut step = total;
    let bump_at: Ptr = e.call_as(SETTING_VALUE, &args![I_XP_BUMP_BASE]);
    let bump = e.mem.i32(bump_at.addr());
    let mut i = 1i32;
    while i < level.wrapping_sub(1) {
        step = step.wrapping_add(bump);
        total = total.wrapping_add(step);
        i += 1;
    }
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00648b50,
        game_play_formulas_get_required_experience_points(i32) -> i32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The settings' values at two words the getter double answers.
    fn engine(base: i32, bump: i32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0000_1000, 8);
        e.mem.set_i32(0x1000, base);
        e.mem.set_i32(0x1004, bump);
        e.register(SETTING_VALUE, |_, a| {
            let at = if a[0] == I_XP_BASE { 0x1000u32 } else { 0x1004 };
            at.into_ret()
        });
        e
    }

    #[test]
    fn levels_up_to_one_need_nothing_and_later_ones_add_growing_steps() {
        let mut e = engine(200, 100);
        let need = |e: &mut Engine, level: i32| e.call(0x0064_8b50, &args![level]).i32();
        assert_eq!(need(&mut e, -3), 0);
        assert_eq!(need(&mut e, 1), 0);
        assert_eq!(need(&mut e, 2), 200);
        // 200, then steps 300 and 400.
        assert_eq!(need(&mut e, 3), 500);
        assert_eq!(need(&mut e, 4), 900);
    }

    #[test]
    fn the_sums_wrap_as_the_exe_adds() {
        let mut e = engine(i32::MAX, 1);
        assert_eq!(
            e.call(0x0064_8b50, &args![3i32]).i32(),
            i32::MAX.wrapping_add(i32::MIN)
        );
    }
}
