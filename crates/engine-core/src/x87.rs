//! x87 behaviour shared by the translations: loads and stores, the NaN an
//! arithmetic instruction gives, and the precision control of the running
//! thread's control word.
//!
//! An `FLD`/`FSTP` round trip of a single-precision value through the x87
//! stack quiets a signalling NaN: the quiet bit (bit 22) is set and the rest
//! of the payload is kept. Every other value, quiet NaNs and infinities
//! included, comes back unchanged. This is done on the bit pattern, because
//! Rust's `as` casts do not guarantee NaN payloads.
//!
//! # Precision control
//!
//! The engine holds x87 values as `f64`. Under the control word's 53-bit
//! precision (`0x027F`, how every thread starts) `FADD`, `FSUB`, `FMUL`,
//! `FDIV` and `FSQRT` round their result to 53 bits, as `f64` arithmetic
//! does. Under 24-bit precision (`0x007F`: the main thread once Direct3D has
//! created the renderer's device, `units::platform::direct3d_device_created`)
//! they round the significand to 24 bits with the control word's rounding
//! mode, while the exponent keeps the extended range (Intel SDM vol. 1,
//! 8.1.5.2): unlike `f32` arithmetic, a result beyond the float range neither
//! overflows nor becomes a float denormal; only a store to a `float`
//! ([`x87_store32`]) does that. Loads, stores, compares and the other
//! instructions are not affected by precision control.
//!
//! [`x87_add`], [`x87_sub`], [`x87_mul`], [`x87_div`] and [`x87_sqrt`] read the
//! precision from [`control_word`] and round each result once, from the exact
//! value, as the FPU does: the `f64` result together with the sign of its
//! rounding error (computed exactly) decides the 24-bit rounding, so an `f64`
//! result that lands exactly between two 24-bit values is not rounded a second
//! time the wrong way. [`x87_round`] rounds an `f64` that a translation
//! computed in plain `f64`.
//!
//! Range: the model holds values in `f64`, whose exponent range is far smaller
//! than the extended format's (2^-16382 to 2^16383). Every nonzero `f64`,
//! subnormals included, is therefore a normal extended value, so the FPU never
//! produces an extended denormal (with its reduced precision and underflow
//! handling) for a value the model can hold: the 24-bit rounding never meets
//! one. Results beyond the `f64` range overflow to infinity, and results below
//! the smallest normal `f64` (2^-1022) lose bits in the `f64` itself, as they
//! already do at 53-bit precision; both are outside the model. 64-bit
//! precision (`0x037F`) is modelled as 53-bit, and so is the reserved
//! precision value 01. At 53-bit precision the rounding mode is not modelled
//! (round to nearest, as before); at 24-bit precision all four modes are.

/// The bit pattern an `FLD dword` / `FSTP dword` pair leaves for `bits`.
pub fn x87_quiet_bits(bits: u32) -> u32 {
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// [`x87_quiet_bits`] on an `f32`.
pub fn x87_quiet(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()))
}

const F64_QUIET: u64 = 1 << 51;
const F64_EXPONENT: u64 = 0x7ff0_0000_0000_0000;
const F64_SIGNIFICAND: u64 = 0x000f_ffff_ffff_ffff;

fn is_nan_bits(bits: u64) -> bool {
    bits & F64_EXPONENT == F64_EXPONENT && bits & F64_SIGNIFICAND != 0
}

/// The NaN an x87 `FADD`/`FSUB`/`FMUL`/`FDIV` (and their `P`/`R` forms)
/// gives when an operand is a NaN, or `None` when neither is (Intel SDM
/// vol. 1, section 4.8.3.5, table 4-7, x87 column). One NaN: that NaN,
/// quieted. Two: a quiet NaN wins over a signalling one; otherwise the one
/// with the larger significand (compared quieted), and on a tie the
/// positive one. The operand order does not matter. f64 arithmetic does not
/// follow this: SSE keeps the first operand's NaN, and LLVM may swap the
/// operands of a commutative operation (#199).
fn nan_result(a: f64, b: f64) -> Option<f64> {
    let (a, b) = (a.to_bits(), b.to_bits());
    let chosen = match (is_nan_bits(a), is_nan_bits(b)) {
        (false, false) => return None,
        (true, false) => a,
        (false, true) => b,
        (true, true) => match (a & F64_QUIET != 0, b & F64_QUIET != 0) {
            (true, false) => a,
            (false, true) => b,
            _ => {
                let (sa, sb) = (
                    (a | F64_QUIET) & F64_SIGNIFICAND,
                    (b | F64_QUIET) & F64_SIGNIFICAND,
                );
                match sa.cmp(&sb) {
                    std::cmp::Ordering::Greater => a,
                    std::cmp::Ordering::Less => b,
                    // Equal significands: the sign bit decides, positive first.
                    std::cmp::Ordering::Equal => a.min(b),
                }
            }
        },
    };
    Some(f64::from_bits(chosen | F64_QUIET))
}

