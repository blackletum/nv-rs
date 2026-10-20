//! `NiXenonRenderer/nixenonrenderer.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_9460, ni_xenon_renderer_create_direct3d9(u8) -> i32),
        (0x00e7_2e60, fn_00e72e60_entry as AbiFn),
    ]
}

// ---- Direct3D creation and the renderer's initialisation (batch b000t) ----

/// The Direct3D globals: the `IDirect3D9` object (`0126f0d8`), the byte set when it is an
/// `IDirect3D9Ex` (`0126f0d0`), and the two creation functions `00e69410` looks up
/// (`Direct3DCreate9` at `0126f0dc`, `Direct3DCreate9Ex` at `0126f0e0`; both `stdcall`).
const DIRECT3D: u32 = 0x0126_f0d8;
const DIRECT3D_IS_EX: u32 = 0x0126_f0d0;
const DIRECT3D_CREATE9: u32 = 0x0126_f0dc;
const DIRECT3D_CREATE9_EX: u32 = 0x0126_f0e0;
/// `00e69410` (unnamed, `nixenonrenderer.cpp`; no arguments): a nonzero byte when the
/// Direct3D library is usable.
const LOAD_DIRECT3D: u32 = 0x00e6_9410;
/// `00a29680` (unnamed; `cdecl`, a message) and the messages `CreateDirect3D9` passes it.
const REPORT_00A29680: u32 = 0x00a2_9680;
const CREATE_EX_FAILED_010ED598: u32 = 0x010e_d598;
const CREATE_EX_SUCCEEDED_010ED570: u32 = 0x010e_d570;
const CREATE_FAILED_010ED550: u32 = 0x010e_d550;
const CREATE_SUCCEEDED_010ED52C: u32 = 0x010e_d52c;
/// The SDK version both creation functions are given.
const D3D_SDK_VERSION: u32 = 0x20;

// Translated from 00e69460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::CreateDirect3D9` (engine map), `cdecl`, one word (its low byte: try the
/// `Ex` interface). Answers 0, or -1 on failure; with the object already made (`0126f0d8`) it
/// answers 0 at once. When the library is not usable (`00e69410`) it reports "Create D3D9
/// instance...FAILED" and answers -1. With `Direct3DCreate9Ex` found and the byte set it calls
/// it (version 0x20, an out word): a null object reports "Create D3D9Ex
/// instance...FAILED" and makes the answer -1 (the plain creation is still tried), otherwise
/// the `Ex` byte and the object are set and "...SUCCESSFUL" reported. Without an object then,
/// `Direct3DCreate9(0x20)` makes it: null reports "Create D3D9 instance...FAILED" and answers
/// -1; otherwise it reports "...SUCCESSFUL" and answers the earlier result.
pub fn ni_xenon_renderer_create_direct3d9(e: &mut Engine, try_ex: u8) -> i32 {
    let mut result = 0i32;
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    if e.call(LOAD_DIRECT3D, &args![]).u8() == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    let create_ex = e.mem.u32(DIRECT3D_CREATE9_EX);
    if create_ex != 0 && try_ex != 0 {
        // The out word is the slot of the pushed ECX.
        let object = e.with_stack(4, |e, out| {
            e.call(create_ex, &args![D3D_SDK_VERSION, out]);
            e.mem.u32(out.addr())
        });
        if object == 0 {
            e.call(REPORT_00A29680, &args![CREATE_EX_FAILED_010ED598]);
            result = -1;
        } else {
            e.mem.set_u8(DIRECT3D_IS_EX, 1);
            e.mem.set_u32(DIRECT3D, object);
            e.call(REPORT_00A29680, &args![CREATE_EX_SUCCEEDED_010ED570]);
        }
    }
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    let create = e.mem.u32(DIRECT3D_CREATE9);
    let object = e.call(create, &args![D3D_SDK_VERSION]).u32();
    e.mem.set_u32(DIRECT3D, object);
    if object == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    e.call(REPORT_00A29680, &args![CREATE_SUCCEEDED_010ED52C]);
    result
}

