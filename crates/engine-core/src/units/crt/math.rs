//! The C runtime's `floor` (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the entry `00ec6940` with its SSE2 body, the x87 body `00ed4563`,
//! and the math-error machinery both reach that nothing had translated: `__sptype`
//! (`00ed5981`), `__frnd` (`00ee23bc`), `__handle_qnan1` (`00ed5837`), `__except1`
//! (`00ed588c`) with `__handle_exc` (`00ed5554`), `__raise_exc` (`00ed5531`), `__errcode`
//! (`00ed5763`), `__umatherr` (`00ed5797`), `__set_errno_from_matherr` (`00ed5736`), the
//! status-flag helper `00ed5ae4`, `00ed59e1` (mantissa and exponent) and `__set_exp`
//! (`00ed5955`), and `___libm_error_support` (`00ed429f`) with the default handler `00ee0a3a`.
//! Names are Ghidra's function ID matches; `floor` is named from the name its error report
//! passes (`"floor"` at `010943f0`, the string the `_umatherr` table gives function code 0xb).
//! `_ctrlfp` (`00ed5abb`), `_errno` (`00ec85e3`) and `__decode_pointer` (`00ed0f92`) are in
//! `crt.rs`. Every call is made by address, as the exe makes it, so a test can double each one.
//!
//! Not translated: `__raise_exc_ex` (`00ed5255`, about 200 instructions, which raises the
//! floating-point exception), called by address from `__raise_exc`.
//!
//! The x87 control word is the engine's (`super::x87_control_word` for `FSTCW`,
//! `_ctrlfp` for every load), `MXCSR` the engine's `super::MXCSR`. `crate::sse` has
//! single-precision lanes only; the double-precision SSE2 instructions of `floor`'s body
//! (`CMPLTPD`, `SUBSD`) are written out where they run, each on operands that are never NaN
//! there, so the IEEE result is the instruction's (see each one).

use super::*;

/// `_ctrlfp` (`00ed5abb`, cdecl: new word, mask), `_errno` (`00ec85e3`, cdecl) and
/// `__decode_pointer` (`00ed0f92`, cdecl, one word): in `crt.rs`.
const CTRLFP: u32 = 0x00ed_5abb;
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const DECODE_POINTER: u32 = 0x00ed_0f92;
/// The functions of this file, called by address as the exe calls them (`floor` jumps to its
/// x87 body, so that one is a direct call).
const SPTYPE: u32 = 0x00ed_5981;
const FRND: u32 = 0x00ee_23bc;
const HANDLE_QNAN1: u32 = 0x00ed_5837;
const EXCEPT1: u32 = 0x00ed_588c;
const HANDLE_EXC: u32 = 0x00ed_5554;
const RAISE_EXC: u32 = 0x00ed_5531;
const ERRCODE: u32 = 0x00ed_5763;
const UMATHERR: u32 = 0x00ed_5797;
const SET_ERRNO_FROM_MATHERR: u32 = 0x00ed_5736;
const SET_STATFP: u32 = 0x00ed_5ae4;
const DECOMP: u32 = 0x00ed_59e1;
const SET_EXP: u32 = 0x00ed_5955;
const LIBM_ERROR_SUPPORT: u32 = 0x00ed_429f;
const DEFAULT_MATHERR: u32 = 0x00ee_0a3a;
/// `__raise_exc_ex` (`00ed5255`, cdecl, seven words): not translated.
const RAISE_EXC_EX: u32 = 0x00ed_5255;

/// `floor`'s SSE2 constants (16 bytes each, only the low qword reaches the result): 1.0
/// (`010f1f30`), the shift base `0x433` (`010f1f40`), -1.0 (`010f1f50`), -0.0 (`010f1f60`)
/// and the exponent mask `0x7ff` (`010f1f70`).
const FLOOR_ONE: u32 = 0x010f_1f30;
const FLOOR_SHIFT_BASE: u32 = 0x010f_1f40;
const FLOOR_MINUS_ONE: u32 = 0x010f_1f50;
const FLOOR_MINUS_ZERO: u32 = 0x010f_1f60;
const FLOOR_EXPONENT_MASK: u32 = 0x010f_1f70;
/// The control word `floor`'s x87 body loads (`011c2408`; 0x173f in the exe: round down).
const FLOOR_CONTROL_WORD: u32 = 0x011c_2408;
/// The double 1.0 the x87 body adds to an infinity or NaN for its report (`01012070`) and the
/// double `__handle_exc` multiplies a lost underflow by (`01012060`, 0.0 in the exe).
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `__handle_exc`'s overflow results: the double at `011c24f8` (+infinity in the exe) and the one
/// at `011c2508` (the largest double).
const OVERFLOW_INFINITY: u32 = 0x011c_24f8;
const OVERFLOW_LARGEST: u32 = 0x011c_2508;
/// The extended constants `00ed5ae4` loads to raise status flags: `011c2520` (huge) and
/// `011c252c` (tiny).
const STATFP_HUGE: u32 = 0x011c_2520;
const STATFP_TINY: u32 = 0x011c_252c;
/// A global `__handle_qnan1` and `__except1` test before reporting through `_umatherr`: nonzero
/// skips the report (no label in the map; the name is unconfirmed).
const MATHERR_SKIP: u32 = 0x011c_2b60;
/// `_umatherr`'s table of 0x1d (function code, name) pairs (`011c2410`).
const MATHERR_NAMES: u32 = 0x011c_2410;
/// The user `_matherr` flag `___libm_error_support` tests (`012704a4`) and the encoded pointer it
/// then decodes (`01270a68`); both unconfirmed names.
const USER_MATHERR_SET: u32 = 0x0127_04a4;
const USER_MATHERR: u32 = 0x0127_0a68;
/// `floor`'s function code (`__sptype`'s callers and `_umatherr`'s table) and the error code it
/// gives `___libm_error_support`.
const FLOOR_CODE: u32 = 0xb;
const FLOOR_ERROR: u32 = 0x3ed;
/// `EDOM` and `ERANGE`.
const EDOM: u32 = 0x21;
const ERANGE: u32 = 0x22;
/// The sign bit of a double (`FCHS` flips it, `OR word ..., 0x8000` sets it).
const SIGN: u64 = 1 << 63;

/// `errno` set through `_errno`.
fn store_errno(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
}

// Translated from 00ec6940 (disassembled, FalloutNV.exe 1.4.0.525)
/// `floor` (LIBCMT; no label in the map: named from the body, which rounds toward -infinity, and
/// from the name `"floor"` (`010943f0`) its error report passes), cdecl, the double on the stack,
/// the result in `ST0`. With `__use_sse2_mathfcns` (`01270a64`) zero it jumps to the x87 body
/// (`00ed4563`, [`floor_x87`]); otherwise it reads `MXCSR` (`STMXCSR`) and jumps there too
/// unless every SSE exception is masked (`& 0x1f80 == 0x1f80`), then the x87 control word
/// (`FNSTCW`) and jumps there unless every x87 exception is masked (`& 0x7f == 0x7f`): the
/// engine's `MXCSR` and the running thread's control word (`super::sse2_math_available`,
/// the same three tests in the same order). Otherwise the SSE2 body (`00ec6980`) computes the
/// result with integer shifts on the bit pattern.
///
/// SSE2 body: the shift count is `0x433 - exponent`. A positive value below 1 (zero included)
/// gives +0 (`FLDZ`); up to the exponent `0x432` its fraction bits are cleared. A negative value
/// above -1 gives -1.0, and -0 stays -0; up to the exponent `0x432` the fraction bits are cleared
/// and 1.0 subtracted when that raised the value. Beyond that the value comes back as it is (an
/// integer, or an infinity), a NaN after `___libm_error_support(&x, &x, &x, 0x3ed)` has had its
/// say about the argument's slot, which is then the result.
pub fn floor(e: &mut Engine, x: f64) -> f64 {
    if !super::sse2_math_available(e) {
        return floor_x87(e, x);
    }
    let bits = x.to_bits();
    // MOVAPD XMM2, [010f1f40]; MOVAPD XMM1, XMM0; MOVAPD XMM7, XMM0; PSRLQ XMM0, 0x34; MOVD EAX.
    let base = e.mem.u64(FLOOR_SHIFT_BASE);
    let shifted = bits >> 52;
    let sign_and_exponent = shifted as u32;
    // ANDPD XMM0, [010f1f70]; PSUBD XMM2, XMM0: each dword of the low qword separately.
    let exponent = shifted & e.mem.u64(FLOOR_EXPONENT_MASK);
    let count_low = (base as u32).wrapping_sub(exponent as u32);
    let count_high = ((base >> 32) as u32).wrapping_sub((exponent >> 32) as u32);
    let count = u64::from(count_low) | (u64::from(count_high) << 32);
    // PSRLQ / PSLLQ by a count above 63 give zero.
    let shift_right = |value: u64| if count > 63 { 0 } else { value >> count };
    let shift_left = |value: u64| if count > 63 { 0 } else { value << count };
    let right = shift_right(bits); // PSRLQ XMM1, XMM2
    if sign_and_exponent & 0x800 == 0 {
        // CMP EAX, 0x3ff; JL 00ec6a3a: FLDZ.
        if (sign_and_exponent as i32) < 0x3ff {
            return 0.0;
        }
        let truncated = shift_left(right); // PSLLQ XMM1, XMM2
        if (sign_and_exponent as i32) > 0x432 {
            return floor_out_of_range(e, x);
        }
        // MOVQ [ESP+4], XMM1; FLD double ptr [ESP+4].
        return x87_load64(truncated);
    }
    // 00ec6a02: MOVQ XMM0, [ESP+4] (the argument again); PSLLQ XMM1, XMM2; MOVAPD XMM3, XMM0.
    let truncated = shift_left(right);
    // CMPLTPD XMM0, XMM1: the argument below its truncation (a NaN goes out of range below
    // before the mask is used).
    let below = x < f64::from_bits(truncated);
    if (sign_and_exponent as i32) < 0xbff {
        // 00ec6a3d: CMPLTPD XMM3, [010f1f60] (below -0: every negative number but -0; a NaN has
        // an exponent above 0xbff); ORPD XMM3, [010f1f60]; ANDPD XMM3, [010f1f50].
        let minus_zero = e.mem.u64(FLOOR_MINUS_ZERO);
        let mask = if x < f64::from_bits(minus_zero) {
            u64::MAX
        } else {
            0
        };
        let result = (mask | minus_zero) & e.mem.u64(FLOOR_MINUS_ONE);
        return x87_load64(result);
    }
    if (sign_and_exponent as i32) > 0xc32 {
        return floor_out_of_range(e, x);
    }
    // ANDPD XMM0, [010f1f30]; SUBSD XMM1, XMM0: the truncation (a nonzero integer of at most 52
    // bits) minus 1.0 or +0, exact, never a NaN.
    let subtrahend = if below { e.mem.u64(FLOOR_ONE) } else { 0 };
    let result = f64::from_bits(truncated) - f64::from_bits(subtrahend);
    x87_load64(result.to_bits())
}

