//! The platform's Direct3D 9 device (ADR-0007): `IDirect3D9::CreateDevice` and the
//! `IDirect3DDevice9` it makes, with its surfaces, following the documented Direct3D 9
//! semantics (the DirectX 9.0c SDK documentation of each method). Answers that depend on the
//! player's hardware come from the machine table (`super::d3d9`, `scripts/d3d9-capture`).
//!
//! As in `super::d3d9`, each object lives in engine memory with its vtable pointer at `+0`, each
//! provided slot a platform function at a code address from [`DEVICE_CODE`] (the device) or
//! [`SURFACE_CODE`] (surfaces); a slot not provided is left unregistered, so a call to it stops
//! the run as an open function with its address (slot = (address - the code base) / 4, named
//! in [`DEVICE_SLOTS`] and [`SURFACE_SLOTS`]). Nothing is drawn yet: the viewer still draws.
//!
//! Provided so far, as the game's start-up reaches them: windowed devices of the HAL type with
//! hardware or mixed vertex processing, without multisampling (anything else stops the run as
//! not provided); the device's reference counting, `GetDirect3D`, `GetDeviceCaps`,
//! `GetDisplayMode`, `GetCreationParameters`, `GetBackBuffer`, the render target and
//! depth-stencil slots and the viewport; a surface's reference counting, `GetDevice`, `GetType`
//! and `GetDesc`.

use super::d3d9::{CODE_BASE, D3DERR_INVALIDCALL, D3D_OK, E_NOINTERFACE};
use crate::{Engine, Ptr};
use std::collections::HashMap;

/// Where the device's slots sit (after `IDirect3D9`'s).
pub const DEVICE_CODE: u32 = CODE_BASE + 0x0200;
/// Where a surface's slots sit.
pub const SURFACE_CODE: u32 = CODE_BASE + 0x0400;

/// `D3DERR_NOTFOUND` (d3d9.h).
pub const D3DERR_NOTFOUND: u32 = 0x8876_0866;

/// `IDirect3DDevice9`'s slots (d3d9.h order).
pub const DEVICE_SLOTS: [&str; 119] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "TestCooperativeLevel",
    "GetAvailableTextureMem",
    "EvictManagedResources",
    "GetDirect3D",
    "GetDeviceCaps",
    "GetDisplayMode",
    "GetCreationParameters",
    "SetCursorProperties",
    "SetCursorPosition",
    "ShowCursor",
    "CreateAdditionalSwapChain",
    "GetSwapChain",
    "GetNumberOfSwapChains",
    "Reset",
    "Present",
    "GetBackBuffer",
    "GetRasterStatus",
    "SetDialogBoxMode",
    "SetGammaRamp",
    "GetGammaRamp",
    "CreateTexture",
    "CreateVolumeTexture",
    "CreateCubeTexture",
    "CreateVertexBuffer",
    "CreateIndexBuffer",
    "CreateRenderTarget",
    "CreateDepthStencilSurface",
    "UpdateSurface",
    "UpdateTexture",
    "GetRenderTargetData",
    "GetFrontBufferData",
    "StretchRect",
    "ColorFill",
    "CreateOffscreenPlainSurface",
    "SetRenderTarget",
    "GetRenderTarget",
    "SetDepthStencilSurface",
    "GetDepthStencilSurface",
    "BeginScene",
    "EndScene",
    "Clear",
    "SetTransform",
    "GetTransform",
    "MultiplyTransform",
    "SetViewport",
    "GetViewport",
    "SetMaterial",
    "GetMaterial",
    "SetLight",
    "GetLight",
    "LightEnable",
    "GetLightEnable",
    "SetClipPlane",
    "GetClipPlane",
    "SetRenderState",
    "GetRenderState",
    "CreateStateBlock",
    "BeginStateBlock",
    "EndStateBlock",
    "SetClipStatus",
    "GetClipStatus",
    "GetTexture",
    "SetTexture",
    "GetTextureStageState",
    "SetTextureStageState",
    "GetSamplerState",
    "SetSamplerState",
    "ValidateDevice",
    "SetPaletteEntries",
    "GetPaletteEntries",
    "SetCurrentTexturePalette",
    "GetCurrentTexturePalette",
    "SetScissorRect",
    "GetScissorRect",
    "SetSoftwareVertexProcessing",
    "GetSoftwareVertexProcessing",
    "SetNPatchMode",
    "GetNPatchMode",
    "DrawPrimitive",
    "DrawIndexedPrimitive",
    "DrawPrimitiveUP",
    "DrawIndexedPrimitiveUP",
    "ProcessVertices",
    "CreateVertexDeclaration",
    "SetVertexDeclaration",
    "GetVertexDeclaration",
    "SetFVF",
    "GetFVF",
    "CreateVertexShader",
    "SetVertexShader",
    "GetVertexShader",
    "SetVertexShaderConstantF",
    "GetVertexShaderConstantF",
    "SetVertexShaderConstantI",
    "GetVertexShaderConstantI",
    "SetVertexShaderConstantB",
    "GetVertexShaderConstantB",
    "SetStreamSource",
    "GetStreamSource",
    "SetStreamSourceFreq",
    "GetStreamSourceFreq",
    "SetIndices",
    "GetIndices",
    "CreatePixelShader",
    "SetPixelShader",
    "GetPixelShader",
    "SetPixelShaderConstantF",
    "GetPixelShaderConstantF",
    "SetPixelShaderConstantI",
    "GetPixelShaderConstantI",
    "SetPixelShaderConstantB",
    "GetPixelShaderConstantB",
    "DrawRectPatch",
    "DrawTriPatch",
    "DeletePatch",
    "CreateQuery",
];