/// `IDirect3D9::CreateDevice` (vtable slot 0x40) and `IDirect3D9Ex::CreateDeviceEx` (0x50),
/// named by the interfaces' slot order; the arguments match.
const CREATE_DEVICE: u32 = 0x40;
const CREATE_DEVICE_EX: u32 = 0x50;
/// The adapter list (`0126f0e8`: element pointer +8, 16-bit count +0xe).
const ADAPTER_LIST: u32 = 0x0126_f0e8;
/// Globals the initialisation clears first (a `float` and a byte).
const ZEROED_VALUE_0126EFD4: u32 = 0x0126_efd4;
const ZEROED_FLAG_0126EFD1: u32 = 0x0126_efd1;
/// The error text (`011f4648`, 0x100 bytes, filled by `_strncpy_s` `00ec8d5f` or by
/// `00e69ad0`, `cdecl` with a message).
const ERROR_MESSAGE_011F4648: u32 = 0x011f_4648;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const SET_ERROR_00E69AD0: u32 = 0x00e6_9ad0;
/// The callees of the initialisation (`nixenonrenderer.cpp` unless noted; unnamed in the
/// engine map unless noted).
const RESET_00E69A60: u32 = 0x00e6_9a60;
const DEVICE_SETTINGS_00E69510: u32 = 0x00e6_9510;
const PRESENT_PARAMETERS_00E6BF50: u32 = 0x00e6_bf50;
const AFTER_DEVICE_00E6C240: u32 = 0x00e6_c240;
const BACK_BUFFER_00E7D6B0: u32 = 0x00e7_d6b0;
const DEPTH_STENCIL_00E7D850: u32 = 0x00e7_d850;
const RENDER_TARGET_GROUP_00EE8710: u32 = 0x00ee_8710;
const NI_POINTER_CONSTRUCT_00B6AED0: u32 = 0x00b6_aed0;
const WINDOW_TARGET_00E72CF0: u32 = 0x00e7_2cf0;
const FORMAT_00E69D00: u32 = 0x00e6_9d00;
const FORMAT_00E69C90: u32 = 0x00e6_9c90;
const TEXTURE_DEFAULTS_00E6EC90: u32 = 0x00e6_ec90;
const SHADER_LIBRARY_00E91590: u32 = 0x00e9_1590;
const INITIALIZE_SHADER_SYSTEM: u32 = 0x00e8_1940;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const CONSTRUCT_00E8D0E0: u32 = 0x00e8_d0e0;
const CONSTRUCT_00E8FB20: u32 = 0x00e8_fb20;
const CONSTRUCT_00E90A80: u32 = 0x00e9_0a80;
const GEOMETRY_GROUP_MANAGER_00E88EB0: u32 = 0x00e8_8eb0;
const SET_FLAG_00E88C20: u32 = 0x00e8_8c20;
const CONSTRUCT_00E8BAE0: u32 = 0x00e8_bae0;
const NI_FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const NI_SHADER_FACTORY_GET_INSTANCE: u32 = 0x00a9_fff0;
const CONSTRUCT_00E909E0: u32 = 0x00e9_09e0;
const SET_SHADER_00E6E6E0: u32 = 0x00e6_e6e0;
/// The `float` constants of the default frustum (`01012054`, `0101e2bc`), the `float` 2^32
/// (`01099378`) and the `double` the height-to-width ratio is compared with (`0101de30`).
const FRUSTUM_SIDE_01012054: u32 = 0x0101_2054;
const FRUSTUM_NEAR_0101E2BC: u32 = 0x0101_e2bc;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const RATIO_LIMIT_0101DE30: u32 = 0x0101_de30;
/// The four words passed to the renderer's virtual slot 0x18c with the frustum and viewport.
const CAMERA_WORDS_SLOT_18C: [u32; 4] = [0x011f_426c, 0x011a_946c, 0x011a_9478, 0x011a_9484];
/// The messages and error texts of the initialisation.
const INVALID_DEVICE_TYPE_010EE20C: u32 = 0x010e_e20c;
const INVALID_DEVICE_ID_010EE1C8: u32 = 0x010e_e1c8;
const INVALID_ADAPTER_010EE1A0: u32 = 0x010e_e1a0;
const INVALID_ADAPTER_REPORT_010EE160: u32 = 0x010e_e160;
const ATTEMPTING_010EE118: u32 = 0x010e_e118;
const DEVICE_NOT_VALID_010EE0E8: u32 = 0x010e_e0e8;
const NO_DEVICE_OF_TYPE_010EE0AC: u32 = 0x010e_e0ac;
const FEWER_BACK_BUFFERS_010EE048: u32 = 0x010e_e048;
const NEXT_LOWER_TYPE_010EDFE0: u32 = 0x010e_dfe0;
const NO_HARDWARE_DEVICE_010EDFAC: u32 = 0x010e_dfac;
const NO_DEVICE_OF_ANY_TYPE_010EDF60: u32 = 0x010e_df60;
const NO_TEXTURE_DEFAULTS_010EDF10: u32 = 0x010e_df10;
const NO_REFERENCE_DEVICE_010EDEDC: u32 = 0x010e_dedc;
const NO_REFERENCE_DEVICE_REPORT_010EDE88: u32 = 0x010e_de88;
const NO_HARDWARE_OF_ANY_TYPE_010EDE30: u32 = 0x010e_de30;

/// The renderer's fields the initialisation uses: the device (+0x288), the device type
/// (+0x5c8) and behaviour flags (+0x5cc) `00e69510` fills, the adapter (+0x5c4), the adapter
/// and device descriptions found (+0x5d8, +0x5dc), and the 14 words kept from +0xa98.
const DEVICE: u32 = 0x288;
const DEVICE_TYPE: u32 = 0x5c8;
const BEHAVIOR_FLAGS: u32 = 0x5cc;
const ADAPTER: u32 = 0x5c4;
const WINDOW: u32 = 0x3bc;
const FOCUS_WINDOW: u32 = 0x3c0;
/// The initialisation's frame (`SUB ESP,0x80` and four pushed registers): the vertex-buffer
/// and depth-stencil out words (+0x10, +0x14), the `CreateDeviceEx` out words (+0x18,
/// +0x1c), the full-screen display mode (+0x24, 0x18 bytes; later the viewport), the frustum
/// (+0x3c) and the present parameters (+0x58, 0x38 bytes).
const INIT_FRAME: u32 = 0x90;
const FRAME_BLOCK_OUT: u32 = 0x10;
const FRAME_DEPTH_OUT: u32 = 0x14;
const FRAME_EX_OUT: u32 = 0x18;
const FRAME_EX_RETRY_OUT: u32 = 0x1c;
const FRAME_MODE: u32 = 0x24;
const FRAME_FRUSTUM: u32 = 0x3c;
const FRAME_PRESENT: u32 = 0x58;

