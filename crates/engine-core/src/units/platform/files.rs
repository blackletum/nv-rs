//! The platform's file calls: `CreateFileA`, `GetFileType`, `SetFilePointer`, `WriteFile`,
//! `ReadFile` and `CloseHandle` on file handles (KERNEL32 imports, the CRT's file layer below
//! `fopen`/`fclose`/`fwrite` uses them: `__tsopen_nolock`, `_lseek`, `_write`, `_close`), with
//! the documented Win32 semantics.
//!
//! Also the file system around them: `GetFileAttributesA`, `CreateDirectoryA`, `DeleteFileA`,
//! `CopyFileA` and `SHGetFolderPathA`.
//!
//! Every file the game makes, changes, copies or deletes lives in [`Disk`], in engine-side
//! memory: nothing reaches the player's disk. The player's own files are read only: opened, they
//! are read into the run's copy; deleted, they are only gone for the game. An open of a file
//! that exists nowhere with a disposition that needs one stops the run (a panic the run
//! records), as other platform calls not provided do.

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
    /// Files the game deleted in this run (the player's files stay where they are).
    pub deleted: std::collections::BTreeSet<Vec<u8>>,
    /// Directories the game made in this run (nothing is made on the player's disk).
    pub directories: std::collections::BTreeSet<Vec<u8>>,
}

/// Files the game deleted in this run (the player's own files stay where they are) and the
/// directories it made (nothing is made on the player's disk) are in [`Disk`].
/// `FILE_ATTRIBUTE_DIRECTORY`, `FILE_ATTRIBUTE_ARCHIVE` and `INVALID_FILE_ATTRIBUTES`.
pub const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
pub const INVALID_FILE_ATTRIBUTES: u32 = 0xffff_ffff;

/// Whether `name` is a full path (a drive letter and a separator, or `\\` for a network path).
pub fn is_full_path(name: &[u8]) -> bool {
    matches!(name, [d, b':', b'\\' | b'/', ..] if d.is_ascii_alphabetic())
        || name.starts_with(b"\\\\")
}

/// `name` as a full path: a relative name is in the current directory
/// (`Kernel::current_directory`).
pub fn full_path(e: &Engine, name: &[u8]) -> Vec<u8> {
    if is_full_path(name) {
        return name.to_vec();
    }
    let mut path = e.kernel.current_directory.clone();
    if !path.ends_with(b"\\") && !path.is_empty() {
        path.push(b'\\');
    }
    path.extend_from_slice(name.strip_prefix(b".\\").unwrap_or(name));
    path
}

fn real_path(path: &[u8]) -> std::path::PathBuf {
    std::path::PathBuf::from(String::from_utf8_lossy(path).into_owned())
}

/// The bytes of the file at the full path `path`: as the game made it in this run, else the
/// player's file (read only); `None` when the game deleted it or there is none.
pub fn read_whole(e: &Engine, path: &[u8]) -> Option<Vec<u8>> {
    let k = key(path);
    if let Some(data) = e.kernel.disk.files.get(&k) {
        return Some(data.clone());
    }
    if e.kernel.disk.deleted.contains(&k) {
        return None;
    }
    std::fs::read(real_path(path)).ok()
}

/// The attributes of the full path `path` as Windows reports them: a file or directory the game
/// made in this run, else the player's (`FILE_ATTRIBUTE_*` as the file system keeps them);
/// `None` when there is none.
fn attributes(e: &Engine, path: &[u8]) -> Option<u32> {
    let k = key(path);
    let trimmed = key(path.strip_suffix(b"\\").unwrap_or(path));
    if e.kernel.disk.files.contains_key(&k) {
        return Some(FILE_ATTRIBUTE_ARCHIVE);
    }
    if e.kernel.disk.directories.contains(&trimmed) {
        return Some(FILE_ATTRIBUTE_DIRECTORY);
    }
    if e.kernel.disk.deleted.contains(&k) {
        return None;
    }
    let meta = std::fs::metadata(real_path(path)).ok()?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        Some(meta.file_attributes())
    }
    #[cfg(not(windows))]
    {
        Some(if meta.is_dir() {
            FILE_ATTRIBUTE_DIRECTORY
        } else {
            FILE_ATTRIBUTE_ARCHIVE
        })
    }
}

// Platform: replaces KERNEL32.DLL GetFileAttributesA (import)
/// `GetFileAttributesA(name)`: the file's or directory's attributes ([`attributes`]), or
/// `INVALID_FILE_ATTRIBUTES` with `ERROR_FILE_NOT_FOUND`.
pub fn get_file_attributes_a(e: &mut Engine, name: Ptr) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    match attributes(e, &path) {
        Some(a) => a,
        None => {
            e.kernel.last_error = ERROR_FILE_NOT_FOUND;
            INVALID_FILE_ATTRIBUTES
        }
    }
}

// Platform: replaces KERNEL32.DLL CreateDirectoryA (import)
/// `CreateDirectoryA(name, security)`: false with `ERROR_ALREADY_EXISTS` when a file or
/// directory of that name exists ([`attributes`]); otherwise the directory is recorded as made
/// (in engine memory: nothing is made on the player's disk) and true.
pub fn create_directory_a(e: &mut Engine, name: Ptr, _security: u32) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    if attributes(e, &path).is_some() {
        e.kernel.last_error = ERROR_ALREADY_EXISTS;
        return 0;
    }
    let trimmed = key(path.strip_suffix(b"\\").unwrap_or(&path));
    e.kernel.disk.directories.insert(trimmed);
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL DeleteFileA (import)
/// `DeleteFileA(name)`: a file the game made goes; a file of the player's is from then on gone
/// for the game (recorded, never deleted on disk); true. False with `ERROR_FILE_NOT_FOUND` when
/// there is none, `ERROR_ACCESS_DENIED` for a directory.
pub fn delete_file_a(e: &mut Engine, name: Ptr) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    match attributes(e, &path) {
        None => {
            e.kernel.last_error = ERROR_FILE_NOT_FOUND;
            0
        }
        Some(a) if a & FILE_ATTRIBUTE_DIRECTORY != 0 => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            0
        }
        Some(_) => {
            let k = key(&path);
            e.kernel.disk.files.remove(&k);
            e.kernel.disk.deleted.insert(k);
            e.kernel.last_error = ERROR_SUCCESS;
            1
        }
    }
}