/// `floor`'s exit for an exponent above `0x432` (`00ec69d3`): `UCOMISD XMM7, XMM7` (unordered for a
/// NaN only); a number comes back as it is (`FLD` of the argument's slot); a NaN is first given to
/// `___libm_error_support` with that slot for all three pointers and the error code 0x3ed, and
/// the slot as it leaves it is loaded.
fn floor_out_of_range(e: &mut Engine, x: f64) -> f64 {
    if !x.is_nan() {
        return x87_load64(x.to_bits());
    }
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        e.call(LIBM_ERROR_SUPPORT, &args![slot, slot, slot, FLOOR_ERROR]);
        x87_load64(e.mem.u64(slot.addr()))
    })
}

// Translated from 00ed4563 (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 body of `floor` (no label; Ghidra counts it into `floor`'s range), reached by a jump
/// from [`floor`] with the same stack: cdecl, the double on the stack, the result in `ST0`. It
/// loads the round-down control word (`011c2408`) through `_ctrlfp(word, 0xffff)`, keeping the
/// old one. The argument's exponent word (`[EBP+0xe] & 0x7ff0`) all ones: `__sptype` of the
/// argument as `FLD`/`FSTP` copy it (a signalling NaN quiet); classes 1 and 2 (the infinities)
/// put the old control word back and return the argument (`FLD`), class 3 (a quiet NaN) is
/// `__handle_qnan1(0xb, x, old)`, anything else `__except1(8, 0xb, x, x + 1.0, old)` (the sum by
/// `FADD` under the loaded word). A finite value is rounded by `__frnd`; when that equals the
/// argument (`FCOMP`, `TEST AH,0x44`, `JP`), or the old word has bit 5 (the precision-exception
/// mask) set, the old word is put back and the rounded value returned, otherwise the inexact
/// result goes to `__except1(0x10, 0xb, x, rounded, old)`.
pub fn floor_x87(e: &mut Engine, x: f64) -> f64 {
    let new_control = e.mem.u32(FLOOR_CONTROL_WORD);
    // EBX: the old word as `_ctrlfp` answers it (sign-extended).
    let old_control = e.call(CTRLFP, &args![new_control, 0xffffu32]).u32();
    let bits = x.to_bits();
    // FLD double ptr [EBP+8]; FSTP double ptr [ESP]: the pushed copy.
    let pushed = x87_load64(bits).to_bits();
    // MOV ECX, dword ptr [EBP+0xe]; AND ECX, 0x7ff0; CMP CX, AX: the double's top word (the two
    // bytes above it are masked away).
    let exponent_word = ((bits >> 48) as u32) & 0x7ff0;
    if exponent_word == 0x7ff0 {
        let class = e.call(SPTYPE, &args![pushed]).i32();
        if class > 0 {
            if class <= 2 {
                // 00ed45c2
                e.call(CTRLFP, &args![old_control, 0xffffu32]);
                return x87_load64(bits);
            }
            if class == 3 {
                // 00ed45ad
                return e
                    .call(HANDLE_QNAN1, &args![FLOOR_CODE, pushed, old_control])
                    .f64();
            }
        }
        // 00ed45d0: FLD x; FADD double ptr [01012070]; FSTP into the result's slot.
        let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
        let sum = x87_add(x87_load64(bits), one).to_bits();
        return e
            .call(EXCEPT1, &args![8u32, FLOOR_CODE, pushed, sum, old_control])
            .f64();
    }
    // 00ed45ed: __frnd of the pushed copy; FSTP double ptr [EBP-8].
    let rounded = e.call(FRND, &args![pushed]).f64().to_bits();
    // FLD x; FCOMP double ptr [EBP-8]; FNSTSW AX; TEST AH, 0x44; JP: equal only when ordered and
    // equal (both are numbers here).
    if x87_load64(bits) == f64::from_bits(rounded) || old_control & 0x20 != 0 {
        // 00ed4604
        e.call(CTRLFP, &args![old_control, 0xffffu32]);
        return x87_load64(rounded);
    }
    // 00ed4617
    e.call(
        EXCEPT1,
        &args![0x10u32, FLOOR_CODE, pushed, rounded, old_control],
    )
    .f64()
}

// Translated from 00ed5981 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sptype` (LIBCMT, Ghidra function ID), cdecl, the double on the stack (low word, high word):
/// 1 for +infinity, 2 for -infinity, 3 for a quiet NaN, 4 for a signalling NaN, 0 otherwise. The
/// NaN tests read the top word (`[EBP+0xe]`, the two bytes above masked away): `& 0x7ff8 ==
/// 0x7ff8` is quiet; `& 0x7ff8 == 0x7ff0` with fraction bits set (`high & 0x7ffff`, or the low
/// word) is signalling.
pub fn sptype(_e: &mut Engine, low: u32, high: u32) -> i32 {
    if high == 0x7ff0_0000 {
        if low == 0 {
            return 1;
        }
    } else if high == 0xfff0_0000 && low == 0 {
        return 2;
    }
    // 00ed59ae
    let top = (high >> 16) & 0x7ff8;
    if top == 0x7ff8 {
        return 3;
    }
    if top == 0x7ff0 && (high & 0x7ffff != 0 || low != 0) {
        return 4;
    }
    0
}

// Translated from 00ee23bc (disassembled, FalloutNV.exe 1.4.0.525)
/// `__frnd` (LIBCMT, Ghidra function ID), cdecl, the double on the stack, the result in `ST0`:
/// `FLD`, `FRNDINT` (the running control word's rounding mode), `FSTP double`, `FLD`.
pub fn frnd(_e: &mut Engine, x: f64) -> f64 {
    let stored = x87_frndint(X87Ext::load64(x)).store64();
    x87_load64(stored.to_bits())
}

// Translated from 00ed5837 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__handle_qnan1` (LIBCMT, Ghidra function ID), cdecl: (function code, the double, the control
/// word to restore); the result in `ST0`. With the global at `011c2b60` zero it is
/// `_umatherr(1, code, x, 0.0, x, cw)` (`x` as `FLD`/`FSTP` copy it, so quiet), whose result it
/// returns. Otherwise `errno` becomes `EDOM`, `_ctrlfp(cw, 0xffff)` restores the control word and
/// `x` is returned (`FLD`).
pub fn handle_qnan1(e: &mut Engine, code: u32, x: f64, control: u32) -> f64 {
    let bits = x.to_bits();
    if e.global::<u32>(MATHERR_SKIP) == 0 {
        let copy = x87_load64(bits).to_bits();
        // FLDZ; FSTP double ptr [ESP+8]: +0.0.
        return e
            .call(UMATHERR, &args![1u32, code, copy, 0u64, copy, control])
            .f64();
    }
    // 00ed586d
    store_errno(e, EDOM);
    e.call(CTRLFP, &args![control, 0xffffu32]);
    x87_load64(bits)
}

