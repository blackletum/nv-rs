//! The platform's file calls: `CreateFileA`, `GetFileType`, `SetFilePointer`, `WriteFile`,
//! `ReadFile` and `CloseHandle` on file handles (KERNEL32 imports, the CRT's file layer below
//! `fopen`/`fclose`/`fwrite` uses them: `__tsopen_nolock`, `_lseek`, `_write`, `_close`), with
//! the documented Win32 semantics.
//!
//! Every file lives in [`Disk`], in engine-side memory: nothing the game writes reaches the
//! player's disk (the game's own files are read through the archive layer, not here). A file the
//! game did not make in this run (an open of an existing file it never wrote) is not provided:
//! the call stops the run (a panic the run records), as other platform calls not provided do.

use super::KernelObject;
use crate::{Engine, Ptr};
use std::collections::BTreeMap;

/// `GENERIC_READ` and `GENERIC_WRITE` (the access mask's top bits).
pub const GENERIC_READ: u32 = 0x8000_0000;
pub const GENERIC_WRITE: u32 = 0x4000_0000;
/// The creation dispositions (`CREATE_NEW` .. `TRUNCATE_EXISTING`).
pub const CREATE_NEW: u32 = 1;
pub const CREATE_ALWAYS: u32 = 2;
pub const OPEN_EXISTING: u32 = 3;
pub const OPEN_ALWAYS: u32 = 4;
pub const TRUNCATE_EXISTING: u32 = 5;
/// `INVALID_HANDLE_VALUE` and `INVALID_SET_FILE_POINTER`.
pub const INVALID_HANDLE_VALUE: u32 = 0xffff_ffff;
pub const INVALID_SET_FILE_POINTER: u32 = 0xffff_ffff;
/// `FILE_TYPE_UNKNOWN` and `FILE_TYPE_DISK`.
pub const FILE_TYPE_UNKNOWN: u32 = 0;
pub const FILE_TYPE_DISK: u32 = 1;
/// The error codes these calls set (winerror.h).
pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_HANDLE: u32 = 6;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_FILE_EXISTS: u32 = 80;
pub const ERROR_NEGATIVE_SEEK: u32 = 131;
pub const ERROR_ALREADY_EXISTS: u32 = 183;

/// The files the game made in this run, by path (ASCII lower case, `/` as `\`), and their bytes.
#[derive(Debug, Default)]
pub struct Disk {
    pub files: BTreeMap<Vec<u8>, Vec<u8>>,
}

/// An open file handle's state ([`KernelObject::File`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFile {
    pub path: Vec<u8>,
    pub position: u64,
    pub read: bool,
    pub write: bool,
}

fn key(path: &[u8]) -> Vec<u8> {
    path.iter()
        .map(|&b| if b == b'/' { b'\\' } else { b.to_ascii_lowercase() })
        .collect()
}

fn open_file(e: &mut Engine, handle: u32) -> Option<&mut OpenFile> {
    match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::File(file)) => Some(file),
        _ => None,
    }
}

