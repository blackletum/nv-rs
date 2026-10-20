//! `BSShader/run_00b6c0d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00b6c200` (unnamed, `cdecl`, one word), asked with the word at `011f9490` for the last
/// out word's first value and by kinds 4 and 0x1f.
const QUERY_00B6C200: u32 = 0x00b6_c200;
const QUERY_ARGUMENT_011F9490: u32 = 0x011f_9490;
/// The renderer's width and height helpers `00b6c1a0` and `00b6c1d0` (unnamed, `thiscall` on
/// the renderer, no stack words).
const RENDERER_WIDTH_00B6C1A0: u32 = 0x00b6_c1a0;
const RENDERER_HEIGHT_00B6C1D0: u32 = 0x00b6_c1d0;
/// The byte that makes the code take the two sizes at `011f9430`/`011f9434` in place of the
/// renderer's.
const SIZE_OVERRIDE_011F9426: u32 = 0x011f_9426;
const WIDTH_011F9430: u32 = 0x011f_9430;
const HEIGHT_011F9434: u32 = 0x011f_9434;
/// The other words and bytes the arms read.
const SIZE_01200054: u32 = 0x0120_0054;
const WIDTH_011AD814: u32 = 0x011a_d814;
const HEIGHT_011AD818: u32 = 0x011a_d818;
const EXTRA_011AD810: u32 = 0x011a_d810;
const WIDTH_011AD81C: u32 = 0x011a_d81c;
const HEIGHT_011AD820: u32 = 0x011a_d820;
const SIZE_011ADBC8: u32 = 0x011a_dbc8;
const SIZE_011AD830: u32 = 0x011a_d830;
const FORMAT_011AD870: u32 = 0x011a_d870;
const BYTE_SETTING_011F941E: u32 = 0x011f_941e;
const BYTE_SETTING_011AD80E: u32 = 0x011a_d80e;
const BYTE_SETTING_012004FC: u32 = 0x0120_04fc;
/// The `float` 2^32 the code adds to a negative `FILD` (`01099378`) and the `double` 0.5 it
/// multiplies by (`01011588`).
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const HALF_01011588: u32 = 0x0101_1588;

/// The renderer's slot +0xc8, then that object's slot `slot` (+0x8c width, +0x90 height)
/// with 0.
fn target_size(e: &mut Engine, renderer: Ptr, slot: u32) -> u32 {
    let target = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    e.vcall(target, slot, &args![0u32]).u32()
}

/// `CDQ; AND EDX,3; ADD EAX,EDX; SAR EAX,2`: the signed word divided by 4, toward zero.
fn signed_quarter(value: u32) -> u32 {
    let value = value as i32;
    (value.wrapping_add((value >> 31) & 3) >> 2) as u32
}

/// `CDQ; SUB EAX,EDX; SAR EAX,1`: the signed word divided by 2, toward zero.
fn signed_half(value: u32) -> u32 {
    let value = value as i32;
    (value.wrapping_sub(value >> 31) >> 1) as u32
}

/// `FILD dword` of an unsigned word: loaded signed, then 2^32 added (the `float` at
/// `01099378`, through the FPU) when the sign bit is set.
fn load_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)))
    } else {
        loaded
    }
}

/// `FISTP qword` with the rounding control set to truncate (the code ORs 0xc00 into the
/// control word around it): toward zero, the integer indefinite 0x8000000000000000 for NaN
/// and values out of range. The code keeps the low word.
fn fistp_qword_truncate(value: f64) -> u64 {
    let truncated = value.trunc();
    if truncated.is_nan()
        || truncated < -9_223_372_036_854_775_808.0
        || truncated >= 9_223_372_036_854_775_808.0
    {
        0x8000_0000_0000_0000
    } else {
        truncated as i64 as u64
    }
}