// ---- The control word ------------------------------------------------------

/// The control word every game thread starts with: all exceptions masked,
/// 53-bit precision, round to nearest. Measured (nv-probe `fpu=1`) on the main
/// thread before the renderer creates its device and on the AI linear task
/// threads for the whole run.
pub const DEFAULT_CONTROL_WORD: u16 = 0x027f;

/// Precision control (bits 8-9): `00` is 24-bit, `10` 53-bit, `11` 64-bit.
const PRECISION_CONTROL: u16 = 0x0300;
/// Rounding control (bits 10-11).
const ROUNDING_CONTROL: u16 = 0x0c00;
const ROUND_NEAREST: u16 = 0x0000;
const ROUND_DOWN: u16 = 0x0400;
const ROUND_UP: u16 = 0x0800;

thread_local! {
    /// The FPU control register of the game thread now running on this host
    /// thread. The engine runs game code on one host thread; the platform
    /// layer loads this when it switches game threads.
    static CONTROL_WORD: std::cell::Cell<u16> = const { std::cell::Cell::new(DEFAULT_CONTROL_WORD) };
}

/// The x87 control word the arithmetic helpers round with: the running game
/// thread's (`FSTCW`).
pub fn control_word() -> u16 {
    CONTROL_WORD.with(|c| c.get())
}

/// Loads the x87 control register (`FLDCW`) the helpers read. Only the CRT and
/// platform units call it: `units::crt::set_x87_control_word`, which also keeps
/// the thread's own copy, `Engine::new` (a new main thread) and the thread
/// switch in `units::platform`.
pub fn set_control_word(word: u16) {
    CONTROL_WORD.with(|c| c.set(word));
}

/// The rounding mode when the control word selects 24-bit precision, `None`
/// for 53-bit (and for 64-bit and the reserved value, which the model treats
/// as 53-bit).
fn single_precision() -> Option<u16> {
    let word = control_word();
    if word & PRECISION_CONTROL == 0 {
        Some(word & ROUNDING_CONTROL)
    } else {
        None
    }
}

fn sign_of(error: f64) -> i32 {
    if error > 0.0 {
        1
    } else if error < 0.0 {
        -1
    } else {
        0
    }
}

/// `q * 2^exponent` for `q < 2^25` and `exponent` in `-1073..=1000` (all the
/// rounding below produces), exactly: the result's lowest bit is never below
/// 2^-1074.
fn scale(q: u64, exponent: i32) -> f64 {
    let power = |k: i32| f64::from_bits(((k + 1023) as u64) << 52);
    let v = q as f64;
    if exponent < -1022 {
        v * power(exponent + 100) * power(-100)
    } else {
        v * power(exponent)
    }
}

/// The exact value `s + error` (with `error` far below `s`'s last bit, only its
/// sign known: `error_sign`) rounded to a 24-bit significand with the rounding
/// mode `rounding`, the exponent unbounded. `s` is finite and nonzero and is
/// the exact value rounded to nearest `f64`, so `|error|` is at most half a unit
/// of `s`'s last place.
///
/// In units of a quarter of `s`'s last place, `4m + sign` (`m` the significand)
/// stands for the exact magnitude: every 24-bit value and every midpoint
/// between two is a multiple of `s`'s unit, and the exact magnitude lies on the
/// same side of each of them as `4m + sign` does (it differs from `m` by at
/// most half a unit, toward `sign`). It also has the exact magnitude's binade:
/// a power of two minus a little has one bit less, so the 24 bits are counted
/// from the right place.
fn round_to_single(s: f64, error_sign: i32, rounding: u16) -> f64 {
    let bits = s.to_bits();
    let negative = bits >> 63 != 0;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & F64_SIGNIFICAND;
    let (m, unit) = if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | 1 << 52, exponent - 1075)
    };
    let magnitude_error = if negative { -error_sign } else { error_sign };
    let x = (m << 2).wrapping_add(magnitude_error as i64 as u64);
    let width = 64 - x.leading_zeros() as i32;
    if width <= 26 {
        // A subnormal below 2^-1050: 24 bits would reach past the f64's last
        // place (outside the model, see the module comment).
        return s;
    }
    let drop = width - 24;
    let q = x >> drop;
    let rest = x & ((1u64 << drop) - 1);
    let half = 1u64 << (drop - 1);
    let up = match rounding {
        ROUND_NEAREST => rest > half || (rest == half && q & 1 != 0),
        ROUND_DOWN => negative && rest != 0,
        ROUND_UP => !negative && rest != 0,
        _ => false, // toward zero
    };
    let magnitude = scale(q + u64::from(up), unit - 2 + drop);
    if negative {
        -magnitude
    } else {
        magnitude
    }
}

