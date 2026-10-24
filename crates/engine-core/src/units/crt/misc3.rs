//! More C runtime entry points the translated code reaches (MSVC 2008 LIBCMT, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `strtoul` (`00ec8d33`),
//! `__set_error_mode` (`00ee1dac`), `siglookup` (`00eddf8a`), `__validdrive` (`00ecca6c`),
//! `__getcwd` (`00eccbdc`) and `__strupr_s_l` (`00ecbc26`).

use super::*;

/// `strtoxl` (`00ec8ad9`; locale, text, end out, base, unsigned flag), `errno` (`00ec85e3`),
/// the invalid-parameter report (`00ec7c30`), `_lock` (`00eddf39`), `__getdcwd_nolock`
/// (`00eccaa5`; drive, target, size), `__getcwd`'s `__finally` block (`00eccc1c`),
/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`; `ECX` the object) and `_strupr_s_l_stat`
/// (`00ecbaa8`; `EDI` the text, then the size and the `_LocaleUpdate` object).
const STRTOXL: u32 = 0x00ec_8ad9;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const LOCK: u32 = 0x00ed_df39;
const GETDCWD_NOLOCK: u32 = 0x00ec_caa5;
const GETCWD_FINALLY: u32 = 0x00ec_cc1c;
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const STRUPR_S_L_STAT: u32 = 0x00ec_baa8;
/// `__locale_changed` (`0126fdf4`), the initial locale (`__initiallocalestructinfo`,
/// `011c2400`), the error mode (`__error_mode`, `0126fdb4`) and the signal table's length
/// (`_XcptActTabCount`, `011c289c`).
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
const INITIAL_LOCALE: u32 = 0x011c_2400;
const ERROR_MODE: u32 = 0x0126_fdb4;
const SIGNAL_TABLE_COUNT: u32 = 0x011c_289c;
/// The import slot of KERNEL32 `GetDriveTypeA` (`00fdf0e8`).
const GET_DRIVE_TYPE_A: u32 = 0x00fd_f0e8;

// Translated from 00ec8d33 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strtoul` (Ghidra's function ID), cdecl: text, end out, base. `strtoxl` with the unsigned
/// flag, in the initial locale while `__locale_changed` is 0, else with no locale (the
/// thread's).
pub fn strtoul(e: &mut Engine, text: u32, end: u32, base: u32) -> u32 {
    let locale = if e.mem.u32(LOCALE_CHANGED) == 0 {
        INITIAL_LOCALE
    } else {
        0
    };
    e.call(STRTOXL, &args![locale, text, end, base, 1u32]).u32()
}

// Translated from 00ee1dac (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_error_mode` (Ghidra's function ID), cdecl, a mode: 0..2 sets the error mode and
/// answers the old one, 3 answers it unchanged; anything else (signed) sets `errno` to
/// `EINVAL`, reports and answers -1.
pub fn set_error_mode(e: &mut Engine, mode: u32) -> u32 {
    match mode as i32 {
        0..=2 => {
            let old = e.mem.u32(ERROR_MODE);
            e.mem.set_u32(ERROR_MODE, mode);
            old
        }
        3 => e.mem.u32(ERROR_MODE),
        _ => {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0x16);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            0xffff_ffff
        }
    }
}

// Translated from 00eddf8a (disassembled, FalloutNV.exe 1.4.0.525)
/// `siglookup` (Ghidra's function ID), `EDX` the signal, the action table on the stack
/// (0xc-byte entries, the signal at +4, `_XcptActTabCount` of them): the first entry with the
/// signal, or null. The first entry is compared before the count is looked at.
pub fn siglookup(e: &mut Engine, signal: u32, table: u32) -> u32 {
    let count = e.mem.u32(SIGNAL_TABLE_COUNT);
    let end = count.wrapping_mul(0xc).wrapping_add(table);
    let mut entry = table;
    loop {
        if e.mem.u32(entry.wrapping_add(4)) == signal {
            break;
        }
        entry = entry.wrapping_add(0xc);
        if entry >= end {
            break;
        }
    }
    if entry < end && e.mem.u32(entry.wrapping_add(4)) == signal {
        entry
    } else {
        0
    }
}

// Translated from 00ecca6c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__validdrive` (Ghidra's function ID), cdecl, a drive number (1 = A): 0 (the current
/// drive) is valid; otherwise its root (`"X:\"`, written over the argument's stack slot) is
/// valid unless `GetDriveTypeA` answers `DRIVE_UNKNOWN` (0) or `DRIVE_NO_ROOT_DIR` (1).
pub fn validdrive(e: &mut Engine, drive: u32) -> u32 {
    if drive == 0 {
        return 1;
    }
    e.with_stack(4, |e, root| {
        let root = root.addr();
        e.mem.set_u32(root, drive);
        e.mem.set_u8(root, (drive as u8).wrapping_add(0x40));
        e.mem.set_u8(root + 1, b':');
        e.mem.set_u8(root + 2, b'\\');
        e.mem.set_u8(root + 3, 0);
        let kind = e.call(GET_DRIVE_TYPE_A, &args![root]).u32();
        u32::from(kind != 0 && kind != 1)
    })
}

