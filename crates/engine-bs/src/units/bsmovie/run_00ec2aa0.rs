//! `BSMovie/run_00ec2aa0` (the engine map's run of unnamed functions after `00ec2aa0`),
//! subsystem `BSMovie`: four methods of the movie player the game's movie code calls
//! (`00ec0280`), translated from FalloutNV.exe 1.4.0.525 (docs/ENGINE_CRATE.md): three scale
//! computations on the player's virtual size getters (x87 arithmetic, the result in `ST0`) and
//! the lookup of a movie's path.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::{x87_fadd, x87_fdiv, x87_fmul, x87_fsub};

/// The player's virtual functions (`thiscall`, no stack words, the answer in `EAX`) the scale
/// methods read: slots +0x28 and +0x2c (two sizes), +0x34 (an offset).
const SLOT_28: u32 = 0x28;
const SLOT_2C: u32 = 0x2c;
const SLOT_34: u32 = 0x34;
/// The double the offset computations divide by (`01011590`, read from the executable's data).
const DIVISOR: u32 = 0x0101_1590;
/// `00853130` (`thiscall` on the player's +0x40, two words: a name and the address of a word
/// that receives a path; answers `AL`) and `strcpy_s` (`00ec65a6`, cdecl).
const FIND_PATH: u32 = 0x0085_3130;
const STRCPY_S: u32 = 0x00ec_65a6;

/// `FILD qword` of a word zero-extended to 64 bits (`MOV [t],EAX; MOV [t+4],0`): exact.
fn fild_word(word: u32) -> X87Ext {
    X87Ext::load64(f64::from(word))
}

/// `FSTP float; FLD float`: the value rounded to a float and loaded back, as `ST0`.
fn through_float(x: X87Ext) -> f64 {
    x87_load32(x.store32())
}

// Translated from 00ec2aa0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The scale of one size (no label; `thiscall`, three words, `RET 0xc`): with the flag byte
/// set, the player's slot +0x28 size over `width`; otherwise its slot +0x2c size over `height`
/// (both zero-extended words, `FILD qword`, `FDIVP`), rounded to a float (`FSTP`/`FLD`) and
/// answered in `ST0`.
pub fn movie_scale(e: &mut Engine, this: Ptr, width: u32, height: u32, flag: u32) -> f64 {
    let (size, by) = if flag as u8 != 0 {
        (e.vcall(this.addr(), SLOT_28, &args![]).u32(), width)
    } else {
        (e.vcall(this.addr(), SLOT_2C, &args![]).u32(), height)
    };
    // FILD size; FILD by; FDIVP ST(1),ST(0): size / by.
    through_float(x87_fdiv(fild_word(size), fild_word(by)))
}

// Translated from 00ec2b20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The horizontal offset (no label; `thiscall`, three words: a float scale, a width, the flag
/// byte; `RET 0xc`). With the flag set: the player's slot +0x34 word as is (`FILD`, not
/// rounded). Otherwise `offset + (size - width * scale) / [01011590]`, with `offset` from slot
/// +0x34 (kept as a double, `FSTP qword`) and `size` from slot +0x28, in the code's x87 order
/// (`FMUL` by the float, `FSUBP`, `FDIV` by the double, `FADD` of the kept double), rounded to a
/// float. Answers `ST0`.
pub fn movie_offset_x(e: &mut Engine, this: Ptr, scale: u32, width: u32, flag: u32) -> f64 {
    let offset = e.vcall(this.addr(), SLOT_34, &args![]).u32();
    if flag as u8 != 0 {
        return fild_word(offset).store64();
    }
    let kept = fild_word(offset).store64();
    let size = e.vcall(this.addr(), SLOT_28, &args![]).u32();
    let scaled = x87_fmul(fild_word(width), X87Ext::load32(scale));
    let difference = x87_fsub(fild_word(size), scaled);
    let divisor = f64::from_bits(e.mem.u64(DIVISOR));
    let quotient = x87_fdiv(difference, X87Ext::load64(divisor));
    through_float(x87_fadd(quotient, X87Ext::load64(kept)))
}

// Translated from 00ec2bb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The vertical offset (no label; `thiscall`, three words: a float scale, a height, the flag
/// byte; `RET 0xc`): 0 (`FLDZ`) without the flag; otherwise `(size - height * scale) /
/// [01011590]` with `size` from the player's slot +0x2c, in the code's x87 order, rounded to a
/// float. Answers `ST0`.
pub fn movie_offset_y(e: &mut Engine, this: Ptr, scale: u32, height: u32, flag: u32) -> f64 {
    if flag as u8 == 0 {
        return 0.0;
    }
    let size = e.vcall(this.addr(), SLOT_2C, &args![]).u32();
    let scaled = x87_fmul(fild_word(height), X87Ext::load32(scale));
    let difference = x87_fsub(fild_word(size), scaled);
    let divisor = f64::from_bits(e.mem.u64(DIVISOR));
    through_float(x87_fdiv(difference, X87Ext::load64(divisor)))
}