/// [`x87_add`]'s rounding of `s = a + b` under 24-bit precision.
fn single_sum(a: f64, b: f64, s: f64, rounding: u16) -> f64 {
    if s == 0.0 {
        // An exact zero (an f64 sum that underflows is exact). Its sign is
        // `+` in every mode but rounding down, which gives `-0` unless both
        // operands are `+0` (IEEE 754, which the x87 follows).
        return if rounding == ROUND_DOWN && (a.to_bits() | b.to_bits()) != 0 {
            -0.0
        } else {
            s
        };
    }
    if !s.is_finite() {
        return s;
    }
    // Knuth's TwoSum: the rounding error of `a + b`, exactly.
    let bb = s - a;
    let error = (a - (s - bb)) + (b - bb);
    round_to_single(s, sign_of(error), rounding)
}

/// x87 `FADD`: `a + b` with the x87 choice of NaN ([`nan_result`]), rounded to
/// the precision of the control word ([`control_word`]).
pub fn x87_add(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a + b;
    match single_precision() {
        None => s,
        Some(rounding) => single_sum(a, b, s, rounding),
    }
}

/// x87 `FSUB`: `a - b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
pub fn x87_sub(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a - b;
    match single_precision() {
        None => s,
        Some(rounding) => single_sum(a, -b, s, rounding),
    }
}

/// x87 `FMUL`: `a * b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
pub fn x87_mul(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a * b;
    match single_precision() {
        // A zero product of nonzero operands underflowed the f64 (outside
        // the model); every other zero is exact.
        Some(rounding) if s != 0.0 && s.is_finite() => {
            round_to_single(s, sign_of(a.mul_add(b, -s)), rounding)
        }
        _ => s,
    }
}

/// x87 `FDIV`: `a / b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
pub fn x87_div(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a / b;
    match single_precision() {
        Some(rounding) if s != 0.0 && s.is_finite() => {
            // The remainder `a - s * b`, exactly; the error of the quotient
            // has its sign times the divisor's.
            let remainder = (-s).mul_add(b, a);
            round_to_single(s, sign_of(remainder) * sign_of(b), rounding)
        }
        _ => s,
    }
}

/// x87 `FSQRT`: the square root, rounded to the precision of the control word.
/// A NaN comes back quieted (its payload kept); a negative number other than
/// `-0` gives the default NaN, as `f64::sqrt` does.
pub fn x87_sqrt(value: f64) -> f64 {
    if value.is_nan() {
        return f64::from_bits(value.to_bits() | F64_QUIET);
    }
    let s = value.sqrt();
    match single_precision() {
        Some(rounding) if s != 0.0 && s.is_finite() => {
            // `value - s * s`, exactly: the root's error has its sign.
            round_to_single(s, sign_of((-s).mul_add(s, value)), rounding)
        }
        _ => s,
    }
}

/// `value` rounded to the precision of the control word: unchanged under
/// 53-bit precision, its significand rounded to 24 bits (rounding mode of the
/// control word, exponent unbounded) under 24-bit. For a translation that
/// computes in plain `f64` and needs the x87 result at one point.
///
/// It rounds `value` itself, which is exact for an `FADD`/`FSUB`/`FMUL`/
/// `FDIV`/`FSQRT` only when `value` is that instruction's exact result or, with
/// round to nearest, when the operands are floats (24-bit values): then
/// rounding to `f64` first never changes the 24-bit result (53 >= 2 * 24 + 2).
/// With `double` operands, or a directed rounding mode, use the instruction's
/// helper. NaNs and infinities come back unchanged.
pub fn x87_round(value: f64) -> f64 {
    match single_precision() {
        Some(rounding) if value != 0.0 && value.is_finite() => round_to_single(value, 0, rounding),
        _ => value,
    }
}