// Platform: replaces KERNEL32.DLL CreateFileA (import)
/// `CreateFileA(path, access, share, security, disposition, attributes, template)`: opens or
/// makes the file in [`Disk`] by the disposition: `CREATE_NEW` fails with `ERROR_FILE_EXISTS`
/// on an existing file; `CREATE_ALWAYS` empties or makes it (`ERROR_ALREADY_EXISTS` when it
/// existed, else 0); `OPEN_EXISTING` and `TRUNCATE_EXISTING` (which needs `GENERIC_WRITE` and
/// empties it) fail with `ERROR_FILE_NOT_FOUND` on a missing file; `OPEN_ALWAYS` makes a missing
/// one (`ERROR_ALREADY_EXISTS` when it existed). Answers the handle (position 0) or
/// `INVALID_HANDLE_VALUE`. Sharing, security and attributes don't change the outcome here.
/// An open of a file the game didn't make in this run is not provided (stops the run).
#[allow(clippy::too_many_arguments)]
pub fn create_file_a(
    e: &mut Engine,
    path: Ptr,
    access: u32,
    _share: u32,
    _security: u32,
    disposition: u32,
    _attributes: u32,
    _template: u32,
) -> u32 {
    let name = e.mem.cstr(path.addr());
    let path = key(&name);
    let exists = e.kernel.disk.files.contains_key(&path);
    let fail = |e: &mut Engine, code: u32| {
        e.kernel.last_error = code;
        INVALID_HANDLE_VALUE
    };
    let mut error = ERROR_SUCCESS;
    match disposition {
        CREATE_NEW if exists => return fail(e, ERROR_FILE_EXISTS),
        CREATE_NEW => {
            e.kernel.disk.files.insert(path.clone(), vec![]);
        }
        CREATE_ALWAYS => {
            if exists {
                error = ERROR_ALREADY_EXISTS;
            }
            e.kernel.disk.files.insert(path.clone(), vec![]);
        }
        OPEN_EXISTING | TRUNCATE_EXISTING if !exists => {
            panic!(
                "CreateFileA: opening \"{}\", a file the game did not make in this run, is not provided",
                String::from_utf8_lossy(&name)
            );
        }
        OPEN_EXISTING => {}
        TRUNCATE_EXISTING => {
            if access & GENERIC_WRITE == 0 {
                return fail(e, ERROR_INVALID_PARAMETER);
            }
            e.kernel.disk.files.insert(path.clone(), vec![]);
        }
        OPEN_ALWAYS => {
            if exists {
                error = ERROR_ALREADY_EXISTS;
            } else {
                e.kernel.disk.files.insert(path.clone(), vec![]);
            }
        }
        _ => return fail(e, ERROR_INVALID_PARAMETER),
    }
    e.kernel.last_error = error;
    e.kernel.insert(KernelObject::File(OpenFile {
        path,
        position: 0,
        read: access & GENERIC_READ != 0,
        write: access & GENERIC_WRITE != 0,
    }))
}

// Platform: replaces KERNEL32.DLL GetFileType (import)
/// `GetFileType(handle)`: `FILE_TYPE_DISK` for a file handle; `FILE_TYPE_UNKNOWN` with
/// `ERROR_INVALID_HANDLE` for anything else.
pub fn get_file_type(e: &mut Engine, handle: u32) -> u32 {
    if open_file(e, handle).is_some() {
        e.kernel.last_error = ERROR_SUCCESS;
        FILE_TYPE_DISK
    } else {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        FILE_TYPE_UNKNOWN
    }
}

