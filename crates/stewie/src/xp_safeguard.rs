//! lStewieAl's Tweaks' row S0185 (`Fixes/InvalidRewardXPSafeGuard.cpp`, `InvalidRewardXPSafeGuard`):
//! an XP award of at least `[Fixes] iInvalidRewardXPSafeGuardThreshold` outside the console and
//! outside `SetLevel` waits for the player's answer to a message box. Its four statics are kept
//! words (`engine_core::extend::stewie::kept`), so they are run-wide as Tweaks' are.

use engine_core::extend::stewie::{console_print, kept, value};
use engine_core::extend::{Owner, SiteSlot, ROUTINE_BASE};
use engine_core::prelude::*;

/// The seam of the XP reward's call of `CharacterProgression::RewardExperience` (`0x94FD24`, in
/// `0094fd10`): Tweaks' `OnRewardXP`.
pub const REWARD_SITE: SiteSlot = SiteSlot::new(Owner::Stewie, 0x0094_fd24);
/// The seam after `PlayerCharacter::Update`'s `UpdateMenuModeButton` call (`0x940C78`): the player
/// update callbacks Tweaks adds while running (S0185's `CheckCanShowUIWarningCallback`).
pub const UPDATE_SITE: SiteSlot = SiteSlot::new(Owner::Stewie, 0x0094_0c78);

/// Tweaks' own code addresses the game calls: a block of the extenders' routine range of its own
/// (xNVSE's routines are at the range's start, JIP LN NVSE's from +0x1000, JohnnyGuitar's from
/// +0xE000).
pub const ROUTINES: u32 = ROUTINE_BASE + 0xa000;
/// `UIWarningCallback`, the callback the message box is shown with.
pub const WARNING_CALLBACK: u32 = ROUTINES;

const _: () = assert!(ROUTINES + 0x100 <= engine_core::mem::EXTENDER_CODE.end);

/// `CharacterProgression::RewardExperience` (`thiscall`, the amount; it does not read `this`).
const REWARD_EXPERIENCE: u32 = 0x008d_5100;
/// The game's `ShowMessageBox` (`cdecl`: the message, 0, the title, the callback, 0, 0x17, two
/// `float`s, the buttons' texts and a 0).
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
/// The interface manager's pointer and its `msgBoxButton` (`+0xE4`, -1 for none).
const INTERFACE_MANAGER: u32 = 0x011d_8a80;
const MESSAGE_BOX_BUTTON: u32 = 0xe4;
/// The player's pointer and the word at `+0x878` Tweaks' header calls `LevelupInfo* levelupInfo`
/// (the game keeps the record itself there: `0094fd10` passes `player + 0x878`).
const PLAYER: u32 = 0x011d_ea3c;
const LEVELUP_INFO: u32 = 0x878;
/// `LevelupInfo::bInSetLevelCommand` as Tweaks reads it (`+8` of the record it is given).
const IN_SET_LEVEL_COMMAND: u32 = 8;

/// The message's text before its amount and after it (Tweaks' `snprintf` format around `%i`).
const TEXT_BEFORE: &str = "The game is attempting to award you an unusually large amount of XP (";
const TEXT_AFTER: &str = "). This is likely due to a bug in some mod's scripts.\nWould you like to accept the XP anyway?";

/// The seam handlers and the callback, put in at Tweaks' install.
pub(crate) fn install(e: &mut Engine) {
    e.on_site(REWARD_SITE, on_reward_xp);
    e.on_site(UPDATE_SITE, on_player_update);
    e.register(WARNING_CALLBACK, warning_callback);
}

// Translated from lStewieAl's Tweaks 10.05 (Fixes/InvalidRewardXPSafeGuard.cpp): OnRewardXP.
/// Words: the level-up record (`ECX`) and the amount. An amount of at least the threshold, outside
/// console mode and with the record's `bInSetLevelCommand` byte 0: unless "No" was answered
/// before, it waits for an answer (a second one while one waits is dropped with a console line);
/// any other amount is awarded. Always answers: the call is Tweaks'.
pub(crate) fn on_reward_xp(e: &mut Engine, a: &[u32]) -> Option<u32> {
    let (levelup, amount) = (a[0], a[1] as i32);
    let threshold = e.stewie_value(value::XP_SAFEGUARD_THRESHOLD) as i32;
    if amount >= threshold
        && !nvse::commands::is_console_mode(e)
        && e.mem.u8(levelup + IN_SET_LEVEL_COMMAND) == 0
    {
        let reject = e.stewie_kept(kept::XP_REJECT_ALL, 1);
        if e.mem.u8(reject) == 0 {
            let attempted = e.stewie_kept(kept::XP_ATTEMPTED_REWARD, 4);
            if e.mem.u32(attempted) != 0 {
                console_print(
                    e,
                    b"[Tweaks] XP Safeguard: ignored a second large XP award (%i) while a confirmation is pending",
                    &[amount as u32],
                );
                return Some(0);
            }
            e.mem.set_u32(attempted, amount as u32);
            let needs = e.stewie_kept(kept::XP_NEEDS_MESSAGE, 1);
            e.mem.set_u8(needs, 1);
            let registered = e.stewie_kept(kept::XP_CALLBACK_REGISTERED, 1);
            e.mem.set_u8(registered, 1);
        }
    } else {
        e.call(REWARD_EXPERIENCE, &args![levelup, amount]);
    }
    Some(0)
}

