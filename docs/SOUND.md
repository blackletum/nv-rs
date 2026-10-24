# Sound: the platform's audio layers, measured

How the platform answers the game's sound imports, and how far each answer has
been checked against the player's own libraries. Measurements are private
recordings under `build43-scratch/measure` (never committed), made by the tools
in `scripts/`.

| Layer | Module | What it replaces | Checked against | Result |
| --- | --- | --- | --- | --- |
| Music | `platform/dshow.rs`, `crates/mp3` | DirectShow's filter graph and MP3 decoder | `scripts/dshow-measure` | Graph answers as measured. All 199 tracks are the same length, and 99.9748% of samples are identical |
| Wave files | `platform/mmio.rs` | `mmio*` | `scripts/mmio-measure` | Edge cases as measured |
| DirectSound | `platform/dsound.rs` | DirectSound 8 objects, cursors | `scripts/dsound-capture` | Answers and cursors as measured |
| Mixer | `platform/mixer.rs`, `platform/sound_out.rs` | DirectSound's mixer, the device | `scripts/dsmix-measure` (WASAPI loopback) | 2D path bit-exact; resampler, 3D, distortion not provided |
| Voices and sounds | `platform/vorbisfile.rs`, `crates/vorbis` | `libvorbisfile.dll`/`libvorbis.dll` (1.2.0) | `scripts/vorbis-record` | Bit-exact at 24-bit x87; 53-bit not provided |

## Mixer (DirectSound 8 to the device)

These were measured on a device without audio enhancements (Virtual Desktop Audio, 48 kHz,
32-bit float stereo). The default device's enhancements smear the signal by
-17 dB, so it cannot be used for these measurements.

**Exact:**
- Samples become `f32` as x/32768.
- The volume and pan gain law is exact for all 429 volumes and 25 pans (see `mixer.rs`).
- A mono buffer goes to both channels.
- Voices are added in `f32`, the one started last first. This holds for 47,996 of
  48,000 samples. The other 4 are the tail of a voice started 2 frames into a device
  period.

**Not exact, so the platform stops with "not provided":**
- **Resampling.** It is a long polyphase filter, not linear; least-squares fits only
  reach -34 dB.
- **3D.** Gains come in 0.1 dB steps, but the rule from position to gain is not derived
  yet.
- **Distortion.** Not analysed.

## Vorbis

`scripts/vorbis-record` drives the player's `libvorbisfile.dll` the way the game
does (`00af1e00`, `00af2100`). It reads every `.ogg` in the archives (63,575 files)
and records the decoded bytes. At 24-bit x87 precision, `crates/vorbis` gives the
same bytes and the same `ov_pcm_total` for all of them. It follows the public
libvorbis 1.2.0 source's order of operations (BSD, see `LEGAL.md`).

The library computes on the x87 unit, so its output depends on the decoding
thread's precision control: 62,819 of the files differ between 24 and 53 bits.
At 24 bits every operation rounds to 24 bits, which is the same as `f32` in the
source's order.

**Which thread decodes is not measured yet.** The thread is the one that runs
`AudioLoadTask::Run` (`00ae7050`), whose path to the decoder is
`00aeb270`, `00aebff0`, `00af2100`. Load tasks go to the IO manager (`01202d98`),
so they probably run on a task-manager thread. Such a thread starts at
`0x027F` (53-bit) unless something on it changes the control word.

### What was ruled out for 53 bits

The recorder's `--blocks` mode calls `libvorbis.dll`'s public API
(`vorbis_synthesis`) and records every block after the inverse MDCT.
`VORBIS_PACKETS=<file>` does the same for crafted streams. A crafted stream has
a floor of 1.0 and exact spectra (impulses, random values), so its blocks are
the DLL's inverse MDCT of a known input.

**At 24 bits:** the crafted blocks match exactly, as the game files do.

**At 53 bits:** none of these models of the transform comes close:
- operations at 53 bits with each C assignment to `float` memory rounded;
- the same with `float` locals also rounded, or kept unrounded (as in registers);
- the trig tables' angles computed at 24 or at 53 bits;
- every combination of 24-bit, 53-bit-with-rounded-locals and 53-bit-with-register-locals
  for the five stages separately (rotation in, first and generic butterflies, 32-point
  butterflies, bit reversal, rotation out): 486 combinations.

The best of these still differs from the DLL in 12,980 of 16,384 values (256-sample
blocks). That is about as far as the DLL's own 24-bit output is from its 53-bit output.

The rest of the decode does not depend on precision: codebook values, floor and
residue are single operations on `f32`.

So the difference is likely the compiler keeping values in x87 registers
across statements, including values it has already stored to memory. Where it
does that is in the DLL's code, which the project does not read.

Until the decoding thread is measured, or a measurement-only route to the model is
found, `platform/vorbisfile.rs` stops with "not provided" on a thread whose
precision control is not 24 bits.
