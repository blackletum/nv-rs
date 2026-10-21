//! `BSSystem/run_00b01510` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSWin32FileCache`'s vtable (its RTTI name).
const WIN32_FILE_CACHE_VTABLE: u32 = 0x010a_483c;
/// `memcpy` (`00ec44d0`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
/// The direct read of a file object (`00afb7a0`, `thiscall`, three words:
/// destination, size, and a third the code passes on).
const READ_DIRECT: u32 = 0x00af_b7a0;
/// The read buffer of the file object: its capacity (`+0x10`), the bytes
/// in it (`+0x14`), the read position in it (`+0x18`) and its storage
/// (`+0x20`).
const BUFFER_CAPACITY: u32 = 0x10;
const BUFFER_FILLED: u32 = 0x14;
const BUFFER_POSITION: u32 = 0x18;
const BUFFER_STORAGE: u32 = 0x20;

fn field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

// Translated from 00b01800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A buffered read of `size` bytes into `destination`: served from the
/// buffer when it holds enough. Otherwise the rest of the buffer is copied
/// first, the buffer emptied, and either the remainder read directly
/// (`00afb7a0`; more than the capacity) or the buffer refilled with up to
/// its capacity and the remainder (limited to what came in) copied from
/// it. Returns the bytes delivered.
pub fn fn_00b01800(e: &mut Engine, this: Ptr, destination: u32, size: u32) -> u32 {
    let mut destination = destination;
    let mut size = size;
    let mut done = 0u32;
    let available = field(e, this, BUFFER_FILLED).wrapping_sub(field(e, this, BUFFER_POSITION));
    if size > available {
        if available > 0 {
            let source =
                field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
            e.call(MEMCPY, &args![destination, source, available]);
            destination = destination.wrapping_add(available);
            size = size.wrapping_sub(available);
            done = available;
        }
        set_field(e, this, BUFFER_FILLED, 0);
        set_field(e, this, BUFFER_POSITION, 0);
        if size > field(e, this, BUFFER_CAPACITY) {
            let read = e
                .call(READ_DIRECT, &args![this, destination, size, done])
                .u32();
            return read.wrapping_add(done);
        }
        let storage = field(e, this, BUFFER_STORAGE);
        let capacity = field(e, this, BUFFER_CAPACITY);
        let filled = e
            .call(READ_DIRECT, &args![this, storage, capacity, done])
            .u32();
        set_field(e, this, BUFFER_FILLED, filled);
        if field(e, this, BUFFER_FILLED) < size {
            size = field(e, this, BUFFER_FILLED);
        }
    }
    let source = field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
    e.call(MEMCPY, &args![destination, source, size]);
    let position = field(e, this, BUFFER_POSITION).wrapping_add(size);
    set_field(e, this, BUFFER_POSITION, position);
    done.wrapping_add(size)
}

// Translated from 00b01bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 1 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01bc0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 2 == 0)
}

// Translated from 00b01be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 2 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01be0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 4 == 0)
}

// Translated from 00b01c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s constructor: the `BSFileCache` base (`00b00f20`),
/// then its vtable. Returns `this`.
pub fn fn_00b01c00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00b0_0f20, &args![this]);
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    this
}

// Translated from 00b01c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s destructor: its vtable, then the `BSFileCache`
/// destructor body (`00b01020`).
pub fn fn_00b01c20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    e.call(0x00b0_1020, &args![this]);
}

/// KERNEL32 import slots (`stdcall`).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const CREATE_THREAD: u32 = 0x00fd_f124;
const RESUME_THREAD: u32 = 0x00fd_f158;
const EXIT_THREAD: u32 = 0x00fd_f148;
const SLEEP: u32 = 0x00fd_f104;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INFINITE: u32 = 0xffff_ffff;
const CREATE_SUSPENDED: u32 = 4;