// Platform: replaces KERNEL32.DLL SetFilePointer (import)
/// `SetFilePointer(handle, low, high, method)`: moves the position by the distance (`low` alone
/// as a signed 32-bit value when `high` is null, else the signed 64-bit `high:low`) from the
/// start (0), the position (1) or the end (2). Answers the new position's low word (its high
/// word stored through `high`) with last error 0; a position before the start fails with
/// `ERROR_NEGATIVE_SEEK`, a bad method with `ERROR_INVALID_PARAMETER`, both answering
/// `INVALID_SET_FILE_POINTER` and leaving the position.
pub fn set_file_pointer(e: &mut Engine, handle: u32, low: u32, high: Ptr, method: u32) -> u32 {
    let distance = if high.addr() == 0 {
        i64::from(low as i32)
    } else {
        ((u64::from(e.mem.u32(high.addr())) << 32) | u64::from(low)) as i64
    };
    let Some(file) = open_file(e, handle) else {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        return INVALID_SET_FILE_POINTER;
    };
    let path = file.path.clone();
    let position = file.position;
    let size = e.kernel.disk.files.get(&path).map_or(0, |d| d.len() as u64);
    let base = match method {
        0 => 0i64,
        1 => position as i64,
        2 => size as i64,
        _ => {
            e.kernel.last_error = ERROR_INVALID_PARAMETER;
            return INVALID_SET_FILE_POINTER;
        }
    };
    let target = base.wrapping_add(distance);
    if target < 0 {
        e.kernel.last_error = ERROR_NEGATIVE_SEEK;
        return INVALID_SET_FILE_POINTER;
    }
    if let Some(file) = open_file(e, handle) {
        file.position = target as u64;
    }
    if high.addr() != 0 {
        e.mem.set_u32(high.addr(), (target as u64 >> 32) as u32);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    target as u32
}

// Platform: replaces KERNEL32.DLL WriteFile (import)
/// `WriteFile(handle, buffer, count, written, overlapped)`: writes `count` bytes at the position
/// (the file grows, a gap left by a seek past the end reads as zeros), moves the position on and
/// stores the count through `written`; a handle opened without `GENERIC_WRITE` fails with
/// `ERROR_ACCESS_DENIED`, any other handle with `ERROR_INVALID_HANDLE` (0 answered and 0 written).
/// Overlapped writes are not provided (stops the run).
pub fn write_file(
    e: &mut Engine,
    handle: u32,
    buffer: Ptr,
    count: u32,
    written: Ptr,
    overlapped: u32,
) -> i32 {
    assert!(overlapped == 0, "WriteFile: overlapped writes are not provided");
    if written.addr() != 0 {
        e.mem.set_u32(written.addr(), 0);
    }
    let (path, position) = match open_file(e, handle) {
        Some(file) if file.write => (file.path.clone(), file.position),
        Some(_) => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            return 0;
        }
        None => {
            e.kernel.last_error = ERROR_INVALID_HANDLE;
            return 0;
        }
    };
    let bytes = e.mem.bytes(buffer.addr(), count);
    let data = e.kernel.disk.files.entry(path).or_default();
    let start = position as usize;
    let end = start + bytes.len();
    if data.len() < end {
        data.resize(end, 0);
    }
    data[start..end].copy_from_slice(&bytes);
    if let Some(file) = open_file(e, handle) {
        file.position = end as u64;
    }
    if written.addr() != 0 {
        e.mem.set_u32(written.addr(), count);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL ReadFile (import)
/// `ReadFile(handle, buffer, count, read, overlapped)`: reads up to `count` bytes from the
/// position (fewer at the end, none past it: success with 0 read), moves the position on and
/// stores the count read through `read`; a handle opened without `GENERIC_READ` fails with
/// `ERROR_ACCESS_DENIED`, any other handle with `ERROR_INVALID_HANDLE`. Overlapped reads are not
/// provided (stops the run).
pub fn read_file(
    e: &mut Engine,
    handle: u32,
    buffer: Ptr,
    count: u32,
    read: Ptr,
    overlapped: u32,
) -> i32 {
    assert!(overlapped == 0, "ReadFile: overlapped reads are not provided");
    if read.addr() != 0 {
        e.mem.set_u32(read.addr(), 0);
    }
    let (path, position) = match open_file(e, handle) {
        Some(file) if file.read => (file.path.clone(), file.position),
        Some(_) => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            return 0;
        }
        None => {
            e.kernel.last_error = ERROR_INVALID_HANDLE;
            return 0;
        }
    };
    let data = e.kernel.disk.files.get(&path).cloned().unwrap_or_default();
    let start = (position as usize).min(data.len());
    let end = start.saturating_add(count as usize).min(data.len());
    e.mem.write(buffer.addr(), &data[start..end]);
    let got = (end - start) as u32;
    if let Some(file) = open_file(e, handle) {
        file.position = position + u64::from(got);
    }
    if read.addr() != 0 {
        e.mem.set_u32(read.addr(), got);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let name = e.mem.alloc(0x40);
        e.mem.write(name, b"C:\\Some\\RendererInfo.txt\0");
        (e, name)
    }

    #[test]
    fn create_always_makes_then_reports_an_existing_file() {
        let (mut e, name) = engine();
        let h = create_file_a(&mut e, Ptr::new(name), GENERIC_WRITE, 0, 0, CREATE_ALWAYS, 0x80, 0);
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, 0);
        assert_eq!(get_file_type(&mut e, h), FILE_TYPE_DISK);
        let h2 = create_file_a(&mut e, Ptr::new(name), GENERIC_WRITE, 0, 0, CREATE_ALWAYS, 0x80, 0);
        assert_ne!(h2, h);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
        assert_eq!(
            create_file_a(&mut e, Ptr::new(name), GENERIC_WRITE, 0, 0, CREATE_NEW, 0, 0),
            INVALID_HANDLE_VALUE
        );
        assert_eq!(e.kernel.last_error, ERROR_FILE_EXISTS);
    }

    #[test]
    fn writes_seeks_and_reads_back() {
        let (mut e, name) = engine();
        let rw = GENERIC_READ | GENERIC_WRITE;
        let h = create_file_a(&mut e, Ptr::new(name), rw, 0, 0, OPEN_ALWAYS, 0, 0);
        let buffer = e.mem.alloc(0x20);
        let count = e.mem.alloc(4);
        e.mem.write(buffer, b"abcdef");
        assert_eq!(write_file(&mut e, h, Ptr::new(buffer), 6, Ptr::new(count), 0), 1);
        assert_eq!(e.mem.u32(count), 6);
        // From the end, -2: position 4.
        assert_eq!(set_file_pointer(&mut e, h, (-2i32) as u32, Ptr::new(0), 2), 4);
        assert_eq!(read_file(&mut e, h, Ptr::new(buffer + 0x10), 8, Ptr::new(count), 0), 1);
        assert_eq!(e.mem.u32(count), 2);
        assert_eq!(e.mem.bytes(buffer + 0x10, 2), b"ef".to_vec());
        // At the end: success, nothing read.
        assert_eq!(read_file(&mut e, h, Ptr::new(buffer), 8, Ptr::new(count), 0), 1);
        assert_eq!(e.mem.u32(count), 0);
        // Before the start fails and keeps the position.
        assert_eq!(
            set_file_pointer(&mut e, h, (-100i32) as u32, Ptr::new(0), 1),
            INVALID_SET_FILE_POINTER
        );
        assert_eq!(e.kernel.last_error, ERROR_NEGATIVE_SEEK);
        // A seek past the end leaves a gap of zeros.
        assert_eq!(set_file_pointer(&mut e, h, 8, Ptr::new(0), 0), 8);
        write_file(&mut e, h, Ptr::new(buffer), 1, Ptr::new(count), 0);
        let path = key(b"c:/some/rendererinfo.txt");
        assert_eq!(e.kernel.disk.files[&path], b"abcdef\0\0a".to_vec());
        assert_eq!(super::super::close_handle(&mut e, h), 1);
        assert_eq!(get_file_type(&mut e, h), FILE_TYPE_UNKNOWN);
        assert_eq!(e.kernel.last_error, ERROR_INVALID_HANDLE);
    }

    #[test]
    fn a_read_only_handle_refuses_writes() {
        let (mut e, name) = engine();
        let h = create_file_a(&mut e, Ptr::new(name), GENERIC_WRITE, 0, 0, CREATE_NEW, 0, 0);
        super::super::close_handle(&mut e, h);
        let h = create_file_a(&mut e, Ptr::new(name), GENERIC_READ, 0, 0, OPEN_EXISTING, 0, 0);
        let buffer = e.mem.alloc(4);
        assert_eq!(write_file(&mut e, h, Ptr::new(buffer), 1, Ptr::new(0), 0), 0);
        assert_eq!(e.kernel.last_error, ERROR_ACCESS_DENIED);
    }

    #[test]
    #[should_panic(expected = "not provided")]
    fn opening_a_file_the_game_did_not_make_stops() {
        let (mut e, name) = engine();
        create_file_a(&mut e, Ptr::new(name), GENERIC_READ, 0, 0, OPEN_EXISTING, 0, 0);
    }
}
