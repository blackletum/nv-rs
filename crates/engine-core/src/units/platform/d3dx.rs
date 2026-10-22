//! The platform's D3DX (`d3dx9_38.dll`), the maintainer's decision (2026-10-22, option b): D3DX
//! is Microsoft's code, a platform layer like the Win32 imports, so it is not translated. Each
//! function the game reaches is written here from its documented meaning and matched bit for
//! bit against what the player's DLL answers: `scripts/d3dx-measure` calls the DLL in a 32-bit
//! process on many inputs (random, edge cases, NaN/Inf/denormals, out and in pointers the same)
//! and records inputs and results; the tests here check against those records. The DLL picks a
//! CPU-specific path, so the records are the measuring machine's (AMD Ryzen AI 9 HX 370,
//! 2026-10-22).
//!
//! Functions reached so far (import slots of FalloutNV.exe 1.4.0.525):
//! - `D3DXMatrixMultiply` (slot `00fdf424`, thunk `00ee6de0`): first reached by
//!   `Do_SetCameraData` `00e6c780` in `NiDX9Renderer::Initialize` `00e72e60`.

use crate::{Engine, Ptr};

/// IEEE single-precision words, with the NaN rules of the SSE instructions spelled out (an
/// operation with a NaN operand answers its first NaN operand, quieted; an invalid operation
/// answers the default NaN `ffc00000`), so results do not depend on how the compiler orders
/// commutative operations or on the host's NaN rules.
mod single {
    fn is_nan(x: u32) -> bool {
        x & 0x7fff_ffff > 0x7f80_0000
    }

    fn op(x: u32, y: u32, f: fn(f32, f32) -> f32) -> u32 {
        if is_nan(x) {
            return x | 0x0040_0000;
        }
        if is_nan(y) {
            return y | 0x0040_0000;
        }
        let r = f(f32::from_bits(x), f32::from_bits(y));
        if r.is_nan() {
            0xffc0_0000
        } else {
            r.to_bits()
        }
    }

    pub fn mul(x: u32, y: u32) -> u32 {
        op(x, y, |a, b| a * b)
    }

    pub fn add(x: u32, y: u32) -> u32 {
        op(x, y, |a, b| a + b)
    }
}

/// The 16 words of a `D3DXMATRIX` (row major) at `at`.
fn matrix(e: &Engine, at: u32) -> [u32; 16] {
    let mut m = [0u32; 16];
    for (i, w) in m.iter_mut().enumerate() {
        *w = e.mem.u32(at.wrapping_add(4 * i as u32));
    }
    m
}

/// `M1 * M2` as the measured DLL works it out. Each word is `sum over k of m1[i][k] * m2[k][j]`
/// in single precision, with these orders (found from the 40010 measured calls, which all match
/// bit for bit, NaN payloads included; the x87 control word and pointer aliasing make no
/// difference):
/// - rows 0, 1 and 2 multiply `m1[i][k] * m2[k][j]`; row 3 multiplies `m2[k][j] * m1[i][k]`
///   (which NaN wins when both are NaNs);
/// - rows 0 and 3 add `(p0 + p1) + (p2 + p3)`; rows 1 and 2 add `((p0 + p1) + p2) + p3`.
pub fn multiply(a: &[u32; 16], b: &[u32; 16]) -> [u32; 16] {
    let mut out = [0u32; 16];
    for i in 0..4 {
        for j in 0..4 {
            let p: [u32; 4] = std::array::from_fn(|k| {
                if i == 3 {
                    single::mul(b[4 * k + j], a[4 * i + k])
                } else {
                    single::mul(a[4 * i + k], b[4 * k + j])
                }
            });
            out[4 * i + j] = if i == 0 || i == 3 {
                single::add(single::add(p[0], p[1]), single::add(p[2], p[3]))
            } else {
                single::add(single::add(single::add(p[0], p[1]), p[2]), p[3])
            };
        }
    }
    out
}

// Platform: replaces d3dx9_38.dll D3DXMatrixMultiply (import)
/// `D3DXMatrixMultiply(out, m1, m2)` (stdcall): `out = m1 * m2` (row-vector convention:
/// `out[i][j] = sum over k of m1[i][k] * m2[k][j]`, in [`multiply`]'s order), both read before
/// `out` is written, so `out` may be either input (measured); answers `out`.
pub fn matrix_multiply(e: &mut Engine, out: Ptr, m1: Ptr, m2: Ptr) -> u32 {
    let a = matrix(e, m1.addr());
    let b = matrix(e, m2.addr());
    let r = multiply(&a, &b);
    for (i, w) in r.iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The measured calls kept as test data (`data/d3dx/matrix_multiply.tsv`, from
    /// `scripts/d3dx-measure`): alias, control word, M1, M2, the DLL's result, returned.
    fn measured() -> Vec<(String, [u32; 16], [u32; 16], [u32; 16])> {
        let text = include_str!("../../../data/d3dx/matrix_multiply.tsv");
        let words = |t: &str| -> [u32; 16] {
            let v: Vec<u32> = t.split(' ').map(|w| u32::from_str_radix(w, 16).unwrap()).collect();
            v.try_into().unwrap()
        };
        text.lines()
            .filter(|l| l.starts_with("MatrixMultiply"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                assert_eq!(f[6], "1");
                (f[1].to_string(), words(f[3]), words(f[4]), words(f[5]))
            })
            .collect()
    }

    #[test]
    fn multiply_matches_the_measured_dll_bit_for_bit() {
        let cases = measured();
        assert!(cases.len() > 200);
        for (alias, a, b, out) in &cases {
            assert_eq!(&multiply(a, b), out, "{alias} {a:08x?} * {b:08x?}");
        }
    }

    #[test]
    fn the_import_writes_out_and_answers_it_even_when_out_is_an_input() {
        for (alias, a, b, out) in measured() {
            let mut e = Engine::new();
            let m1 = e.mem.alloc(64);
            let m2 = if alias.contains("m1=m2") { m1 } else { e.mem.alloc(64) };
            let target = match alias.as_str() {
                "out=m1" | "out=m1=m2" => m1,
                "out=m2" => m2,
                _ => e.mem.alloc(64),
            };
            for k in 0..16u32 {
                e.mem.set_u32(m1 + 4 * k, a[k as usize]);
                if m2 != m1 {
                    e.mem.set_u32(m2 + 4 * k, b[k as usize]);
                }
            }
            let answer = matrix_multiply(&mut e, Ptr::new(target), Ptr::new(m1), Ptr::new(m2));
            assert_eq!(answer, target);
            assert_eq!(matrix(&e, target), out, "{alias}");
        }
    }

    #[test]
    fn the_import_slot_is_registered_by_name() {
        let table = crate::units::platform::imports();
        assert!(table
            .iter()
            .any(|(dll, name, _)| *dll == "d3dx9_38.dll" && *name == "D3DXMatrixMultiply"));
    }
}
