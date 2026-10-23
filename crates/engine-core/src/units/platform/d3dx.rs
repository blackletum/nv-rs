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

/// `D3DXERR_INVALIDDATA`.
pub const D3DXERR_INVALIDDATA: u32 = 0x8876_0b59;
/// `D3DRTYPE_TEXTURE`, `D3DXIFF_DDS`.
const D3DRTYPE_TEXTURE: u32 = 3;
const D3DXIFF_DDS: u32 = 4;
/// `DDS_HEADER` fields (after the 4-byte magic): height, width, depth, mip count, the pixel
/// format's flags, FourCC, bit count and the red, green, blue and alpha masks, `dwCaps2`.
const DDS_HEIGHT: usize = 12;
const DDS_WIDTH: usize = 16;
const DDS_MIP_COUNT: usize = 28;
const DDS_PF_FLAGS: usize = 80;
const DDS_PF_FOURCC: usize = 84;
const DDS_PF_BITS: usize = 88;
const DDS_PF_MASKS: usize = 92;
const DDS_CAPS2: usize = 112;
const DDS_HEADER_SIZE: usize = 128;
const DDPF_ALPHAPIXELS: u32 = 0x1;
const DDPF_FOURCC: u32 = 0x4;
const DDPF_RGB: u32 = 0x40;
const DDSCAPS2_CUBEMAP: u32 = 0x200;
const DDSCAPS2_VOLUME: u32 = 0x20_0000;

/// What a DDS file describes (`D3DXIMAGE_INFO`'s words) and where its levels are.
#[derive(Debug, Clone, PartialEq)]
pub struct DdsImage {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub format: u32,
    /// Each level's bytes: (offset in the file, length), largest first.
    pub levels: Vec<(usize, usize)>,
}

/// The Direct3D 9 format of a DDS pixel format, for the formats measured with the player's
/// DLL (2026-10-22: DXT1, DXT3, DXT5; 24-bit RGB as `D3DFMT_R8G8B8`, 32-bit ARGB with alpha as
/// `D3DFMT_A8R8G8B8`, 16-bit 4444 as `D3DFMT_A4R4G4B4`), with each one's block side and bytes per
/// block. Others are not provided.
fn dds_format(flags: u32, fourcc: u32, bits: u32, masks: [u32; 4]) -> (u32, u32, u32) {
    if flags & DDPF_FOURCC != 0 {
        return match &fourcc.to_le_bytes() {
            b"DXT1" => (fourcc, 4, 8),
            b"DXT2" | b"DXT3" | b"DXT4" | b"DXT5" => (fourcc, 4, 16),
            _ => panic!("D3DX: a DDS file with FourCC {fourcc:#x} is not provided"),
        };
    }
    let alpha = flags & DDPF_ALPHAPIXELS != 0;
    match (flags & DDPF_RGB != 0, bits, masks, alpha) {
        (true, 24, [0xff_0000, 0xff00, 0xff, 0], false) => (20, 1, 3),
        (true, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000], true) => (21, 1, 4),
        (true, 16, [0xf00, 0xf0, 0xf, 0xf000], true) => (26, 1, 2),
        _ => panic!(
            "D3DX: a DDS pixel format (flags {flags:#x}, {bits} bits, masks {masks:08x?}) is not provided"
        ),
    }
}

