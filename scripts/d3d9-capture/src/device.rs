//! `d3d9-capture device`: how the retail Direct3D 9 runtime answers the device calls the game
//! makes, measured on a windowed HAL device with hardware vertex processing on a hidden window
//! (never shown). These answers are the runtime's general behaviour (they do not describe the
//! player's hardware); the platform's layer is written from them. Each line: the call, the case,
//! the HRESULT (hex), and what was read back afterwards where the case says so.
//!
//! `cargo run --release --manifest-path scripts/d3d9-capture/Cargo.toml -- device` (64-bit, as
//! the rest of the tool; the game runs the 32-bit runtime).
use std::ffi::c_void;
use std::io::Write;

/// `IDirect3DDevice9` slots used here (d3d9.h order).
const RELEASE: usize = 2;
const BEGIN_SCENE: usize = 41;
const END_SCENE: usize = 42;
const CREATE_VERTEX_BUFFER: usize = 26;
const CREATE_INDEX_BUFFER: usize = 27;
const DRAW_INDEXED_PRIMITIVE: usize = 82;
const SET_INDICES: usize = 104;
const DRAW_PRIMITIVE: usize = 81;
const DRAW_PRIMITIVE_UP: usize = 83;
const SET_FVF: usize = 89;
const SET_VERTEX_DECLARATION: usize = 87;
const SET_STREAM_SOURCE: usize = 100;
const GET_STREAM_SOURCE: usize = 101;
const SET_RENDER_STATE: usize = 57;
const SET_TEXTURE: usize = 65;
const PRESENT: usize = 17;

type P = *mut c_void;

#[repr(C)]
struct PresentParameters {
    width: u32,
    height: u32,
    format: u32,
    count: u32,
    multisample: u32,
    quality: u32,
    swap_effect: u32,
    window: P,
    windowed: i32,
    auto_depth: i32,
    depth_format: u32,
    flags: u32,
    refresh: u32,
    interval: u32,
}

unsafe fn slot(object: P, index: usize) -> usize {
    *(*(object as *const *const usize)).add(index)
}

/// Calls slot `index` of `object` with `args` (all words; at most 6).
unsafe fn call(object: P, index: usize, args: &[usize]) -> u32 {
    let f = slot(object, index);
    match args.len() {
        0 => std::mem::transmute::<usize, unsafe extern "system" fn(P) -> u32>(f)(object),
        1 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize) -> u32>(f)(object, args[0]),
        2 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize) -> u32>(f)(object, args[0], args[1]),
        3 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2]),
        4 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3]),
        5 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4]),
        _ => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5]),
    }
}

/// The stream 0 binding read back: buffer, offset, stride.
unsafe fn stream0(device: P) -> (usize, u32, u32) {
    let (mut buffer, mut offset, mut stride) = (0usize, 0xaaaa_aaaau32, 0xaaaa_aaaau32);
    call(device, GET_STREAM_SOURCE, &[0, &mut buffer as *mut usize as usize, &mut offset as *mut u32 as usize, &mut stride as *mut u32 as usize]);
    if buffer != 0 {
        // GetStreamSource adds a reference.
        call(buffer as P, RELEASE, &[]);
    }
    (buffer, offset, stride)
}