// Translated from 00eccbdc (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00eccbdc - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fe90) is not translated; its __finally block (00eccc1c) is called as the exe calls it on the way out.
/// `__getcwd` (Ghidra's function ID), cdecl: target, size. Under lock 7,
/// `__getdcwd_nolock(0, target, size)` answers; the `__finally` block gives the lock back.
pub fn getcwd(e: &mut Engine, target: u32, size: u32) -> u32 {
    e.call(LOCK, &args![7u32]);
    let result = e.call(GETDCWD_NOLOCK, &args![0u32, target, size]).u32();
    e.call(GETCWD_FINALLY, &args![]);
    result
}

// Translated from 00ecbc26 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__strupr_s_l` (Ghidra's function ID), cdecl: text, size, locale. With a `_LocaleUpdate`
/// for the locale, `_strupr_s_l_stat(text, size, update)` answers; the update's destructor
/// clears bit 0x2 of the thread's `_ownlocale` when it was set.
pub fn strupr_s_l(e: &mut Engine, text: u32, size: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let result = e.call(STRUPR_S_L_STAT, &args![text, size, update]).u32();
        if e.mem.u8(update.wrapping_add(0xc)) != 0 {
            let ptd = e.mem.u32(update.wrapping_add(8));
            let own = e.mem.u32(ptd.wrapping_add(0x70));
            e.mem.set_u32(ptd.wrapping_add(0x70), own & 0xffff_fffd);
        }
        result
    })
}

/// The registry entry of [`siglookup`]: `EDX` first.
fn siglookup_entry(e: &mut Engine, words: &[u32]) -> Ret {
    siglookup(e, words[0], words[1]).into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_8d33, strtoul(u32, u32, u32) -> u32),
        entry!(0x00ee_1dac, set_error_mode(u32) -> u32),
        (0x00ed_df8a, siglookup_entry as AbiFn),
        entry!(0x00ec_ca6c, validdrive(u32) -> u32),
        entry!(0x00ec_cbdc, getcwd(u32, u32) -> u32),
        entry!(0x00ec_bc26, strupr_s_l(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn strtoul_picks_the_locale_and_asks_for_unsigned() {
        let mut e = Engine::new();
        e.register_double(STRTOXL, |_, a| (a[0] ^ a[4]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_8d33, &args![1u32, 2u32, 16u32]).u32(),
            INITIAL_LOCALE ^ 1
        );
        e.mem.set_u32(LOCALE_CHANGED, 1);
        assert_eq!(strtoul(&mut e, 1, 2, 16), 1);
        assert_eq!(calls(&e, STRTOXL)[1], vec![0, 1, 2, 16, 1]);
    }

    #[test]
    fn set_error_mode_sets_reads_or_refuses() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        e.mem.map(ERROR_MODE, 4);
        assert_eq!(e.call(0x00ee_1dac, &args![2u32]).u32(), 0);
        assert_eq!(set_error_mode(&mut e, 1), 2);
        assert_eq!(set_error_mode(&mut e, 3), 1);
        assert_eq!(set_error_mode(&mut e, 4), 0xffff_ffff);
        assert_eq!(set_error_mode(&mut e, 0xffff_ffff), 0xffff_ffff);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(e.mem.u32(ERROR_MODE), 1);
    }

    #[test]
    fn siglookup_finds_the_signal_entry() {
        let mut e = Engine::new();
        e.mem.map(SIGNAL_TABLE_COUNT, 4);
        e.mem.set_u32(SIGNAL_TABLE_COUNT, 3);
        let table = e.mem.alloc(0x30);
        for (i, signal) in [8u32, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(table + i as u32 * 0xc + 4, signal);
        }
        assert_eq!(e.call(0x00ed_df8a, &args![8u32, table]).u32(), table);
        assert_eq!(siglookup(&mut e, 4, table), table + 0xc);
        assert_eq!(siglookup(&mut e, 2, table), 0);
        // With no entries the first is still compared, but not answered.
        e.mem.set_u32(SIGNAL_TABLE_COUNT, 0);
        assert_eq!(siglookup(&mut e, 8, table), 0);
    }

    #[test]
    fn validdrive_asks_for_the_root_type() {
        let mut e = Engine::new();
        e.register_double(GET_DRIVE_TYPE_A, |e, a| match e.mem.cstr(a[0]).as_slice() {
            b"C:\\" => 3u32.into_ret(),
            b"D:\\" => 1u32.into_ret(),
            _ => 0u32.into_ret(),
        });
        assert_eq!(e.call(0x00ec_ca6c, &args![0u32]).u32(), 1);
        assert_eq!(validdrive(&mut e, 3), 1);
        assert_eq!(validdrive(&mut e, 4), 0);
        assert_eq!(validdrive(&mut e, 26), 0);
    }

    #[test]
    fn getcwd_and_strupr_s_l_wrap_their_bodies() {
        let mut e = Engine::new();
        e.register_double(LOCK, |_, _| Ret::default());
        e.register_double(GETCWD_FINALLY, |_, _| Ret::default());
        e.register_double(GETDCWD_NOLOCK, |_, a| (a[1] + a[2]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_cbdc, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls(&e, GETDCWD_NOLOCK), vec![vec![0, 0x100, 0x20]]);
        assert_eq!(calls(&e, LOCK), vec![vec![7]]);
        assert_eq!(calls(&e, GETCWD_FINALLY).len(), 1);
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0] + 8, ptd);
            e.mem.set_u8(a[0] + 0xc, 1);
            a[0].into_ret()
        });
        e.register_double(STRUPR_S_L_STAT, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(strupr_s_l(&mut e, 0x10, 0x20, 0), 0x30);
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
    }
}