/// Reads a DDS file the way the measured DLL does (2026-10-22, 983 of the game's textures and
/// variants of 99 of them): the magic `DDS ` and a 128-byte header; the mip count is the
/// header's field whatever `DDSD_MIPMAPCOUNT` says, 0 counting as 1; the file must hold every
/// level, or the answer is `D3DXERR_INVALIDDATA` (a file one byte short, the header alone, a
/// mip count beyond the data, a broken magic). Not provided (the run stops): other image file
/// formats, cube and volume textures, more levels than the full chain, and the pixel formats
/// [`dds_format`] doesn't list.
pub fn read_dds(data: &[u8]) -> Result<DdsImage, u32> {
    // Measured: a DDS file with its first byte broken, or cut short of its header, is refused.
    // Anything else that is not a DDS file may be another format the DLL reads: not provided.
    if data.len() < 4 || &data[1..4] != b"DS " {
        panic!("D3DX: image files other than DDS are not provided");
    }
    if data[0] != b'D' || data.len() < DDS_HEADER_SIZE {
        return Err(D3DXERR_INVALIDDATA);
    }
    let word = |at: usize| u32::from_le_bytes(data[at..at + 4].try_into().unwrap());
    if word(DDS_CAPS2) & (DDSCAPS2_CUBEMAP | DDSCAPS2_VOLUME) != 0 {
        panic!("D3DX: cube and volume DDS files are not provided");
    }
    let (width, height) = (word(DDS_WIDTH), word(DDS_HEIGHT));
    let mip_levels = word(DDS_MIP_COUNT).max(1);
    let full = 32 - width.max(height).max(1).leading_zeros();
    if mip_levels > full {
        panic!("D3DX: a DDS file with {mip_levels} levels for {width}x{height} is not provided");
    }
    let masks = [0, 1, 2, 3].map(|i| word(DDS_PF_MASKS + 4 * i));
    let (format, block, bytes) =
        dds_format(word(DDS_PF_FLAGS), word(DDS_PF_FOURCC), word(DDS_PF_BITS), masks);
    let mut levels = Vec::new();
    let mut at = DDS_HEADER_SIZE;
    for level in 0..mip_levels {
        let (w, h) = ((width >> level).max(1), (height >> level).max(1));
        let size = (w.div_ceil(block) * h.div_ceil(block) * bytes) as usize;
        levels.push((at, size));
        at += size;
    }
    if at > data.len() {
        return Err(D3DXERR_INVALIDDATA);
    }
    Ok(DdsImage {
        width,
        height,
        mip_levels,
        format,
        levels,
    })
}