/// The adapter description for the renderer's adapter (+0x5c4): the list's element when the
/// index is below its 16-bit count, else null.
fn adapter_description(e: &Engine, this: u32) -> u32 {
    let list = e.mem.u32(ADAPTER_LIST);
    let index = e.mem.u32(this + ADAPTER);
    if index < u32::from(e.mem.u16(list.wrapping_add(0xe))) {
        let elements = e.mem.u32(list.wrapping_add(8));
        e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

/// The adapter description's device description for the renderer's device type: +0x460 for
/// type 1, else +0x464.
fn device_description(e: &Engine, this: u32, adapter: u32) -> u32 {
    if e.mem.u32(this + DEVICE_TYPE) == 1 {
        e.mem.u32(adapter.wrapping_add(0x460))
    } else {
        e.mem.u32(adapter.wrapping_add(0x464))
    }
}

/// `CreateDevice` or, on an `Ex` object, `CreateDeviceEx` (with the full-screen display mode
/// only when the present parameters are not windowed, +0x20, and the device through `out`);
/// answers the `HRESULT`.
fn create_device(e: &mut Engine, this: u32, frame: u32, out: u32) -> i32 {
    let direct3d = e.mem.u32(DIRECT3D);
    let present = frame + FRAME_PRESENT;
    let adapter = e.mem.u32(this + ADAPTER);
    let device_type = e.mem.u32(this + DEVICE_TYPE);
    let focus = e.mem.u32(this + FOCUS_WINDOW);
    let behavior = e.mem.u32(this + BEHAVIOR_FLAGS);
    if e.mem.u8(DIRECT3D_IS_EX) != 0 {
        let mode = if e.mem.u32(present + 0x20) != 0 {
            0
        } else {
            frame + FRAME_MODE
        };
        let result = e
            .vcall(
                direct3d,
                CREATE_DEVICE_EX,
                &args![adapter, device_type, focus, behavior, present, mode, out],
            )
            .i32();
        let device = e.mem.u32(out);
        e.mem.set_u32(this + DEVICE, device);
        result
    } else {
        e.vcall(
            direct3d,
            CREATE_DEVICE,
            &args![
                adapter,
                device_type,
                focus,
                behavior,
                present,
                this + DEVICE
            ],
        )
        .i32()
    }
}

/// A word made unsigned by `FILD` and adding the `float` 2^32 when negative.
fn unsigned_on_x87(e: &Engine, word: u32) -> f64 {
    let value = f64::from(word as i32);
    if (word as i32) < 0 {
        x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        value
    }
}

/// What ends a pass of the device loop.
enum DeviceAttempt {
    Created,
    NextType,
}

// Translated from 00e72e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`; its messages call it
/// `NiDX9Renderer::Initialize`), `thiscall` with 14 words; answers a byte, 1 on success. The
/// parameter names follow what the code does with them: the width and height go to the
/// full-screen display mode, `flags` bits 0x40 and 0x20 add behaviour flags 2 and 4, the window
/// (+0x3bc) goes to the present parameters' set-up and the window target, the focus window
/// (+0x3c0) and adapter (+0x5c4) to `CreateDevice`, the device kind to `00e69510`, and the
/// last word is the display mode's refresh rate; the others are only kept and passed on.
///
/// Keeps the 14 words at +0xa98, clears the `float` at `0126efd4` and the byte `0126efd1`,
/// runs `00e69a60`, and lets `00e69510` turn the device kind into the device type and
/// behaviour flags (failing: "Invalid 3D device type"). The adapter's description (none:
/// "Invalid Adapter") and its device description for the type (none, or without caps: "Invalid
/// 3D device type") are kept. Then, with the present parameters cleared, for each device kind
/// from the requested one: the device description must still be valid ("Requested device not
/// valid"); `00e6bf50` fills the present parameters; the device is created (`CreateDevice`, or
/// `CreateDeviceEx` with a full-screen display mode of size 0x18: width, height, refresh rate,
/// the back buffer format, scan-line ordering 1), +0xad0 is set when height / width (as
/// unsigned values on the x87) is below 0.75 or unordered; a failure clears +0x6fc and, when
/// the call changed the back buffer count, is retried once ("trying fewer backbuffers"). A
/// failed kind moves to the next one: after the reference kind (4) "Could not create requested
/// reference device"; reaching kind 4 "Could not create hardware device of any type"; a kind
/// `00e69510` refuses "Invalid 3D device type". With a device: none at +0x288 fails ("Could not
/// create hardware device"); `00e6c240` must succeed; +0x5d4 and +0x5d5 get the behaviour
/// flags' inverted bit 6 and bit 7; the back buffer and (with automatic depth-stencil) the
/// depth surface are fetched; the render target group (`00ee8710`) gets them (slots 0xa8,
/// 0xac), the window's target is set (`00e72cf0` with an `NiPointer` to the group), the format
/// of the group's first buffer is kept at +0x5d0; the texture defaults (`00e6ec90`, failing:
/// "Could not initialize texture defaults") and the shader and geometry objects are made
/// (+0x8b8 to +0x8c8, `NiD3DShaderInterface::InitializeShaderSystem`); the default frustum
/// (-1, 1, 1, -1, the near `float` at `0101e2bc`, 1) and viewport (0, 1, 1, 0) go to the
/// renderer's slot 0x18c; then `NiShaderFactory::GetInstance`, a 0x40-byte object (`00e909e0`;
/// its slot 0x4c runs even when the allocation failed) handed to `00e6e6e0`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e72e60(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    flags: u32,
    window: u32,
    focus_window: u32,
    adapter: u32,
    device_kind: u32,
    eighth: u32,
    ninth: u32,
    tenth: u32,
    eleventh: u32,
    twelfth: u32,
    thirteenth: u32,
    refresh_rate: u32,
) -> u8 {
    let this = this.addr();
    let words = [
        width,
        height,
        flags,
        window,
        focus_window,
        adapter,
        device_kind,
        eighth,
        ninth,
        tenth,
        eleventh,
        twelfth,
        thirteenth,
        refresh_rate,
    ];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(this + 0xa98 + 4 * i as u32, *word);
    }
    // FLDZ / FSTP float.
    e.mem.set_u32(ZEROED_VALUE_0126EFD4, x87_store32(0.0));
    e.mem.set_u8(ZEROED_FLAG_0126EFD1, 0);
    e.call(RESET_00E69A60, &args![]);
    e.mem.set_u32(this + ADAPTER, adapter);
    e.mem.set_u32(this + WINDOW, window);
    e.mem.set_u32(this + FOCUS_WINDOW, focus_window);
    let device_type_at = this + DEVICE_TYPE;
    let behavior_at = this + BEHAVIOR_FLAGS;
    let report = |e: &mut Engine, message: u32| {
        e.call(REPORT_00A29680, &args![message]);
    };
    let copy_error = |e: &mut Engine, message: u32| {
        e.call(
            STRNCPY_S,
            &args![ERROR_MESSAGE_011F4648, 0x100u32, message, 0xffu32],
        );
    };
    let invalid_device = |e: &mut Engine| {
        copy_error(e, INVALID_DEVICE_TYPE_010EE20C);
        report(e, INVALID_DEVICE_ID_010EE1C8);
        0u8
    };
    if e.call(
        DEVICE_SETTINGS_00E69510,
        &args![device_kind, device_type_at, behavior_at],
    )
    .u8()
        == 0
    {
        return invalid_device(e);
    }
    if flags as u8 & 0x40 != 0 {
        let value = e.mem.u32(behavior_at) | 2;
        e.mem.set_u32(behavior_at, value);
    }
    if flags as u8 & 0x20 != 0 {
        let value = e.mem.u32(behavior_at) | 4;
        e.mem.set_u32(behavior_at, value);
    }
    let adapter_desc = adapter_description(e, this);
    e.mem.set_u32(this + 0x5d8, adapter_desc);
    if adapter_desc == 0 {
        copy_error(e, INVALID_ADAPTER_010EE1A0);
        report(e, INVALID_ADAPTER_REPORT_010EE160);
        return 0;
    }
    let device_desc = device_description(e, this, adapter_desc);
    let kept = if e.mem.u32(device_desc.wrapping_add(4)) != 0 {
        device_desc
    } else {
        0
    };
    e.mem.set_u32(this + 0x5dc, kept);
    if kept == 0 {
        return invalid_device(e);
    }

    e.with_stack(INIT_FRAME, |e, frame| {
        let frame = frame.addr();
        let present = frame + FRAME_PRESENT;
        e.call(MEMSET_ADDR, &args![present, 0u32, 0x38u32]);
        let mut kind = device_kind;
        loop {
            report(e, ATTEMPTING_010EE118);
            let adapter_desc = adapter_description(e, this);
            let device_desc = device_description(e, this, adapter_desc);
            let mut attempt = DeviceAttempt::NextType;
            if e.mem.u32(device_desc.wrapping_add(4)) == 0 || device_desc == 0 {
                copy_error(e, DEVICE_NOT_VALID_010EE0E8);
                report(e, NO_DEVICE_OF_TYPE_010EE0AC);
            } else {
                let filled = e
                    .call(
                        PRESENT_PARAMETERS_00E6BF50,
                        &args![
                            this,
                            e.mem.u32(this + WINDOW),
                            width,
                            height,
                            flags,
                            twelfth,
                            eighth,
                            ninth,
                            thirteenth,
                            eleventh,
                            refresh_rate,
                            tenth,
                            present
                        ],
                    )
                    .u8();
                if filled != 0 {
                    let back_buffers = e.mem.u32(present + 0xc);
                    let mode = frame + FRAME_MODE;
                    for i in 0..6 {
                        e.mem.set_u32(mode + 4 * i, 0);
                    }
                    let back_buffer_format = e.mem.u32(present + 8);
                    e.mem.set_u32(mode + 0xc, refresh_rate);
                    e.mem.set_u32(mode, 0x18);
                    e.mem.set_u32(mode + 8, height);
                    e.mem.set_u32(mode + 4, width);
                    e.mem.set_u32(mode + 0x14, 1);
                    e.mem.set_u32(mode + 0x10, back_buffer_format);
                    let result = create_device(e, this, frame, frame + FRAME_EX_OUT);
                    let tall = unsigned_on_x87(e, e.mem.u32(present + 4));
                    let wide = unsigned_on_x87(e, e.mem.u32(present));
                    let ratio = x87_div(tall, wide);
                    let limit = e.mem.f64(RATIO_LIMIT_0101DE30);
                    // FCOMP / FNSTSW / TEST AH,1: C0 is set below the limit or unordered.
                    let below = !(ratio >= limit);
                    e.mem.set_u8(this + 0xad0, u8::from(below));
                    if result >= 0 {
                        attempt = DeviceAttempt::Created;
                    } else {
                        e.mem.set_u8(this + 0x6fc, 0);
                        if back_buffers != e.mem.u32(present + 0xc) {
                            report(e, FEWER_BACK_BUFFERS_010EE048);
                            let retry = create_device(e, this, frame, frame + FRAME_EX_RETRY_OUT);
                            if retry >= 0 {
                                attempt = DeviceAttempt::Created;
                            }
                        }
                    }
                }
            }
            if let DeviceAttempt::Created = attempt {
                break;
            }
            if kind == 4 {
                e.call(SET_ERROR_00E69AD0, &args![NO_REFERENCE_DEVICE_010EDEDC]);
                report(e, NO_REFERENCE_DEVICE_REPORT_010EDE88);
                return 0;
            }
            kind = kind.wrapping_add(1);
            if kind == 4 {
                report(e, NO_HARDWARE_OF_ANY_TYPE_010EDE30);
                return 0;
            }
            report(e, NEXT_LOWER_TYPE_010EDFE0);
            let accepted = e
                .call(
                    DEVICE_SETTINGS_00E69510,
                    &args![kind, device_type_at, behavior_at],
                )
                .u8();
            if accepted == 0 {
                e.call(SET_ERROR_00E69AD0, &args![INVALID_DEVICE_TYPE_010EE20C]);
                report(e, INVALID_DEVICE_ID_010EE1C8);
                return 0;
            }
        }

        if e.mem.u32(this + DEVICE) == 0 {
            e.call(SET_ERROR_00E69AD0, &args![NO_HARDWARE_DEVICE_010EDFAC]);
            report(e, NO_DEVICE_OF_ANY_TYPE_010EDF60);
            return 0;
        }
        if e.call(AFTER_DEVICE_00E6C240, &args![this, present]).u8() == 0 {
            return 0;
        }
        let behavior = e.mem.u32(behavior_at);
        e.mem.set_u8(this + 0x5d4, !((behavior >> 6) as u8) & 1);
        e.mem.set_u8(this + 0x5d5, ((behavior >> 7) as u8) & 1);
        let block_out = frame + FRAME_BLOCK_OUT;
        let depth_out = frame + FRAME_DEPTH_OUT;
        e.mem.set_u32(block_out, 0);
        e.mem.set_u32(depth_out, 0);
        let device = e.mem.u32(this + DEVICE);
        let blocks = e
            .call(BACK_BUFFER_00E7D6B0, &args![device, present, block_out])
            .u32();
        if e.mem.u32(present + 0x24) != 0 {
            let device = e.mem.u32(this + DEVICE);
            e.call(DEPTH_STENCIL_00E7D850, &args![device, depth_out]);
        }
        let group = e
            .call(RENDER_TARGET_GROUP_00EE8710, &args![1u32, this])
            .u32();
        e.mem.set_u32(this + 0x884, group);
        let back_buffer = e.mem.u32(block_out);
        e.vcall(group, 0xa8, &args![back_buffer, 0u32]);
        let group = e.mem.u32(this + 0x884);
        let depth = e.mem.u32(depth_out);
        e.vcall(group, 0xac, &args![depth]);
        // An `NiPointer` to the group made in the outgoing argument slot, passed by value.
        let pointer = e.with_stack(4, |e, slot| {
            let group = e.mem.u32(this + 0x884);
            e.call(NI_POINTER_CONSTRUCT_00B6AED0, &args![slot, group]);
            e.mem.u32(slot.addr())
        });
        e.call(
            WINDOW_TARGET_00E72CF0,
            &args![this + 0x890, window, pointer],
        );
        let group = e.mem.u32(this + 0x884);
        let buffer = e.vcall(group, 0x9c, &args![0u32]).u32();
        let raw_format = e.mem.u32(buffer.wrapping_add(0xc));
        let converted = e.call(FORMAT_00E69D00, &args![raw_format]).u32();
        let format = e.call(FORMAT_00E69C90, &args![converted]).u32();
        e.mem.set_u32(this + 0x5d0, format);
        if e.call(TEXTURE_DEFAULTS_00E6EC90, &args![this]).u8() == 0 {
            report(e, NO_TEXTURE_DEFAULTS_010EDF10);
            return 0;
        }
        let library = e
            .call(SHADER_LIBRARY_00E91590, &args![this, this + 0x28c, 1u32])
            .u32();
        e.mem.set_u32(this + 0x8b8, library);
        e.call(INITIALIZE_SHADER_SYSTEM, &args![this]);
        let made = |e: &mut Engine, size: u32, construct: u32, words: &[u32]| {
            let block = e.call(NEW_OBJECT, &args![size]).u32();
            if block == 0 {
                return 0;
            }
            let mut all = vec![block];
            all.extend_from_slice(words);
            e.call(construct, &all).u32()
        };
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x100, CONSTRUCT_00E8D0E0, &[device]);
        e.mem.set_u32(this + 0x8bc, object);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x4c, CONSTRUCT_00E8FB20, &[device]);
        e.mem.set_u32(this + 0x8c0, object);
        let object = made(e, 0x10, CONSTRUCT_00E90A80, &[this]);
        e.mem.set_u32(this + 0x8c4, object);
        let device = e.mem.u32(this + DEVICE);
        let first = e.mem.u32(this + 0x8bc);
        let manager = e
            .call(GEOMETRY_GROUP_MANAGER_00E88EB0, &args![device, first])
            .u32();
        e.mem.set_u32(this + 0x8ac, manager);
        let renderer_flags = e.vcall(this, 0xa0, &args![]).u32();
        let manager = e.mem.u32(this + 0x8ac);
        e.call(
            SET_FLAG_00E88C20,
            &args![manager, u32::from((renderer_flags >> 1) as u8 & 1)],
        );
        let manager = e.mem.u32(this + 0x8ac);
        let group_1 = e.vcall(manager, 4, &args![1u32]).u32();
        e.mem.set_u32(this + 0x8b0, group_1);
        let manager = e.mem.u32(this + 0x8ac);
        let group_2 = e.vcall(manager, 4, &args![2u32]).u32();
        e.mem.set_u32(this + 0x8b4, group_2);
        let library = e.mem.u32(this + 0x8b8);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x23c, CONSTRUCT_00E8BAE0, &[library, device]);
        e.mem.set_u32(this + 0x8c8, object);
        if e.mem.u32(blocks.wrapping_add(0x30)) != 0 {
            let library = e.mem.u32(this + 0x8b8);
            e.vcall(library, 0x34, &args![1u32]);
        }
        let group = e.mem.u32(this + 0x884);
        e.mem.set_u32(this + 0x888, group);
        let frustum = frame + FRAME_FRUSTUM;
        e.call(NI_FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let side = x87_quiet_bits(e.mem.u32(FRUSTUM_SIDE_01012054));
        let near = x87_quiet_bits(e.mem.u32(FRUSTUM_NEAR_0101E2BC));
        let one = x87_store32(1.0);
        let zero = x87_store32(0.0);
        for (offset, value) in [
            (0xc, side),
            (0, side),
            (8, one),
            (4, one),
            (0x10, near),
            (0x14, one),
        ] {
            e.mem.set_u32(frustum + offset, value);
        }
        let viewport = frame + FRAME_MODE;
        for (offset, value) in [(0, zero), (0xc, zero), (4, one), (8, one)] {
            e.mem.set_u32(viewport + offset, value);
        }
        let [a, b, c, d] = CAMERA_WORDS_SLOT_18C;
        e.vcall(this, 0x18c, &args![a, b, c, d, frustum, viewport]);
        e.mem.set_u32(this + 0x888, 0);
        e.mem.set_u32(this + 0x88c, 0);
        e.call(NI_SHADER_FACTORY_GET_INSTANCE, &args![]);
        let block = e.call(NEW_OBJECT, &args![0x40u32]).u32();
        let shader = if block != 0 {
            e.call(CONSTRUCT_00E909E0, &args![block]).u32()
        } else {
            0
        };
        e.vcall(shader, 0x4c, &args![]);
        e.call(SET_SHADER_00E6E6E0, &args![this, shader]);
        1
    })
}

