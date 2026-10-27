//! The platform's clock and time-zone calls: `GetSystemTimeAsFileTime` (the real system clock,
//! read when the game asks), `GetTimeZoneInformation` and `FileTimeToLocalFileTime` (the
//! player's time zone from the machine table, `scripts/d3d9-capture`, with the bias in effect
//! now worked out by [`zone`]), `FileTimeToSystemTime`, and `WideCharToMultiByte` for the
//! ASCII text the CRT converts (the time-zone names).
//!
//! Measured on this machine (2026-10-21, `build43-scratch/measure/time-2026-10-21.txt`, a
//! program calling the real functions; zone US Eastern, daylight time in effect):
//! - `GetTimeZoneInformation` answered `TIME_ZONE_ID_DAYLIGHT` (2) and the zone's rules; the
//!   rules ([`zone::bias_at`]) gave the same local time as `SystemTimeToTzSpecificLocalTime`
//!   for every half hour of 2025-2027 (52,560 times).
//! - `FileTimeToLocalFileTime` subtracts the bias in effect *now* (not at the given time) from
//!   any `FILETIME`, wrapping around 2^64 (0, 1, 0x8000_0000_0000_0000 and u64::MAX included),
//!   and always answers TRUE.
//! - `FileTimeToSystemTime` converts every value up to 0x7FFF_FFFF_FFFF_FFFF (30828-09-14
//!   02:48:05.477); above it answers FALSE with `ERROR_INVALID_PARAMETER` (87) and leaves the
//!   output as it was. Equal to [`zone::system_time`] for every half hour of 2025-2027.
//! - `WideCharToMultiByte(CP_ACP (1252 here), 0, text, -1, out, size, NULL, &used)` copies
//!   ASCII as is and counts the zero; `used` becomes FALSE; with `size` 0 it answers the size
//!   needed and writes nothing; too small a buffer is filled as far as it goes and the call
//!   answers 0 with `ERROR_INSUFFICIENT_BUFFER` (122). `é` became 0xE9 and a character outside
//!   code page 1252 `?` with `used` TRUE: non-ASCII text is not provided here (the code page's
//!   table is not captured).

use super::zone::{self, ZoneInfo};
use crate::{Engine, Ptr};

/// The real system clock as a `FILETIME` (100 ns since 1601-01-01 UTC).
fn now_filetime() -> u64 {
    let since = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the system clock is before 1970");
    zone::UNIX_EPOCH_TICKS
        + since.as_secs() * zone::TICKS_PER_SECOND
        + u64::from(since.subsec_nanos() / 100)
}

fn zone_info(e: &mut Engine) -> ZoneInfo {
    ZoneInfo::from_bytes(&super::d3d9::time_zone(e))
}

// Platform: replaces KERNEL32.DLL GetSystemTimeAsFileTime (import)
/// `GetSystemTimeAsFileTime(out)`: the real system clock now (UTC).
pub fn get_system_time_as_file_time(e: &mut Engine, out: Ptr) {
    let now = now_filetime();
    e.mem.set_u32(out.addr(), now as u32);
    e.mem.set_u32(out.addr() + 4, (now >> 32) as u32);
}

// Platform: replaces KERNEL32.DLL GetTimeZoneInformation (import)
/// `GetTimeZoneInformation(out)`: the player's `TIME_ZONE_INFORMATION` (172 bytes, from the
/// machine table) and `TIME_ZONE_ID_UNKNOWN` (0) for a zone without daylight time, else
/// `TIME_ZONE_ID_DAYLIGHT` (2) or `TIME_ZONE_ID_STANDARD` (1) as the real clock now falls.
pub fn get_time_zone_information(e: &mut Engine, out: Ptr) -> u32 {
    let bytes = super::d3d9::time_zone(e);
    e.mem.write(out.addr(), &bytes);
    let zone = ZoneInfo::from_bytes(&bytes);
    match zone::in_daylight(&zone, zone::filetime_seconds(now_filetime())) {
        None => 0,
        Some(true) => 2,
        Some(false) => 1,
    }
}