// Platform: replaces d3dx9_38.dll D3DXGetImageInfoFromFileInMemory (import)
/// `D3DXGetImageInfoFromFileInMemory(data, size, info)` (stdcall): for a DDS file
/// ([`read_dds`]) `D3D_OK` and the `D3DXIMAGE_INFO` (width, height, depth 1, mip levels,
/// format, `D3DRTYPE_TEXTURE`, `D3DXIFF_DDS`); a file it refuses answers its error and writes
/// nothing (measured). A null info is not provided.
pub fn get_image_info_from_file_in_memory(e: &mut Engine, data: Ptr, size: u32, info: Ptr) -> u32 {
    assert!(!info.is_null(), "D3DXGetImageInfoFromFileInMemory without an info is not provided");
    let bytes = e.mem.bytes(data.addr(), size);
    match read_dds(&bytes) {
        Ok(image) => {
            let words = [
                image.width,
                image.height,
                1,
                image.mip_levels,
                image.format,
                D3DRTYPE_TEXTURE,
                D3DXIFF_DDS,
            ];
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(info.addr() + 4 * i as u32, *w);
            }
            0
        }
        Err(hr) => hr,
    }
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

    /// A DDS header (TEST DATA, made from the documented layout): `width` x `height` DXT1 with
    /// `mips` levels, followed by `extra` bytes of level data.
    fn dxt1(width: u32, height: u32, mips: u32, data: usize) -> Vec<u8> {
        let mut v = vec![0u8; 128 + data];
        v[..4].copy_from_slice(b"DDS ");
        let mut put = |at: usize, w: u32| v[at..at + 4].copy_from_slice(&w.to_le_bytes());
        put(4, 124);
        put(8, 0x000a_1007);
        put(DDS_HEIGHT, height);
        put(DDS_WIDTH, width);
        put(DDS_MIP_COUNT, mips);
        put(76, 32);
        put(DDS_PF_FLAGS, DDPF_FOURCC);
        put(DDS_PF_FOURCC, u32::from_le_bytes(*b"DXT1"));
        v
    }

    #[test]
    fn dds_info_follows_the_measured_rules() {
        // 8x4 DXT1, 4 levels: 2x1 blocks, then 1x1, 1x1, 1x1: 16 + 8 + 8 + 8 bytes.
        let image = read_dds(&dxt1(8, 4, 4, 40)).unwrap();
        assert_eq!((image.width, image.height, image.mip_levels), (8, 4, 4));
        assert_eq!(image.format, u32::from_le_bytes(*b"DXT1"));
        assert_eq!(image.levels, vec![(128, 16), (144, 8), (152, 8), (160, 8)]);
        assert_eq!(read_dds(&dxt1(8, 4, 4, 39)), Err(D3DXERR_INVALIDDATA));
        assert_eq!(read_dds(&dxt1(8, 4, 0, 16)).unwrap().mip_levels, 1);
        assert_eq!(read_dds(&dxt1(8, 4, 1, 16)[..127]), Err(D3DXERR_INVALIDDATA));
        let mut broken = dxt1(8, 4, 1, 16);
        broken[0] = b'X';
        assert_eq!(read_dds(&broken), Err(D3DXERR_INVALIDDATA));
    }

    #[test]
    fn image_info_is_written_only_for_a_good_file() {
        let mut e = Engine::new();
        let file = dxt1(8, 4, 4, 40);
        let data = e.mem.alloc(file.len() as u32);
        e.mem.write(data, &file);
        let info = e.mem.alloc(28);
        let hr = get_image_info_from_file_in_memory(&mut e, Ptr::new(data), file.len() as u32, Ptr::new(info));
        assert_eq!(hr, 0);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(info + 4 * i)).collect();
        assert_eq!(words, [8, 4, 1, 4, u32::from_le_bytes(*b"DXT1"), 3, 4]);
        e.mem.write(info, &[0x55; 28]);
        let hr = get_image_info_from_file_in_memory(&mut e, Ptr::new(data), file.len() as u32 - 1, Ptr::new(info));
        assert_eq!(hr, D3DXERR_INVALIDDATA);
        assert_eq!(e.mem.u32(info), 0x5555_5555);
    }

    /// Checks every measured `D3DXGetImageInfoFromFileInMemory` answer against [`read_dds`]:
    /// `NV_D3DX_IMAGE_INFO=<measure dir>` with `d3dx-imageinfo-*.tsv` and the `dds` copies
    /// (`scripts/d3dx-measure image-info`; the files are the player's, so they stay there).
    #[test]
    #[ignore]
    fn image_info_matches_the_measured_dll() {
        let dir = std::path::PathBuf::from(std::env::var("NV_D3DX_IMAGE_INFO").expect("the measure dir"));
        let table = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|d| d.path())
            .find(|p| p.file_name().unwrap().to_string_lossy().starts_with("d3dx-imageinfo-"))
            .expect("a d3dx-imageinfo table");
        let mut checked = 0;
        for line in std::fs::read_to_string(table).unwrap().lines() {
            let f: Vec<&str> = line.split('\t').collect();
            if f[0] != "ImageInfo" {
                continue;
            }
            let file = std::fs::read(dir.join("dds").join(f[1])).unwrap();
            let size = usize::from_str_radix(f[3], 16).unwrap();
            let mut data = file.clone();
            match f[2] {
                "as-is" => {}
                "header-only" | "short-127" | "minus-1" => data.truncate(size),
                "no-mipcount-flag" => {
                    let flags = u32::from_le_bytes(data[8..12].try_into().unwrap()) & !0x2_0000;
                    data[8..12].copy_from_slice(&flags.to_le_bytes());
                }
                "mipcount-0" => data[28..32].copy_from_slice(&0u32.to_le_bytes()),
                "mipcount-99" => continue, // more levels than the chain: not provided
                "bad-magic" => data[0] = b'X',
                other => panic!("variant {other}"),
            }
            let hr = u32::from_str_radix(f[4], 16).unwrap();
            let mut e = Engine::new();
            let at = e.mem.alloc(data.len() as u32 + 1);
            e.mem.write(at, &data);
            let info = e.mem.alloc(28);
            e.mem.write(info, &[0x55; 28]);
            let ours = get_image_info_from_file_in_memory(&mut e, Ptr::new(at), data.len() as u32, Ptr::new(info));
            let bytes: String = e.mem.bytes(info, 28).iter().map(|b| format!("{b:02x}")).collect();
            assert_eq!((ours, bytes.as_str()), (hr, f[5]), "{} {}", f[1], f[2]);
            checked += 1;
        }
        assert!(checked > 1000, "{checked}");
    }
}