/// The two vtables: the base part's (`010a48ac`, `00b02720`) and the manager's (`010a4880`).
const BASE_VTABLE: u32 = 0x010a_48ac;
const MANAGER_VTABLE: u32 = 0x010a_4880;
/// The worker and starter thread starts.
const WORKER_START: u32 = 0x00b0_23f0;
const STARTER_START: u32 = 0x00b0_23b0;
/// `BSTaskletManager`'s two hook lists the worker runs around its loop (`00b00c90`, `00b00cf0`).
const WORKER_ENTER_HOOKS: u32 = 0x00b0_0c90;
const WORKER_LEAVE_HOOKS: u32 = 0x00b0_0cf0;
/// The manager's fields.
const STARTER: u32 = 0x14;
const THREAD_LIMIT: u32 = 0x18;
const THREAD_COUNT: u32 = 0x1c;
const THREAD_HANDLES: u32 = 0x20;
const THREAD_IDS: u32 = 0x38;
const STOPPING: u32 = 0x50;
const LOCK: u32 = 0x54;
const LISTS: u32 = 0x6c;
const TAILS: u32 = 0x16c;
const WORK: u32 = 0x26c;

/// A counted semaphore record as `00b02720` and `00b01d50` lay it out: the count at `+0`, the
/// handle at `+4`, the maximum at `+8`; `CreateSemaphoreA(0, count, maximum, 0)`.
fn create_counted_semaphore(e: &mut Engine, record: u32) {
    let maximum = e.mem.u32(record + 8);
    let count = e.mem.u32(record);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, count, maximum, 0u32])
        .u32();
    e.mem.set_u32(record + 4, handle);
}

/// `ReleaseSemaphore(handle, 1, 0)` on a counted record, then `InterlockedIncrement` on its count.
fn release_counted(e: &mut Engine, record: u32) {
    let handle = e.mem.u32(record + 4);
    e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![record]);
}

/// `WaitForSingleObject(handle, INFINITE)` on a counted record, then `InterlockedDecrement`.
fn wait_counted(e: &mut Engine, record: u32) {
    let handle = e.mem.u32(record + 4);
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, INFINITE]);
    e.call(INTERLOCKED_DECREMENT, &args![record]);
}

// Translated from 00b02720 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; the tasklet manager's base
/// constructor): the vtable `010a48ac`, the bytes `+4` and `+5` cleared, and the semaphore record
/// at `+8` (count 0, maximum 1, `CreateSemaphoreA`). Answers `this`.
pub fn fn_00b02720(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE);
    e.mem.set_u8(base + 4, 0);
    e.mem.set_u8(base + 5, 0);
    let record = base + 8;
    e.mem.set_u32(record, 0);
    e.mem.set_u32(record + 8, 1);
    create_counted_semaphore(e, record);
    this
}

// Translated from 00b01d50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`; the tasklet manager's
/// constructor, `QInstance` asks for 6): the base (`00b02720`), the vtable `010a4880`, no starter
/// (`+0x14`), the thread limit `+0x18` = `threads` capped at 6, no threads yet (`+0x1c`), the
/// stopping byte `+0x50` cleared, the critical section at `+0x54`, the work semaphore record at
/// `+0x26c` (count 0, maximum 0x7FFFFFFF), the 64 list heads at `+0x6c` and the six handle and
/// id words (`+0x20`, `+0x38`) cleared. Then up to the limit, one suspended worker thread each
/// (`CreateThread(0, 0, 00b023f0, this, CREATE_SUSPENDED, &id)`), stopping at the first that
/// fails, counted in `+0x1c`; the byte `+4` says whether any was made, and then the starter
/// thread (`00b023b0`, running) goes to `+0x14`. Answers `this`.
pub fn fn_00b01d50(e: &mut Engine, this: Ptr, threads: u32) -> Ptr {
    let base = this.addr();
    fn_00b02720(e, this);
    e.mem.set_u32(base, MANAGER_VTABLE);
    e.mem.set_u32(base + STARTER, 0);
    let limit = if threads > 6 { 6 } else { threads };
    e.mem.set_u32(base + THREAD_LIMIT, limit);
    e.mem.set_u32(base + THREAD_COUNT, 0);
    e.mem.set_u8(base + STOPPING, 0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![base + LOCK]);
    let work = base + WORK;
    e.mem.set_u32(work, 0);
    e.mem.set_u32(work + 8, 0x7fff_ffff);
    create_counted_semaphore(e, work);
    for i in 0..0x40u32 {
        e.mem.set_u32(base + LISTS + 4 * i, 0);
    }
    for i in 0..6u32 {
        e.mem.set_u32(base + THREAD_HANDLES + 4 * i, 0);
        e.mem.set_u32(base + THREAD_IDS + 4 * i, 0);
    }
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_LIMIT) {
        let id_out = base + THREAD_IDS + 4 * i;
        let handle = e
            .call(
                CREATE_THREAD,
                &args![0u32, 0u32, WORKER_START, base, CREATE_SUSPENDED, id_out],
            )
            .u32();
        e.mem.set_u32(base + THREAD_HANDLES + 4 * i, handle);
        if e.mem.u32(base + THREAD_HANDLES + 4 * i) == 0 {
            break;
        }
        let count = e.mem.u32(base + THREAD_COUNT);
        e.mem.set_u32(base + THREAD_COUNT, count.wrapping_add(1));
        i += 1;
    }
    let any = e.mem.u32(base + THREAD_COUNT) != 0;
    e.mem.set_u8(base + 4, u8::from(any));
    if e.mem.u8(base + 4) != 0 {
        e.with_stack(4, |e, id| {
            let handle = e
                .call(
                    CREATE_THREAD,
                    &args![0u32, 0u32, STARTER_START, base, 0u32, id],
                )
                .u32();
            e.mem.set_u32(base + STARTER, handle);
        });
    }
    this
}