// Platform: replaces KERNEL32.DLL FileTimeToLocalFileTime (import)
/// `FileTimeToLocalFileTime(utc, local)`: `utc` less the player's bias in effect now (minutes,
/// 600,000,000 ticks each), wrapping; TRUE (measured, module comment).
pub fn file_time_to_local_file_time(e: &mut Engine, utc: Ptr, local: Ptr) -> u32 {
    let ticks = u64::from(e.mem.u32(utc.addr())) | (u64::from(e.mem.u32(utc.addr() + 4)) << 32);
    let zone = zone_info(e);
    let bias = zone::bias_at(&zone, zone::filetime_seconds(now_filetime()));
    let result = ticks.wrapping_sub((i64::from(bias) * 600_000_000) as u64);
    e.mem.set_u32(local.addr(), result as u32);
    e.mem.set_u32(local.addr() + 4, (result >> 32) as u32);
    1
}

// Platform: replaces KERNEL32.DLL FileTimeToSystemTime (import)
/// `FileTimeToSystemTime(time, out)`: the `SYSTEMTIME` of a `FILETIME` ([`zone::system_time`]);
/// above 0x7FFF_FFFF_FFFF_FFFF FALSE with `ERROR_INVALID_PARAMETER` and `out` untouched
/// (measured).
pub fn file_time_to_system_time(e: &mut Engine, time: Ptr, out: Ptr) -> u32 {
    let ticks = u64::from(e.mem.u32(time.addr())) | (u64::from(e.mem.u32(time.addr() + 4)) << 32);
    match zone::system_time(ticks) {
        Some(words) => {
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u16(out.addr() + 2 * i as u32, *w);
            }
            1
        }
        None => {
            e.kernel.last_error = 87;
            0
        }
    }
}