// Translated from 00ed588c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__except1` (LIBCMT, Ghidra function ID), cdecl: (operation flags, function code, argument,
/// result, control word), the doubles two words each; the result in `ST0`. Its frame keeps the
/// argument, result and control word where the callees can change them. `__handle_exc(flags,
/// &result, cw)` first; when it answers 0 the word at +0x40 of a 0x80-byte local loses bit 0 and
/// `__raise_exc(&local, &cw, flags, code, &argument, &result)` runs. Then `__errcode(flags)`: with
/// the global at `011c2b60` zero and a nonzero code the answer is `_umatherr(errcode, code,
/// argument, 0.0, result, cw)` (doubles as `FLD`/`FSTP` copy them); otherwise
/// `__set_errno_from_matherr(errcode)`, `_ctrlfp(cw, 0xffff)` and the result (`FLD`).
// Translation incomplete: 00ed588c - the /GS stack cookie check (00ec408c) is not translated.
pub fn except1(
    e: &mut Engine,
    flags: u32,
    code: u32,
    argument: f64,
    result: f64,
    control: u32,
) -> f64 {
    // The 0x80-byte local (EBP-0x80), then the argument (EBX+0x10), result (EBX+0x18) and control
    // word (EBX+0x20).
    e.with_stack(0x98, |e, block| {
        let local = block.addr();
        let argument_at = local + 0x80;
        let result_at = local + 0x88;
        let control_at = local + 0x90;
        e.mem.set_u64(argument_at, argument.to_bits());
        e.mem.set_u64(result_at, result.to_bits());
        e.mem.set_u32(control_at, control);
        let word = e.mem.u32(control_at);
        let handled = e.call(HANDLE_EXC, &args![flags, result_at, word]).u32();
        if handled == 0 {
            // 00ed58c9: AND dword ptr [EBP-0x40], 0xfffffffe.
            let field = e.mem.u32(local + 0x40);
            e.mem.set_u32(local + 0x40, field & !1);
            e.call(
                RAISE_EXC,
                &args![local, control_at, flags, code, argument_at, result_at],
            );
        }
        let error = e.call(ERRCODE, &args![flags]).u32();
        if e.global::<u32>(MATHERR_SKIP) == 0 && error != 0 {
            // 00ed5903
            let word = e.mem.u32(control_at);
            let result_copy = x87_load64(e.mem.u64(result_at)).to_bits();
            let argument_copy = x87_load64(e.mem.u64(argument_at)).to_bits();
            return e
                .call(
                    UMATHERR,
                    &args![error, code, argument_copy, 0u64, result_copy, word],
                )
                .f64();
        }
        // 00ed592a
        e.call(SET_ERRNO_FROM_MATHERR, &args![error]);
        let word = e.mem.u32(control_at);
        e.call(CTRLFP, &args![word, 0xffffu32]);
        x87_load64(e.mem.u64(result_at))
    })
}

/// `FLD` of a double, `FCHS` when `negate`, `FSTP` into `at`.
fn store_double(e: &mut Engine, at: u32, bits: u64, negate: bool) {
    let loaded = x87_load64(bits).to_bits();
    e.mem
        .set_u64(at, if negate { loaded ^ SIGN } else { loaded });
}

// Translated from 00ed5554 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__handle_exc` (LIBCMT, Ghidra function ID), cdecl: (operation flags, pointer to the result,
/// control word); answers 1 when every flag of `flags & 0x1f` was handled by a masked exception,
/// else 0. The first of these that applies runs, each raising its status flag through
/// `00ed5ae4`:
///
/// - flag 8 with mask bit 1: (status 1) handled;
/// - flag 4 with mask bit 4: (status 4) handled;
/// - flag 1 with mask bit 8 (status 8): the result becomes the overflow value of the control
///   word's rounding mode (bits 10-11) and the result's sign (`0 < result` is positive; a NaN or
///   zero counts as negative): nearest: +-`011c24f8`; down: `011c2508` or -`011c24f8`; up:
///   `011c24f8` or -`011c2508`; toward zero: +-`011c2508`; handled;
/// - flag 2 with mask bit 0x10: an exact zero result is inexact-flagged as it is; otherwise
///   `00ed59e1` splits it into a mantissa and an exponent, the exponent less 0x600: below -0x432
///   the result is the mantissa times `01012060` (inexact); else the mantissa's top word keeps its
///   low four bits with the lowest exponent bit set, is shifted right one bit at a time while the
///   exponent is below -0x3fd (a 1 shifted out is inexact), and negated when the mantissa was
///   negative; it is stored as the result. Inexact (or flag 0x10 given) raises status 0x10;
///   handled.
///
/// Then flag 0x10 with mask bit 0x20 (status 0x20) is handled too.
pub fn handle_exc(e: &mut Engine, flags: u32, result: Ptr, control: u32) -> i32 {
    let at = result.addr();
    let mut remaining = flags & 0x1f; // ESI
    let saved = remaining; // [EBP-4]
    if flags & 8 != 0 && control & 1 != 0 {
        e.call(SET_STATFP, &args![1u32]);
        remaining &= !8;
    } else if flags & 4 != 0 && control & 4 != 0 {
        e.call(SET_STATFP, &args![4u32]);
        remaining &= !4;
    } else if flags & 1 != 0 && control & 8 != 0 {
        e.call(SET_STATFP, &args![8u32]);
        // FLDZ; FCOMP double ptr [ECX]; FNSTSW AX; TEST AH, 5: parity odd (JNP) exactly when
        // 0 < result (C0 alone).
        let positive = 0.0 < f64::from_bits(e.mem.u64(at));
        let infinity = e.mem.u64(OVERFLOW_INFINITY);
        let largest = e.mem.u64(OVERFLOW_LARGEST);
        match control & 0xc00 {
            0 => store_double(e, at, infinity, !positive),
            0x400 => {
                if positive {
                    store_double(e, at, largest, false);
                } else {
                    store_double(e, at, infinity, true);
                }
            }
            0x800 => {
                if positive {
                    store_double(e, at, infinity, false);
                } else {
                    store_double(e, at, largest, true);
                }
            }
            0xc00 => store_double(e, at, largest, !positive),
            // CMP EAX, ECX; JNZ 00ed5638: unreachable (the mask leaves only the four values
            // above); nothing is stored.
            _ => {}
        }
        remaining &= !1;
    } else if flags & 2 != 0 && control & 0x10 != 0 {
        let mut inexact = flags & 0x10 != 0;
        // FLDZ; FCOMP double ptr [EDI]; TEST AH, 0x44; JNP 00ed56fe: equal (C3 alone).
        let value = e.mem.u64(at);
        if 0.0 == f64::from_bits(value) {
            inexact = true;
        } else {
            let copy = x87_load64(value).to_bits();
            let (mantissa, exponent) = e.with_stack(4, |e, slot| {
                let mantissa = e.call(DECOMP, &args![copy, slot]).f64().to_bits();
                (mantissa, e.mem.i32(slot.addr()))
            });
            let mut bits = mantissa; // [EBP-0x14]
            let biased = exponent.wrapping_add(-0x600); // ADD ECX, 0xfffffa00
            if biased < -0x432 {
                inexact = true;
                let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
                bits = x87_mul(x87_load64(bits), zero).to_bits();
            } else {
                // FLDZ; FCOMP; TEST AH, 0x41; JNZ: none of C0 and C3 set exactly when 0 > mantissa.
                let negative = 0.0 > f64::from_bits(bits);
                // MOV EAX, dword ptr [EBP-0xe]; AND EAX, 0xf; OR EAX, 0x10; MOV word ptr
                // [EBP-0xe], AX.
                let top = ((bits >> 48) & 0xf) | 0x10;
                bits = (bits & 0x0000_ffff_ffff_ffff) | (top << 48);
                if biased < -0x3fd {
                    let mut count = (-0x3fd_i32).wrapping_sub(biased);
                    loop {
                        if bits & 1 != 0 && !inexact {
                            inexact = true;
                        }
                        bits >>= 1;
                        count -= 1;
                        if count == 0 {
                            break;
                        }
                    }
                }
                if negative {
                    // FLD; FCHS; FSTP double ptr [EBP-0x14].
                    bits = x87_load64(bits).to_bits() ^ SIGN;
                }
            }
            // 00ed56f7
            store_double(e, at, bits, false);
        }
        // 00ed5700
        if inexact {
            e.call(SET_STATFP, &args![0x10u32]);
        }
        remaining = saved & !2;
    }
    // 00ed5714
    if flags & 0x10 != 0 && control & 0x20 != 0 {
        e.call(SET_STATFP, &args![0x20u32]);
        remaining &= !0x10;
    }
    i32::from(remaining == 0)
}

// Translated from 00ed5531 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__raise_exc` (LIBCMT, Ghidra function ID), cdecl, six words: `__raise_exc_ex` (`00ed5255`)
/// with the same six words and a seventh of 0; its `EAX` comes back.
pub fn raise_exc(e: &mut Engine, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    e.call(RAISE_EXC_EX, &args![a0, a1, a2, a3, a4, a5, 0u32])
        .u32()
}

// Translated from 00ed5763 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__errcode` (LIBCMT, Ghidra function ID), cdecl, the operation flags (their low byte read):
/// the `_exception` type: 5 for 0x20, else 1 for 8, 2 for 4, 3 for 1, else 4 for 2 and 0.
pub fn errcode(_e: &mut Engine, flags: u32) -> i32 {
    let low = flags as u8;
    if low & 0x20 != 0 {
        5
    } else if low & 8 != 0 {
        1
    } else if low & 4 != 0 {
        2
    } else if low & 1 != 0 {
        3
    } else {
        // MOVZX EAX, AL; AND EAX, 2; ADD EAX, EAX.
        i32::from(low & 2) * 2
    }
}

// Translated from 00ed5736 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_errno_from_matherr` (LIBCMT, Ghidra function ID), cdecl, the `_exception` type: 1 sets
/// `errno` `EDOM`, 2 and 3 `ERANGE` (signed compares), others nothing.
pub fn set_errno_from_matherr(e: &mut Engine, kind: i32) {
    if kind == 1 {
        store_errno(e, EDOM);
    } else if kind > 1 && kind <= 3 {
        store_errno(e, ERANGE);
    }
}