// Translated from 00b023b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The starter thread's start (`stdcall`, one word: the manager; it never returns): waits on the
/// base semaphore record at `+8` (`WaitForSingleObject` INFINITE, `InterlockedDecrement`),
/// resumes the workers (`00b02420`) and ends with `ExitThread(0)`.
pub fn fn_00b023b0(e: &mut Engine, manager: u32) -> u32 {
    wait_counted(e, manager + 8);
    fn_00b02420(e, Ptr::new(manager));
    e.call(EXIT_THREAD, &args![0u32]);
    // `ExitThread` does not come back on a game thread.
    0
}

// Translated from 00b023f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// A worker thread's start (`stdcall`, one word: the manager; it never returns): the tasklet
/// manager's enter hooks (`00b00c90`), the work loop (`00b02460`), the leave hooks (`00b00cf0`),
/// then `ExitThread(0)`.
pub fn fn_00b023f0(e: &mut Engine, manager: u32) -> u32 {
    e.call(WORKER_ENTER_HOOKS, &args![manager]);
    fn_00b02460(e, Ptr::new(manager));
    e.call(WORKER_LEAVE_HOOKS, &args![manager]);
    e.call(EXIT_THREAD, &args![0u32]);
    // `ExitThread` does not come back on a game thread.
    0
}

// Translated from 00b02420 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`): `ResumeThread` on each of
/// the `+0x1c` worker handles at `+0x20`.
pub fn fn_00b02420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_COUNT) {
        let handle = e.mem.u32(base + THREAD_HANDLES + 4 * i);
        e.call(RESUME_THREAD, &args![handle]);
        i += 1;
    }
}

// Translated from 00b02460 (disassembled, FalloutNV.exe 1.4.0.525)
/// A worker's loop (`thiscall`, no stack words, plain `RET` once the stopping byte `+0x50` is
/// set): waits for work (the record at `+0x26c`); unless stopping, takes the first task of the
/// 64 lists under the lock (the list head moves to the task's `+0x14`; its group is the task's
/// `+8`). A task with a group whose byte `+0x2e` is clear is run: when its byte `+0xc` is set,
/// slot `+4` decides (else it runs), then `+0xc` is cleared and slot `+8` runs it. Then, with a
/// group: a task whose byte `+4` is set in a group still open goes back to its list
/// (`00b02130`); otherwise slot `+0xc` finishes it, `+8` and `+0x10` are cleared and its group
/// counts it done (`00b02670`). Each pass ends with `Sleep(0)`.
pub fn fn_00b02460(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    loop {
        if e.mem.u8(base + STOPPING) != 0 {
            return;
        }
        let mut task = 0u32;
        let mut group = 0u32;
        wait_counted(e, base + WORK);
        if e.mem.u8(base + STOPPING) == 0 {
            e.call(ENTER_CRITICAL_SECTION, &args![base + LOCK]);
            let mut slot = base + LISTS;
            let mut i = 0u32;
            while i < 0x40 {
                task = e.mem.u32(slot);
                if task != 0 {
                    let next = e.mem.u32(task + 0x14);
                    e.mem.set_u32(slot, next);
                    group = e.mem.u32(task + 8);
                    break;
                }
                i += 1;
                slot += 4;
            }
            e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK]);
            // The exe tests the task twice (`00b02524`, `00b02530`).
            let tested = [task != 0, group != 0, task != 0];
            if tested.iter().all(|&t| t) && e.mem.u8(group + 0x2e) == 0 {
                let go = if e.mem.u8(task + 0xc) == 0 {
                    true
                } else {
                    e.vcall(task, 4, &args![]).u8() != 0
                };
                if go {
                    e.mem.set_u8(task + 0xc, 0);
                    e.vcall(task, 8, &args![]);
                }
            }
            if task != 0 && group != 0 {
                if e.mem.u8(task + 4) != 0 && e.mem.u8(group + 0x2e) == 0 {
                    fn_00b02130(e, this, task);
                } else {
                    e.vcall(task, 0xc, &args![]);
                    e.mem.set_u32(task + 8, 0);
                    e.mem.set_u32(task + 0x10, 0);
                    fn_00b02670(e, Ptr::new(group));
                }
            }
        }
        e.call(SLEEP, &args![0u32]);
    }
}

