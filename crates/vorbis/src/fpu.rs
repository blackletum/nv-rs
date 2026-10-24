//! The x87 arithmetic the player's library runs on, at the decoding thread's precision control.
//!
//! Values are carried as `f64`. At 24-bit precision every add, subtract and multiply rounds
//! its result to a 24-bit mantissa (the same as `f32` arithmetic in the same order). At 53-bit
//! precision the operations round to 53 bits (`f64` arithmetic), and a value is rounded to
//! `f32` where the C source assigns it to a `float` (`store`), and, depending on how the
//! library was compiled, where it assigns it to a `float` local (`local`). The x87 unit's wider
//! exponent range is not modelled; audio does not come near `f32`'s limits.

/// The precision control of the decoding thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    /// `PC_24`: Direct3D's setting on the thread that made the device.
    Bits24,
    /// `PC_53`: the default of a new Windows thread. Not provided yet: the models of this
    /// module (locals rounded or kept, the angles at either precision) do not reproduce the
    /// player's library at 53 bits (see `diag`).
    Bits53,
}

#[derive(Debug, Clone, Copy)]
pub struct Fpu {
    p24: bool,
    locals: bool,
    trig24: bool,
}

impl Fpu {
    pub fn new(p: Precision) -> Fpu {
        match p {
            Precision::Bits24 => Fpu::model(true, true, true),
            Precision::Bits53 => panic!(
                "Vorbis decoding at 53-bit x87 precision is not provided (no model matches the \n                 player's library yet)"
            ),
        }
    }

    /// A model of the arithmetic: operations rounded to 24 bits or kept at 53, `float` locals
    /// rounded where assigned or not, the tables' angles at 24 bits or 53.
    pub fn model(p24: bool, locals: bool, trig24: bool) -> Fpu {
        Fpu {
            p24,
            locals,
            trig24,
        }
    }

    /// Whether the double-precision steps (the tables' angles) round to 24 bits.
    pub fn trig_24(self) -> bool {
        self.trig24
    }

    #[inline(always)]
    fn r(self, x: f64) -> f64 {
        if self.p24 {
            f64::from(x as f32)
        } else {
            x
        }
    }

    #[inline(always)]
    pub fn add(self, a: f64, b: f64) -> f64 {
        self.r(a + b)
    }

    #[inline(always)]
    pub fn sub(self, a: f64, b: f64) -> f64 {
        self.r(a - b)
    }

    #[inline(always)]
    pub fn mul(self, a: f64, b: f64) -> f64 {
        self.r(a * b)
    }

    /// A value assigned to a `float` local.
    #[inline(always)]
    pub fn local(self, x: f64) -> f64 {
        if self.locals {
            f64::from(x as f32)
        } else {
            x
        }
    }
}

/// A value stored to `float` memory.
#[inline(always)]
pub fn store(x: f64) -> f32 {
    x as f32
}