/// A `float` memory operand of an x87 arithmetic instruction (`FMUL dword
/// ptr [..]` and the like) as the instruction sees it: widened exactly, a
/// signalling NaN still signalling, so the helpers above can tell it from a
/// quiet one. A value loaded by `FLD dword` is quiet: use
/// `f64::from(x87_quiet(..))` for that one.
pub fn x87_m32(bits: u32) -> f64 {
    let sign = u64::from(bits >> 31) << 63;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = u64::from(bits & 0x007f_ffff);
    if exponent == 0xff && mantissa != 0 {
        f64::from_bits(sign | F64_EXPONENT | (mantissa << 29))
    } else {
        f64::from(f32::from_bits(bits))
    }
}

/// `FLD dword`: the value the x87 stack holds after loading the float with
/// the bit pattern `bits` (a signalling NaN quieted, everything else exact).
pub fn x87_load32(bits: u32) -> f64 {
    f64::from(f32::from_bits(x87_quiet_bits(bits)))
}

/// `FSTP dword`: the stack value rounded to a float, as its bits (a NaN keeps
/// its sign and the top of its payload).
pub fn x87_store32(value: f64) -> u32 {
    (value as f32).to_bits()
}

// Helpers of several engine crates, kept here so each reaches them: `fistp`
// (from `units/bsmenu/font.rs`) and `rsqrt_estimate` (from
// `units/fallout_shared/tesobjectrefr.rs`, which the Havok units call too).

/// `FISTP` with the default rounding mode: nearest, ties to even; the
/// integer indefinite 0x80000000 for NaN and values out of range.
pub fn fistp(value: f64) -> i32 {
    // Round half to even (the FPU's default mode) without `round_ties_even`,
    // which is newer than the crate's MSRV: `round` goes half away from zero,
    // so an odd result of a tie steps back toward zero.
    let mut r = value.round();
    if (value - value.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r -= value.signum();
    }
    if r.is_nan() || r < i32::MIN as f64 || r > i32::MAX as f64 {
        i32::MIN
    } else {
        r as i32
    }
}