// Translated from 00b02130 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`: the task): a task without a
/// group (`+8`) answers 0. Otherwise, under the lock, it goes to the end of the list its group's
/// priority picks (`+0x30`, capped at 0x3F; an empty list's tail is its head), its `+0x14`
/// cleared and its byte `+4` cleared; then the work semaphore is released once. Answers 1.
pub fn fn_00b02130(e: &mut Engine, this: Ptr, task: u32) -> u8 {
    let base = this.addr();
    if e.mem.u32(task + 8) == 0 {
        return 0;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![base + LOCK]);
    let group = e.mem.u32(task + 8);
    let mut list = e.mem.u32(group + 0x30);
    if list >= 0x40 {
        list = 0x3f;
    }
    if e.mem.u32(base + LISTS + 4 * list) == 0 {
        e.mem
            .set_u32(base + TAILS + 4 * list, base + LISTS + 4 * list);
    }
    let tail = e.mem.u32(base + TAILS + 4 * list);
    e.mem.set_u32(tail, task);
    e.mem.set_u32(base + TAILS + 4 * list, task + 0x14);
    e.mem.set_u32(task + 0x14, 0);
    e.mem.set_u8(task + 4, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK]);
    release_counted(e, base + WORK);
    1
}

// Translated from 00b02670 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; on a task group): under the
/// group's critical section (`+0x14`) counts one more task done (`+0x38`); when the group isn't
/// held open (byte `+0x2d`) and that reaches its task count (`+0x34`), its semaphore record at
/// `+8` is released once.
pub fn fn_00b02670(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![base + 0x14]);
    let done = e.mem.u32(base + 0x38).wrapping_add(1);
    e.mem.set_u32(base + 0x38, done);
    let finished = e.mem.u8(base + 0x2d) == 0 && e.mem.u32(base + 0x38) == e.mem.u32(base + 0x34);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + 0x14]);
    if finished {
        release_counted(e, base + 8);
    }
}

/// KERNEL32 import slots the stop uses (`stdcall`).
const WAIT_FOR_MULTIPLE_OBJECTS: u32 = 0x00fd_f108;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;

// Translated from 00b01f50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; slot +8 of the manager's
/// vtable `010a4880`, at `010a4888`): sets the stopping byte `+0x50`; when the byte `+5` is set,
/// releases the work semaphore record at `+0x26c` once per worker (`ReleaseSemaphore(handle, 1,
/// 0)`, `InterlockedIncrement` on its count; the count `+0x1c` re-read each pass) and waits for
/// all of them (`WaitForMultipleObjects(+0x1c, +0x20, TRUE, INFINITE)`). Then `CloseHandle` on
/// each worker handle and on the starter's (`+0x14`).
pub fn fn_00b01f50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + STOPPING, 1);
    if e.mem.u8(base + 5) != 0 {
        let mut i = 0u32;
        while i < e.mem.u32(base + THREAD_COUNT) {
            release_counted(e, base + WORK);
            i += 1;
        }
        let count = e.mem.u32(base + THREAD_COUNT);
        e.call(
            WAIT_FOR_MULTIPLE_OBJECTS,
            &args![count, base + THREAD_HANDLES, 1u32, INFINITE],
        );
    }
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_COUNT) {
        let handle = e.mem.u32(base + THREAD_HANDLES + 4 * i);
        e.call(CLOSE_HANDLE, &args![handle]);
        i += 1;
    }
    let starter = e.mem.u32(base + STARTER);
    e.call(CLOSE_HANDLE, &args![starter]);
}

