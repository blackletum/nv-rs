//! `BSDevices/xcontrols.obj` (Xbox PDB source unit), subsystem `BSDevices`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The game's `Controls` object reading its devices each frame. Its device calls go through the
//! `IDirectInputDevice8A` vtables (the platform's DirectInput 8 layer,
//! `units::platform::dinput`) and `XInputGetState`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `Controls` fields (offsets as `00a23010` uses them): the joystick devices (`+0xc`, one word
/// each), the joystick states (`+0x34`, 0xa0 bytes apart: this frame's 0x50 bytes, the last
/// frame's at `+0x50` after it), the keyboard (`+0x2c`) and mouse (`+0x30`) devices, the joystick
/// count (`+0x18f4`), the keyboard state (`+0x18f8`, the last frame's at `+0x19f8`), the mouse
/// state (`+0x1b24`, the last frame's at `+0x1b38`) and the 8 bytes cleared each poll at
/// `+0x1b58`.
const JOYSTICK_DEVICES: u32 = 0xc;
const JOYSTICK_STATES: u32 = 0x34;
const JOYSTICK_STRIDE: u32 = 0xa0;
const JOYSTICK_STATE_SIZE: u32 = 0x50;
const KEYBOARD: u32 = 0x2c;
const MOUSE: u32 = 0x30;
const JOYSTICK_COUNT: u32 = 0x18f4;
const KEYBOARD_STATE: u32 = 0x18f8;
const KEYBOARD_LAST: u32 = 0x19f8;
const KEYBOARD_STATE_SIZE: u32 = 0x100;
const MOUSE_STATE: u32 = 0x1b24;
const MOUSE_LAST: u32 = 0x1b38;
const MOUSE_STATE_SIZE: u32 = 0x14;
const MOUSE_CLEARED: u32 = 0x1b58;
/// The pad state `XInputGetState` fills (`011f35a8`, 16 bytes), last frame's copy (`011f35b8`)
/// and the "pad connected" byte (`011f35c8`).
const PAD_STATE: u32 = 0x011f_35a8;
const PAD_LAST: u32 = 0x011f_35b8;
const PAD_CONNECTED: u32 = 0x011f_35c8;
/// The `XInputGetState` thunk (`009f996e`), `memcpy` and `memset` (CRT, `cdecl`).
const XINPUT_GET_STATE: u32 = 0x009f_996e;
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;
/// `IDirectInputDevice8A` slots: `Acquire`, `GetDeviceState`, `Poll`.
const ACQUIRE: u32 = 0x1c;
const GET_DEVICE_STATE: u32 = 0x24;
const POLL: u32 = 0x64;
/// The mouse reader's end (`00a232c0`).
const AFTER_MOUSE: u32 = 0x00a2_32c0;

/// One device: acquire it; on `DI_OK` or `S_FALSE` (already acquired) poll it and read its
/// state into `state` (`size` bytes); otherwise clear `state`.
fn read_device(e: &mut Engine, device: u32, state: u32, size: u32) {
    let result = e.vcall(device, ACQUIRE, &args![]).u32();
    if result == 0 || result == 1 {
        e.vcall(device, POLL, &args![]);
        e.vcall(device, GET_DEVICE_STATE, &args![size, state]);
    } else {
        e.call(MEMSET, &args![state, 0u32, size]);
    }
}

