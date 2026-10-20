//! The platform's DirectSound 8 layer: `DirectSoundCreate8` (DSOUND import by ordinal 11, slot
//! `00fdf034`, behind the thunk `00ee81e0`), an `IDirectSound8`, its primary buffer and the
//! primary buffer's `IDirectSound3DListener`, with the documented DirectSound 8 semantics.
//! Nothing is heard: the device mixes nothing (the movement build needs no sound output); every
//! call the game makes is a real call on the object's state.
//!
//! As in the DirectInput layer (`super::dinput`), the objects live in engine memory with a vtable
//! pointer at `+0`, each provided slot a platform function (code addresses from [`CODE_BASE`]);
//! a slot not provided is left unregistered, so a call to it stops the run as an open function
//! and is recorded with its address (`IDirectSound8` slots from [`SOUND_CODE`], buffers from
//! [`BUFFER_CODE`], the listener from [`LISTENER_CODE`]; slot = (address - base) / 4, named in
//! [`SOUND_SLOTS`], [`BUFFER_SLOTS`], [`LISTENER_SLOTS`]).
//!
//! What the game calls: `BSXenonAudio::Init` (`00aea230`): `CoInitialize`,
//! `DirectSoundCreate8(NULL)`, `SetCooperativeLevel(window, DSSCL_PRIORITY)`, `GetCaps`,
//! `CreateSoundBuffer` (the primary buffer, `DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D |
//! DSBCAPS_CTRLVOLUME`) and the primary buffer's `QueryInterface(IID_IDirectSound3DListener)`.

use crate::{Engine, Ptr};

/// Where the platform's DirectSound functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e00_0400;
pub const SOUND_CODE: u32 = CODE_BASE;
pub const BUFFER_CODE: u32 = CODE_BASE + 0x100;
pub const LISTENER_CODE: u32 = CODE_BASE + 0x200;

/// `IDirectSound8`'s slots (dsound.h order).
pub const SOUND_SLOTS: [&str; 12] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "CreateSoundBuffer",
    "GetCaps",
    "DuplicateSoundBuffer",
    "SetCooperativeLevel",
    "Compact",
    "GetSpeakerConfig",
    "SetSpeakerConfig",
    "Initialize",
    "VerifyCertification",
];
/// `IDirectSoundBuffer8`'s slots (the primary buffer is an `IDirectSoundBuffer`, the first 21).
pub const BUFFER_SLOTS: [&str; 24] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetCaps",
    "GetCurrentPosition",
    "GetFormat",
    "GetVolume",
    "GetPan",
    "GetFrequency",
    "GetStatus",
    "Initialize",
    "Lock",
    "Play",
    "SetCurrentPosition",
    "SetFormat",
    "SetVolume",
    "SetPan",
    "SetFrequency",
    "Stop",
    "Unlock",
    "Restore",
    "SetFX",
    "AcquireResources",
    "GetObjectInPath",
];
/// `IDirectSound3DListener`'s slots.
pub const LISTENER_SLOTS: [&str; 18] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetAllParameters",
    "GetDistanceFactor",
    "GetDopplerFactor",
    "GetOrientation",
    "GetPosition",
    "GetRolloffFactor",
    "GetVelocity",
    "SetAllParameters",
    "SetDistanceFactor",
    "SetDopplerFactor",
    "SetOrientation",
    "SetPosition",
    "SetRolloffFactor",
    "SetVelocity",
    "CommitDeferredSettings",
];

/// Result codes (dsound.h / winerror.h).
pub const DS_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const DSERR_INVALIDPARAM: u32 = 0x8007_0057;
pub const DSERR_UNSUPPORTED: u32 = 0x8000_4001;
pub const DSERR_ALREADYINITIALIZED: u32 = 0x8878_0082;