/// The memory manager (`011f6238`) and its deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00b01c40 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x04 (`thiscall`, one word, `RET 4`): stores the byte `value` at
/// `+0x45`.
pub fn fn_00b01c40(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x45), value);
}

// Translated from 00b01c60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x08 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01c60(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01c70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x0C (`thiscall`, two words, `RET 8`): answers 0 (`XOR EAX,EAX`);
/// neither word is read.
pub fn fn_00b01c70(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00b01c80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x10 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01c80(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01c90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x14 (`thiscall`, one word, `RET 4`): answers 0 (`XOR EAX,EAX`);
/// the word (the path `00b011b0` passes) is not read.
pub fn fn_00b01c90(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00b01ca0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x18 (`thiscall`, two words, `RET 8`): does nothing.
pub fn fn_00b01ca0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) {}

// Translated from 00b01cb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x1C (`thiscall`, one word, `RET 4`): answers false (`XOR AL,AL`).
pub fn fn_00b01cb0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    false
}

// Translated from 00b01cc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x20 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01cc0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01cd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x24 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01cd0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01ce0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x28 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01ce0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01cf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x2C (`thiscall`, four words, `RET 0x10`): does nothing.
pub fn fn_00b01cf0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 00b01d00 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x30 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01d00(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01d10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x34 (`thiscall`, one word, `RET 4`): answers false (`XOR AL,AL`).
pub fn fn_00b01d10(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    false
}

// Translated from 00b01d20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x00 (`thiscall`, one word, `RET 4`): the scalar deleting
/// destructor. Runs the destructor (`fn_00b01c20`), then with bit 0 of `flags` frees `this`
/// through the memory manager (`00aa4060` on `011f6238`). Answers `this`.
pub fn fn_00b01d20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b01c20(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_1800, fn_00b01800(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1bc0, fn_00b01bc0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1be0, fn_00b01be0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1c00, fn_00b01c00(Ptr) -> Ptr),
        entry!(0x00b0_1c20, fn_00b01c20(Ptr)),
        entry!(0x00b0_1c40, fn_00b01c40(Ptr, u8)),
        entry!(0x00b0_1c60, fn_00b01c60(Ptr)),
        entry!(0x00b0_1c70, fn_00b01c70(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1c80, fn_00b01c80(Ptr, u32)),
        entry!(0x00b0_1c90, fn_00b01c90(Ptr, u32) -> u32),
        entry!(0x00b0_1ca0, fn_00b01ca0(Ptr, u32, u32)),
        entry!(0x00b0_1cb0, fn_00b01cb0(Ptr, u32) -> bool),
        entry!(0x00b0_1cc0, fn_00b01cc0(Ptr, u32)),
        entry!(0x00b0_1cd0, fn_00b01cd0(Ptr)),
        entry!(0x00b0_1ce0, fn_00b01ce0(Ptr, u32)),
        entry!(0x00b0_1cf0, fn_00b01cf0(Ptr, u32, u32, u32, u32)),
        entry!(0x00b0_1d00, fn_00b01d00(Ptr)),
        entry!(0x00b0_1d10, fn_00b01d10(Ptr, u32) -> bool),
        entry!(0x00b0_1d20, fn_00b01d20(Ptr, u32) -> Ptr),
        entry!(0x00b0_1d50, fn_00b01d50(Ptr, u32) -> Ptr),
        entry!(0x00b0_1f50, fn_00b01f50(Ptr)),
        entry!(0x00b0_2130, fn_00b02130(Ptr, u32) -> u8),
        entry!(0x00b0_23b0, fn_00b023b0(u32) -> u32),
        entry!(0x00b0_23f0, fn_00b023f0(u32) -> u32),
        entry!(0x00b0_2420, fn_00b02420(Ptr)),
        entry!(0x00b0_2460, fn_00b02460(Ptr)),
        entry!(0x00b0_2670, fn_00b02670(Ptr)),
        entry!(0x00b0_2720, fn_00b02720(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A file object with a 16-byte buffer holding `filled`
    /// bytes read up to `position`.
    fn file(e: &mut Engine, filled: u32, position: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let storage = e.mem.alloc(0x10);
        e.mem.set_u32(this + BUFFER_CAPACITY, 0x10);
        e.mem.set_u32(this + BUFFER_FILLED, filled);
        e.mem.set_u32(this + BUFFER_POSITION, position);
        e.mem.set_u32(this + BUFFER_STORAGE, storage);
        (this, storage)
    }

    #[test]
    fn reads_from_the_buffer_when_it_holds_enough() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        let (this, storage) = file(&mut e, 0x10, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 8u32]).u32(), 8);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 4, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 12);
    }

    #[test]
    fn large_reads_go_direct_and_small_ones_refill() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(READ_DIRECT, |_, a| (a[2] / 2).into_ret());
        let (this, storage) = file(&mut e, 0x10, 0xc);
        e.call_log = Some(vec![]);
        // 4 buffered bytes, then 0x20 more read directly (half arrive).
        assert_eq!(
            e.call(0x00b0_1800, &args![this, 0x9000u32, 0x24u32]).u32(),
            4 + 0x10
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 0xc, 4]]);
        assert_eq!(calls_to(&e, READ_DIRECT), vec![vec![this, 0x9004, 0x20, 4]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 0);
        // Empty buffer, 12 wanted: a refill brings 8, so 8 are delivered.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 12u32]).u32(), 8);
        assert_eq!(
            calls_to(&e, READ_DIRECT),
            vec![vec![this, storage, 0x10, 0]]
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 8);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 8);
    }

    #[test]
    fn flag_tests() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 2u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 4u32, 9u32]).u32(), 1);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 4u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 2u32, 9u32]).u32(), 1);
    }

    #[test]
    fn win32_file_cache_wraps_the_base() {
        let mut e = Engine::new();
        e.register(0x00b0_0f20, |_, a| a[0].into_ret());
        e.register(0x00b0_1020, |_, _| Ret::default());
        let cache = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1c00, &args![cache]).u32(), cache);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        e.mem.set_u32(cache, 0);
        e.call(0x00b0_1c20, &args![cache]);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        assert_eq!(calls_to(&e, 0x00b0_0f20), vec![vec![cache]]);
        assert_eq!(calls_to(&e, 0x00b0_1020), vec![vec![cache]]);
    }
}