// Translated from 00a23010 (disassembled, FalloutNV.exe 1.4.0.525)
/// `Controls::Poll` (engine map; `thiscall`, no stack words, plain `RET`): the pad state is kept
/// as last frame's (`011f35a8` -> `011f35b8`, four words) and read again
/// (`XInputGetState(0, 011f35a8)`); the pad counts as connected (`011f35c8` = 1) when that
/// answers 0 and the byte at `+0` is clear, else 0. Then, for each joystick (the count compared
/// signed), for the keyboard and for the mouse (when there is one): this frame's state becomes
/// last frame's (`memcpy`), the device is acquired and, on `DI_OK` / `S_FALSE`, polled and read
/// (`GetDeviceState`: 0x50, 0x100, 0x14 bytes), else its state is cleared; the mouse also
/// clears the 8 bytes at `+0x1b58` before and ends with `00a232c0`.
pub fn controls_poll(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for i in 0..4 {
        let word = e.mem.u32(PAD_STATE + 4 * i);
        e.mem.set_u32(PAD_LAST + 4 * i, word);
    }
    let result = e.call(XINPUT_GET_STATE, &args![0u32, PAD_STATE]).u32();
    let connected = result == 0 && e.mem.u8(base) == 0;
    e.mem.set_u8(PAD_CONNECTED, u8::from(connected));
    let mut i: i32 = 0;
    while i < e.mem.i32(base + JOYSTICK_COUNT) {
        let at = base + (i as u32).wrapping_mul(JOYSTICK_STRIDE);
        e.call(
            MEMCPY,
            &args![
                at + JOYSTICK_STATES + JOYSTICK_STATE_SIZE,
                at + JOYSTICK_STATES,
                JOYSTICK_STATE_SIZE
            ],
        );
        let device = e.mem.u32(base + JOYSTICK_DEVICES + 4 * i as u32);
        read_device(e, device, at + JOYSTICK_STATES, JOYSTICK_STATE_SIZE);
        i += 1;
    }
    if e.mem.u32(base + KEYBOARD) != 0 {
        e.call(
            MEMCPY,
            &args![
                base + KEYBOARD_LAST,
                base + KEYBOARD_STATE,
                KEYBOARD_STATE_SIZE
            ],
        );
        let keyboard = e.mem.u32(base + KEYBOARD);
        read_device(e, keyboard, base + KEYBOARD_STATE, KEYBOARD_STATE_SIZE);
    }
    if e.mem.u32(base + MOUSE) != 0 {
        e.call(
            MEMCPY,
            &args![base + MOUSE_LAST, base + MOUSE_STATE, MOUSE_STATE_SIZE],
        );
        e.call(MEMSET, &args![base + MOUSE_CLEARED, 0u32, 8u32]);
        let mouse = e.mem.u32(base + MOUSE);
        read_device(e, mouse, base + MOUSE_STATE, MOUSE_STATE_SIZE);
        e.call(AFTER_MOUSE, &args![this]);
    }
}

// Translated from 00a232c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`): does nothing (it keeps
/// `this` in a local and returns).
pub fn fn_00a232c0(_e: &mut Engine, _this: Ptr) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_3010, controls_poll(Ptr)),
        entry!(0x00a2_32c0, fn_00a232c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A device double at `object` whose `Acquire` answers `acquire`; its slots record calls.
    fn device(e: &mut Engine, object: u32, table: u32, acquire: u32, log: &Log) {
        let mut slots = vec![0u32; 26];
        for (slot, code) in [
            (7usize, 0x00ab_0000u32),
            (9, 0x00ab_0004),
            (25, 0x00ab_0008),
        ] {
            slots[slot] = code + (table & 0xff00);
        }
        e.put_vtable(table, &slots);
        e.mem.set_u32(object, table);
        for (k, code) in slots.iter().enumerate().filter(|(_, c)| **c != 0) {
            let log = log.clone();
            let code = *code;
            e.register_double(code, move |_, a| {
                log.borrow_mut().push((code, a.to_vec()));
                crate::abi::RetVal::into_ret(if k == 7 { acquire } else { 0u32 })
            });
        }
    }

    #[test]
    fn poll_reads_each_device_and_keeps_last_frames_state() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(0x0010_0000, 0x4000);
        let log: Log = std::rc::Rc::default();
        let this = 0x0010_0000u32;
        // A keyboard that acquires, a mouse that doesn't (cleared), no joystick.
        device(&mut e, 0x0010_3000, 0x0300_0100, 0, &log);
        device(&mut e, 0x0010_3010, 0x0300_0200, 0x8007_000c, &log);
        e.mem.set_u32(this + KEYBOARD, 0x0010_3000);
        e.mem.set_u32(this + MOUSE, 0x0010_3010);
        e.mem.set_u32(PAD_STATE, 0x55);
        for (f, ret) in [
            (XINPUT_GET_STATE, 0x48fu32),
            (MEMCPY, 0),
            (MEMSET, 0),
            (AFTER_MOUSE, 0),
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                crate::abi::RetVal::into_ret(ret)
            });
        }
        e.call(0x00a2_3010, &args![this]);
        let calls: Vec<(u32, Vec<u32>)> = log.borrow().clone();
        assert_eq!(e.mem.u32(PAD_LAST), 0x55);
        assert_eq!(e.mem.u8(PAD_CONNECTED), 0);
        assert_eq!(
            calls,
            vec![
                (XINPUT_GET_STATE, vec![0, PAD_STATE]),
                (
                    MEMCPY,
                    vec![this + KEYBOARD_LAST, this + KEYBOARD_STATE, 0x100]
                ),
                (0x00ab_0100, vec![0x0010_3000]),
                (0x00ab_0108, vec![0x0010_3000]),
                (0x00ab_0104, vec![0x0010_3000, 0x100, this + KEYBOARD_STATE]),
                (MEMCPY, vec![this + MOUSE_LAST, this + MOUSE_STATE, 0x14]),
                (MEMSET, vec![this + MOUSE_CLEARED, 0, 8]),
                (0x00ab_0200, vec![0x0010_3010]),
                (MEMSET, vec![this + MOUSE_STATE, 0, 0x14]),
                (AFTER_MOUSE, vec![this]),
            ]
        );
    }
}