// Platform: replaces KERNEL32.DLL WideCharToMultiByte (import)
/// `WideCharToMultiByte(code page, flags, wide, count, out, size, default, used)` for ASCII
/// text in `CP_ACP` (0) or 1252 with no flags and no default character: each character as its
/// byte (`count` -1: through the zero, which is counted); `used` (when given) FALSE; `size` 0
/// answers the size needed; too small a buffer is filled as far as it goes and answers 0 with
/// `ERROR_INSUFFICIENT_BUFFER` (122). As measured (module comment). Other code pages, flags,
/// a default character or non-ASCII text are not provided (they stop the run).
#[allow(clippy::too_many_arguments)]
pub fn wide_char_to_multi_byte(
    e: &mut Engine,
    code_page: u32,
    flags: u32,
    wide: Ptr,
    count: i32,
    out: Ptr,
    size: i32,
    default: Ptr,
    used: Ptr,
) -> u32 {
    assert!(
        (code_page == 0 || code_page == 1252) && flags == 0 && default.addr() == 0,
        "WideCharToMultiByte(code page {code_page}, flags {flags:#x}, default {:#x}) is not provided",
        default.addr()
    );
    let mut text = Vec::new();
    let mut at = wide.addr();
    loop {
        if count >= 0 && text.len() as i32 >= count {
            break;
        }
        let c = e.mem.u16(at);
        assert!(
            c < 0x80,
            "WideCharToMultiByte: the non-ASCII character {c:#x} is not provided"
        );
        text.push(c as u8);
        at += 2;
        if count < 0 && c == 0 {
            break;
        }
    }
    if used.addr() != 0 {
        e.mem.set_u32(used.addr(), 0);
    }
    if size == 0 {
        return text.len() as u32;
    }
    let room = size.max(0) as usize;
    let written = text.len().min(room);
    e.mem.write(out.addr(), &text[..written]);
    if written < text.len() {
        e.kernel.last_error = 122;
        return 0;
    }
    text.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The US Eastern zone's `TIME_ZONE_INFORMATION` as a capture line would give it.
    fn eastern(e: &mut Engine) {
        let mut b = vec![0u8; 172];
        b[0..4].copy_from_slice(&300i32.to_le_bytes());
        for (i, w) in [0u16, 11, 0, 1, 2, 0, 0, 0].iter().enumerate() {
            b[0x44 + 2 * i..0x46 + 2 * i].copy_from_slice(&w.to_le_bytes());
        }
        for (i, w) in [0u16, 3, 0, 2, 2, 0, 0, 0].iter().enumerate() {
            b[0x98 + 2 * i..0x9a + 2 * i].copy_from_slice(&w.to_le_bytes());
        }
        b[0xa8..0xac].copy_from_slice(&(-60i32).to_le_bytes());
        let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
        super::super::d3d9::load_adapter_table(e, &format!("GetTimeZoneInformation\t\t2\t{hex}\n"));
    }

    #[test]
    fn local_file_times_take_the_bias_in_effect_now() {
        let mut e = Engine::new();
        eastern(&mut e);
        let utc = e.mem.alloc(8);
        let local = e.mem.alloc(8);
        e.mem.set_u32(utc, 0);
        e.mem.set_u32(utc + 4, 0);
        assert_eq!(
            file_time_to_local_file_time(&mut e, Ptr::new(utc), Ptr::new(local)),
            1
        );
        let zone = zone_info(&mut e);
        let bias = zone::bias_at(&zone, zone::filetime_seconds(now_filetime()));
        let got = u64::from(e.mem.u32(local)) | (u64::from(e.mem.u32(local + 4)) << 32);
        assert_eq!(got, 0u64.wrapping_sub(bias as u64 * 600_000_000));
        let info = e.mem.alloc(172);
        let id = get_time_zone_information(&mut e, Ptr::new(info));
        assert_eq!(id, if bias == 240 { 2 } else { 1 });
        assert_eq!(e.mem.u32(info), 300);
    }

    #[test]
    fn system_times_and_their_limit() {
        let mut e = Engine::new();
        let time = e.mem.alloc(8);
        let out = e.mem.alloc(16);
        e.mem.set_u32(time, 0);
        e.mem.set_u32(time + 4, 0x8000_0000);
        e.mem.set_u32(out, 0xaaaa_aaaa);
        assert_eq!(
            file_time_to_system_time(&mut e, Ptr::new(time), Ptr::new(out)),
            0
        );
        assert_eq!(e.kernel.last_error, 87);
        assert_eq!(e.mem.u32(out), 0xaaaa_aaaa);
        // 1970-01-01 00:00:12.345, a Thursday.
        let ticks = zone::UNIX_EPOCH_TICKS + 123_450_000;
        e.mem.set_u32(time, ticks as u32);
        e.mem.set_u32(time + 4, (ticks >> 32) as u32);
        assert_eq!(
            file_time_to_system_time(&mut e, Ptr::new(time), Ptr::new(out)),
            1
        );
        let words: Vec<u16> = (0..8).map(|i| e.mem.u16(out + 2 * i)).collect();
        assert_eq!(words, [1970, 1, 4, 1, 0, 0, 12, 345]);
    }

    #[test]
    fn ascii_names_convert_and_short_buffers_fail() {
        let mut e = Engine::new();
        let wide = e.mem.alloc(64);
        for (i, c) in "Eastern\0".encode_utf16().enumerate() {
            e.mem.set_u16(wide + 2 * i as u32, c);
        }
        let out = e.mem.alloc(16);
        let used = e.mem.alloc(4);
        e.mem.set_u32(used, 7);
        let call = |e: &mut Engine, size: i32| {
            wide_char_to_multi_byte(
                e,
                0,
                0,
                Ptr::new(wide),
                -1,
                Ptr::new(out),
                size,
                Ptr::new(0),
                Ptr::new(used),
            )
        };
        assert_eq!(call(&mut e, 0x3f), 8);
        assert_eq!(e.mem.cstr(out), b"Eastern".to_vec());
        assert_eq!(e.mem.u32(used), 0);
        assert_eq!(call(&mut e, 0), 8);
        e.mem.write(out, &[0xaa; 8]);
        assert_eq!(call(&mut e, 4), 0);
        assert_eq!(e.kernel.last_error, 122);
        assert_eq!(e.mem.bytes(out, 5), b"East\xaa".to_vec());
    }
}