// Translated from 00ed5797 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__umatherr` (LIBCMT, Ghidra function ID), cdecl: (type, function code, argument 1,
/// argument 2, result, control word), the doubles two words each; the result in `ST0`. The
/// function code is looked up in the 0x1d (code, name) pairs at `011c2410`. Without a name (none
/// found, or a null one), `_ctrlfp(cw, 0xffff)`, `__set_errno_from_matherr(type)` and the
/// result (`FLD`). Otherwise an `_exception` (type, name, the three doubles copied as words) on
/// the stack, `_ctrlfp(cw, 0xffff)`, `_matherr` (`00ee0a3a`) with its address, which when it
/// answers 0 is followed by `__set_errno_from_matherr(type)`; the result is the `_exception`'s
/// (`FLD`).
pub fn umatherr(
    e: &mut Engine,
    kind: u32,
    code: u32,
    argument1: f64,
    argument2: f64,
    result: f64,
    control: u32,
) -> f64 {
    let mut name = 0;
    for i in 0..0x1d {
        if e.mem.u32(MATHERR_NAMES + 8 * i) == code {
            name = e.mem.u32(MATHERR_NAMES + 8 * i + 4);
            break;
        }
    }
    if name == 0 {
        // 00ed581a
        e.call(CTRLFP, &args![control, 0xffffu32]);
        e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
        return x87_load64(result.to_bits());
    }
    // The `_exception` at EBP-0x20: type, name, arg1, arg2, retval.
    e.with_stack(0x20, |e, exception| {
        let at = exception.addr();
        e.mem.set_u32(at, kind);
        e.mem.set_u32(at + 4, name);
        e.mem.set_u64(at + 8, argument1.to_bits());
        e.mem.set_u64(at + 0x10, argument2.to_bits());
        e.mem.set_u64(at + 0x18, result.to_bits());
        e.call(CTRLFP, &args![control, 0xffffu32]);
        let answer = e.call(DEFAULT_MATHERR, &args![exception]).u32();
        if answer == 0 {
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
        }
        x87_load64(e.mem.u64(at + 0x18))
    })
}

// Translated from 00ee0a3a (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_matherr` `__umatherr` calls and `___libm_error_support` defaults to (no label; Ghidra
/// gives the body no name): `XOR EAX, EAX; RET`, cdecl, the `_exception` pointer unread: answers 0
/// (the error is not handled, so `errno` is set).
pub fn fn_00ee0a3a(_e: &mut Engine, _exception: Ptr) -> u32 {
    0
}

// Translated from 00ed5ae4 (disassembled, FalloutNV.exe 1.4.0.525)
/// The status-flag helper of `__handle_exc` (no label; `_set_statfp` by its use, unconfirmed),
/// cdecl, the flags (low byte read): each set bit runs an x87 operation that raises one exception
/// flag in the status word, `WAIT` after each: 1 `FISTP dword` of the huge extended at `011c2520`
/// (invalid), 8 `FSTP double` of it (overflow), 0x10 `FSTP double` of the tiny one at `011c252c`
/// (underflow), 4 `1 / 0` (`FLDZ; FLD1; FDIVRP`, zero divide), 0x20 `FSTP double` of `FLDPI`
/// (inexact). The values go to the function's own stack slots and are not read again; the
/// constants are read here as the loads read them.
// Translation incomplete: 00ed5ae4 - the x87 status word's exception flags it raises are not modelled (the engine keeps no status word).
pub fn fn_00ed5ae4(e: &mut Engine, flags: u32) {
    let low = flags as u8;
    if low & 1 != 0 {
        let _ = e.mem.bytes(STATFP_HUGE, 10);
    }
    if low & 8 != 0 {
        let _ = e.mem.bytes(STATFP_HUGE, 10);
    }
    if low & 0x10 != 0 {
        let _ = e.mem.bytes(STATFP_TINY, 10);
    }
    // Bits 4 (`FLDZ; FLD1; FDIVRP; FSTP ST0`) and 0x20 (`FLDPI; FSTP double`) read no memory and
    // leave nothing but the status flags.
}

// Translated from 00ed59e1 (disassembled, FalloutNV.exe 1.4.0.525)
/// The mantissa and exponent of a double (no label; `_decomp` by its use, unconfirmed), cdecl:
/// (the double, pointer to the exponent); the mantissa in `ST0`. A zero (`FCOM` equal) gives +0
/// and exponent 0. A value whose exponent field is nonzero (or, unreachable after the zero test,
/// a zero bit pattern) gives `__set_exp(x, 0)` and the exponent field less 0x3fe. A denormal is
/// shifted left (as 64 bits; the sign shifts out) until bit 52 is set, the exponent counting down
/// from -0x3fd; bit 52 is cleared, the sign set again for a negative value (`0 > x`), and
/// `__set_exp` of that and 0 is the mantissa.
pub fn fn_00ed59e1(e: &mut Engine, x: f64, exponent: Ptr) -> f64 {
    let bits = x.to_bits();
    // FLDZ; FCOM double ptr [EBP+8]; TEST AH, 0x44; JP: equal falls through.
    if 0.0 == x {
        e.mem.set_i32(exponent.addr(), 0);
        return 0.0;
    }
    // MOV EDX, dword ptr [EBP+0xe]: the top word (the two bytes above it are never used).
    let top = (bits >> 48) as u32 & 0xffff;
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    if top & 0x7ff0 != 0 || (high & 0xfffff == 0 && low == 0) {
        // 00ed5a71: FSTP ST0; FLD x; FSTP double ptr [ESP].
        let copy = x87_load64(bits).to_bits();
        let mantissa = e.call(SET_EXP, &args![copy, 0u32]).f64();
        let field = ((top >> 4) & 0x7ff) as i32;
        e.mem.set_i32(exponent.addr(), field - 0x3fe);
        return mantissa;
    }
    // 00ed5a14: FCOMP double ptr [EBP+8]; TEST AH, 0x41; JNZ: neither C0 nor C3 when 0 > x.
    let negative = 0.0 > x;
    let mut count = -0x3fd_i32;
    let mut shifted = bits;
    while shifted & (1 << 52) == 0 {
        shifted <<= 1;
        count -= 1;
    }
    shifted &= !(1 << 52); // AND word ptr [EBP+0xe], 0xffef
    if negative {
        shifted |= SIGN; // OR word ptr [EBP+0xe], 0x8000
    }
    let copy = x87_load64(shifted).to_bits();
    let mantissa = e.call(SET_EXP, &args![copy, 0u32]).f64();
    e.mem.set_i32(exponent.addr(), count);
    mantissa
}

// Translated from 00ed5955 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_exp` (LIBCMT, Ghidra function ID), cdecl: (the double, an exponent); the result in
/// `ST0`. The double as `FLD`/`FSTP` copy it, its top word replaced by the low 16 bits of
/// `(exponent + 0x3fe) << 4` ORed with the argument's own top word `& 0x800f` (sign and the top
/// fraction bits, from the argument as passed), then `FLD`.
pub fn set_exp(_e: &mut Engine, x: f64, exponent: i32) -> f64 {
    let bits = x.to_bits();
    let copy = x87_load64(bits).to_bits(); // FSTP double ptr [EBP-8]
    let field = ((bits >> 48) as u32) & 0x800f; // MOV ECX, dword ptr [EBP+0xe]; AND ECX, 0x800f
    let top = ((exponent.wrapping_add(0x3fe) as u32) << 4) | field;
    let replaced = (copy & 0x0000_ffff_ffff_ffff) | (u64::from(top as u16) << 48);
    x87_load64(replaced)
}

/// How `___libm_error_support` reports an error code: its `_exception` type and name and which
/// path fills the `_exception`.
#[derive(Clone, Copy)]
enum Report {
    /// `00ed4350`: the given type; `errno` `ERANGE` when the handler answers 0.
    Range(u32, u32),
    /// `00ed4385`: type 4 (underflow); `errno` left alone.
    Underflow(u32),
    /// `00ed43be`: type 1 (domain); `errno` `EDOM` when the handler answers 0.
    Domain(u32),
    /// `00ed4434`: the result becomes argument 1 first, then as `Domain`.
    CopyDomain(u32),
    /// `00ed44e0`: the result becomes argument 1 times +0 (`FMUL` by the zeroed local), which is
    /// also the `_exception`'s result, then as `Domain`.
    ZeroDomain(u32),
    /// `00ed4452`: the result becomes 1.0 (`FLD1`), no report.
    One,
    /// `00ed4516`: nothing.
    Nothing,
}

/// `___libm_error_support`'s dispatch on the error code (signed compares, then the jump table at
/// `00ed451b` for 1000..1012): the names are the strings' addresses.
fn libm_report(code: i32) -> Report {
    match code {
        0xa6 => Report::Range(3, 0x010f_3184),
        2 => Report::Range(2, 0x010f_319c),
        3 => Report::Domain(0x010f_319c),
        8 => Report::Range(2, 0x010f_31a0),
        9 => Report::Domain(0x010f_31a0),
        14 => Report::Range(3, 0x010f_31a8),
        15 => Report::Underflow(0x010f_31a8),
        24 => Report::Range(3, 0x010f_31ac),
        0x19 => Report::Underflow(0x010f_31ac),
        0x1a => Report::One,
        0x1b => Report::Range(2, 0x010f_31ac),
        0x1c => Report::Domain(0x010f_31ac),
        0x1d => Report::CopyDomain(0x010f_31ac),
        0x3a => Report::Domain(0x010f_318c),
        0x3d => Report::Domain(0x010f_3194),
        1000 => Report::CopyDomain(0x010f_319c),
        1001 => Report::CopyDomain(0x010f_31a0),
        1002 => Report::CopyDomain(0x010f_31a8),
        1003 => Report::CopyDomain(0x010f_317c),
        1004 => Report::CopyDomain(0x0109_4408),
        1005 => Report::CopyDomain(0x0109_43f0),
        1006 => Report::Domain(0x010f_31ac),
        1007 => Report::CopyDomain(0x010f_3174),
        1008 => Report::Domain(0x010f_318c),
        1009 => Report::Domain(0x010f_3194),
        1010 => Report::ZeroDomain(0x010f_3170),
        1011 => Report::ZeroDomain(0x010f_316c),
        1012 => Report::ZeroDomain(0x010f_3168),
        _ => Report::Nothing,
    }
}