// Translated from 00ec2df0 (disassembled, FalloutNV.exe 1.4.0.525)
/// A movie's path (no label; `thiscall`, two words: the name and the output buffer, `RET 8`):
/// `00853130` on the player's +0x40 with the name and the address of a zeroed local; when it
/// answers true (`AL`), the path it left there is copied into the buffer with `strcpy_s(buffer,
/// 0x104, path)` and true is answered; otherwise false.
pub fn movie_find_path(e: &mut Engine, this: Ptr, name: u32, buffer: u32) -> bool {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        let found = e
            .call(
                FIND_PATH,
                &args![this.addr().wrapping_add(0x40), name, local.addr()],
            )
            .u8();
        if found == 0 {
            return false;
        }
        let path = e.mem.u32(local.addr());
        e.call(STRCPY_S, &args![buffer, 0x104_u32, path]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_2aa0, movie_scale(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2b20, movie_offset_x(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2bb0, movie_offset_y(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2df0, movie_find_path(Ptr, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    const PLAYER: u32 = 0x0800_0000;
    const VTABLE: u32 = 0x0900_0000;
    const GET_28: u32 = 0x0a00_0028;
    const GET_2C: u32 = 0x0a00_002c;
    const GET_34: u32 = 0x0a00_0034;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .cloned()
            .collect()
    }

    /// A player whose slots +0x28, +0x2c and +0x34 answer `a`, `b` and `c`, the divisor 2.0,
    /// and the call log on.
    fn engine(a: u32, b: u32, c: u32) -> Engine {
        let mut e = Engine::new();
        e.map(PLAYER, 0x100);
        e.map(VTABLE, 0x100);
        e.mem.set_u32(PLAYER, VTABLE);
        e.mem.set_u32(VTABLE + SLOT_28, GET_28);
        e.mem.set_u32(VTABLE + SLOT_2C, GET_2C);
        e.mem.set_u32(VTABLE + SLOT_34, GET_34);
        e.register_double(GET_28, move |_, _| a.into_ret());
        e.register_double(GET_2C, move |_, _| b.into_ret());
        e.register_double(GET_34, move |_, _| c.into_ret());
        e.map(DIVISOR, 8);
        e.mem.set_u64(DIVISOR, 2.0f64.to_bits());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn scale_divides_the_chosen_size_and_rounds_to_a_float() {
        let mut e = engine(1280, 720, 0);
        let p = Ptr::new(PLAYER);
        assert_eq!(movie_scale(&mut e, p, 640, 480, 1), 2.0);
        assert_eq!(movie_scale(&mut e, p, 640, 480, 0), 1.5);
        // 1280 / 3 rounded to a float.
        assert_eq!(
            movie_scale(&mut e, p, 3, 480, 1),
            f64::from(1280.0f32 / 3.0)
        );
        // Only the low byte of the flag word counts.
        assert_eq!(movie_scale(&mut e, p, 640, 480, 0x100), 1.5);
        assert_eq!(calls(&e, GET_28).len(), 2);
        assert_eq!(calls(&e, GET_2C).len(), 2);
        // Through the entry, with the player as `this`.
        let r = e.call(0x00ec_2aa0, &args![PLAYER, 640u32, 480u32, 1u32]);
        assert_eq!(r.st0, 2.0);
    }

    #[test]
    fn offsets_follow_the_x87_order() {
        let mut e = engine(1280, 720, 7);
        let p = Ptr::new(PLAYER);
        let half = 0.5f32.to_bits();
        // With the flag: the slot +0x34 word as is.
        assert_eq!(movie_offset_x(&mut e, p, half, 100, 1), 7.0);
        // 7 + (1280 - 100 * 0.5) / 2 = 622.
        assert_eq!(movie_offset_x(&mut e, p, half, 100, 0), 622.0);
        // (720 - 100 * 0.5) / 2 = 335; without the flag 0.
        assert_eq!(movie_offset_y(&mut e, p, half, 100, 1), 335.0);
        assert_eq!(movie_offset_y(&mut e, p, half, 100, 0), 0.0);
        // A result past float precision is rounded to a float: 7 + 1/3 * 0.5.
        e.mem.set_u64(DIVISOR, 3.0f64.to_bits());
        let x = movie_offset_x(&mut e, p, half, 2558, 0);
        assert_eq!(x, f64::from((7.0f64 + (1280.0 - 1279.0) / 3.0) as f32));
        // A negative difference stays negative.
        e.mem.set_u64(DIVISOR, 2.0f64.to_bits());
        assert_eq!(movie_offset_y(&mut e, p, half, 2000, 1), -140.0);
    }

    #[test]
    fn find_path_copies_what_00853130_found() {
        let mut e = engine(0, 0, 0);
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"Data\\Video");
        e.register_double(FIND_PATH, move |e, a| {
            e.mem.set_u32(a[2], path);
            1u32.into_ret()
        });
        e.register(STRCPY_S, |_, _| 0u32.into_ret());
        let name = e.mem.alloc(8);
        let buffer = e.mem.alloc(0x104);
        assert!(movie_find_path(&mut e, Ptr::new(PLAYER), name, buffer));
        let find = calls(&e, FIND_PATH);
        assert_eq!((find[0].1[0], find[0].1[1]), (PLAYER + 0x40, name));
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![(STRCPY_S, vec![buffer, 0x104, path])]
        );
        // Not found (the low byte only): nothing copied.
        e.register(FIND_PATH, |_, _| 0x100u32.into_ret());
        assert!(!movie_find_path(&mut e, Ptr::new(PLAYER), name, buffer));
        assert_eq!(calls(&e, STRCPY_S).len(), 1);
    }
}