/// `IDirect3DSurface9`'s slots (d3d9.h order: `IUnknown`, `IDirect3DResource9`, its own).
pub const SURFACE_SLOTS: [&str; 17] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetDevice",
    "SetPrivateData",
    "GetPrivateData",
    "FreePrivateData",
    "SetPriority",
    "GetPriority",
    "PreLoad",
    "GetType",
    "GetContainer",
    "GetDesc",
    "LockRect",
    "UnlockRect",
    "GetDC",
    "ReleaseDC",
];

/// `D3DDEVTYPE_HAL`, the vertex-processing behaviour flags (`D3DCREATE_SOFTWARE_`,
/// `HARDWARE_`, `MIXED_VERTEXPROCESSING`), `D3DFMT_UNKNOWN`, `D3DRTYPE_SURFACE`, the surface
/// usages `D3DUSAGE_RENDERTARGET` and `D3DUSAGE_DEPTHSTENCIL`, `D3DPRESENT_BACK_BUFFERS_MAX`.
const D3DDEVTYPE_HAL: u32 = 1;
const SOFTWARE_VERTEXPROCESSING: u32 = 0x20;
const HARDWARE_VERTEXPROCESSING: u32 = 0x40;
const MIXED_VERTEXPROCESSING: u32 = 0x80;
const D3DFMT_UNKNOWN: u32 = 0;
const D3DRTYPE_SURFACE: u32 = 1;
const D3DUSAGE_RENDERTARGET: u32 = 1;
const D3DUSAGE_DEPTHSTENCIL: u32 = 2;
const BACK_BUFFERS_MAX: u32 = 3;
/// `D3DPRESENT_PARAMETERS` (0x38 bytes) fields.
const PP_WIDTH: u32 = 0x0;
const PP_HEIGHT: u32 = 0x4;
const PP_FORMAT: u32 = 0x8;
const PP_COUNT: u32 = 0xc;
const PP_MULTISAMPLE: u32 = 0x10;
const PP_QUALITY: u32 = 0x14;
const PP_WINDOW: u32 = 0x1c;
const PP_WINDOWED: u32 = 0x20;
const PP_AUTO_DEPTH: u32 = 0x24;
const PP_DEPTH_FORMAT: u32 = 0x28;
const PP_REFRESH: u32 = 0x30;
const PRESENT_SIZE: u32 = 0x38;
/// `D3DCAPS9::NumSimultaneousRTs`.
const CAPS_SIMULTANEOUS_RTS: usize = 0xf0;
/// The render target slots a device keeps (`D3D_MAX_SIMULTANEOUS_RENDERTARGETS`).
const RENDER_TARGETS: usize = 4;

/// `IID_IUnknown` and `IID_IDirect3DDevice9`, `IID_IDirect3DResource9`, `IID_IDirect3DSurface9`
/// as their 16 bytes (d3d9.h, unknwn.h).
const IID_IUNKNOWN: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0xc0, 0, 0, 0, 0, 0, 0, 0x46];
const IID_DEVICE9: [u8; 16] = [
    0x96, 0x3b, 0x22, 0xd0, 0x7a, 0xbf, 0xfd, 0x43, 0x92, 0xbd, 0xa4, 0x3b, 0x0d, 0x82, 0xb9, 0xeb,
];
const IID_RESOURCE9: [u8; 16] = [
    0x0b, 0x11, 0xbb, 0x05, 0x5a, 0xa8, 0x2f, 0x45, 0xbe, 0x9b, 0x8d, 0x7f, 0x54, 0xbc, 0x13, 0x67,
];
const IID_SURFACE9: [u8; 16] = [
    0x9a, 0x28, 0xc8, 0x0c, 0xfa, 0x3b, 0x8c, 0x4f, 0x8d, 0x75, 0xa3, 0x69, 0x70, 0xa0, 0x7e, 0x9f,
];