// Translated from 00ed429f (disassembled, FalloutNV.exe 1.4.0.525)
/// `___libm_error_support` (LIBCMT, Ghidra function ID), cdecl: (pointer to argument 1, pointer to
/// argument 2, pointer to the result, error code). The handler is the decoded user `_matherr`
/// (`__decode_pointer` of `01270a68`) when `012704a4` is nonzero, else `00ee0a3a`. The error code
/// picks an `_exception` type and function name ([`libm_report`]); the `_exception` (on the
/// stack: type, name, argument 1, argument 2, result, each double as `FLD`/`FSTP` copy it) goes to
/// the handler, and its result (`FLD`/`FSTP`) becomes the result. Codes it does not know leave
/// everything alone; code 0x1a stores 1.0.
pub fn libm_error_support(e: &mut Engine, argument1: Ptr, argument2: Ptr, result: Ptr, code: u32) {
    let handler = if e.global::<u32>(USER_MATHERR_SET) != 0 {
        let encoded = e.global::<u32>(USER_MATHERR);
        e.call(DECODE_POINTER, &args![encoded]).u32()
    } else {
        DEFAULT_MATHERR
    };
    let (first, second, out) = (argument1.addr(), argument2.addr(), result.addr());
    let (kind, name, errno_code) = match libm_report(code as i32) {
        Report::Nothing => return,
        Report::One => {
            // FLD1; FSTP double ptr [ESI].
            e.mem.set_u64(out, 1.0f64.to_bits());
            return;
        }
        Report::Range(kind, name) => (kind, name, Some(ERANGE)),
        Report::Underflow(name) => (4, name, None),
        Report::Domain(name) => (1, name, Some(EDOM)),
        Report::CopyDomain(name) => {
            // 00ed4434: FLD double ptr [EDI]; FSTP double ptr [ESI].
            let copy = x87_load64(e.mem.u64(first)).to_bits();
            e.mem.set_u64(out, copy);
            (1, name, Some(EDOM))
        }
        Report::ZeroDomain(name) => {
            // 00ed44e0: FLD double ptr [EDI]; FMUL double ptr [EBP-8] (the zeroed local); FST
            // double ptr [ESI].
            let product = x87_mul(x87_load64(e.mem.u64(first)), 0.0).to_bits();
            e.mem.set_u64(out, product);
            (1, name, Some(EDOM))
        }
    };
    // The `_exception` at EBP-0x28: type, name, arg1, arg2, retval.
    e.with_stack(0x28, |e, exception| {
        let at = exception.addr();
        e.mem.set_u32(at, kind);
        e.mem.set_u32(at + 4, name);
        let copy1 = x87_load64(e.mem.u64(first)).to_bits();
        e.mem.set_u64(at + 8, copy1);
        let copy2 = x87_load64(e.mem.u64(second)).to_bits();
        e.mem.set_u64(at + 0x10, copy2);
        // The result: the product still in ST0 for `ZeroDomain` (FSTP of the register), else
        // `FLD double ptr [ESI]`.
        let retval = x87_load64(e.mem.u64(out)).to_bits();
        e.mem.set_u64(at + 0x18, retval);
        let answer = e.call(handler, &args![exception]).u32();
        // TEST EAX, EAX; JNZ 00ed4511, then errno (the underflow path skips the test).
        if let (0, Some(code)) = (answer, errno_code) {
            store_errno(e, code);
        }
        // 00ed4511: FLD double ptr [EBP-0x10]; FSTP double ptr [ESI].
        let back = x87_load64(e.mem.u64(at + 0x18)).to_bits();
        e.mem.set_u64(out, back);
    })
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_6940, floor(f64) -> f64),
        entry!(0x00ed_4563, floor_x87(f64) -> f64),
        entry!(0x00ed_5981, sptype(u32, u32) -> i32),
        entry!(0x00ee_23bc, frnd(f64) -> f64),
        entry!(0x00ed_5837, handle_qnan1(u32, f64, u32) -> f64),
        entry!(0x00ed_588c, except1(u32, u32, f64, f64, u32) -> f64),
        entry!(0x00ed_5554, handle_exc(u32, Ptr, u32) -> i32),
        entry!(0x00ed_5531, raise_exc(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed_5763, errcode(u32) -> i32),
        entry!(0x00ed_5736, set_errno_from_matherr(i32)),
        entry!(0x00ed_5797, umatherr(u32, u32, f64, f64, f64, u32) -> f64),
        entry!(0x00ee_0a3a, fn_00ee0a3a(Ptr) -> u32),
        entry!(0x00ed_5ae4, fn_00ed5ae4(u32)),
        entry!(0x00ed_59e1, fn_00ed59e1(f64, Ptr) -> f64),
        entry!(0x00ed_5955, set_exp(f64, i32) -> f64),
        entry!(0x00ed_429f, libm_error_support(Ptr, Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a double saw: the words of each call.
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The `_exception` a double handler saw: type, name, the three doubles.
    type Exception = (u32, u32, u64, u64, u64);
    type Exceptions = Rc<RefCell<Vec<Exception>>>;

    /// `__use_sse2_mathfcns`.
    const SSE2_FLAG: u32 = 0x0127_0a64;
    /// A user `_matherr` for the tests (not a game address).
    const USER_HANDLER: u32 = 0x0040_1230;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the constants these functions read set to the exe's values, the
    /// `_umatherr` table entries for `ceil` (index 15), `floor` (16) and the last one (28), the
    /// skip flag at `011c2b60` as the exe has it (nonzero), `_errno` doubled to answer one slot,
    /// `__use_sse2_mathfcns` as given and the call log on. Answers the engine and the slot.
    fn engine(sse2: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.map(FLOOR_ONE, 0x50);
        for (at, low, high) in [
            (FLOOR_ONE, 1.0f64.to_bits(), 1.0f64.to_bits()),
            (FLOOR_SHIFT_BASE, 0x433, 0x433),
            (FLOOR_MINUS_ONE, (-1.0f64).to_bits(), 0x4330_0000_0000_0000),
            (FLOOR_MINUS_ZERO, SIGN, SIGN),
            (FLOOR_EXPONENT_MASK, 0x7ff, 0),
        ] {
            e.mem.set_u64(at, low);
            e.mem.set_u64(at + 8, high);
        }
        e.map(FLOOR_CONTROL_WORD, 4);
        e.set_global(FLOOR_CONTROL_WORD, 0x173fu32);
        e.map(ZERO_DOUBLE, 0x18);
        e.mem.set_u64(ZERO_DOUBLE, 0);
        e.mem.set_u64(ONE_DOUBLE, 1.0f64.to_bits());
        e.map(OVERFLOW_INFINITY, 0x18);
        e.mem.set_u64(OVERFLOW_INFINITY, f64::INFINITY.to_bits());
        e.mem.set_u64(OVERFLOW_LARGEST, f64::MAX.to_bits());
        e.map(STATFP_HUGE, 0x18);
        e.mem
            .write(STATFP_HUGE, &[0, 0, 0, 0, 0, 0, 0, 0x80, 0x10, 0x44]);
        e.mem
            .write(STATFP_TINY, &[1, 0, 0, 0, 0, 0, 0, 0x80, 0, 0x30]);
        e.map(MATHERR_SKIP, 4);
        e.set_global(MATHERR_SKIP, 0x2694u32);
        e.map(MATHERR_NAMES, 0xe8);
        for (index, code, name) in [
            (15, 0xcu32, 0x0109_4408u32),
            (16, FLOOR_CODE, 0x0109_43f0),
            (28, 0x26, 0x010f_320c),
        ] {
            e.mem.set_u32(MATHERR_NAMES + 8 * index, code);
            e.mem.set_u32(MATHERR_NAMES + 8 * index + 4, name);
        }
        e.map(USER_MATHERR_SET, 4);
        e.map(USER_MATHERR, 4);
        e.map(SSE2_FLAG, 4);
        e.set_global(SSE2_FLAG, u32::from(sse2));
        e.call_log = Some(vec![]);
        (e, slot)
    }

    fn floor_of(e: &mut Engine, x: f64) -> f64 {
        e.call(0x00ec_6940, &args![x]).f64()
    }

    /// Values at, around and between integers, the edges of the shift (2^52), large magnitudes,
    /// the smallest numbers and the infinities, each with both signs.
    fn values() -> Vec<f64> {
        let two52 = 2f64.powi(52);
        let positive = [
            0.0,
            0.25,
            0.5,
            0.75,
            f64::from_bits(1.0f64.to_bits() - 1),
            1.0,
            f64::from_bits(1.0f64.to_bits() + 1),
            1.5,
            2.0,
            f64::from_bits(2.0f64.to_bits() - 1),
            2.5,
            3.0,
            3.5,
            123.456,
            1.0e15 + 0.5,
            two52 - 0.5,
            two52 - 1.0,
            two52,
            two52 + 1.0,
            2f64.powi(53),
            1.0e300,
            f64::MAX,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::INFINITY,
        ];
        positive.iter().flat_map(|&x| [x, -x]).collect()
    }

    #[test]
    fn floor_sse2_matches_the_host_floor_bit_for_bit() {
        let (mut e, _) = engine(true);
        for x in values() {
            let got = floor_of(&mut e, x);
            assert_eq!(got.to_bits(), x.floor().to_bits(), "floor({x:e}) = {got:e}");
        }
        // Nothing of the x87 body ran.
        assert!(calls(&e, CTRLFP).is_empty());
        assert!(calls(&e, FRND).is_empty());
        assert!(calls(&e, LIBM_ERROR_SUPPORT).is_empty());
    }

    #[test]
    fn floor_sse2_of_a_nan_goes_through_the_error_support_with_its_own_slot() {
        let (mut e, errno) = engine(true);
        // The error support as the exe has it: the default _matherr, errno EDOM, the NaN quiet.
        let signalling = f64::from_bits(0x7ff0_0000_0000_0123);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        let support = calls(&e, LIBM_ERROR_SUPPORT);
        assert_eq!(support.len(), 1);
        let slot = support[0][0];
        assert_eq!(support[0], vec![slot, slot, slot, 0x3ed]);
        assert_eq!(calls(&e, DEFAULT_MATHERR).len(), 1);
        let negative = f64::from_bits(0xfff8_0000_0000_0042);
        assert_eq!(floor_of(&mut e, negative).to_bits(), 0xfff8_0000_0000_0042);
        // A support routine that rewrites the slot: the slot, loaded (FLD quiets it).
        e.register(LIBM_ERROR_SUPPORT, |e, a| {
            assert_eq!(e.mem.u64(a[2]), 0x7ff0_0000_0000_0123);
            e.mem.set_u64(a[2], 0x7ff0_0000_0000_0777);
            Ret::default()
        });
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0777
        );
    }

    #[test]
    fn floor_picks_its_body_from_the_flag_and_the_control_words_masks() {
        // The flag clear: the x87 body, whatever the masks.
        let (mut e, _) = engine(false);
        assert_eq!(floor_of(&mut e, -1.5), -2.0);
        assert_eq!(calls(&e, CTRLFP).len(), 2);
        // The flag set (any nonzero value), every exception masked: the SSE2 body, also at
        // 24-bit precision.
        let (mut e, _) = engine(true);
        e.set_global(SSE2_FLAG, 2u32);
        assert_eq!(floor_of(&mut e, -1.5), -2.0);
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(floor_of(&mut e, 2.5), 2.0);
        assert!(calls(&e, CTRLFP).is_empty());
        // One x87 exception unmasked: the x87 body.
        set_x87_control_word(&mut e, 0x027e);
        assert_eq!(floor_of(&mut e, 2.5), 2.0);
        assert_eq!(
            calls(&e, CTRLFP),
            vec![vec![0x173f, 0xffff], vec![0x027e, 0xffff]]
        );
        assert_eq!(calls(&e, FRND).len(), 1);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_matches_the_host_floor_under_every_old_control_word() {
        let (mut e, _) = engine(false);
        for old in [0x027fu16, 0x007f, 0x0e7f, 0x0a7f, 0x037f] {
            set_x87_control_word(&mut e, old);
            for x in values() {
                let got = floor_of(&mut e, x);
                assert_eq!(
                    got.to_bits(),
                    x.floor().to_bits(),
                    "floor({x:e}) = {got:e} under {old:04x}"
                );
                assert_eq!(x87_control_word(&mut e), old, "the old word is back");
            }
        }
        // __frnd ran under the round-down word (bit 6 reads 1).
        let (mut e, _) = engine(false);
        e.register_double(FRND, |e, a| {
            assert_eq!(x87_control_word(e), 0x177f);
            frnd(e, f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))).into_ret()
        });
        assert_eq!(floor_of(&mut e, 7.75), 7.0);
        assert_eq!(calls(&e, FRND), vec![args![7.75f64]]);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_reports_an_inexact_result_only_with_the_precision_exception_unmasked() {
        let (mut e, _) = engine(false);
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(EXCEPT1, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            42.0f64.into_ret()
        });
        set_x87_control_word(&mut e, 0x025f);
        // Exact: no report.
        assert_eq!(floor_of(&mut e, -3.0), -3.0);
        assert!(seen.borrow().is_empty());
        assert_eq!(x87_control_word(&mut e), 0x025f);
        // Inexact: __except1(0x10, 0xb, x, rounded, old) answers.
        assert_eq!(floor_of(&mut e, 2.5), 42.0);
        assert_eq!(
            *seen.borrow(),
            vec![args![0x10u32, 0xbu32, 2.5f64, 2.0f64, 0x025fu32]]
        );
        // The real __except1: not handled (inexact unmasked), so __raise_exc runs; no error code,
        // so no errno; the old word is put back and the rounded value returned.
        let (mut e, errno) = engine(false);
        let raised: Seen = Rc::default();
        let record = raised.clone();
        e.register_double(RAISE_EXC_EX, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.mem.set_u32(errno, 0);
        set_x87_control_word(&mut e, 0x025f);
        assert_eq!(floor_of(&mut e, -2.5), -3.0);
        assert_eq!(x87_control_word(&mut e), 0x025f);
        assert_eq!(e.mem.u32(errno), 0);
        let raised = raised.borrow();
        assert_eq!(raised.len(), 1);
        assert_eq!(raised[0][2..4], [0x10, 0xb]);
        assert_eq!(raised[0][6], 0);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_sorts_infinities_and_nans_by_sptype() {
        let (mut e, errno) = engine(false);
        // The infinities: back as they are, the old word restored.
        assert_eq!(floor_of(&mut e, f64::INFINITY), f64::INFINITY);
        assert_eq!(floor_of(&mut e, f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert_eq!(
            calls(&e, SPTYPE),
            vec![vec![0, 0x7ff0_0000], vec![0, 0xfff0_0000]]
        );
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // A quiet NaN: __handle_qnan1(0xb, x, old); with the exe's skip flag errno is EDOM and
        // the NaN comes back.
        let quiet = f64::from_bits(0xfff8_0000_0000_0123);
        assert_eq!(floor_of(&mut e, quiet).to_bits(), 0xfff8_0000_0000_0123);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(
            calls(&e, HANDLE_QNAN1),
            vec![args![0xbu32, 0xfff8_0000_0000_0123u64, 0x027fu32]]
        );
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // A signalling NaN is quiet in the pushed copy, so it is class 3 too.
        let signalling = f64::from_bits(0x7ff0_0000_0000_0123);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(
            calls(&e, SPTYPE).last().unwrap(),
            &args![0x7ff8_0000_0000_0123u64]
        );
        // Without the skip flag: _umatherr(1, 0xb, x, 0, x, old), named "floor", errno EDOM.
        e.set_global(MATHERR_SKIP, 0u32);
        e.mem.set_u32(errno, 0);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![
                1u32,
                0xbu32,
                0x7ff8_0000_0000_0123u64,
                0u64,
                0x7ff8_0000_0000_0123u64,
                0x027fu32
            ]]
        );
        assert_eq!(calls(&e, DEFAULT_MATHERR).len(), 1);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // Any other class: __except1(8, 0xb, x, x + 1.0, old).
        e.register(SPTYPE, |_, _| 4i32.into_ret());
        e.register(EXCEPT1, |_, _| 9.0f64.into_ret());
        assert_eq!(floor_of(&mut e, f64::INFINITY), 9.0);
        assert_eq!(
            calls(&e, EXCEPT1),
            vec![args![8u32, 0xbu32, f64::INFINITY, f64::INFINITY, 0x027fu32]]
        );
    }

    #[test]
    fn sptype_classifies_infinities_and_nans() {
        let (mut e, _) = engine(false);
        let class = |e: &mut Engine, bits: u64| e.call(SPTYPE, &args![bits]).i32();
        assert_eq!(class(&mut e, 0x7ff0_0000_0000_0000), 1);
        assert_eq!(class(&mut e, 0xfff0_0000_0000_0000), 2);
        assert_eq!(class(&mut e, 0x7ff8_0000_0000_0000), 3);
        assert_eq!(class(&mut e, 0xffff_ffff_ffff_ffff), 3);
        assert_eq!(class(&mut e, 0x7ff0_0000_0000_0001), 4);
        assert_eq!(class(&mut e, 0xfff4_0000_0000_0000), 4);
        assert_eq!(class(&mut e, 0x7ff7_ffff_0000_0000), 4);
        for number in [0.0f64, -0.0, 1.0, -2.5, f64::MAX, f64::from_bits(1)] {
            assert_eq!(class(&mut e, number.to_bits()), 0);
        }
    }

    #[test]
    fn frnd_rounds_with_the_control_words_mode_and_ignores_precision() {
        let (mut e, _) = engine(false);
        let cases: [(u16, f64, f64); 18] = [
            // Nearest, ties to even; the sign of a zero result kept.
            (0x027f, 2.5, 2.0),
            (0x027f, 3.5, 4.0),
            (0x027f, -2.5, -2.0),
            (0x027f, -0.4, -0.0),
            (0x027f, 0.5, 0.0),
            // Down.
            (0x067f, 2.5, 2.0),
            (0x067f, -2.5, -3.0),
            (0x067f, -0.4, -1.0),
            (0x067f, 0.4, 0.0),
            // Up.
            (0x0a7f, 2.1, 3.0),
            (0x0a7f, -2.9, -2.0),
            (0x0a7f, -0.4, -0.0),
            // Toward zero.
            (0x0e7f, 2.9, 2.0),
            (0x0e7f, -2.9, -2.0),
            // 24-bit precision does not apply to FRNDINT: 2^24 + 1 needs 25 bits.
            (0x047f, 16_777_217.5, 16_777_217.0),
            (0x007f, 16_777_216.75, 16_777_217.0),
            // Large values and infinities are integers already.
            (0x067f, 1.0e300, 1.0e300),
            (0x0a7f, f64::NEG_INFINITY, f64::NEG_INFINITY),
        ];
        for (word, x, want) in cases {
            set_x87_control_word(&mut e, word);
            let got = e.call(FRND, &args![x]).f64();
            assert_eq!(got.to_bits(), want.to_bits(), "frnd({x}) under {word:04x}");
        }
        // A signalling NaN comes back quiet.
        let got = e.call(FRND, &args![0x7ff0_0000_0000_0005u64]).f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0005);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn handle_qnan1_reports_or_sets_edom() {
        let (mut e, errno) = engine(false);
        // The skip flag set: errno EDOM, the word restored, x back (FLD quiets).
        set_x87_control_word(&mut e, 0x0c7f);
        let got = e
            .call(
                HANDLE_QNAN1,
                &args![0xcu32, 0x7ff0_0000_0000_0009u64, 0x027fu32],
            )
            .f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0009);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(calls(&e, UMATHERR).is_empty());
        // Clear: _umatherr's answer.
        e.set_global(MATHERR_SKIP, 0u32);
        e.register(UMATHERR, |_, _| 5.0f64.into_ret());
        let got = e
            .call(
                HANDLE_QNAN1,
                &args![0xcu32, 0x7ff0_0000_0000_0009u64, 0x007fu32],
            )
            .f64();
        assert_eq!(got, 5.0);
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![
                1u32,
                0xcu32,
                0x7ff8_0000_0000_0009u64,
                0u64,
                0x7ff8_0000_0000_0009u64,
                0x007fu32
            ]]
        );
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn errcode_and_set_errno_from_matherr_follow_the_flag_order() {
        let (mut e, errno) = engine(false);
        let code = |e: &mut Engine, flags: u32| e.call(ERRCODE, &args![flags]).i32();
        assert_eq!(code(&mut e, 0x3f), 5);
        assert_eq!(code(&mut e, 0x1f), 1);
        assert_eq!(code(&mut e, 0x17), 2);
        assert_eq!(code(&mut e, 0x13), 3);
        assert_eq!(code(&mut e, 0x12), 4);
        assert_eq!(code(&mut e, 0x10), 0);
        // Only the low byte counts.
        assert_eq!(code(&mut e, 0x100), 0);
        for (kind, want) in [(1i32, EDOM), (2, ERANGE), (3, ERANGE)] {
            e.mem.set_u32(errno, 0);
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
            assert_eq!(e.mem.u32(errno), want);
        }
        for kind in [0i32, 4, 5, -1] {
            e.mem.set_u32(errno, 0);
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
            assert_eq!(e.mem.u32(errno), 0, "type {kind}");
        }
    }

    #[test]
    fn umatherr_names_the_function_or_sets_errno_directly() {
        let (mut e, errno) = engine(false);
        // No name for the code: the word restored, errno from the type, the result back.
        set_x87_control_word(&mut e, 0x0c7f);
        let got = e
            .call(
                UMATHERR,
                &args![3u32, 0x99u32, 1.0f64, 2.0f64, 3.0f64, 0x027fu32],
            )
            .f64();
        assert_eq!(got, 3.0);
        assert_eq!(e.mem.u32(errno), ERANGE);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(calls(&e, DEFAULT_MATHERR).is_empty());
        // A name (the table's last entry): the _exception to _matherr, whose result is the result;
        // a nonzero answer leaves errno alone.
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(DEFAULT_MATHERR, move |e, a| {
            let words = (0..8).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            record.borrow_mut().push(words);
            e.mem.set_u64(a[0] + 0x18, 0x7ff0_0000_0000_0001);
            1u32.into_ret()
        });
        e.mem.set_u32(errno, 0);
        let got = e
            .call(
                UMATHERR,
                &args![2u32, 0x26u32, 1.0f64, -2.0f64, 3.0f64, 0x007fu32],
            )
            .f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0001, "FLD quiets it");
        assert_eq!(e.mem.u32(errno), 0);
        let mut want = vec![2u32, 0x010f_320c];
        want.extend(args![1.0f64, -2.0f64, 3.0f64]);
        assert_eq!(*seen.borrow(), vec![want]);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        // Answering 0: errno from the type.
        e.register(DEFAULT_MATHERR, |_, _| 0u32.into_ret());
        e.call(
            UMATHERR,
            &args![1u32, FLOOR_CODE, 1.0f64, 0.0f64, 1.0f64, 0x027fu32],
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn default_matherr_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(DEFAULT_MATHERR, &args![0x1234u32]).u32(), 0);
    }

    /// Three doubles in engine memory (two arguments and the result); answers their addresses.
    fn slots(e: &mut Engine, a: u64, b: u64, r: u64) -> [u32; 3] {
        let at = e.mem.alloc(0x18);
        e.mem.set_u64(at, a);
        e.mem.set_u64(at + 8, b);
        e.mem.set_u64(at + 0x10, r);
        [at, at + 8, at + 0x10]
    }

    #[test]
    fn libm_error_support_reports_each_code_its_way() {
        let (mut e, errno) = engine(false);
        let seen: Exceptions = Rc::default();
        let record = seen.clone();
        e.register_double(DEFAULT_MATHERR, move |e, a| {
            let at = a[0];
            record.borrow_mut().push((
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u64(at + 8),
                e.mem.u64(at + 0x10),
                e.mem.u64(at + 0x18),
            ));
            0u32.into_ret()
        });
        let support = |e: &mut Engine, s: [u32; 3], code: u32| {
            e.call(LIBM_ERROR_SUPPORT, &args![s[0], s[1], s[2], code]);
        };
        // 0x3ed (floor): the result becomes argument 1, a domain error named "floor".
        let s = slots(
            &mut e,
            0x7ff0_0000_0000_0001,
            2.0f64.to_bits(),
            3.0f64.to_bits(),
        );
        support(&mut e, s, 0x3ed);
        assert_eq!(e.mem.u64(s[2]), 0x7ff8_0000_0000_0001);
        assert_eq!(
            seen.borrow().last(),
            Some(&(
                1,
                0x0109_43f0,
                0x7ff8_0000_0000_0001,
                2.0f64.to_bits(),
                0x7ff8_0000_0000_0001
            ))
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        // 2: a range error of type 2, errno ERANGE, the result as it was.
        let s = slots(&mut e, 1.0f64.to_bits(), 0, 5.0f64.to_bits());
        support(&mut e, s, 2);
        assert_eq!(seen.borrow().last().unwrap().0, 2);
        assert_eq!(seen.borrow().last().unwrap().1, 0x010f_319c);
        assert_eq!(e.mem.u32(errno), ERANGE);
        assert_eq!(e.mem.u64(s[2]), 5.0f64.to_bits());
        // 15: an underflow (type 4); errno left alone.
        e.mem.set_u32(errno, 0);
        support(&mut e, s, 15);
        assert_eq!(seen.borrow().last().unwrap().0, 4);
        assert_eq!(e.mem.u32(errno), 0);
        // 1010: the result is argument 1 times +0.
        let s = slots(&mut e, (-5.0f64).to_bits(), 0, 7.0f64.to_bits());
        support(&mut e, s, 1010);
        assert_eq!(e.mem.u64(s[2]), (-0.0f64).to_bits());
        assert_eq!(seen.borrow().last().unwrap().4, (-0.0f64).to_bits());
        let s = slots(&mut e, f64::INFINITY.to_bits(), 0, 7.0f64.to_bits());
        support(&mut e, s, 1012);
        assert!(f64::from_bits(e.mem.u64(s[2])).is_nan());
        // 0x1a: 1.0, no report; an unknown code: nothing at all.
        let reports = seen.borrow().len();
        let s = slots(&mut e, 0, 0, 7.0f64.to_bits());
        support(&mut e, s, 0x1a);
        assert_eq!(e.mem.u64(s[2]), 1.0f64.to_bits());
        for code in [0x99u32, 0, 999, 1013, 0xffff_ffff] {
            e.mem.set_u64(s[2], 7.0f64.to_bits());
            support(&mut e, s, code);
            assert_eq!(e.mem.u64(s[2]), 7.0f64.to_bits());
        }
        assert_eq!(seen.borrow().len(), reports);
        // A user _matherr (decoded) that handles it and changes the result.
        e.set_global(USER_MATHERR_SET, 1u32);
        e.set_global(USER_MATHERR, 0x1234_5678u32);
        e.register(DECODE_POINTER, |_, a| {
            assert_eq!(a[0], 0x1234_5678);
            USER_HANDLER.into_ret()
        });
        e.register(USER_HANDLER, |e, a| {
            e.mem.set_u64(a[0] + 0x18, 11.0f64.to_bits());
            1u32.into_ret()
        });
        e.mem.set_u32(errno, 0);
        let s = slots(&mut e, 0, 0, 7.0f64.to_bits());
        support(&mut e, s, 1006);
        assert_eq!(e.mem.u64(s[2]), 11.0f64.to_bits());
        assert_eq!(e.mem.u32(errno), 0);
        assert_eq!(seen.borrow().len(), reports);
    }

    /// `__handle_exc(flags, &value, control)`: its answer and the value after.
    fn handle(e: &mut Engine, flags: u32, value: f64, control: u32) -> (i32, f64) {
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, value.to_bits());
        let handled = e.call(HANDLE_EXC, &args![flags, at, control]).i32();
        (handled, e.mem.f64(at))
    }

    #[test]
    fn handle_exc_handles_masked_exceptions() {
        let (mut e, _) = engine(false);
        // Bits 8 and 4 with their masks.
        assert_eq!(handle(&mut e, 8, 1.0, 1), (1, 1.0));
        assert_eq!(handle(&mut e, 9, 1.0, 1).0, 0, "bit 1 left over");
        assert_eq!(handle(&mut e, 8, 1.0, 0).0, 0);
        assert_eq!(handle(&mut e, 4, 1.0, 4), (1, 1.0));
        assert_eq!(calls(&e, SET_STATFP), vec![vec![1], vec![1], vec![4]]);
        // Bit 0x10 with mask 0x20, also after another one.
        assert_eq!(handle(&mut e, 0x10, 1.0, 0x20), (1, 1.0));
        assert_eq!(handle(&mut e, 0x10, 1.0, 0x1f).0, 0);
        assert_eq!(handle(&mut e, 0x14, 1.0, 0x24), (1, 1.0));
        assert_eq!(
            handle(&mut e, 0x20, 1.0, 0x3f).0,
            1,
            "bits above 0x1f are ignored"
        );
    }

    #[test]
    fn handle_exc_gives_the_overflow_value_of_each_rounding_mode() {
        let (mut e, _) = engine(false);
        let inf = f64::INFINITY;
        let max = f64::MAX;
        let nan = f64::from_bits(0x7ff8_0000_0000_0000);
        for (rounding, plus, minus) in [
            (0u32, inf, -inf),
            (0x400, max, -inf),
            (0x800, inf, -max),
            (0xc00, max, -max),
        ] {
            let control = 0x8 | rounding;
            assert_eq!(handle(&mut e, 1, 2.0, control), (1, plus));
            assert_eq!(handle(&mut e, 1, -2.0, control), (1, minus));
            // A NaN or zero counts as negative.
            assert_eq!(handle(&mut e, 1, nan, control).1, minus);
            assert_eq!(handle(&mut e, 1, 0.0, control).1, minus);
        }
        // Unmasked: nothing stored, not handled.
        assert_eq!(handle(&mut e, 1, 2.0, 0x37), (0, 2.0));
    }

    #[test]
    fn handle_exc_denormalizes_an_underflow() {
        let (mut e, _) = engine(false);
        let p = |k: i32| 2f64.powi(k);
        // 2^-1030, a denormal (powi would round through an overflow).
        let tiny = f64::from_bits(1 << 44);
        // The result comes scaled by 2^0x600 (the wrapped exponent): 2^506 is 2^-1030.
        assert_eq!(handle(&mut e, 2, p(506), 0x10), (1, tiny));
        assert_eq!(handle(&mut e, 2, -p(506), 0x10), (1, -tiny));
        assert!(calls(&e, SET_STATFP).is_empty(), "exact");
        // A bit shifted out is inexact.
        let value = f64::from_bits(p(506).to_bits() + 1);
        assert_eq!(handle(&mut e, 2, value, 0x10), (1, tiny));
        assert_eq!(calls(&e, SET_STATFP), vec![vec![0x10]]);
        // At the smallest normal no shift is made.
        assert_eq!(handle(&mut e, 2, p(514), 0x10), (1, f64::MIN_POSITIVE));
        // Below the smallest denormal: the mantissa times 0, inexact.
        let (handled, got) = handle(&mut e, 2, p(456), 0x10);
        assert_eq!((handled, got.to_bits()), (1, 0));
        let (_, got) = handle(&mut e, 2, -p(456), 0x10);
        assert_eq!(got.to_bits(), SIGN);
        // A zero result stays, flagged inexact; flag 0x10 given flags it too, and is left over
        // without its mask.
        assert_eq!(handle(&mut e, 2, 0.0, 0x10), (1, 0.0));
        let statfp = calls(&e, SET_STATFP).len();
        assert_eq!(handle(&mut e, 0x12, p(506), 0x10), (0, tiny));
        assert_eq!(calls(&e, SET_STATFP).len(), statfp + 1);
        // Unmasked: nothing.
        assert_eq!(handle(&mut e, 2, p(506), 0x2f), (0, p(506)));
    }

    #[test]
    fn decomp_and_set_exp_split_and_rebuild_doubles() {
        let (mut e, _) = engine(false);
        let split = |e: &mut Engine, x: f64| {
            let at = e.mem.alloc(4);
            let mantissa = e.call(DECOMP, &args![x, at]).f64();
            (mantissa.to_bits(), e.mem.i32(at))
        };
        let tiny = f64::from_bits(1);
        let three_tiny = -f64::from_bits(3);
        assert_eq!(split(&mut e, 6.0), (0.75f64.to_bits(), 3));
        assert_eq!(split(&mut e, -6.0), ((-0.75f64).to_bits(), 3));
        assert_eq!(split(&mut e, 0.0), (0, 0));
        assert_eq!(split(&mut e, -0.0), (0, 0), "+0 for -0");
        assert_eq!(split(&mut e, tiny), (0.5f64.to_bits(), -1073));
        assert_eq!(split(&mut e, three_tiny), ((-0.75f64).to_bits(), -1072));
        assert_eq!(split(&mut e, f64::INFINITY), (0.5f64.to_bits(), 0x401));
        let rebuild = |e: &mut Engine, x: u64, exponent: i32| {
            e.call(SET_EXP, &args![x, exponent]).f64().to_bits()
        };
        assert_eq!(rebuild(&mut e, 6.0f64.to_bits(), 0), 0.75f64.to_bits());
        assert_eq!(rebuild(&mut e, (-6.0f64).to_bits(), 2), (-3.0f64).to_bits());
        // The top word from the argument as passed, then FLD: a NaN pattern comes back quiet.
        assert_eq!(
            rebuild(&mut e, 0x7ff0_0000_0000_0001, 0x401),
            0x7ff8_0000_0000_0001
        );
        // The exponent wraps into the sign bit, leaving the exponent field 0.
        assert_eq!(
            rebuild(&mut e, 1.0f64.to_bits(), 0x402),
            (-0.0f64).to_bits()
        );
    }

    #[test]
    fn except1_handles_or_raises_then_reports() {
        let (mut e, errno) = engine(false);
        let raised: Seen = Rc::default();
        let record = raised.clone();
        e.register_double(RAISE_EXC_EX, move |e, a| {
            record.borrow_mut().push(a.to_vec());
            // The handler may change the result and the control word in place.
            e.mem.set_u64(a[5], 7.0f64.to_bits());
            e.mem.set_u32(a[1], 0x0a7f);
            Ret::default()
        });
        let except = |e: &mut Engine, flags: u32, x: f64, r: f64, control: u32| {
            e.call(EXCEPT1, &args![flags, 0xbu32, x, r, control]).f64()
        };
        // Handled (invalid masked), the skip flag set: errno from the type, the word restored.
        set_x87_control_word(&mut e, 0x0c7f);
        let got = except(&mut e, 8, f64::INFINITY, f64::INFINITY, 0x027f);
        assert_eq!(got, f64::INFINITY);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(raised.borrow().is_empty());
        assert!(calls(&e, UMATHERR).is_empty());
        // The skip flag clear: _umatherr(errcode, code, x, 0, result, cw).
        e.set_global(MATHERR_SKIP, 0u32);
        let got = except(&mut e, 1, 3.0, 1.0e308, 0x027f);
        assert_eq!(got, f64::INFINITY, "overflow to +inf (round to nearest)");
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![3u32, 0xbu32, 3.0f64, 0u64, f64::INFINITY, 0x027fu32]]
        );
        assert_eq!(e.mem.u32(errno), ERANGE);
        // Not handled: __raise_exc(&local, &cw, flags, code, &x, &result) with a 0 after; what it
        // leaves is used.
        e.set_global(MATHERR_SKIP, 1u32);
        let got = except(&mut e, 0x10, 2.5, 2.0, 0x025f);
        assert_eq!(got, 7.0);
        assert_eq!(x87_control_word(&mut e), 0x0a7f);
        let raised = raised.borrow();
        assert_eq!(raised.len(), 1);
        let call = &raised[0];
        assert_eq!(call[1], call[0] + 0x90);
        assert_eq!(call[2..4], [0x10, 0xb]);
        assert_eq!(
            (call[4], call[5], call[6]),
            (call[0] + 0x80, call[0] + 0x88, 0)
        );
        assert_eq!(calls(&e, RAISE_EXC), vec![call[..6].to_vec()]);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn status_flag_helper_reads_its_constants_only() {
        let (mut e, _) = engine(false);
        for flags in [0u32, 1, 4, 8, 0x10, 0x20, 0x3f] {
            assert_eq!(e.call(SET_STATFP, &args![flags]), Ret::default());
        }
        // The loads read the constants: without them the run stops; the other bits read nothing.
        let mut bare = Engine::new();
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bare.call(SET_STATFP, &args![1u32]);
        }));
        assert!(stopped.is_err());
        bare.call(SET_STATFP, &args![0x24u32]);
    }
}