// Translated from lStewieAl's Tweaks 10.05 (Fixes/InvalidRewardXPSafeGuard.cpp):
// CheckCanShowUIWarningCallback.
/// Once its callback is in the list and a message is due: the message box ("Yes", "No") with the
/// amount (scaled by `[Reduced XP] fXPMultiplier` with `[Tweaks] bReduceXP`, as an `int`).
pub(crate) fn on_player_update(e: &mut Engine, _: &[u32]) -> Option<u32> {
    let registered = e.stewie_kept(kept::XP_CALLBACK_REGISTERED, 1);
    let needs = e.stewie_kept(kept::XP_NEEDS_MESSAGE, 1);
    if e.mem.u8(registered) == 0 || e.mem.u8(needs) == 0 {
        return None;
    }
    e.mem.set_u8(needs, 0);
    let attempted = e.stewie_kept(kept::XP_ATTEMPTED_REWARD, 4);
    let attempted = e.mem.u32(attempted) as i32;
    let reward = if e.stewie_value(value::REDUCE_XP) != 0 {
        let multiplier = f32::from_bits(e.stewie_value(value::XP_MULTIPLIER));
        float_to_int(attempted as f32 * multiplier)
    } else {
        attempted
    };
    let text = format!("{TEXT_BEFORE}{reward}{TEXT_AFTER}");
    // `snprintf` into `char buf[0x100]`: at most 255 bytes and the 0.
    let text = &text.as_bytes()[..text.len().min(0xff)];
    let title = nvse::table::guest_str(e, b"Stewie Tweaks XP Safeguard");
    let yes = nvse::table::guest_str(e, b"Yes");
    let no = nvse::table::guest_str(e, b"No");
    e.with_stack(0x100, |e, buffer| {
        let at = buffer.addr();
        for (i, b) in text.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *b);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
        // Tweaks passes the `int`s 0 and 0 where the two `float`s go.
        e.call(
            SHOW_MESSAGE_BOX,
            &args![
                at,
                0u32,
                title,
                WARNING_CALLBACK,
                0u32,
                0x17u32,
                0u32,
                0u32,
                yes,
                no,
                0u32
            ],
        );
    });
    None
}

// Translated from lStewieAl's Tweaks 10.05 (Fixes/InvalidRewardXPSafeGuard.cpp): UIWarningCallback.
/// The answer (`GetMessageBoxResult`: the byte taken, -1 left): "Yes" (0) awards the waiting
/// amount (`RewardExperience` given the word at the player's `+0x878` as Tweaks reads it), "No"
/// drops it and every later large award. Nothing waits after it.
pub(crate) fn warning_callback(e: &mut Engine, _: &[u32]) -> Ret {
    let interface = e.mem.u32(INTERFACE_MANAGER);
    let button = e.mem.u8(interface + MESSAGE_BOX_BUTTON);
    e.mem.set_u8(interface + MESSAGE_BOX_BUTTON, 0xff);
    let attempted = e.stewie_kept(kept::XP_ATTEMPTED_REWARD, 4);
    if button == 0 {
        let player = e.mem.u32(PLAYER);
        let levelup = e.mem.u32(player + LEVELUP_INFO);
        let amount = e.mem.u32(attempted);
        e.call(REWARD_EXPERIENCE, &args![levelup, amount]);
    } else {
        let reject = e.stewie_kept(kept::XP_REJECT_ALL, 1);
        e.mem.set_u8(reject, 1);
    }
    e.mem.set_u32(attempted, 0);
    Ret::default()
}

/// A `float` stored to an `int` (`cvttss2si`): toward zero; a NaN or one out of range is
/// `0x80000000`.
fn float_to_int(v: f32) -> i32 {
    if v.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&v) {
        i32::MIN
    } else {
        v as i32
    }
}