/// A device of this layer.
#[derive(Debug, Clone)]
struct Device {
    references: u32,
    direct3d: u32,
    /// `D3DDEVICE_CREATION_PARAMETERS`: adapter, device type, focus window, behaviour flags.
    creation: [u32; 4],
    /// The present parameters as the device took them (after filling in the defaults).
    present: Vec<u8>,
    back_buffers: Vec<u32>,
    render_targets: [u32; RENDER_TARGETS],
    depth_stencil: u32,
    /// `D3DVIEWPORT9`: X, Y, Width, Height, then MinZ and MaxZ as `float` bits.
    viewport: [u32; 6],
}

/// A surface of this layer.
#[derive(Debug, Clone)]
struct Surface {
    /// References the program holds (`AddRef`/`Release`).
    references: u32,
    device: u32,
    /// Whether the device owns it (the back buffers and the automatic depth-stencil surface):
    /// it stays while the device lives, whatever the program's count.
    implicit: bool,
    /// `D3DSURFACE_DESC`: Format, Type, Usage, Pool, MultiSampleType, MultiSampleQuality,
    /// Width, Height.
    desc: [u32; 8],
}

/// The devices and their resources (`Kernel::graphics`).
#[derive(Debug, Default)]
pub struct Objects {
    device_vtable: u32,
    surface_vtable: u32,
    devices: HashMap<u32, Device>,
    surfaces: HashMap<u32, Surface>,
}

fn vtable(e: &mut Engine, base: u32, slots: usize) -> u32 {
    let table = e.mem.alloc(4 * slots as u32);
    for i in 0..slots as u32 {
        e.mem.set_u32(table + 4 * i, base + 4 * i);
    }
    table
}