// Platform: replaces KERNEL32.DLL CopyFileA (import)
/// `CopyFileA(source, target, fail if exists)`: the source's bytes ([`read_whole`]) become the
/// target, a file of this run (engine memory, never on disk); false with `ERROR_FILE_NOT_FOUND`
/// without a source, `ERROR_FILE_EXISTS` when the target exists and `fail if exists` is set.
pub fn copy_file_a(e: &mut Engine, source: Ptr, target: Ptr, fail_if_exists: u32) -> u32 {
    let from = full_path(e, &e.mem.cstr(source.addr()));
    let to = full_path(e, &e.mem.cstr(target.addr()));
    let Some(data) = read_whole(e, &from) else {
        e.kernel.last_error = ERROR_FILE_NOT_FOUND;
        return 0;
    };
    if fail_if_exists != 0 && attributes(e, &to).is_some() {
        e.kernel.last_error = ERROR_FILE_EXISTS;
        return 0;
    }
    let k = key(&to);
    e.kernel.disk.deleted.remove(&k);
    e.kernel.disk.files.insert(k, data);
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces SHELL32.DLL SHGetFolderPathA (import)
/// `SHGetFolderPathA(owner, folder, token, flags, path)`: the player's folder for that CSIDL as
/// the player's table recorded it (`d3d9::folder_path`), copied into `path`, with its result. A
/// folder the table doesn't have, or `CSIDL_FLAG_CREATE`, is not provided.
pub fn sh_get_folder_path_a(
    e: &mut Engine,
    _owner: u32,
    folder: u32,
    _token: u32,
    flags: u32,
    path: Ptr,
) -> u32 {
    assert!(
        folder & 0x8000 == 0,
        "SHGetFolderPathA with CSIDL_FLAG_CREATE is not provided"
    );
    let Some((hr, text)) = super::d3d9::folder_path(e, folder) else {
        panic!(
            "SHGetFolderPathA({folder:#x}, flags {flags:#x}) was not captured in the adapter table: not provided"
        );
    };
    e.mem.set_cstr(path.addr(), &text);
    hr
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
    let name = full_path(e, &e.mem.cstr(path.addr()));
    let path = key(&name);
    // A file of the player's is read into the run's copy when it is opened.
    if !e.kernel.disk.files.contains_key(&path) && matches!(disposition, OPEN_EXISTING | OPEN_ALWAYS) {
        if let Some(data) = read_whole(e, &name) {
            e.kernel.disk.files.insert(path.clone(), data);
        }
    }
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

    #[test]
    fn directories_copies_and_deletes_stay_in_the_run() {
        let (mut e, name) = engine();
        let dir = e.mem.alloc(0x40);
        e.mem.set_cstr(dir, br"Q:\nv-rs-test-dir");
        assert_eq!(get_file_attributes_a(&mut e, Ptr::new(dir)), INVALID_FILE_ATTRIBUTES);
        assert_eq!(create_directory_a(&mut e, Ptr::new(dir), 0), 1);
        assert_eq!(get_file_attributes_a(&mut e, Ptr::new(dir)), FILE_ATTRIBUTE_DIRECTORY);
        assert_eq!(create_directory_a(&mut e, Ptr::new(dir), 0), 0);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
        let h = create_file_a(&mut e, Ptr::new(name), GENERIC_WRITE, 0, 0, CREATE_NEW, 0, 0);
        let buffer = e.mem.alloc(4);
        e.mem.write(buffer, b"ab");
        write_file(&mut e, h, Ptr::new(buffer), 2, Ptr::new(0), 0);
        let copy = e.mem.alloc(0x40);
        e.mem.set_cstr(copy, br"Q:\nv-rs-test-dir\copy.txt");
        assert_eq!(copy_file_a(&mut e, Ptr::new(name), Ptr::new(copy), 1), 1);
        assert_eq!(copy_file_a(&mut e, Ptr::new(name), Ptr::new(copy), 1), 0);
        assert_eq!(e.kernel.last_error, ERROR_FILE_EXISTS);
        assert_eq!(
            read_whole(&e, br"q:\NV-RS-TEST-DIR\copy.txt"),
            Some(b"ab".to_vec())
        );
        assert_eq!(delete_file_a(&mut e, Ptr::new(copy)), 1);
        assert_eq!(delete_file_a(&mut e, Ptr::new(copy)), 0);
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
        assert_eq!(delete_file_a(&mut e, Ptr::new(dir)), 0);
        assert_eq!(e.kernel.last_error, ERROR_ACCESS_DENIED);
    }

    #[test]
    fn relative_names_are_in_the_current_directory() {
        let mut e = Engine::new();
        e.kernel.current_directory = br"C:\Games\FNV".to_vec();
        assert_eq!(
            full_path(&e, b"Warnings.txt"),
            br"C:\Games\FNV\Warnings.txt".to_vec()
        );
        assert_eq!(full_path(&e, br"D:\x"), br"D:\x".to_vec());
        assert!(is_full_path(br"\\server\share"));
    }
}