/// The width and height from the renderer's helpers (`00b6c1a0`, `00b6c1d0`) or, with the
/// byte at `011f9426` set, the words at `011f9430`/`011f9434`; the width stored before the
/// height is asked.
fn helper_or_override_size(e: &mut Engine, renderer: Ptr, width: Ptr, height: Ptr) {
    if e.global::<u8>(SIZE_OVERRIDE_011F9426) == 0 {
        let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
        e.mem.set_u32(width.addr(), w);
        let h = e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32();
        e.mem.set_u32(height.addr(), h);
    } else {
        e.mem.set_u32(width.addr(), e.global::<u32>(WIDTH_011F9430));
        e.mem
            .set_u32(height.addr(), e.global::<u32>(HEIGHT_011F9434));
    }
}

/// The same with the renderer's slot +0xc8 target sizes (+0x8c, +0x90) in place of the
/// helpers.
fn target_or_override_size(e: &mut Engine, renderer: Ptr, width: Ptr, height: Ptr) {
    if e.global::<u8>(SIZE_OVERRIDE_011F9426) == 0 {
        let w = target_size(e, renderer, 0x8c);
        e.mem.set_u32(width.addr(), w);
        let h = target_size(e, renderer, 0x90);
        e.mem.set_u32(height.addr(), h);
    } else {
        e.mem.set_u32(width.addr(), e.global::<u32>(WIDTH_011F9430));
        e.mem
            .set_u32(height.addr(), e.global::<u32>(HEIGHT_011F9434));
    }
}

/// The arms of kinds 0x2d, 0x31 (`flag_word` 4) and 0x30 (0x40): the width set to 0x500,
/// the renderer's slot +0xc8 asked twice; the first object's height (+0x90) and the second's
/// width (+0x8c), each loaded as unsigned, give `height / width` (`FDIVR`), times the width
/// word read back as unsigned (`FMULP`), truncated into the height (low word of a `FISTP
/// qword`); then the flags and a format of 0.
fn aspect_size(
    e: &mut Engine,
    renderer: Ptr,
    width: Ptr,
    height: Ptr,
    format: Ptr,
    flags: Ptr,
    flag_word: u32,
) {
    e.mem.set_u32(width.addr(), 0x500);
    let first = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let second = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let target_height = e.vcall(first, 0x90, &args![0u32]).u32();
    let target_height = load_unsigned(e, target_height);
    let target_width = e.vcall(second, 0x8c, &args![0u32]).u32();
    let target_width = load_unsigned(e, target_width);
    let ratio = x87_div(target_height, target_width);
    let width_back = e.mem.u32(width.addr());
    let product = x87_mul(ratio, load_unsigned(e, width_back));
    e.mem
        .set_u32(height.addr(), fistp_qword_truncate(product) as u32);
    e.mem.set_u32(flags.addr(), flag_word);
    e.mem.set_u32(format.addr(), 0);
}