/// The `CMPEQSS`/`RSQRTSS` pair of the Havok normalizer: the hardware
/// estimate of `1 / sqrt(x)` on x86, exact elsewhere.
#[cfg(target_arch = "x86_64")]
pub fn rsqrt_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn rsqrt_estimate(x: f32) -> f32 {
    1.0 / x.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signalling_nan_gets_the_quiet_bit_and_keeps_its_payload() {
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0xff80_1234), 0xffc0_1234);
    }

    #[test]
    fn other_values_are_unchanged() {
        for bits in [
            0,
            0x8000_0000,
            0x3f80_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            0x7fc0_0005,
        ] {
            assert_eq!(x87_quiet_bits(bits), bits);
        }
        assert_eq!(x87_quiet(1.5), 1.5);
    }

    fn f32_result(value: f64) -> u32 {
        (value as f32).to_bits()
    }

    #[test]
    fn two_nans_keep_the_larger_significand_then_the_positive_one() {
        // The vectors of #199: -qNaN and +qNaN with equal significands.
        let (neg, pos) = (x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff));
        assert_eq!(f32_result(x87_mul(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(pos, neg)), 0x7fff_ffff);
        // Key 0x7fc00000 times scale 0xffffffff: the larger significand wins.
        let small = x87_m32(0x7fc0_0000);
        assert_eq!(f32_result(x87_mul(small, neg)), 0xffff_ffff);
        assert_eq!(f32_result(x87_mul(neg, small)), 0xffff_ffff);
        // x and |x| for x = 0xffc00000.
        let x = x87_m32(0xffc0_0000);
        assert_eq!(f32_result(x87_mul(x, x.abs())), 0x7fc0_0000);
        assert_eq!(f32_result(x87_sub(x.abs(), x)), 0x7fc0_0000);
        assert_eq!(f32_result(x87_div(x, x.abs())), 0x7fc0_0000);
    }

    #[test]
    fn a_quiet_nan_wins_over_a_signalling_one() {
        let signalling = x87_m32(0x7fbf_ffff);
        let quiet = x87_m32(0xffc0_0001);
        assert_eq!(f32_result(x87_mul(signalling, quiet)), 0xffc0_0001);
        assert_eq!(f32_result(x87_mul(quiet, signalling)), 0xffc0_0001);
        // Two signalling NaNs: the larger significand, quieted.
        let other = x87_m32(0xff80_0001);
        assert_eq!(f32_result(x87_add(signalling, other)), 0x7fff_ffff);
    }

    #[test]
    fn one_nan_comes_back_quieted_and_numbers_are_unchanged() {
        assert_eq!(f32_result(x87_mul(x87_m32(0xff80_0005), 2.0)), 0xffc0_0005);
        assert_eq!(f32_result(x87_sub(3.0, x87_m32(0x7fc0_1234))), 0x7fc0_1234);
        assert_eq!(x87_add(1.5, 2.25), 3.75);
        assert_eq!(x87_sub(1.5, 2.25), -0.75);
        assert_eq!(x87_mul(1.5, -2.0), -3.0);
        assert_eq!(x87_div(1.0, 4.0), 0.25);
        assert_eq!(x87_m32(0x3fc0_0000), 1.5);
        assert_eq!(x87_m32(0x7f80_0000), f64::INFINITY);
    }

    /// Runs `f` with the control word `word` loaded, then the default again.
    fn with_word<R>(word: u16, f: impl FnOnce() -> R) -> R {
        set_control_word(word);
        let r = f();
        set_control_word(DEFAULT_CONTROL_WORD);
        r
    }

    #[test]
    fn at_53_bit_precision_the_helpers_are_f64_arithmetic() {
        assert_eq!(control_word(), 0x027f);
        let mut x = 0x1234_5678_9abc_def1u64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            f64::from_bits(x)
        };
        for word in [0x027f, 0x037f, 0x0e7f] {
            with_word(word, || {
                for _ in 0..2000 {
                    let (a, b) = (next(), next());
                    if a.is_nan() || b.is_nan() {
                        continue;
                    }
                    assert_eq!(x87_add(a, b).to_bits(), (a + b).to_bits());
                    assert_eq!(x87_sub(a, b).to_bits(), (a - b).to_bits());
                    assert_eq!(x87_mul(a, b).to_bits(), (a * b).to_bits());
                    assert_eq!(x87_div(a, b).to_bits(), (a / b).to_bits());
                    assert_eq!(x87_round(a).to_bits(), a.to_bits());
                }
            });
        }
    }

    #[test]
    fn at_24_bit_precision_results_have_24_bit_significands_and_the_extended_range() {
        with_word(0x007f, || {
            assert_eq!(x87_div(1.0, 3.0), f64::from(1.0f32 / 3.0));
            assert_eq!(x87_add(1.0, 2f64.powi(-30)), 1.0);
            assert_eq!(x87_round(1.0 + 2f64.powi(-30)), 1.0);
            // 1e30 * 1e30 stays finite, 1e-30 * 1e-30 stays normal.
            let big = x87_mul(1e30, 1e30);
            assert!(big.is_finite() && big != 1e60);
            assert_eq!(big.to_bits() & ((1 << 29) - 1), 0, "24-bit significand");
            let small = x87_mul(1e-30, 1e-30);
            assert!(small > 0.0 && small.to_bits() & ((1 << 29) - 1) == 0);
            assert_eq!(x87_sqrt(2.0), f64::from(2f32.sqrt()));
        });
    }

    #[test]
    fn the_control_word_belongs_to_the_host_thread() {
        with_word(0x007f, || {
            let other = std::thread::spawn(control_word).join().unwrap();
            assert_eq!(other, DEFAULT_CONTROL_WORD);
            assert_eq!(control_word(), 0x007f);
        });
    }

    #[test]
    fn load32_quiets_and_store32_rounds_to_a_float() {
        assert_eq!(x87_load32(0x3fc0_0000), 1.5);
        assert_eq!(x87_store32(x87_load32(0x7f80_0001)), 0x7fc0_0001);
        assert_eq!(x87_store32(x87_load32(0xffc0_1234)), 0xffc0_1234);
        assert_eq!(x87_store32(x87_load32(0x0000_0001)), 0x0000_0001);
        // 1 + 2^-30 rounds to 1.0 in a float.
        assert_eq!(x87_store32(1.0 + 2f64.powi(-30)), 0x3f80_0000);
        assert_eq!(
            x87_store32(x87_add(x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff))),
            0x7fff_ffff
        );
    }
}