#[cfg(test)]
mod tests_branch {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine whose KERNEL32 imports record their calls; `CreateThread` answers handles from
    /// 0x100 and `CreateSemaphoreA` 0x80.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        let log: Log = Rc::default();
        for f in [
            INITIALIZE_CRITICAL_SECTION,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            RELEASE_SEMAPHORE,
            RESUME_THREAD,
            EXIT_THREAD,
            WAIT_FOR_SINGLE_OBJECT,
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            CREATE_SEMAPHORE_A,
            CREATE_THREAD,
            SLEEP,
            WORKER_ENTER_HOOKS,
            WORKER_LEAVE_HOOKS,
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                let mut log = log.borrow_mut();
                let n = log.iter().filter(|(c, _)| *c == f).count() as u32;
                log.push((f, a.to_vec()));
                match f {
                    CREATE_THREAD => (0x100 + n).into_ret(),
                    CREATE_SEMAPHORE_A => 0x80u32.into_ret(),
                    _ => Ret::default(),
                }
            });
        }
        (e, log)
    }

    fn calls(log: &Log, f: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(c, _)| *c == f)
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// Six suspended workers (a request of 9 is capped), then the running starter; the
    /// semaphores have counts 0 and maxima 1 and 0x7FFFFFFF.
    #[test]
    fn the_constructor_makes_capped_suspended_workers_and_a_starter() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        assert_eq!(e.call(0x00b0_1d50, &args![m, 9u32]).u32(), m);
        assert_eq!(e.mem.u32(m), MANAGER_VTABLE);
        assert_eq!(e.mem.u32(m + THREAD_LIMIT), 6);
        assert_eq!(e.mem.u32(m + THREAD_COUNT), 6);
        assert_eq!(e.mem.u8(m + 4), 1);
        assert_eq!(
            calls(&log, CREATE_SEMAPHORE_A),
            [vec![0, 0, 1, 0], vec![0, 0, 0x7fff_ffff, 0]]
        );
        let threads = calls(&log, CREATE_THREAD);
        assert_eq!(threads.len(), 7);
        for (i, t) in threads[..6].iter().enumerate() {
            assert_eq!(
                t,
                &vec![
                    0,
                    0,
                    WORKER_START,
                    m,
                    CREATE_SUSPENDED,
                    m + THREAD_IDS + 4 * i as u32
                ]
            );
            assert_eq!(
                e.mem.u32(m + THREAD_HANDLES + 4 * i as u32),
                0x100 + i as u32
            );
        }
        assert_eq!(&threads[6][..5], &[0, 0, STARTER_START, m, 0]);
        assert_eq!(e.mem.u32(m + STARTER), 0x106);
        assert_eq!(e.mem.u32(m + WORK + 4), 0x80);
    }

    /// The starter waits on its semaphore, resumes every worker and exits; a worker runs the
    /// hooks around its loop (stopped at once here) and exits.
    #[test]
    fn the_thread_starts_resume_and_run_the_loop() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u32(m + THREAD_COUNT, 2);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + THREAD_HANDLES + 4, 0x204);
        e.mem.set_u32(m + 0xc, 0x90);
        e.call(0x00b0_23b0, &args![m]);
        assert_eq!(calls(&log, WAIT_FOR_SINGLE_OBJECT), [vec![0x90, INFINITE]]);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT), [vec![m + 8]]);
        assert_eq!(calls(&log, RESUME_THREAD), [vec![0x200], vec![0x204]]);
        assert_eq!(calls(&log, EXIT_THREAD), [vec![0]]);
        e.mem.set_u8(m + STOPPING, 1);
        e.call(0x00b0_23f0, &args![m]);
        assert_eq!(calls(&log, WORKER_ENTER_HOOKS), [vec![m]]);
        assert_eq!(calls(&log, WORKER_LEAVE_HOOKS), [vec![m]]);
        assert_eq!(calls(&log, EXIT_THREAD).len(), 2);
    }

    /// One pass of the loop: the first task of list 2 runs (no decide byte), is finished and
    /// counted on its group; the next pass sees the stopping byte (set by `Sleep`'s double).
    #[test]
    fn a_worker_runs_the_first_task_and_counts_it_done() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        let (task, group, vtable) = (0x1400u32, 0x1500u32, 0x1600u32);
        e.mem.set_u32(m + LISTS + 8, task);
        e.mem.set_u32(task, vtable);
        e.mem.set_u32(task + 8, group);
        e.mem.set_u32(task + 0x14, 0x1450);
        e.mem.set_u32(group + 0x34, 1);
        e.mem.set_u32(group + 0xc, 0x91);
        e.put_vtable(vtable, &[0, 0x00ab_0004, 0x00ab_0008, 0x00ab_000c]);
        for f in [0x00ab_0004u32, 0x00ab_0008, 0x00ab_000c] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                Ret::default()
            });
        }
        e.register(SLEEP, move |e, _| {
            e.mem.set_u8(0x1000 + STOPPING, 1);
            Ret::default()
        });
        e.call(0x00b0_2460, &args![m]);
        assert_eq!(e.mem.u32(m + LISTS + 8), 0x1450, "the list moves on");
        assert!(calls(&log, 0x00ab_0004).is_empty(), "no decide byte");
        assert_eq!(calls(&log, 0x00ab_0008), [vec![task]]);
        assert_eq!(calls(&log, 0x00ab_000c), [vec![task]]);
        assert_eq!((e.mem.u32(task + 8), e.mem.u32(task + 0x10)), (0, 0));
        assert_eq!(e.mem.u32(group + 0x38), 1);
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), [vec![0x91, 1, 0]]);
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT), [vec![group + 8]]);
    }

    /// Requeueing: a task goes to the end of its group's list (capped at 0x3F) and the work
    /// semaphore is released; a task without a group answers 0.
    #[test]
    fn a_task_goes_back_to_the_end_of_its_list() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        let (task, group) = (0x1400u32, 0x1500u32);
        e.mem.set_u32(m + WORK + 4, 0x92);
        assert_eq!(e.call(0x00b0_2130, &args![m, task]).u32() & 0xff, 0);
        e.mem.set_u32(task + 8, group);
        e.mem.set_u32(group + 0x30, 0x55);
        e.mem.set_u8(task + 4, 1);
        e.mem.set_u32(task + 0x14, 0xdead);
        assert_eq!(e.call(0x00b0_2130, &args![m, task]).u32() & 0xff, 1);
        assert_eq!(e.mem.u32(m + LISTS + 4 * 0x3f), task);
        assert_eq!(e.mem.u32(m + TAILS + 4 * 0x3f), task + 0x14);
        assert_eq!((e.mem.u32(task + 0x14), e.mem.u8(task + 4)), (0, 0));
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), [vec![0x92, 1, 0]]);
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT), [vec![m + WORK]]);
    }
}