// Translated from 00b6c2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`): the description of a rendered-texture kind.
/// `stdcall`, `RET 0x20`, eight words: the renderer, the kind, and the addresses of the
/// width, height, format, a word it clears, the flags and a last word. Clears the fifth word
/// and sets the last to `00b6c200(word at 011f9490)`; for kinds 0 to 0x33 (the jump table at
/// `00b6d048`, read in Ghidra) stores the kind's width, height, flags and format (each arm in
/// the exe's order; see the arms below); kind 0xb and kinds above 0x33 store nothing. At the
/// end the last word is cleared unless the flags (as they then are) have bit 0x40.
///
/// Sizes come from constants, from words of the exe's data, from the renderer (its slot
/// +0xc8 object's +0x8c/+0x90 with 0, or its helpers `00b6c1a0`/`00b6c1d0`) or, with the
/// byte at `011f9426` set, from the words at `011f9430`/`011f9434`; some arms take a
/// quarter or a half (logical shifts of the renderer's sizes, signed divisions toward zero
/// of the words). Kind 1 takes half the helper width through the FPU (unsigned load,
/// `FMUL` by 0.5, truncation), at least 0x100.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b6c2c0(
    e: &mut Engine,
    renderer: Ptr,
    kind: u32,
    width: Ptr,
    height: Ptr,
    format: Ptr,
    cleared: Ptr,
    flags: Ptr,
    last: Ptr,
) {
    let put = |e: &mut Engine, at: Ptr, value: u32| e.mem.set_u32(at.addr(), value);
    put(e, cleared, 0);
    let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
    let first = e.call(QUERY_00B6C200, &args![word]).u32();
    put(e, last, first);
    let overridden = |e: &Engine| e.global::<u8>(SIZE_OVERRIDE_011F9426) != 0;
    match kind {
        0 | 2 => {
            put(e, flags, 0x26);
            if target_size(e, renderer, 0x8c) <= 0x500 {
                put(e, height, 0x100);
                put(e, width, 0x100);
                put(e, format, 0x71);
            } else {
                let w = if overridden(e) {
                    signed_quarter(e.global::<u32>(WIDTH_011F9430))
                } else {
                    target_size(e, renderer, 0x8c) >> 2
                };
                put(e, width, w);
                let h = if overridden(e) {
                    signed_quarter(e.global::<u32>(HEIGHT_011F9434))
                } else {
                    target_size(e, renderer, 0x90) >> 2
                };
                put(e, height, h);
                put(e, format, 0x71);
            }
        }
        1 => {
            put(e, flags, 0x26);
            let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
            let half = f64::from_bits(e.global::<u64>(HALF_01011588));
            let product = x87_mul(load_unsigned(e, w), half);
            let mut value = fistp_qword_truncate(product) as u32;
            if value < 0x100 {
                value = 0x100;
            }
            put(e, width, value);
            put(e, height, 0x100);
            put(e, format, 0x71);
        }
        3 => {
            if overridden(e) {
                put(e, width, e.global::<u32>(WIDTH_011F9430));
                put(e, height, e.global::<u32>(HEIGHT_011F9434));
            } else {
                let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
                put(e, width, w);
                let h = e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32();
                put(e, height, h);
            }
            put(e, flags, 0x26);
            put(e, format, 0);
        }
        4 => {
            helper_or_override_size(e, renderer, width, height);
            let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
            let answer = e.call(QUERY_00B6C200, &args![word]).i32();
            put(e, flags, if answer <= 0 { 0x22 } else { 0x62 });
            let byte = e.global::<u8>(BYTE_SETTING_011F941E) != 0;
            put(e, format, if byte { 0x71 } else { 0 });
        }
        5 => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, 0x22);
            put(e, format, 0x71);
        }
        6 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 6);
            put(e, format, 0);
        }
        7 | 0xf | 0x10 | 0x15 => {
            put(e, height, e.global::<u32>(SIZE_01200054));
            put(e, width, e.global::<u32>(SIZE_01200054));
            let (flag_word, format_word) = match kind {
                7 => (6, 0x74),
                0xf => (0xc, 0x72),
                0x10 => (0xc, 0x74),
                _ => (4, 0),
            };
            put(e, flags, flag_word);
            put(e, format, format_word);
        }
        8 => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 2);
            put(e, format, 0x24);
        }
        9 | 0x16 => {
            put(e, width, e.global::<u32>(WIDTH_011AD814));
            put(e, height, e.global::<u32>(HEIGHT_011AD818));
            put(e, flags, 0x40);
            put(e, format, 0);
            put(e, last, e.global::<u32>(EXTRA_011AD810));
            let value = e.mem.u32(flags.addr()) | 0x20;
            put(e, flags, value);
        }
        0xa => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 0x40);
            put(e, format, 0);
        }
        0xb => {}
        0xc => {
            put(e, height, 0x80);
            put(e, width, 0x80);
            put(e, flags, 6);
            put(e, format, 0x24);
        }
        0xd | 0xe => {
            put(e, width, e.global::<u32>(SIZE_01200054));
            put(e, height, if kind == 0xd { 1 } else { 0x10 });
            put(e, flags, 0xc);
            put(e, format, if kind == 0xd { 0x51 } else { 0x74 });
        }
        0x11 => {
            put(e, width, 0x200);
            put(e, height, 0x200);
            put(e, flags, 0x122);
            put(e, format, 0x15);
        }
        0x12 => {
            put(e, width, e.global::<u32>(WIDTH_011AD81C));
            put(e, height, e.global::<u32>(HEIGHT_011AD820));
            put(e, flags, 0xc);
            put(e, format, 0x15);
        }
        0x13 => {
            put(e, width, e.global::<u32>(WIDTH_011AD81C));
            put(e, height, e.global::<u32>(HEIGHT_011AD820));
            put(e, flags, 4);
            put(e, format, 0x15);
            let value = e.mem.u32(flags.addr()) | 0x20;
            put(e, flags, value);
        }
        0x14 => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 6);
            put(e, format, 0x71);
        }
        0x17 | 0x29 | 0x2c | 0x32 => {
            put(e, height, 0x100);
            put(e, width, 0x100);
            let flag_word = match kind {
                0x17 => 6,
                0x29 => 2,
                0x2c => 0x10,
                _ => 4,
            };
            put(e, flags, flag_word);
            put(e, format, 0);
        }
        0x18 => {
            let h = if overridden(e) {
                put(e, width, signed_quarter(e.global::<u32>(WIDTH_011F9430)));
                signed_quarter(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = target_size(e, renderer, 0x8c) >> 2;
                put(e, width, w);
                target_size(e, renderer, 0x90) >> 2
            };
            put(e, height, h);
            put(e, flags, 6);
            put(e, format, 0);
        }
        0x19 | 0x22 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 0x26);
            put(e, format, 0);
        }
        0x1a | 0x1b => {
            put(e, height, e.global::<u32>(SIZE_011ADBC8));
            put(e, width, e.global::<u32>(SIZE_011ADBC8));
            put(e, flags, 6);
            put(e, format, 0);
        }
        0x1c..=0x1e => {
            let size = if e.global::<u8>(BYTE_SETTING_012004FC) != 0 {
                0x200
            } else {
                0x80
            };
            put(e, height, size);
            put(e, width, size);
            put(e, flags, if kind == 0x1c { 4 } else { 0 });
            put(e, format, 0x17);
        }
        0x1f => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, 6);
            let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
            if e.call(QUERY_00B6C200, &args![word]).i32() > 0 {
                put(e, flags, 0x46);
            }
            put(e, format, 0);
        }
        0x20 | 0x21 => {
            if kind == 0x20 {
                target_or_override_size(e, renderer, width, height);
            } else {
                helper_or_override_size(e, renderer, width, height);
            }
            put(e, flags, if kind == 0x20 { 0x42 } else { 2 });
            let byte = e.global::<u8>(BYTE_SETTING_011AD80E) != 0;
            let format_word = match (kind, byte) {
                (0x20, true) => 0x71,
                (_, true) => 0x72,
                _ => 0x15,
            };
            put(e, format, format_word);
        }
        0x23 | 0x24 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 0x42);
            put(e, format, 0x15);
        }
        0x25 | 0x26 | 0x28 => {
            let h = if overridden(e) {
                put(e, width, signed_quarter(e.global::<u32>(WIDTH_011F9430)));
                signed_quarter(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32() >> 2;
                put(e, width, w);
                e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32() >> 2
            };
            put(e, height, h);
            put(e, flags, 6);
            put(e, format, 0x15);
        }
        0x27 | 0x2f => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, if kind == 0x27 { 2 } else { 0x40 });
            put(e, format, if kind == 0x27 { 0x15 } else { 0 });
        }
        0x2a => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 6);
            put(e, format, 0x72);
        }
        0x2b => {
            let size = u32::from(e.global::<u16>(SIZE_011AD830));
            put(e, height, size);
            put(e, width, size);
            put(e, flags, 6);
            put(e, format, e.global::<u32>(FORMAT_011AD870));
        }
        0x2d | 0x31 => aspect_size(e, renderer, width, height, format, flags, 4),
        0x30 => aspect_size(e, renderer, width, height, format, flags, 0x40),
        0x2e => {
            let h = if overridden(e) {
                put(e, width, signed_half(e.global::<u32>(WIDTH_011F9430)));
                signed_half(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = target_size(e, renderer, 0x8c) >> 1;
                put(e, width, w);
                target_size(e, renderer, 0x90) >> 1
            };
            put(e, height, h);
            put(e, flags, 4);
            put(e, format, 0);
        }
        0x33 => {
            put(e, height, 8);
            put(e, width, 8);
            put(e, flags, 0);
            put(e, format, 0);
        }
        _ => {}
    }
    if e.mem.u32(flags.addr()) & 0x40 == 0 {
        put(e, last, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b6_c2c0,
        fn_00b6c2c0(Ptr, u32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: u32 = 0x0f00_1000;
    const TARGET_WIDTH: u32 = 0x0f00_208c;
    const TARGET_HEIGHT: u32 = 0x0f00_2090;

    /// The renderer: slot +0xc8 answers a target sized `target` (its words at +4, +8); the
    /// helpers answer `helpers`. `00b6c200` answers `query`. The data words are distinct.
    struct Rig {
        e: Engine,
        renderer: u32,
        out: u32,
    }

    fn rig(target: (u32, u32), helpers: (u32, u32), query: i32) -> Rig {
        let mut e = Engine::new();
        e.map(0x0109_9000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        e.set_global(HALF_01011588, 0.5f64.to_bits());
        e.set_global(WIDTH_011F9430, 0xffff_fffdu32);
        e.set_global(HEIGHT_011F9434, 0x0000_0402u32);
        e.set_global(SIZE_01200054, 0x54u32);
        e.set_global(WIDTH_011AD814, 0x814u32);
        e.set_global(HEIGHT_011AD818, 0x818u32);
        e.set_global(EXTRA_011AD810, 0x810u32);
        e.set_global(WIDTH_011AD81C, 0x81cu32);
        e.set_global(HEIGHT_011AD820, 0x820u32);
        e.set_global(SIZE_011ADBC8, 0xbc8u32);
        e.set_global(SIZE_011AD830, 0x1234_0830u32);
        e.set_global(FORMAT_011AD870, 0x870u32);
        let mut renderer_slots = vec![0u32; 0x33];
        renderer_slots[0x32] = 0x0f00_00c8;
        e.put_vtable(0x0f00_0000, &renderer_slots);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, 0x0f00_0000);
        e.register(0x0f00_00c8, |_, _| TARGET.into_ret());
        let mut target_slots = vec![0u32; 0x25];
        target_slots[0x23] = TARGET_WIDTH;
        target_slots[0x24] = TARGET_HEIGHT;
        e.put_vtable(0x0f00_2000, &target_slots);
        e.map(TARGET, 0x10);
        e.mem.set_u32(TARGET, 0x0f00_2000);
        e.mem.set_u32(TARGET + 4, target.0);
        e.mem.set_u32(TARGET + 8, target.1);
        e.register(TARGET_WIDTH, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TARGET_HEIGHT, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register_double(RENDERER_WIDTH_00B6C1A0, move |_, _| helpers.0.into_ret());
        e.register_double(RENDERER_HEIGHT_00B6C1D0, move |_, _| helpers.1.into_ret());
        e.register_double(QUERY_00B6C200, move |_, _| (query as u32).into_ret());
        let out = e.mem.alloc(0x20);
        Rig { e, renderer, out }
    }

    /// Runs the kind and answers (width, height, format, cleared, flags, last).
    fn describe(r: &mut Rig, kind: u32) -> [u32; 6] {
        let o = r.out;
        for i in 0..6 {
            r.e.mem.set_u32(o + 4 * i, 0xdead_0000 + i);
        }
        r.e.call(
            0x00b6_c2c0,
            &args![
                r.renderer,
                kind,
                o,
                o + 4,
                o + 8,
                o + 0xc,
                o + 0x10,
                o + 0x14
            ],
        );
        [0, 1, 2, 3, 4, 5].map(|i| r.e.mem.u32(o + 4 * i))
    }

    #[test]
    fn fn_00b6c2c0_kind_0_takes_a_quarter_of_a_large_target() {
        let mut r = rig((1920, 1080), (0, 0), 7);
        assert_eq!(describe(&mut r, 0), [480, 270, 0x71, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 2), [480, 270, 0x71, 0, 0x26, 0]);
        // A target of at most 0x500 wide: 0x100 square.
        let mut r = rig((0x500, 720), (0, 0), 7);
        assert_eq!(describe(&mut r, 0), [0x100, 0x100, 0x71, 0, 0x26, 0]);
        // With the override: signed quarters of the words (toward zero).
        let mut r = rig((1920, 1080), (0, 0), 7);
        r.e.set_global(SIZE_OVERRIDE_011F9426, 1u8);
        assert_eq!(describe(&mut r, 0), [0, 0x100, 0x71, 0, 0x26, 0]);
    }

    #[test]
    fn fn_00b6c2c0_kind_1_halves_the_helper_width_through_the_fpu() {
        let mut r = rig((0, 0), (1001, 0), 0);
        assert_eq!(describe(&mut r, 1), [500, 0x100, 0x71, 0, 0x26, 0]);
        let mut r = rig((0, 0), (0x1ff, 0), 0);
        assert_eq!(describe(&mut r, 1), [0x100, 0x100, 0x71, 0, 0x26, 0]);
        // Above 2^31: loaded unsigned.
        let mut r = rig((0, 0), (0xffff_fffe, 0), 0);
        assert_eq!(describe(&mut r, 1)[0], 0x7fff_ffff);
    }

    #[test]
    fn fn_00b6c2c0_helper_and_target_sizes_follow_the_override() {
        let mut r = rig((1920, 1080), (800, 600), 0);
        assert_eq!(describe(&mut r, 3), [800, 600, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 6), [1920, 1080, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x18), [480, 270, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x25), [200, 150, 0x15, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2e), [960, 540, 0, 0, 4, 0]);
        r.e.set_global(SIZE_OVERRIDE_011F9426, 1u8);
        assert_eq!(describe(&mut r, 3), [0xffff_fffd, 0x402, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 0x19), [0xffff_fffd, 0x402, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 0x18), [0, 0x100, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2e), [0xffff_ffff, 0x201, 0, 0, 4, 0]);
    }

    #[test]
    fn fn_00b6c2c0_flags_with_bit_0x40_keep_the_last_word() {
        // Kind 4: flags 0x62 when 00b6c200 answers above 0; the last word kept.
        let mut r = rig((0, 0), (64, 32), 1);
        assert_eq!(describe(&mut r, 4), [64, 32, 0, 0, 0x62, 1]);
        let mut r2 = rig((0, 0), (64, 32), 0);
        r2.e.set_global(BYTE_SETTING_011F941E, 1u8);
        assert_eq!(describe(&mut r2, 4), [64, 32, 0x71, 0, 0x22, 0]);
        // Kind 9 takes the last word from 011ad810 and keeps it (flags 0x60).
        assert_eq!(describe(&mut r, 9), [0x814, 0x818, 0, 0, 0x60, 0x810]);
        assert_eq!(describe(&mut r, 0x16), [0x814, 0x818, 0, 0, 0x60, 0x810]);
        // Kind 0x1f: 0x46 with a positive answer.
        assert_eq!(describe(&mut r, 0x1f), [64, 32, 0, 0, 0x46, 1]);
        assert_eq!(describe(&mut r2, 0x1f), [64, 32, 0, 0, 6, 0]);
    }

    #[test]
    fn fn_00b6c2c0_constant_and_data_kinds() {
        let mut r = rig((0, 0), (0, 0), 5);
        assert_eq!(describe(&mut r, 7), [0x54, 0x54, 0x74, 0, 6, 0]);
        assert_eq!(describe(&mut r, 8), [0x200, 0x200, 0x24, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0xa), [0x200, 0x200, 0, 0, 0x40, 5]);
        assert_eq!(describe(&mut r, 0xc), [0x80, 0x80, 0x24, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0xd), [0x54, 1, 0x51, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0xe), [0x54, 0x10, 0x74, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0xf), [0x54, 0x54, 0x72, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x10), [0x54, 0x54, 0x74, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x11), [0x200, 0x200, 0x15, 0, 0x122, 0]);
        assert_eq!(describe(&mut r, 0x12), [0x81c, 0x820, 0x15, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x13), [0x81c, 0x820, 0x15, 0, 0x24, 0]);
        assert_eq!(describe(&mut r, 0x14), [0x200, 0x200, 0x71, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x15), [0x54, 0x54, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x17), [0x100, 0x100, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x1a), [0xbc8, 0xbc8, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x1c), [0x80, 0x80, 0x17, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x1d), [0x80, 0x80, 0x17, 0, 0, 0]);
        assert_eq!(describe(&mut r, 0x29), [0x100, 0x100, 0, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x2a), [0x200, 0x200, 0x72, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2b), [0x830, 0x830, 0x870, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2c), [0x100, 0x100, 0, 0, 0x10, 0]);
        assert_eq!(describe(&mut r, 0x32), [0x100, 0x100, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x33), [8, 8, 0, 0, 0, 0]);
        r.e.set_global(BYTE_SETTING_012004FC, 1u8);
        assert_eq!(describe(&mut r, 0x1e), [0x200, 0x200, 0x17, 0, 0, 0]);
    }

    #[test]
    fn fn_00b6c2c0_kinds_0x20_0x21_take_the_format_byte() {
        let mut r = rig((1920, 1080), (800, 600), 0);
        assert_eq!(describe(&mut r, 0x20), [1920, 1080, 0x15, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x21), [800, 600, 0x15, 0, 2, 0]);
        r.e.set_global(BYTE_SETTING_011AD80E, 1u8);
        assert_eq!(describe(&mut r, 0x20), [1920, 1080, 0x71, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x21), [800, 600, 0x72, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x23), [1920, 1080, 0x15, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x27), [800, 600, 0x15, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x2f), [800, 600, 0, 0, 0x40, 0]);
        assert_eq!(describe(&mut r, 5), [800, 600, 0x71, 0, 0x22, 0]);
    }

    #[test]
    fn fn_00b6c2c0_aspect_kinds_scale_the_height_to_0x500_wide() {
        let mut r = rig((1920, 1080), (0, 0), 3);
        assert_eq!(describe(&mut r, 0x2d), [0x500, 720, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x31), [0x500, 720, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x30), [0x500, 720, 0, 0, 0x40, 3]);
        // A zero-wide target: infinity, the indefinite integer's low word.
        let mut r = rig((0, 1080), (0, 0), 3);
        assert_eq!(describe(&mut r, 0x2d)[1], 0);
    }

    #[test]
    fn fn_00b6c2c0_kind_0xb_and_out_of_range_kinds_store_nothing_but_the_ends() {
        let mut r = rig((0, 0), (0, 0), 9);
        let o = r.out;
        let result = describe(&mut r, 0xb);
        // The flags word is the caller's (0xdead0004: bit 0x40 clear), so the last is cleared.
        assert_eq!(
            result,
            [0xdead_0000, 0xdead_0001, 0xdead_0002, 0, 0xdead_0004, 0]
        );
        r.e.mem.set_u32(o + 0x10, 0x40);
        r.e.call(
            0x00b6_c2c0,
            &args![
                r.renderer,
                0x34u32,
                o,
                o + 4,
                o + 8,
                o + 0xc,
                o + 0x10,
                o + 0x14
            ],
        );
        assert_eq!(r.e.mem.u32(o + 0x14), 9);
        assert_eq!(fistp_qword_truncate(f64::NAN), 0x8000_0000_0000_0000);
        assert_eq!(signed_quarter(0xffff_fffb), 0xffff_ffff);
        assert_eq!(signed_half(0xffff_fffd), 0xffff_ffff);
    }
}