/// `IID_IDirectSound3DListener` {279AFA84-4981-11CE-A521-0020AF0BE560}.
const IID_LISTENER: [u8; 16] = [
    0x84, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
/// `DSSCL_*` cooperative levels.
const DSSCL_NORMAL: u32 = 1;
const DSSCL_WRITEPRIMARY: u32 = 4;
/// `DSBCAPS_*` flags this layer understands.
const DSBCAPS_PRIMARYBUFFER: u32 = 0x1;
const DSBCAPS_CTRL3D: u32 = 0x10;
const DSBCAPS_CTRLVOLUME: u32 = 0x80;
/// `DSCAPS` (0x60 bytes) and `DSBUFFERDESC` (0x24 bytes, DirectX 7 and later) sizes.
const DSCAPS_SIZE: u32 = 0x60;
const BUFFER_DESC_SIZE: u32 = 0x24;
/// The device: software mixing only, as DirectSound on Windows Vista and later reports: primary
/// mono, stereo, 8 and 16 bits, continuous rates, secondary mono, stereo, 8 and 16 bits
/// (`DSCAPS_*` 0xF1F), secondary rates 100 to 200000 Hz, one primary buffer, no hardware buffers
/// or memory.
const CAPS_FLAGS: u32 = 0x0f1f;
const MIN_SECONDARY_RATE: u32 = 100;
const MAX_SECONDARY_RATE: u32 = 200_000;
/// The listener's defaults (`DS3D_DEFAULT*`): distance factor 1, rolloff 1, Doppler 1, at the
/// origin, still, looking along +z with +y up.
const LISTENER_DEFAULT_FACTOR: f32 = 1.0;

/// `DS3DLISTENER` (0x40 bytes): size, position, velocity, front, top, distance factor, rolloff,
/// Doppler.
#[derive(Debug, Clone, Copy)]
struct Listener {
    position: [f32; 3],
    velocity: [f32; 3],
    front: [f32; 3],
    top: [f32; 3],
    distance_factor: f32,
    rolloff: f32,
    doppler: f32,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            position: [0.0; 3],
            velocity: [0.0; 3],
            front: [0.0, 0.0, 1.0],
            top: [0.0, 1.0, 0.0],
            distance_factor: LISTENER_DEFAULT_FACTOR,
            rolloff: LISTENER_DEFAULT_FACTOR,
            doppler: LISTENER_DEFAULT_FACTOR,
        }
    }
}

#[derive(Debug, Clone)]
struct Buffer {
    references: u32,
    flags: u32,
    /// The listener this primary buffer gave out (0: none yet).
    listener: u32,
}

/// The DirectSound objects (`Kernel::sound`).
#[derive(Debug, Default)]
pub struct Sound {
    sound_vtable: u32,
    buffer_vtable: u32,
    listener_vtable: u32,
    devices: std::collections::BTreeMap<u32, (u32, u32)>,
    buffers: std::collections::BTreeMap<u32, Buffer>,
    listeners: std::collections::BTreeMap<u32, (u32, Listener)>,
    /// `CoInitialize` calls on the main thread (the first answers `S_OK`, later ones `S_FALSE`).
    com_initialized: u32,
}

