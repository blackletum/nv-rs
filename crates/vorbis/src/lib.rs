// Follows libvorbis 1.2.0, libvorbisfile 1.2.0 and libogg 1.1.3, Copyright (c) 2002-2004
// Xiph.org Foundation, BSD licence: see ../LICENSE-libvorbis.
//! An Ogg Vorbis decoder for the game's voice and sound files (63,575 `.ogg` files in its
//! archives), in place of the player's `libvorbisfile.dll` / `libvorbis.dll` (libVorbis I
//! 20070622, that is 1.2.0), which the game calls from `00af1e00` (open: `ov_open_callbacks`,
//! `ov_info`, `ov_seekable`, `ov_pcm_total`) and `00af2100` (`ov_read` to 16-bit signed
//! little-endian).
//!
//! The steps and their order of floating-point operations follow the public source of the same
//! library versions (BSD licence, `LICENSE-libvorbis`): Ogg framing, the setup header,
//! codebooks, floor 1, residues 0 to 2, channel coupling, the inverse MDCT, windowed
//! overlap-add, the granule-position trimming and `ov_read`'s conversion. Nothing is taken from
//! the player's DLLs; they are only measured (`scripts/vorbis-record` records the PCM they give
//! for every game file).
//!
//! The player's library computes on the x87 unit, so its output depends on the decoding
//! thread's precision control (62,819 of the 63,575 files differ between 24 and 53 bits). This
//! decoder takes the precision as a parameter and does the library's arithmetic at it (see
//! `fpu`).

mod bits;
mod codebook;
mod decode;
mod file;
mod fpu;
mod mdct;
mod ogg;
mod setup;
mod tables;

pub use file::VorbisFile;
pub use fpu::Precision;
pub use setup::Info;

/// Decodes a whole file as the game does (`ov_read` until it answers 0): the open result and the
/// 16-bit PCM.
pub fn decode_all(data: Vec<u8>, chunk: usize, precision: Precision) -> Result<Vec<u8>, i32> {
    let mut vf = VorbisFile::open(data, precision)?;
    let mut out = Vec::new();
    let mut buf = vec![0u8; chunk];
    loop {
        let n = vf.read(&mut buf);
        if n <= 0 {
            return Ok(out);
        }
        out.extend_from_slice(&buf[..n as usize]);
    }
}

/// For measuring tools: the file's packets in order (data, granule position, packet number,
/// end of stream, beginning of stream), as one logical stream gives them; and this decoder's
/// block outputs (`vorbis_synthesis`: each channel's `n` samples after the inverse MDCT) for
/// the audio packets.
#[doc(hidden)]
pub mod diag {
    use crate::ogg::{next_page, Out, Stream};

    pub type RawPacket = (Vec<u8>, i64, i64, bool, bool);

    pub fn packets(data: &[u8]) -> Vec<RawPacket> {
        let mut out = Vec::new();
        let mut at = 0;
        let mut os: Option<Stream> = None;
        while let Some((start, page)) = next_page(data, at) {
            at = start + page.header.len() + page.body.len();
            let s = os.get_or_insert_with(|| Stream::new(page.serialno()));
            s.pagein(&page);
            while let Out::Packet(p) = s.packetout() {
                out.push((p.data, p.granulepos, p.packetno, p.eos, p.bos));
            }
        }
        out
    }

    /// The blocks with a model of the arithmetic (`fpu::Fpu::model`).
    pub fn blocks(data: &[u8], p24: bool, locals: bool, trig24: bool) -> Vec<Vec<Vec<f32>>> {
        let ps = packets(data);
        let mut info = crate::Info::default();
        for p in &ps[..3] {
            assert_eq!(info.headerin(&p.0, p.4), 0);
        }
        let dsp = crate::decode::Dsp::new(info, crate::fpu::Fpu::model(p24, locals, trig24));
        ps[3..]
            .iter()
            .filter_map(|p| dsp.synthesis(&p.0, p.1, p.2, p.3).ok().map(|b| b.pcm))
            .collect()
    }
}