pub unsafe fn measure(o: &mut impl Write) {
    let instance = crate::GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs d3d9-capture device\0";
    let class = crate::WndClassA {
        style: 3,
        procedure: crate::DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    crate::RegisterClassA(&class);
    // WS_OVERLAPPED without WS_VISIBLE: the window is never shown.
    let window = crate::CreateWindowExA(0, name.as_ptr(), name.as_ptr(), 0, 0, 0, 320, 240,
        std::ptr::null_mut(), std::ptr::null_mut(), instance, std::ptr::null_mut());
    assert!(!window.is_null());
    let d3d = crate::Direct3DCreate9(32) as P;
    assert!(!d3d.is_null());
    // D3DPRESENT_PARAMETERS: 320x240, D3DFMT_UNKNOWN back buffer, one, discard, windowed,
    // automatic D24S8 depth.
    let mut pp = PresentParameters {
        width: 320,
        height: 240,
        format: 0,
        count: 1,
        multisample: 0,
        quality: 0,
        swap_effect: 1,
        window,
        windowed: 1,
        auto_depth: 1,
        depth_format: 75,
        flags: 0,
        refresh: 0,
        interval: 0,
    };
    let mut device: P = std::ptr::null_mut();
    // IDirect3D9::CreateDevice (slot 16): adapter 0, HAL, window, hardware vertex processing.
    let hr = call(d3d, 16, &[0, 1, window as usize, 0x40, &mut pp as *mut PresentParameters as usize, &mut device as *mut P as usize]);
    writeln!(o, "CreateDevice\thal windowed hardware\t{hr:x}").unwrap(); o.flush().unwrap();
    assert!(!device.is_null(), "no device");

    // The vertices the movie player draws: XYZRHW | TEX1 (FVF 0x104), 0x18 bytes each.
    let vertices = [0f32; 6 * 6];
    let data = vertices.as_ptr() as usize;
    let up = |device: P, kind: usize, count: usize, data: usize, stride: usize| call(device, DRAW_PRIMITIVE_UP, &[kind, count, data, stride]);

    // `device outside`: which condition made the first draws from memory fail: each step on
    // the fresh device, outside any scene, reported in turn.
    if std::env::args().nth(2).as_deref() == Some("outside") {
        call(device, SET_FVF, &[0x104]);
        let mut buffer: P = std::ptr::null_mut();
        call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let hr = up(device, 5, 2, data, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP	fresh, fvf, a vertex buffer on stream 0, never in a scene	{hr:x}	after: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap();
        call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfresh, fvf, no stream 0, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        call(device, BEGIN_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after an empty scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        // A draw from a vertex buffer on a fresh device's history is the other check: a second
        // device below.
        o.flush().unwrap();
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("outside-present") {
        call(device, SET_FVF, &[0x104]);
        writeln!(o, "Present\tfresh\t{:x}", call(device, PRESENT, &[0, 0, 0, 0])).unwrap();
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a Present, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap();
        o.flush().unwrap();
        return;
    }
    // Without an FVF or a declaration.
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\tno fvf, outside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_FVF, &[0x104]);
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    for (case, kind, count, d, stride) in [
        ("strip 2", 5usize, 2usize, data, 0x18usize),
        ("list 6 (movie quads)", 2, 6, data, 0x18),
        ("type 1 points", 1, 2, data, 0x18),
        ("type 3 lines", 3, 1, data, 0x18),
        ("stride 0", 5, 2, data, 0),
        ("stride 4", 5, 2, data, 4),
    ] {
        let hr = up(device, kind, count, d, stride);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {case}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    // Stream 0 after a draw from memory: with a vertex buffer bound before.
    let mut buffer: P = std::ptr::null_mut();
    let hr = call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
    writeln!(o, "CreateVertexBuffer\t0x90 bytes, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let before = stream0(device);
    writeln!(o, "GetStreamSource\tafter SetStreamSource\tbuffer {} offset {:x} stride {:x}", before.0 != 0, before.1, before.2).unwrap(); o.flush().unwrap();
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();

    // The cases that may fault inside the runtime, one per run (`device <case>`): the line
    // before the call says it was tried; EndScene and Present follow, each reported.
    let risky = std::env::args().nth(2).unwrap_or_default();
    if !risky.is_empty() {
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\ttrying").unwrap(); o.flush().unwrap();
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let (kind, count, d) = match risky.as_str() {
            "null-data" => (5, 2, 0),
            "type-0" => (0, 2, data),
            "type-7" => (7, 2, data),
            "type-9" => (9, 2, data),
            "count-0" => (5, 0, data),
            _ => {
                call(device, SET_VERTEX_DECLARATION, &[0]);
                (5, 2, data)
            }
        };
        let hr = up(device, kind, count, d, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
        let hr = call(device, END_SCENE, &[]);
        writeln!(o, "EndScene\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        let hr = call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "Present\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        return;
    }
    writeln!(o, "EndScene\tafter the draws\t{:x}", call(device, END_SCENE, &[])).unwrap(); o.flush().unwrap();
    // A draw from a vertex buffer outside a scene.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0, 0x18]);
    let hr = call(device, DRAW_PRIMITIVE, &[5, 0, 2]);
    writeln!(o, "DrawPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // An indexed draw outside a scene (16-bit indices, write-only, default pool).
    let mut indices: P = std::ptr::null_mut();
    let hr = call(device, CREATE_INDEX_BUFFER, &[12, 8, 101, 0, &mut indices as *mut P as usize, 0]);
    writeln!(o, "CreateIndexBuffer\t12 bytes, 16-bit, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[indices as usize]);
    let hr = call(device, DRAW_INDEXED_PRIMITIVE, &[4, 0, 0, 4, 0, 2]);
    writeln!(o, "DrawIndexedPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[0]);
    call(indices, RELEASE, &[]);
    // Stream 0 after a draw from memory that fails: outside a scene, and with stride 0.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\toutside a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, stride 0, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    let hr = call(device, BEGIN_SCENE, &[]);
    writeln!(o, "BeginScene\tinside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, END_SCENE, &[]);
    let hr = call(device, END_SCENE, &[]);
    writeln!(o, "EndScene\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // State calls outside a scene.
    let hr = call(device, SET_RENDER_STATE, &[7, 0]);
    writeln!(o, "SetRenderState\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, SET_TEXTURE, &[0, 0]);
    writeln!(o, "SetTexture\toutside a scene, null\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, PRESENT, &[0, 0, 0, 0]);
    writeln!(o, "Present\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
    call(buffer, RELEASE, &[]);
    let refs = call(device, RELEASE, &[]);
    writeln!(o, "Release\tdevice\t{refs:x}").unwrap(); o.flush().unwrap();
    call(d3d, RELEASE, &[]);
    crate::DestroyWindow(window);
    crate::UnregisterClassA(name.as_ptr(), instance);
}