fn register_code(e: &mut Engine) {
    let sound: [(u32, crate::AbiFn); 6] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, sound_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, sound_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, create_sound_buffer(Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (4, crate::entry!(0, get_caps(Ptr, Ptr) -> u32).1),
        (
            6,
            crate::entry!(0, set_cooperative_level(Ptr, u32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in sound {
        e.register(SOUND_CODE + 4 * slot, f);
    }
    let buffer: [(u32, crate::AbiFn); 3] = [
        (
            0,
            crate::entry!(0, buffer_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, buffer_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer_release(Ptr) -> u32).1),
    ];
    for (slot, f) in buffer {
        e.register(BUFFER_CODE + 4 * slot, f);
    }
    let listener: [(u32, crate::AbiFn); 13] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, listener_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, listener_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, listener_get_all_parameters(Ptr, Ptr) -> u32).1,
        ),
        (7, crate::entry!(0, listener_get_position(Ptr, Ptr) -> u32).1),
        (
            10,
            crate::entry!(0, listener_set_all_parameters(Ptr, Ptr, u32) -> u32).1,
        ),
        (
            11,
            crate::entry!(0, listener_set_distance_factor(Ptr, f32, u32) -> u32).1,
        ),
        (
            12,
            crate::entry!(0, listener_set_doppler_factor(Ptr, f32, u32) -> u32).1,
        ),
        (
            13,
            crate::entry!(0, listener_set_orientation(Ptr, f32, f32, f32, f32, f32, f32, u32) -> u32).1,
        ),
        (
            14,
            crate::entry!(0, listener_set_position(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
        (
            15,
            crate::entry!(0, listener_set_rolloff_factor(Ptr, f32, u32) -> u32).1,
        ),
        (
            16,
            crate::entry!(0, listener_set_velocity(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
        (17, crate::entry!(0, listener_commit(Ptr) -> u32).1),
    ];
    for (slot, f) in listener {
        e.register(LISTENER_CODE + 4 * slot, f);
    }
}

fn vtable(e: &mut Engine, code: u32, slots: u32) -> u32 {
    let table = e.mem.alloc(4 * slots);
    for i in 0..slots {
        e.mem.set_u32(table + 4 * i, code + 4 * i);
    }
    table
}

fn object(e: &mut Engine, vtable: u32) -> u32 {
    let object = e.mem.alloc(8);
    e.mem.set_u32(object, vtable);
    object
}

// Platform: replaces OLE32.DLL CoInitialize (import)
/// `CoInitialize(NULL)`: COM on the calling thread; `S_OK` the first time, `S_FALSE` after.
pub fn co_initialize(e: &mut Engine, reserved: u32) -> u32 {
    if reserved != 0 {
        return DSERR_INVALIDPARAM;
    }
    e.kernel.sound.com_initialized += 1;
    if e.kernel.sound.com_initialized == 1 {
        DS_OK
    } else {
        S_FALSE
    }
}

// Platform: replaces DSOUND.DLL DirectSoundCreate8 (import by ordinal 11)
/// `DirectSoundCreate8(device, out, outer)`: the default device (`device` NULL) as a new
/// `IDirectSound8` (reference count 1, no cooperative level yet) through `out`, `DS_OK`. A
/// chosen device or an outer object is not provided.
pub fn direct_sound_create8(e: &mut Engine, device: u32, out: Ptr, outer: u32) -> u32 {
    if device != 0 || outer != 0 {
        panic!("DirectSoundCreate8 with a device GUID or an outer object is not provided");
    }
    if e.kernel.sound.sound_vtable == 0 {
        register_code(e);
        e.kernel.sound.sound_vtable = vtable(e, SOUND_CODE, SOUND_SLOTS.len() as u32);
        e.kernel.sound.buffer_vtable = vtable(e, BUFFER_CODE, BUFFER_SLOTS.len() as u32);
        e.kernel.sound.listener_vtable = vtable(e, LISTENER_CODE, LISTENER_SLOTS.len() as u32);
    }
    let table = e.kernel.sound.sound_vtable;
    let device = object(e, table);
    e.kernel.sound.devices.insert(device, (1, 0));
    e.mem.set_u32(out.addr(), device);
    DS_OK
}

fn no_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn sound_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = e
        .kernel
        .sound
        .devices
        .get_mut(&this.addr())
        .expect("an IDirectSound8 of this layer");
    d.0 += 1;
    d.0
}

fn sound_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = e
        .kernel
        .sound
        .devices
        .get_mut(&this.addr())
        .expect("an IDirectSound8 of this layer");
    d.0 -= 1;
    let left = d.0;
    if left == 0 {
        e.kernel.sound.devices.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `SetCooperativeLevel(window, level)`: `DSSCL_NORMAL` (1) to `DSSCL_WRITEPRIMARY` (4); `DS_OK`.
fn set_cooperative_level(e: &mut Engine, this: Ptr, _window: u32, level: u32) -> u32 {
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&level) {
        return DSERR_INVALIDPARAM;
    }
    let d = e
        .kernel
        .sound
        .devices
        .get_mut(&this.addr())
        .expect("an IDirectSound8 of this layer");
    d.1 = level;
    DS_OK
}

/// `GetCaps(caps)`: `caps->dwSize` must be 0x60; the device's capabilities (`CAPS_FLAGS` and
/// the rest of `DSCAPS` as the constants above say, everything else 0).
fn get_caps(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    let _ = e
        .kernel
        .sound
        .devices
        .get(&this.addr())
        .expect("an IDirectSound8 of this layer");
    let at = caps.addr();
    if e.mem.u32(at) != DSCAPS_SIZE {
        return DSERR_INVALIDPARAM;
    }
    for i in 1..(DSCAPS_SIZE / 4) {
        e.mem.set_u32(at + 4 * i, 0);
    }
    e.mem.set_u32(at + 4, CAPS_FLAGS);
    e.mem.set_u32(at + 8, MIN_SECONDARY_RATE);
    e.mem.set_u32(at + 0xc, MAX_SECONDARY_RATE);
    e.mem.set_u32(at + 0x10, 1);
    DS_OK
}

/// `CreateSoundBuffer(desc, out, outer)`: the primary buffer (`DSBCAPS_PRIMARYBUFFER`, with
/// `DSBCAPS_CTRL3D` and `DSBCAPS_CTRLVOLUME` at most; size and format 0, as DirectSound wants
/// them for it) as a new buffer through `out`, `DS_OK`. Secondary buffers are not provided yet.
fn create_sound_buffer(e: &mut Engine, this: Ptr, desc: Ptr, out: Ptr, outer: u32) -> u32 {
    let _ = e
        .kernel
        .sound
        .devices
        .get(&this.addr())
        .expect("an IDirectSound8 of this layer");
    if outer != 0 {
        return DSERR_INVALIDPARAM;
    }
    let d = desc.addr();
    if e.mem.u32(d) != BUFFER_DESC_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let flags = e.mem.u32(d + 4);
    if flags & DSBCAPS_PRIMARYBUFFER == 0 {
        panic!("IDirectSound8::CreateSoundBuffer for a secondary buffer is not provided yet");
    }
    if flags & !(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D | DSBCAPS_CTRLVOLUME) != 0 {
        panic!("IDirectSound8::CreateSoundBuffer with flags {flags:#x} is not provided");
    }
    if e.mem.u32(d + 8) != 0 || e.mem.u32(d + 0x10) != 0 {
        return DSERR_INVALIDPARAM;
    }
    let table = e.kernel.sound.buffer_vtable;
    let buffer = object(e, table);
    e.kernel.sound.buffers.insert(
        buffer,
        Buffer {
            references: 1,
            flags,
            listener: 0,
        },
    );
    e.mem.set_u32(out.addr(), buffer);
    DS_OK
}

/// `QueryInterface(iid, out)` on a buffer: the primary buffer made with `DSBCAPS_CTRL3D` gives
/// its `IDirectSound3DListener` (one per buffer, a new reference each time); any other interface
/// is `E_NOINTERFACE`.
fn buffer_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    let id = e.mem.bytes(iid.addr(), 16);
    let b = e
        .kernel
        .sound
        .buffers
        .get(&this.addr())
        .expect("a buffer of this layer")
        .clone();
    let is_3d_primary = b.flags & DSBCAPS_PRIMARYBUFFER != 0 && b.flags & DSBCAPS_CTRL3D != 0;
    if id != IID_LISTENER || !is_3d_primary {
        return no_interface(e, this, iid, out);
    }
    let listener = if b.listener != 0 {
        let l = e
            .kernel
            .sound
            .listeners
            .get_mut(&b.listener)
            .expect("its listener");
        l.0 += 1;
        b.listener
    } else {
        let table = e.kernel.sound.listener_vtable;
        let listener = object(e, table);
        e.kernel
            .sound
            .listeners
            .insert(listener, (1, Listener::default()));
        e.kernel
            .sound
            .buffers
            .get_mut(&this.addr())
            .expect("the buffer")
            .listener = listener;
        listener
    };
    e.mem.set_u32(out.addr(), listener);
    DS_OK
}

fn buffer_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let b = e
        .kernel
        .sound
        .buffers
        .get_mut(&this.addr())
        .expect("a buffer of this layer");
    b.references += 1;
    b.references
}

fn buffer_release(e: &mut Engine, this: Ptr) -> u32 {
    let b = e
        .kernel
        .sound
        .buffers
        .get_mut(&this.addr())
        .expect("a buffer of this layer");
    b.references -= 1;
    let left = b.references;
    if left == 0 {
        e.kernel.sound.buffers.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

fn listener(e: &mut Engine, this: Ptr) -> &mut (u32, Listener) {
    e.kernel
        .sound
        .listeners
        .get_mut(&this.addr())
        .expect("an IDirectSound3DListener of this layer")
}

fn listener_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 += 1;
    l.0
}

fn listener_release(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 -= 1;
    let left = l.0;
    if left == 0 {
        e.kernel.sound.listeners.remove(&this.addr());
        for b in e.kernel.sound.buffers.values_mut() {
            if b.listener == this.addr() {
                b.listener = 0;
            }
        }
        e.mem.free(this.addr());
    }
    left
}

fn put3(e: &mut Engine, at: u32, v: [f32; 3]) {
    for (i, f) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, f);
    }
}

fn get3(e: &Engine, at: u32) -> [f32; 3] {
    [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
}

/// `DS3DLISTENER` is 0x40 bytes.
const LISTENER_SIZE: u32 = 0x40;

/// `GetAllParameters(params)`: `params->dwSize` must be 0x40; the listener's state.
fn listener_get_all_parameters(e: &mut Engine, this: Ptr, params: Ptr) -> u32 {
    let p = params.addr();
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let l = listener(e, this).1;
    put3(e, p + 4, l.position);
    put3(e, p + 0x10, l.velocity);
    put3(e, p + 0x1c, l.front);
    put3(e, p + 0x28, l.top);
    e.mem.set_f32(p + 0x34, l.distance_factor);
    e.mem.set_f32(p + 0x38, l.rolloff);
    e.mem.set_f32(p + 0x3c, l.doppler);
    DS_OK
}

/// `GetPosition(out)`.
fn listener_get_position(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let position = listener(e, this).1.position;
    put3(e, out.addr(), position);
    DS_OK
}

/// `SetAllParameters(params, apply)`: `params->dwSize` must be 0x40. Every change takes effect
/// at once here (`DS3D_IMMEDIATE` or `DS3D_DEFERRED`: nothing is mixed, so a deferred setting is
/// the same state as an applied one).
fn listener_set_all_parameters(e: &mut Engine, this: Ptr, params: Ptr, _apply: u32) -> u32 {
    let p = params.addr();
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let new = Listener {
        position: get3(e, p + 4),
        velocity: get3(e, p + 0x10),
        front: get3(e, p + 0x1c),
        top: get3(e, p + 0x28),
        distance_factor: e.mem.f32(p + 0x34),
        rolloff: e.mem.f32(p + 0x38),
        doppler: e.mem.f32(p + 0x3c),
    };
    listener(e, this).1 = new;
    DS_OK
}

fn listener_set_distance_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.distance_factor = value;
    DS_OK
}

fn listener_set_doppler_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.doppler = value;
    DS_OK
}

fn listener_set_rolloff_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.rolloff = value;
    DS_OK
}

#[allow(clippy::too_many_arguments)] // The interface's receiver, six floats and the apply word.
fn listener_set_orientation(
    e: &mut Engine,
    this: Ptr,
    fx: f32,
    fy: f32,
    fz: f32,
    tx: f32,
    ty: f32,
    tz: f32,
    _apply: u32,
) -> u32 {
    let l = &mut listener(e, this).1;
    l.front = [fx, fy, fz];
    l.top = [tx, ty, tz];
    DS_OK
}

fn listener_set_position(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, _apply: u32) -> u32 {
    listener(e, this).1.position = [x, y, z];
    DS_OK
}

fn listener_set_velocity(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, _apply: u32) -> u32 {
    listener(e, this).1.velocity = [x, y, z];
    DS_OK
}

/// `CommitDeferredSettings()`: settings already apply (see `SetAllParameters`); `DS_OK`.
fn listener_commit(e: &mut Engine, this: Ptr) -> u32 {
    let _ = listener(e, this);
    DS_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `BSXenonAudio::Init`'s calls in its order.
    #[test]
    fn the_audio_start_up_gets_a_device_a_primary_buffer_and_a_listener() {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        assert_eq!(co_initialize(&mut e, 0), DS_OK);
        assert_eq!(co_initialize(&mut e, 0), S_FALSE);
        assert_eq!(direct_sound_create8(&mut e, 0, Ptr::new(0x1000), 0), DS_OK);
        let ds = e.mem.u32(0x1000);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 2]).u32(), DS_OK);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 9]).u32(), DSERR_INVALIDPARAM);
        e.mem.set_u32(0x1100, DSCAPS_SIZE);
        assert_eq!(e.vcall(ds, 0x10, &[0x1100]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1104), CAPS_FLAGS);
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        assert_eq!(e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]).u32(), DS_OK);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x1008]).u32(), DS_OK);
        let listener = e.mem.u32(0x1008);
        assert_ne!(listener, 0);
        // The listener keeps what it is given.
        assert_eq!(
            e.vcall(
                listener,
                0x38,
                &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 1]
            )
            .u32(),
            DS_OK
        );
        assert_eq!(e.vcall(listener, 0x1c, &[0x1400]).u32(), DS_OK);
        assert_eq!([0, 4, 8].map(|o| e.mem.f32(0x1400 + o)), [1.0, 2.0, 3.0]);
        e.mem.set_u32(0x1500, LISTENER_SIZE);
        assert_eq!(e.vcall(listener, 0xc, &[0x1500]).u32(), DS_OK);
        assert_eq!(e.mem.f32(0x1500 + 0x34), 1.0, "distance factor");
        assert_eq!(e.mem.f32(0x1500 + 0x28 + 4), 1.0, "top is +y");
        // Another interface isn't offered.
        e.mem.write(0x1300, &[9u8; 16]);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x100c]).u32(), E_NOINTERFACE);
        // References.
        assert_eq!(e.vcall(listener, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(primary, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(ds, 8, &[]).u32(), 0);
    }

    #[test]
    fn a_slot_not_provided_stops() {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        direct_sound_create8(&mut e, 0, Ptr::new(0x1000), 0);
        let ds = e.mem.u32(0x1000);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(ds, 0x1c, &[]); // Compact
        }));
        assert!(stopped.is_err());
    }
}