#[cfg(test)]
mod tests_stop {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        let log: Log = Rc::default();
        for f in [
            RELEASE_SEMAPHORE,
            INTERLOCKED_INCREMENT,
            WAIT_FOR_MULTIPLE_OBJECTS,
            CLOSE_HANDLE,
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    fn calls(log: &Log, f: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(c, _)| *c == f)
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// Two workers with the byte `+5` set: the work semaphore is released twice, all are waited
    /// for, then both worker handles and the starter's are closed.
    #[test]
    fn the_stop_wakes_waits_for_and_closes_the_workers() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u8(m + 5, 1);
        e.mem.set_u32(m + THREAD_COUNT, 2);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + THREAD_HANDLES + 4, 0x204);
        e.mem.set_u32(m + STARTER, 0x300);
        e.mem.set_u32(m + WORK + 4, 0x80);
        e.call(0x00b0_1f50, &args![m]);
        assert_eq!(e.mem.u8(m + STOPPING), 1);
        assert_eq!(
            calls(&log, RELEASE_SEMAPHORE),
            [vec![0x80, 1, 0], vec![0x80, 1, 0]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_INCREMENT),
            [vec![m + WORK], vec![m + WORK]]
        );
        assert_eq!(
            calls(&log, WAIT_FOR_MULTIPLE_OBJECTS),
            [vec![2, m + THREAD_HANDLES, 1, INFINITE]]
        );
        assert_eq!(
            calls(&log, CLOSE_HANDLE),
            [vec![0x200], vec![0x204], vec![0x300]]
        );
    }

    /// With the byte `+5` clear nothing is released or waited for (the byte `+4` is not the
    /// one tested); the handles are still closed.
    #[test]
    fn the_stop_without_the_byte_only_closes_the_handles() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u8(m + 4, 1);
        e.mem.set_u32(m + THREAD_COUNT, 1);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + STARTER, 0x300);
        e.call(0x00b0_1f50, &args![m]);
        assert_eq!(e.mem.u8(m + STOPPING), 1);
        assert!(calls(&log, RELEASE_SEMAPHORE).is_empty());
        assert!(calls(&log, WAIT_FOR_MULTIPLE_OBJECTS).is_empty());
        assert_eq!(calls(&log, CLOSE_HANDLE), [vec![0x200], vec![0x300]]);
    }
}

#[cfg(test)]
mod tests_file_cache_slots {
    use super::*;

    #[test]
    fn the_lookup_slot_answers_zero_and_the_flag_slots_false() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00b0_1c90, &args![this, 0x1234u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1c70, &args![this, 1u32, 2u32]).u32(), 0);
        assert!(!e.call(0x00b0_1cb0, &args![this, 1u32]).bool());
        assert!(!e.call(0x00b0_1d10, &args![this, 1u32]).bool());
        e.call(0x00b0_1c40, &args![this, 0x5au32]);
        assert_eq!(e.mem.u8(this + 0x45), 0x5a);
    }

    #[test]
    fn the_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(0x00b0_1020, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1d20, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), WIN32_FILE_CACHE_VTABLE);
        e.call(0x00b0_1d20, &args![this, 1u32]);
        let frees: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(frees, [vec![MEMORY_MANAGER, this]]);
    }
}