fn register_code(e: &mut Engine) {
    let device: [(u32, crate::AbiFn); 14] = [
        (
            0,
            crate::entry!(0, device_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, device_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, device_release(Ptr) -> u32).1),
        (6, crate::entry!(0, get_direct3d(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, get_device_caps(Ptr, Ptr) -> u32).1),
        (
            8,
            crate::entry!(0, get_display_mode(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            9,
            crate::entry!(0, get_creation_parameters(Ptr, Ptr) -> u32).1,
        ),
        (
            18,
            crate::entry!(0, get_back_buffer(Ptr, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            37,
            crate::entry!(0, set_render_target(Ptr, u32, u32) -> u32).1,
        ),
        (
            38,
            crate::entry!(0, get_render_target(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            39,
            crate::entry!(0, set_depth_stencil_surface(Ptr, u32) -> u32).1,
        ),
        (
            40,
            crate::entry!(0, get_depth_stencil_surface(Ptr, Ptr) -> u32).1,
        ),
        (47, crate::entry!(0, set_viewport(Ptr, Ptr) -> u32).1),
        (48, crate::entry!(0, get_viewport(Ptr, Ptr) -> u32).1),
    ];
    for (slot, f) in device {
        e.register(DEVICE_CODE + 4 * slot, f);
    }
    let surface: [(u32, crate::AbiFn); 6] = [
        (
            0,
            crate::entry!(0, surface_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, surface_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, surface_release(Ptr) -> u32).1),
        (3, crate::entry!(0, surface_get_device(Ptr, Ptr) -> u32).1),
        (10, crate::entry!(0, surface_get_type(Ptr) -> u32).1),
        (12, crate::entry!(0, surface_get_desc(Ptr, Ptr) -> u32).1),
    ];
    for (slot, f) in surface {
        e.register(SURFACE_CODE + 4 * slot, f);
    }
    if e.kernel.graphics.device_objects.device_vtable == 0 {
        let device = vtable(e, DEVICE_CODE, DEVICE_SLOTS.len());
        let surface = vtable(e, SURFACE_CODE, SURFACE_SLOTS.len());
        let objects = &mut e.kernel.graphics.device_objects;
        objects.device_vtable = device;
        objects.surface_vtable = surface;
    }
}

fn not_provided(what: &str) -> ! {
    panic!("IDirect3D9::CreateDevice: {what} is not provided")
}

fn new_surface(e: &mut Engine, device: u32, implicit: bool, desc: [u32; 8]) -> u32 {
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.surface_vtable;
    e.mem.set_u32(object, table);
    let references = u32::from(!implicit);
    e.kernel.graphics.device_objects.surfaces.insert(
        object,
        Surface {
            references,
            device,
            implicit,
            desc,
        },
    );
    object
}

// Platform: replaces D3D9.DLL IDirect3D9::CreateDevice (vtable +0x40)
/// `CreateDevice(adapter, device type, focus window, behaviour flags, present parameters,
/// device out)`. A null present parameters or out, another adapter, or behaviour flags without
/// exactly one of the vertex-processing flags answer `D3DERR_INVALIDCALL`; so does a windowed
/// device with neither a device window nor a focus window, or with a full-screen refresh rate,
/// and a back buffer count above `D3DPRESENT_BACK_BUFFERS_MAX` (3; the count is set to 3, so a
/// second call can succeed). A count of 0 becomes 1 and the format `D3DFMT_UNKNOWN` the
/// display mode's format, written back into the caller's parameters as Direct3D does. The
/// device gets its back buffers (render-target surfaces of the parameters' size and format;
/// the first is render target 0), the automatic depth-stencil surface when asked for, the
/// viewport of the whole back buffer (depth 0 to 1), a reference to this `IDirect3D9`, and its
/// creation runs [`crate::units::platform::direct3d_device_created`] (the calling thread's
/// x87 precision). Not provided (the run stops): other device types, software vertex
/// processing, full-screen devices, a zero width or height (the window's client area),
/// multisampling, and a back buffer format the table's `CheckDeviceType` refuses.
#[allow(clippy::too_many_arguments)]
pub fn create_device(
    e: &mut Engine,
    this: Ptr,
    adapter: u32,
    device_type: u32,
    focus: u32,
    behavior: u32,
    present: Ptr,
    out: Ptr,
) -> u32 {
    if present.is_null() || out.is_null() || adapter != 0 {
        return D3DERR_INVALIDCALL;
    }
    let processing =
        behavior & (SOFTWARE_VERTEXPROCESSING | HARDWARE_VERTEXPROCESSING | MIXED_VERTEXPROCESSING);
    if processing.count_ones() != 1 {
        return D3DERR_INVALIDCALL;
    }
    if device_type != D3DDEVTYPE_HAL {
        not_provided(&format!("device type {device_type}"));
    }
    if processing == SOFTWARE_VERTEXPROCESSING {
        not_provided("software vertex processing");
    }
    let p = present.addr();
    let field = |e: &Engine, offset: u32| e.mem.u32(p + offset);
    if field(e, PP_WINDOWED) == 0 {
        not_provided("a full-screen device");
    }
    if field(e, PP_WINDOW) == 0 && focus == 0 {
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_REFRESH) != 0 {
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_COUNT) > BACK_BUFFERS_MAX {
        e.mem.set_u32(p + PP_COUNT, BACK_BUFFERS_MAX);
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_WIDTH) == 0 || field(e, PP_HEIGHT) == 0 {
        not_provided("a back buffer size taken from the window");
    }
    if field(e, PP_MULTISAMPLE) != 0 {
        not_provided("multisampling");
    }
    // The display mode's format (`D3DDISPLAYMODE` +0xC).
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(this.addr(), 4 * 8, &[0, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for CreateDevice");
        e.mem.u32(mode.addr() + 0xc)
    });
    if field(e, PP_COUNT) == 0 {
        e.mem.set_u32(p + PP_COUNT, 1);
    }
    if field(e, PP_FORMAT) == D3DFMT_UNKNOWN {
        e.mem.set_u32(p + PP_FORMAT, display_format);
    }
    let format = field(e, PP_FORMAT);
    let check = e
        .vcall(
            this.addr(),
            4 * 9,
            &[adapter, device_type, display_format, format, 1],
        )
        .u32();
    if check != D3D_OK {
        not_provided(&format!(
            "a windowed back buffer of format {format} (CheckDeviceType answers {check:#x})"
        ));
    }
    register_code(e);
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.device_vtable;
    e.mem.set_u32(object, table);
    let (width, height) = (field(e, PP_WIDTH), field(e, PP_HEIGHT));
    let quality = field(e, PP_QUALITY);
    let back_buffers: Vec<u32> = (0..field(e, PP_COUNT))
        .map(|_| {
            let desc = [
                format,
                D3DRTYPE_SURFACE,
                D3DUSAGE_RENDERTARGET,
                0,
                0,
                quality,
                width,
                height,
            ];
            new_surface(e, object, true, desc)
        })
        .collect();
    let depth_stencil = if field(e, PP_AUTO_DEPTH) != 0 {
        let depth_format = field(e, PP_DEPTH_FORMAT);
        let desc = [
            depth_format,
            D3DRTYPE_SURFACE,
            D3DUSAGE_DEPTHSTENCIL,
            0,
            0,
            quality,
            width,
            height,
        ];
        new_surface(e, object, true, desc)
    } else {
        0
    };
    let mut render_targets = [0; RENDER_TARGETS];
    render_targets[0] = back_buffers[0];
    // The device holds its render target and depth surface (counted, as `SetRenderTarget` and
    // `SetDepthStencilSurface` do).
    surface_add_ref(e, Ptr::new(back_buffers[0]));
    if depth_stencil != 0 {
        surface_add_ref(e, Ptr::new(depth_stencil));
    }
    e.vcall(this.addr(), 4, &[]);
    let device = Device {
        references: 1,
        direct3d: this.addr(),
        creation: [adapter, device_type, focus, behavior],
        present: e.mem.bytes(p, PRESENT_SIZE),
        back_buffers,
        render_targets,
        depth_stencil,
        viewport: [0, 0, width, height, 0f32.to_bits(), 1f32.to_bits()],
    };
    e.kernel
        .graphics
        .device_objects
        .devices
        .insert(object, device);
    crate::units::platform::direct3d_device_created(e, behavior);
    e.mem.set_u32(out.addr(), object);
    D3D_OK
}

fn device(e: &mut Engine, this: Ptr) -> &mut Device {
    e.kernel
        .graphics
        .device_objects
        .devices
        .get_mut(&this.addr())
        .expect("an IDirect3DDevice9 of this layer")
}

fn iid_is(e: &Engine, iid: Ptr, ids: &[[u8; 16]]) -> bool {
    !iid.is_null()
        && ids
            .iter()
            .any(|id| e.mem.bytes(iid.addr(), 16) == id.as_slice())
}

/// `QueryInterface(iid, out)`: `IUnknown` and `IDirect3DDevice9` answer the device (counted);
/// others `E_NOINTERFACE` with a null out.
fn device_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(e, iid, &[IID_IUNKNOWN, IID_DEVICE9]) {
        device_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn device_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references += 1;
    d.references
}

/// `Release()`: the last reference destroys the device: its implicit surfaces go, the surfaces
/// it holds in its render target and depth slots are released, and its `IDirect3D9` reference.
fn device_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references -= 1;
    let left = d.references;
    if left == 0 {
        let d = e
            .kernel
            .graphics
            .device_objects
            .devices
            .remove(&this.addr())
            .expect("the device");
        for held in d.render_targets.iter().chain([&d.depth_stencil]) {
            if *held != 0 {
                surface_release(e, Ptr::new(*held));
            }
        }
        let implicit: Vec<u32> = e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .iter()
            .filter(|(_, s)| s.device == this.addr() && s.implicit)
            .map(|(a, _)| *a)
            .collect();
        for surface in implicit {
            e.kernel.graphics.device_objects.surfaces.remove(&surface);
            e.mem.free(surface);
        }
        e.vcall(d.direct3d, 4 * 2, &[]);
        e.mem.free(this.addr());
    }
    left
}

/// `GetDirect3D(out)`: the `IDirect3D9` that made the device, counted.
fn get_direct3d(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let direct3d = device(e, this).direct3d;
    e.vcall(direct3d, 4, &[]);
    e.mem.set_u32(out.addr(), direct3d);
    D3D_OK
}

/// `GetDeviceCaps(out)`: the adapter's caps for the device's type
/// (`IDirect3D9::GetDeviceCaps`, from the table); with hardware or mixed vertex processing the
/// device reports the same caps.
fn get_device_caps(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    e.vcall(direct3d, 4 * 14, &[adapter, device_type, out.addr()])
        .u32()
}

/// `GetDisplayMode(swap chain, out)`: for the implicit swap chain (0) of a windowed device the
/// adapter's display mode; another swap chain `D3DERR_INVALIDCALL`.
fn get_display_mode(e: &mut Engine, this: Ptr, swap_chain: u32, out: Ptr) -> u32 {
    if swap_chain != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, adapter) = (d.direct3d, d.creation[0]);
    e.vcall(direct3d, 4 * 8, &[adapter, out.addr()]).u32()
}

/// `GetCreationParameters(out)`: the adapter, device type, focus window and behaviour flags
/// `CreateDevice` was given.
fn get_creation_parameters(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let creation = device(e, this).creation;
    for (i, word) in creation.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// `GetBackBuffer(swap chain, index, type, out)`: back buffer `index` of the implicit swap
/// chain (0), counted; another swap chain, an index past the count, a type other than
/// `D3DBACKBUFFER_TYPE_MONO` (0) or a null out answer `D3DERR_INVALIDCALL`.
fn get_back_buffer(
    e: &mut Engine,
    this: Ptr,
    swap_chain: u32,
    index: u32,
    kind: u32,
    out: Ptr,
) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let buffers = device(e, this).back_buffers.clone();
    if swap_chain != 0 || kind != 0 || index as usize >= buffers.len() {
        return D3DERR_INVALIDCALL;
    }
    let surface = buffers[index as usize];
    surface_add_ref(e, Ptr::new(surface));
    e.mem.set_u32(out.addr(), surface);
    D3D_OK
}

fn simultaneous_render_targets(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(0x130, |e, caps| {
        let hr = get_device_caps(e, this, caps);
        assert_eq!(hr, D3D_OK, "the device's caps");
        e.mem.u32(caps.addr() + CAPS_SIMULTANEOUS_RTS as u32)
    })
}

/// `SetRenderTarget(index, surface)`: an index at or past the caps' `NumSimultaneousRTs`, a
/// null surface for index 0, or a surface without `D3DUSAGE_RENDERTARGET` answer
/// `D3DERR_INVALIDCALL`. The device holds the new surface (counted) and releases the old one;
/// setting index 0 resets the viewport to the whole surface (depth 0 to 1).
fn set_render_target(e: &mut Engine, this: Ptr, index: u32, surface: u32) -> u32 {
    if index >= simultaneous_render_targets(e, this) || index as usize >= RENDER_TARGETS {
        return D3DERR_INVALIDCALL;
    }
    if surface == 0 && index == 0 {
        return D3DERR_INVALIDCALL;
    }
    let desc = if surface != 0 {
        match e.kernel.graphics.device_objects.surfaces.get(&surface) {
            Some(s) if s.desc[2] & D3DUSAGE_RENDERTARGET != 0 => Some(s.desc),
            _ => return D3DERR_INVALIDCALL,
        }
    } else {
        None
    };
    if surface != 0 {
        surface_add_ref(e, Ptr::new(surface));
    }
    let old = std::mem::replace(&mut device(e, this).render_targets[index as usize], surface);
    if old != 0 {
        surface_release(e, Ptr::new(old));
    }
    if let (0, Some(desc)) = (index, desc) {
        device(e, this).viewport = [0, 0, desc[6], desc[7], 0f32.to_bits(), 1f32.to_bits()];
    }
    D3D_OK
}

/// `GetRenderTarget(index, out)`: the surface in that slot, counted; an empty slot answers
/// `D3DERR_NOTFOUND` (out null); an index at or past `NumSimultaneousRTs` or a null out
/// `D3DERR_INVALIDCALL`.
fn get_render_target(e: &mut Engine, this: Ptr, index: u32, out: Ptr) -> u32 {
    if out.is_null()
        || index >= simultaneous_render_targets(e, this)
        || index as usize >= RENDER_TARGETS
    {
        return D3DERR_INVALIDCALL;
    }
    let surface = device(e, this).render_targets[index as usize];
    e.mem.set_u32(out.addr(), surface);
    if surface == 0 {
        return D3DERR_NOTFOUND;
    }
    surface_add_ref(e, Ptr::new(surface));
    D3D_OK
}

/// `SetDepthStencilSurface(surface)`: null removes the depth buffer; a surface without
/// `D3DUSAGE_DEPTHSTENCIL` answers `D3DERR_INVALIDCALL`. The device holds the new surface
/// (counted) and releases the old one.
fn set_depth_stencil_surface(e: &mut Engine, this: Ptr, surface: u32) -> u32 {
    if surface != 0 {
        match e.kernel.graphics.device_objects.surfaces.get(&surface) {
            Some(s) if s.desc[2] & D3DUSAGE_DEPTHSTENCIL != 0 => {}
            _ => return D3DERR_INVALIDCALL,
        }
        surface_add_ref(e, Ptr::new(surface));
    }
    let old = std::mem::replace(&mut device(e, this).depth_stencil, surface);
    if old != 0 {
        surface_release(e, Ptr::new(old));
    }
    D3D_OK
}

/// `GetDepthStencilSurface(out)`: the depth surface, counted; none answers `D3DERR_NOTFOUND`
/// (out null).
fn get_depth_stencil_surface(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let surface = device(e, this).depth_stencil;
    e.mem.set_u32(out.addr(), surface);
    if surface == 0 {
        return D3DERR_NOTFOUND;
    }
    surface_add_ref(e, Ptr::new(surface));
    D3D_OK
}

/// `SetViewport(viewport)`: keeps the six words of the `D3DVIEWPORT9`; a null pointer, or a
/// rectangle outside render target 0, answers `D3DERR_INVALIDCALL`.
fn set_viewport(e: &mut Engine, this: Ptr, viewport: Ptr) -> u32 {
    if viewport.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let words: Vec<u32> = (0..6).map(|i| e.mem.u32(viewport.addr() + 4 * i)).collect();
    let target = device(e, this).render_targets[0];
    let desc = e.kernel.graphics.device_objects.surfaces[&target].desc;
    let right = u64::from(words[0]) + u64::from(words[2]);
    let bottom = u64::from(words[1]) + u64::from(words[3]);
    if right > u64::from(desc[6]) || bottom > u64::from(desc[7]) {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    d.viewport.copy_from_slice(&words);
    D3D_OK
}

/// `GetViewport(out)`: the viewport's six words.
fn get_viewport(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let viewport = device(e, this).viewport;
    for (i, word) in viewport.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

fn surface(e: &mut Engine, this: Ptr) -> &mut Surface {
    e.kernel
        .graphics
        .device_objects
        .surfaces
        .get_mut(&this.addr())
        .expect("an IDirect3DSurface9 of this layer")
}

/// `QueryInterface(iid, out)`: `IUnknown`, `IDirect3DResource9` and `IDirect3DSurface9` answer
/// the surface (counted); others `E_NOINTERFACE` with a null out.
fn surface_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(e, iid, &[IID_IUNKNOWN, IID_RESOURCE9, IID_SURFACE9]) {
        surface_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn surface_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let s = surface(e, this);
    s.references += 1;
    s.references
}

/// `Release()`: the count falls; a surface the program made goes at 0, one the device owns
/// (back buffers, the automatic depth surface) stays until the device goes.
fn surface_release(e: &mut Engine, this: Ptr) -> u32 {
    let s = surface(e, this);
    s.references = s.references.saturating_sub(1);
    let (left, implicit) = (s.references, s.implicit);
    if left == 0 && !implicit {
        e.kernel
            .graphics
            .device_objects
            .surfaces
            .remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `GetDevice(out)`: the surface's device, counted.
fn surface_get_device(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let owner = surface(e, this).device;
    device_add_ref(e, Ptr::new(owner));
    e.mem.set_u32(out.addr(), owner);
    D3D_OK
}

/// `GetType()`: `D3DRTYPE_SURFACE`.
fn surface_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    D3DRTYPE_SURFACE
}

/// `GetDesc(out)`: the 0x20-byte `D3DSURFACE_DESC`.
fn surface_get_desc(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let desc = surface(e, this).desc;
    for (i, word) in desc.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// The present parameters a device took (for the viewer and tests).
pub fn device_present_parameters(e: &Engine, device: u32) -> Option<Vec<u8>> {
    e.kernel
        .graphics
        .device_objects
        .devices
        .get(&device)
        .map(|d| d.present.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::platform::d3d9;

    /// TEST DATA, not a capture: a small adapter table in `scripts/d3d9-capture`'s format (a
    /// 640x480 X8R8G8B8 desktop, HAL caps with four simultaneous render targets, a windowed
    /// X8R8G8B8 back buffer accepted).
    fn test_table() -> String {
        let hex = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
        let mut mode = vec![0u8; 16];
        mode[..4].copy_from_slice(&640u32.to_le_bytes());
        mode[4..8].copy_from_slice(&480u32.to_le_bytes());
        mode[8..12].copy_from_slice(&60u32.to_le_bytes());
        mode[12..].copy_from_slice(&0x16u32.to_le_bytes());
        let mut caps = vec![0u8; 304];
        caps[..4].copy_from_slice(&1u32.to_le_bytes());
        caps[CAPS_SIMULTANEOUS_RTS..CAPS_SIMULTANEOUS_RTS + 4].copy_from_slice(&4u32.to_le_bytes());
        [
            format!("GetAdapterDisplayMode\t0\t0\t{}", hex(&mode)),
            format!("GetDeviceCaps\t0 1\t0\t{}", hex(&caps)),
            "Domain\td0\t0,1".to_string(),
            "CheckDeviceType\t0 1 16 16\t0\t\td0".to_string(),
        ]
        .join("\n")
    }

    /// An engine with the table and an `IDirect3D9`, and windowed present parameters (640x480,
    /// format and count left for the defaults, device window 0x11c).
    fn d3d() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        d3d9::load_adapter_table(&mut e, &test_table());
        let create = d3d9::proc_address(&mut e, b"Direct3DCreate9").unwrap();
        let d3d = e.call(create, &[d3d9::D3D_SDK_VERSION]).u32();
        let present = e.mem.alloc(PRESENT_SIZE);
        e.mem.set_u32(present + PP_WIDTH, 640);
        e.mem.set_u32(present + PP_HEIGHT, 480);
        e.mem.set_u32(present + PP_WINDOW, 0x11c);
        e.mem.set_u32(present + PP_WINDOWED, 1);
        (e, d3d, present)
    }

    const CREATE_DEVICE: u32 = 0x40;

    #[test]
    fn create_device_fills_the_defaults_and_makes_the_back_buffer_render_target_0() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        let hr = e
            .vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out])
            .u32();
        assert_eq!(hr, D3D_OK);
        let device = e.mem.u32(out);
        assert_eq!(
            e.mem.u32(device),
            e.kernel.graphics.device_objects.device_vtable
        );
        assert_eq!(e.mem.u32(present + PP_COUNT), 1);
        assert_eq!(e.mem.u32(present + PP_FORMAT), 0x16);
        // The device holds a reference to the IDirect3D9.
        assert_eq!(e.vcall(d3d, 4, &[]).u32(), 3);
        e.vcall(d3d, 8, &[]);
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 38, &[0, slot]).u32(), D3D_OK);
        let target = e.mem.u32(slot);
        assert_eq!(e.vcall(device, 4 * 18, &[0, 0, 0, slot]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(slot), target);
        assert_eq!(
            e.vcall(device, 4 * 18, &[0, 1, 0, slot]).u32(),
            D3DERR_INVALIDCALL
        );
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(target, 4 * 12, &[desc]).u32(), D3D_OK);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(desc + 4 * i)).collect();
        assert_eq!(words, [0x16, 1, 1, 0, 0, 0, 640, 480]);
        assert_eq!(e.vcall(target, 4 * 10, &[]).u32(), D3DRTYPE_SURFACE);
        // Two references taken, two released; the back buffer stays with the device.
        assert_eq!(e.vcall(target, 8, &[]).u32(), 2);
        assert_eq!(e.vcall(target, 8, &[]).u32(), 1);
        assert!(e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .contains_key(&target));
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.vcall(device, 4 * 38, &[1, slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(
            e.vcall(device, 4 * 38, &[4, slot]).u32(),
            D3DERR_INVALIDCALL
        );
        let viewport = e.mem.alloc(0x18);
        assert_eq!(e.vcall(device, 4 * 48, &[viewport]).u32(), D3D_OK);
        assert_eq!(
            (e.mem.u32(viewport + 8), e.mem.u32(viewport + 0xc)),
            (640, 480)
        );
        assert_eq!(e.mem.u32(viewport + 0x14), 1f32.to_bits());
        e.mem.set_u32(viewport + 8, 641);
        assert_eq!(
            e.vcall(device, 4 * 47, &[viewport]).u32(),
            D3DERR_INVALIDCALL
        );
        let params = e.mem.alloc(0x10);
        assert_eq!(e.vcall(device, 4 * 9, &[params]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(params + 0xc), 0x40);
        let caps = e.mem.alloc(304);
        assert_eq!(e.vcall(device, 4 * 7, &[caps]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(caps), 1);
        assert_eq!(e.vcall(device, 4 * 2, &[]).u32(), 0);
        assert!(e.kernel.graphics.device_objects.devices.is_empty());
        assert!(e.kernel.graphics.device_objects.surfaces.is_empty());
        assert_eq!(e.vcall(d3d, 4, &[]).u32(), 2);
    }

    #[test]
    fn create_device_refuses_what_direct3d_refuses() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        let create = |e: &mut Engine, behavior: u32| {
            e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, behavior, present, out])
                .u32()
        };
        assert_eq!(create(&mut e, 0), D3DERR_INVALIDCALL);
        assert_eq!(create(&mut e, 0x40 | 0x80), D3DERR_INVALIDCALL);
        e.mem.set_u32(present + PP_COUNT, 4);
        assert_eq!(create(&mut e, 0x40), D3DERR_INVALIDCALL);
        assert_eq!(e.mem.u32(present + PP_COUNT), 3);
        e.mem.set_u32(present + PP_REFRESH, 60);
        assert_eq!(create(&mut e, 0x40), D3DERR_INVALIDCALL);
        e.mem.set_u32(present + PP_REFRESH, 0);
        assert_eq!(create(&mut e, 0x40), D3D_OK);
        let device = e.mem.u32(out);
        let slot = e.mem.alloc(4);
        for i in 0..3 {
            assert_eq!(e.vcall(device, 4 * 18, &[0, i, 0, slot]).u32(), D3D_OK);
        }
        let hr = e
            .vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, 0, out])
            .u32();
        assert_eq!(hr, D3DERR_INVALIDCALL);
    }

    #[test]
    fn render_target_and_depth_slots_hold_their_surfaces() {
        let (mut e, d3d, present) = d3d();
        e.mem.set_u32(present + PP_AUTO_DEPTH, 1);
        e.mem.set_u32(present + PP_DEPTH_FORMAT, 0x4b);
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3D_OK);
        let depth = e.mem.u32(slot);
        assert_eq!(e.vcall(device, 4 * 38, &[0, slot]).u32(), D3D_OK);
        let target = e.mem.u32(slot);
        // The depth surface is no render target, the back buffer no depth surface.
        assert_eq!(
            e.vcall(device, 4 * 37, &[0, depth]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 39, &[target]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 37, &[0, 0]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 37, &[1, target]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 37, &[1, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 39, &[0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(e.vcall(device, 4 * 39, &[depth]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(slot), depth);
        let got = e.mem.alloc(4);
        assert_eq!(e.vcall(depth, 4 * 3, &[got]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(got), device);
        assert_eq!(e.vcall(device, 4 * 2, &[]).u32(), 1);
    }
}