/// `memset` (`00ec61c0`).
const MEMSET_ADDR: u32 = 0x00ec_61c0;

/// The registry entry of [`fn_00e72e60`]: `this` and its 14 words.
fn fn_00e72e60_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    u32::from(fn_00e72e60(
        e,
        Ptr::new(w(0)),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        w(12),
        w(13),
        w(14),
    ))
    .into_ret()
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    const CREATE9: u32 = 0x7200_0000;
    const CREATE9_EX: u32 = 0x7200_0010;
    const OBJECT: u32 = 0x7300_0000;
    const OBJECT_EX: u32 = 0x7300_0100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn direct3d_rig(usable: bool, ex: u32, ex_object: u32, object: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(DIRECT3D_CREATE9, CREATE9);
        e.mem.set_u32(DIRECT3D_CREATE9_EX, ex);
        e.register_double(LOAD_DIRECT3D, move |_, _| u32::from(usable).into_ret());
        e.register(REPORT_00A29680, |_, _| Ret::default());
        e.register_double(CREATE9_EX, move |e, a| {
            e.mem.set_u32(a[1], ex_object);
            Ret::default()
        });
        e.register_double(CREATE9, move |_, _| object.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_direct3d9_prefers_ex_when_asked() {
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT_EX);
        assert_eq!(e.mem.u8(DIRECT3D_IS_EX), 1);
        assert!(calls(&e, CREATE9).is_empty());
        assert_eq!(calls(&e, CREATE9_EX)[0][0], D3D_SDK_VERSION);
        // Made already: nothing more.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert!(calls(&e, LOAD_DIRECT3D).is_empty());
    }

    #[test]
    fn create_direct3d9_falls_back_and_keeps_the_ex_failure() {
        let mut e = direct3d_rig(true, CREATE9_EX, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_EX_FAILED_010ED598],
                vec![CREATE_SUCCEEDED_010ED52C]
            ]
        );
        // Without the byte (or the Ex function) only the plain creation runs.
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![0x100u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert!(calls(&e, CREATE9_EX).is_empty());
    }

    #[test]
    fn create_direct3d9_failures() {
        let mut e = direct3d_rig(false, 0, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_FAILED_010ED550]]
        );
        let mut e = direct3d_rig(true, 0, 0, 0);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(calls(&e, CREATE9), vec![vec![D3D_SDK_VERSION]]);
    }

    // ---- 00e72e60 ----

    const D3D_VTABLE: u32 = 0x7400_0000;
    const D3D_SLOTS: u32 = 0x7500_0000;
    const DEVICE_OBJECT: u32 = 0x7600_0000;
    const RENDERER_VTABLE: u32 = 0x7410_0000;
    const RENDERER_SLOTS: u32 = 0x7510_0000;
    const GROUP_VTABLE: u32 = 0x7420_0000;
    const GROUP_SLOTS: u32 = 0x7520_0000;
    const ADAPTER_INDEX: u32 = 0;
    const WORDS: [u32; 14] = [
        640,
        480,
        0x60,
        0x11,
        0x22,
        ADAPTER_INDEX,
        1,
        8,
        9,
        10,
        11,
        12,
        13,
        60,
    ];

    /// The Direct3D and renderer doubles. `create_results` are the `CreateDevice` results in
    /// order (the last repeats); `00e6bf50` sets the back buffer count to `back_buffers`.
    struct Setup {
        ex: bool,
        create_results: Vec<i32>,
        adapter_ok: bool,
        caps: u32,
    }

    fn vtable(e: &mut Engine, at: u32, slots: u32, count: u32) {
        let targets: Vec<u32> = (0..count).map(|i| slots + 4 * i).collect();
        e.put_vtable(at, &targets);
    }

    fn init_rig(setup: Setup) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_e000, 0x2000);
        e.map(0x0101_0000, 0x10000);
        e.map(0x0109_9000, 0x1000);
        e.mem.set_u32(FRUSTUM_SIDE_01012054, (-1.0f32).to_bits());
        e.mem.set_u32(FRUSTUM_NEAR_0101E2BC, 0.1f32.to_bits());
        e.mem.set_f64(RATIO_LIMIT_0101DE30, 0.75);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        // The Direct3D object and its slots.
        vtable(&mut e, D3D_VTABLE, D3D_SLOTS, 0x20);
        let direct3d = e.mem.alloc(4);
        e.mem.set_u32(direct3d, D3D_VTABLE);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.mem.set_u8(DIRECT3D_IS_EX, u8::from(setup.ex));
        let mut results = setup.create_results.clone().into_iter();
        let mut last = 0;
        let slot = if setup.ex {
            CREATE_DEVICE_EX
        } else {
            CREATE_DEVICE
        };
        e.register_double(D3D_SLOTS + slot, move |e, a| {
            let result = results.next().unwrap_or(last);
            last = result;
            // Present parameters: a changed back buffer count on the first failure.
            if result < 0 {
                e.mem.set_u32(a[5] + 0xc, 1);
            }
            let out = if setup.ex { a[7] } else { a[6] };
            e.mem
                .set_u32(out, if result >= 0 { DEVICE_OBJECT } else { 0 });
            (result as u32).into_ret()
        });
        // The adapter list: one adapter with a device description for type 1.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(ADAPTER_LIST, list);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(list + 8, elements);
        e.mem.set_u16(list + 0xe, 1);
        let adapter = e.mem.alloc(0x468);
        if setup.adapter_ok {
            e.mem.set_u32(elements, adapter);
        }
        let device_desc = e.mem.alloc(0x144);
        e.mem.set_u32(device_desc + 4, setup.caps);
        e.mem.set_u32(adapter + 0x460, device_desc);
        e.mem.set_u32(adapter + 0x464, device_desc);
        // Device kind -> type 1 for kinds below 3, behaviour flags 0x40.
        e.register(DEVICE_SETTINGS_00E69510, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[2], 0x40);
            u32::from(a[0] < 3).into_ret()
        });
        e.register(PRESENT_PARAMETERS_00E6BF50, |e, a| {
            let present = a[12];
            e.mem.set_u32(present, 640);
            e.mem.set_u32(present + 4, 480);
            e.mem.set_u32(present + 8, 0x16);
            e.mem.set_u32(present + 0xc, 2);
            e.mem.set_u32(present + 0x24, 1);
            1u32.into_ret()
        });
        let blocks = e.mem.alloc(0x40);
        e.mem.set_u32(blocks + 0x30, 1);
        e.register_double(BACK_BUFFER_00E7D6B0, move |e, a| {
            e.mem.set_u32(a[2], 0x5151);
            blocks.into_ret()
        });
        e.register(DEPTH_STENCIL_00E7D850, |e, a| {
            e.mem.set_u32(a[1], 0x5252);
            Ret::default()
        });
        // The render target group.
        vtable(&mut e, GROUP_VTABLE, GROUP_SLOTS, 0x30);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group, GROUP_VTABLE);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer + 0xc, 0x77);
        e.register_double(RENDER_TARGET_GROUP_00EE8710, move |_, _| group.into_ret());
        e.register_double(GROUP_SLOTS + 0x9c, move |_, _| buffer.into_ret());
        e.register(GROUP_SLOTS + 0x4c, |_, _| Ret::default());
        e.register(GROUP_SLOTS + 4, |_, a| (0x900 + a[1]).into_ret());
        for slot in [0xa8, 0xac, 0x34] {
            e.register(GROUP_SLOTS + slot, |_, _| Ret::default());
        }
        e.register(NI_POINTER_CONSTRUCT_00B6AED0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // Every constructor answers an object with the group's table.
        for construct in [
            CONSTRUCT_00E8D0E0,
            CONSTRUCT_00E8FB20,
            CONSTRUCT_00E90A80,
            CONSTRUCT_00E8BAE0,
            CONSTRUCT_00E909E0,
        ] {
            e.register(construct, |e, a| {
                e.mem.set_u32(a[0], GROUP_VTABLE);
                a[0].into_ret()
            });
        }
        e.register_double(GEOMETRY_GROUP_MANAGER_00E88EB0, move |_, _| {
            group.into_ret()
        });
        e.register_double(SHADER_LIBRARY_00E91590, move |_, _| group.into_ret());
        for no_op in [
            RESET_00E69A60,
            STRNCPY_S,
            REPORT_00A29680,
            SET_ERROR_00E69AD0,
            WINDOW_TARGET_00E72CF0,
            INITIALIZE_SHADER_SYSTEM,
            SET_FLAG_00E88C20,
            NI_FRUSTUM_CONSTRUCT,
            NI_SHADER_FACTORY_GET_INSTANCE,
            SET_SHADER_00E6E6E0,
            MEMSET_ADDR,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(FORMAT_00E69D00, |_, a| (a[0] + 1).into_ret());
        e.register(FORMAT_00E69C90, |_, a| (a[0] + 1).into_ret());
        for ok in [AFTER_DEVICE_00E6C240, TEXTURE_DEFAULTS_00E6EC90] {
            e.register(ok, |_, _| 1u32.into_ret());
        }
        // The renderer and its slots 0xa0 and 0x18c.
        vtable(&mut e, RENDERER_VTABLE, RENDERER_SLOTS, 0x70);
        e.register(RENDERER_SLOTS + 0xa0, |_, _| 2u32.into_ret());
        e.register(RENDERER_SLOTS + 0x18c, |e, a| {
            // Frustum and viewport as the renderer receives them.
            let frustum: Vec<f32> = (0..6).map(|i| e.mem.f32(a[5] + 4 * i)).collect();
            let viewport: Vec<f32> = (0..4).map(|i| e.mem.f32(a[6] + 4 * i)).collect();
            assert_eq!(frustum, vec![-1.0, 1.0, 1.0, -1.0, 0.1, 1.0]);
            assert_eq!(viewport, vec![0.0, 1.0, 1.0, 0.0]);
            Ret::default()
        });
        let this = e.mem.alloc(0xb80);
        e.mem.set_u32(this, RENDERER_VTABLE);
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn initialize(e: &mut Engine, this: u32) -> u8 {
        let mut words = vec![this];
        words.extend_from_slice(&WORDS);
        e.call(0x00e7_2e60, &words).u8()
    }

    #[test]
    fn initialize_creates_the_device_and_the_renderer_objects() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        assert_eq!(e.mem.u32(this + 0xa98), 640);
        assert_eq!(e.mem.u32(this + 0xacc), 60);
        // Behaviour flags: 0x40 from 00e69510, 2 for flags & 0x40, 4 for flags & 0x20.
        assert_eq!(e.mem.u32(this + BEHAVIOR_FLAGS), 0x46);
        assert_eq!(e.mem.u8(this + 0x5d4), 0);
        assert_eq!(e.mem.u8(this + 0x5d5), 0);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE);
        assert_eq!(create.len(), 1);
        assert_eq!(&create[0][1..5], &[ADAPTER_INDEX, 1, 0x22, 0x46]);
        assert_eq!(create[0][6], this + DEVICE);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        // 480 / 640 = 0.75 is not below 0.75.
        assert_eq!(e.mem.u8(this + 0xad0), 0);
        let group = e.mem.u32(this + 0x884);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xa8), vec![vec![group, 0x5151, 0]]);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xac), vec![vec![group, 0x5252]]);
        assert_eq!(
            calls(&e, WINDOW_TARGET_00E72CF0),
            vec![vec![this + 0x890, 0x11, group]]
        );
        assert_eq!(e.mem.u32(this + 0x5d0), 0x79);
        assert_eq!(e.mem.u32(this + 0x8b0), 0x901);
        assert_eq!(e.mem.u32(this + 0x8b4), 0x902);
        assert_eq!(calls(&e, SET_FLAG_00E88C20)[0][1], 1);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x34).len(), 1);
        assert_eq!(calls(&e, RENDERER_SLOTS + 0x18c).len(), 1);
        assert_eq!(e.mem.u32(this + 0x888), 0);
        let shader = calls(&e, SET_SHADER_00E6E6E0)[0][1];
        assert_ne!(shader, 0);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x4c), vec![vec![shader]]);
    }

    #[test]
    fn initialize_retries_with_fewer_back_buffers_through_create_device_ex() {
        let (mut e, this) = init_rig(Setup {
            ex: true,
            create_results: vec![-1, 0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE_EX);
        assert_eq!(create.len(), 2);
        // Windowed is 0 in the present parameters: the display mode is passed.
        assert_ne!(create[0][6], 0);
        assert_ne!(create[1][7], create[0][7]);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        assert!(calls(&e, REPORT_00A29680).contains(&vec![FEWER_BACK_BUFFERS_010EE048]));
    }

    #[test]
    fn initialize_walks_the_device_kinds_down_and_fails() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![-1],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        // Kinds 1 and 2 (each with its retry: the failure changed the back buffer count),
        // then kind 3 refused.
        assert_eq!(calls(&e, D3D_SLOTS + CREATE_DEVICE).len(), 4);
        let kinds: Vec<u32> = calls(&e, DEVICE_SETTINGS_00E69510)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(kinds, vec![1, 2, 3]);
        assert_eq!(
            calls(&e, SET_ERROR_00E69AD0),
            vec![vec![INVALID_DEVICE_TYPE_010EE20C]]
        );
    }

    #[test]
    fn initialize_refuses_a_missing_adapter_or_device() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: false,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![INVALID_ADAPTER_REPORT_010EE160]]
        );
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 0,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(e.mem.u32(this + 0x5dc), 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], INVALID_DEVICE_TYPE_010EE20C);
        assert!(calls(&e, D3D_SLOTS + CREATE_DEVICE).is_empty());
    }
}
