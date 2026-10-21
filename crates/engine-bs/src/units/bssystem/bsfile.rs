//! `BSSystem/bsfile.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC `BSFile` vtable (`010a4764`, RTTI `.?AVBSFile@@`) has the Xbox
//! PDB's slots in the PDB's order without `GetDevice`: +0x10
//! `SetEndianSwap` (`00aff990`), +0x1c `GetFileSize` (`00aff610`), +0x20
//! `Open` (`00aff300`), +0x28 `GetSize`, +0x34 `GetLine` (`00affc90`), +0x40
//! `Exist` (`00affc60`), +0x44 `ReadF`, +0x48 `WriteF` (`00aff950`). Slot
//! names other than the engine map's are read from that order and are not
//! otherwise confirmed.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nisystem::run_00aa1380::NiFile;

layout! {
    /// `BSFile` (Xbox PDB), 0x158 bytes on PC: an `NiFile` (0x30 bytes on
    /// PC, its fields through [`NiFile`]) and its own fields, 0x44 lower
    /// than the PDB's from `pFileName` on.
    pub struct BSFile: 0x158 {
        /// `pFileName` (Xbox PDB, +0x88 there): `char[260]`.
        0x044 pFileName: u8,
        /// `iTrueFilePos` (Xbox PDB, +0x194 there).
        0x150 iTrueFilePos: u32,
        /// `iFileSize` (Xbox PDB, +0x198 there).
        0x154 iFileSize: u32,
    }
}

/// The `BSFile` vtable (RTTI `.?AVBSFile@@`).
const BSFILE_VTABLE: u32 = 0x010a_4764;
/// `BSFile` vtable slots (byte offsets, see the module comment).
const GET_FILE_SIZE_SLOT: u32 = 0x1c;
const GET_SIZE_SLOT: u32 = 0x28;
const READ_F_SLOT: u32 = 0x44;
const WRITE_F_SLOT: u32 = 0x48;
/// The read and write functions `00aff990` installs in `m_pfnRead` (+8) and
/// `m_pfnWrite` (+0xc): the plain pair, and the byte-swapping pair
/// (`00affa10`, `00affa60`, not translated here).
const PLAIN_READ: u32 = 0x00af_f9d0;
const PLAIN_WRITE: u32 = 0x00af_f9f0;
const SWAPPING_READ: u32 = 0x00af_fa10;
const SWAPPING_WRITE: u32 = 0x00af_fa60;
/// `_access(path, mode)` (`00ecbf05`, cdecl), -1 on failure.
const ACCESS: u32 = 0x00ec_bf05;
/// `fopen_s(&file, name, mode)` (`00ec9a5e`, cdecl).
const FOPEN_S: u32 = 0x00ec_9a5e;
/// `fclose` (`00ec9907`, cdecl).
const FCLOSE: u32 = 0x00ec_9907;
/// The `NiFile` functions `BSFile` calls (`NiSystem/run_00aa1380`): the
/// buffer flush (thiscall), the direct read (thiscall: buffer, count), the
/// buffered write (thiscall: buffer, count) and the destructor body.
const NI_FILE_FLUSH: u32 = 0x00aa_15a0;
const NI_FILE_READ_DIRECT: u32 = 0x00aa_1570;
const NI_FILE_WRITE: u32 = 0x00aa_1810;
const NI_FILE_DESTRUCTOR: u32 = 0x00aa_1690;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, thiscall:
/// size) and deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The thread block's word at +0x2b4 (the allocation context the memory
/// manager reads); `CheckIsGood` sets it to 0x17 around its allocation.
const ALLOCATION_CONTEXT: u32 = 0x2b4;
const FILE_BUFFER_CONTEXT: u32 = 0x17;
/// `fopen_s` mode strings in `.rdata`.
const MODE_RT: u32 = 0x010a_46cc;
const MODE_RB: u32 = 0x010a_46d0;
const MODE_R_PLUS_T: u32 = 0x010a_46d4;
const MODE_R_PLUS_B: u32 = 0x010a_46d8;
const MODE_A_PLUS_T: u32 = 0x010a_46dc;
const MODE_A_PLUS_B: u32 = 0x010a_46e0;
const MODE_WT: u32 = 0x010a_46e4;
const MODE_WB: u32 = 0x010a_46e8;

fn ni_file(this: Ptr<BSFile>) -> Ptr<NiFile> {
    this.cast()
}

// Translated from 00aff990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x10 (`SetEndianSwap` by the
/// PDB's slot order): with `swap` set installs `00affa10` / `00affa60` as
/// `m_pfnRead` / `m_pfnWrite`, otherwise `00aff9d0` / `00aff9f0`.
pub fn fn_00aff990(e: &mut Engine, this: Ptr<BSFile>, swap: bool) {
    let (read, write) = if swap {
        (SWAPPING_READ, SWAPPING_WRITE)
    } else {
        (PLAIN_READ, PLAIN_WRITE)
    };
    e.mem.set_u32(this.addr() + 8, read);
    e.mem.set_u32(this.addr() + 0xc, write);
}

// Translated from 00aff9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the plain `m_pfnRead`:
/// calls `stream`'s vtable +0x44 (`ReadF`) with `(buffer, size)` and passes
/// its answer on (`EAX` is left as the call returns it).
pub fn fn_00aff9d0(e: &mut Engine, stream: Ptr, buffer: Ptr, size: u32) -> u32 {
    e.vcall(stream.addr(), READ_F_SLOT, &args![buffer, size])
        .u32()
}

// Translated from 00aff9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the plain `m_pfnWrite`:
/// calls `stream`'s vtable +0x48 (`WriteF`) with `(buffer, size)` and
/// passes its answer on.
pub fn fn_00aff9f0(e: &mut Engine, stream: Ptr, buffer: Ptr, size: u32) -> u32 {
    e.vcall(stream.addr(), WRITE_F_SLOT, &args![buffer, size])
        .u32()
}

// Translated from 00aff610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x1c (`GetFileSize` by the
/// PDB's slot order): when `iFileSize` is 0 calls vtable +0x28 (which is
/// expected to fill it; its answer is not used), then answers `iFileSize`.
pub fn fn_00aff610(e: &mut Engine, this: Ptr<BSFile>) -> u32 {
    if e.get(this, BSFile::iFileSize) == 0 {
        e.vcall(this.addr(), GET_SIZE_SLOT, &args![]);
    }
    e.get(this, BSFile::iFileSize)
}

// Translated from 00aff950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x48 (`WriteF` by the PDB's
/// slot order): the buffered `NiFile` write `00aa1810(buffer, count)`, whose
/// answer is added to `iTrueFilePos` and returned.
pub fn fn_00aff950(e: &mut Engine, this: Ptr<BSFile>, buffer: Ptr, count: u32) -> u32 {
    let written = e.call(NI_FILE_WRITE, &args![this, buffer, count]).u32();
    let position = e.get(this, BSFile::iTrueFilePos);
    e.set(this, BSFile::iTrueFilePos, position.wrapping_add(written));
    written
}

// Translated from 00affc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Exist` (Xbox PDB): whether `_access(pFileName, 0)` succeeds.
pub fn bs_file_exist(e: &mut Engine, this: Ptr<BSFile>) -> bool {
    let name = this.addr() + BSFile::pFileName.off;
    e.call(ACCESS, &args![name, 0u32]).i32() != -1
}

// Translated from 00affc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x34 (`GetLine` by the PDB's
/// slot order): reads one byte at a time through vtable +0x44 (`ReadF`)
/// into `buffer`, at most `size - 1` of them, stopping at a short read or
/// at the byte `terminator` (compared as a sign-extended byte against the
/// zero-extended 16-bit `terminator`; neither is stored). The buffer is
/// then terminated; the answer is the sum of the reads' answers.
pub fn fn_00affc90(
    e: &mut Engine,
    this: Ptr<BSFile>,
    buffer: Ptr,
    size: u32,
    terminator: u16,
) -> u32 {
    let mut total = 0u32;
    let mut index = 0u32;
    e.with_stack(4, |e, byte| {
        while index.wrapping_add(1) < size {
            let read = e.vcall(this.addr(), READ_F_SLOT, &args![byte, 1u32]).u32();
            total = total.wrapping_add(read);
            let value = e.mem.u8(byte.addr());
            if read != 1 || i32::from(value as i8) == i32::from(terminator) {
                break;
            }
            e.mem.set_u8(buffer.addr().wrapping_add(index), value);
            index += 1;
        }
    });
    e.mem.set_u8(buffer.addr().wrapping_add(index), 0);
    total
}

// Translated from 00affd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Close` (Xbox PDB): when the file is good and open, flushes it
/// (`00aa15a0`) and closes it (`fclose`); then frees the buffer through the
/// memory manager and clears `m_pBuffer` and `m_pFile`.
pub fn bs_file_close(e: &mut Engine, this: Ptr<BSFile>) {
    let file = ni_file(this);
    if e.get(file, NiFile::m_bGood) != 0 && e.get(file, NiFile::m_pFile) != 0 {
        e.call(NI_FILE_FLUSH, &args![this]);
        let handle = e.get(file, NiFile::m_pFile);
        e.call(FCLOSE, &args![handle]);
    }
    let buffer = e.get(file, NiFile::m_pBuffer);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    e.set(file, NiFile::m_pBuffer, 0);
    e.set(file, NiFile::m_pFile, 0);
}

// Translated from 00aff240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aff240 - the FS:[0] exception frame (the unwind state around Close) is not translated
/// `BSFile::~BSFile` (Xbox PDB): installs the `BSFile` vtable, closes the
/// file ([`bs_file_close`]) and runs the `NiFile` destructor body
/// (`00aa1690`).
pub fn bs_file_destructor(e: &mut Engine, this: Ptr<BSFile>) {
    e.mem.set_u32(this.addr(), BSFILE_VTABLE);
    bs_file_close(e, this);
    e.call(NI_FILE_DESTRUCTOR, &args![this]);
}

// Translated from 00afff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0: the scalar deleting
/// destructor. Runs [`bs_file_destructor`], then with bit 0 of `flags`
/// frees `this` through the memory manager. Returns `this`.
pub fn fn_00afff20(e: &mut Engine, this: Ptr<BSFile>, flags: u32) -> Ptr<BSFile> {
    bs_file_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aff490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aff490 - the FS:[0] exception frame (the unwind state around the allocation) is not translated
/// `BSFile::CheckIsGood` (Xbox PDB): `m_bGood` is whether a `FILE` is open.
/// An open file with a nonzero `m_uiBufferAllocSize` and no buffer gets
/// one, allocated with the thread's allocation context (TLS +0x2b4) set to
/// 0x17 and restored after. A buffer size of -1 means "the whole file":
/// the size is taken from vtable +0x1c (`GetFileSize`), the whole file is
/// read into the buffer (`00aa1570`, read size = size, position 0), and a
/// short read clears `m_bGood`.
pub fn bs_file_check_is_good(e: &mut Engine, this: Ptr<BSFile>) {
    let file = ni_file(this);
    if e.get(file, NiFile::m_pFile) == 0 {
        e.set(file, NiFile::m_bGood, 0);
        return;
    }
    e.set(file, NiFile::m_bGood, 1);
    if e.get(file, NiFile::m_uiBufferAllocSize) == 0 || e.get(file, NiFile::m_pBuffer) != 0 {
        return;
    }
    let context = e.tls() + ALLOCATION_CONTEXT;
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, FILE_BUFFER_CONTEXT);
    let whole = e.get(file, NiFile::m_uiBufferAllocSize) == u32::MAX;
    if whole {
        let size = e.vcall(this.addr(), GET_FILE_SIZE_SLOT, &args![]).u32();
        e.set(file, NiFile::m_uiBufferAllocSize, size);
    }
    let size = e.get(file, NiFile::m_uiBufferAllocSize);
    let buffer = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    e.set(file, NiFile::m_pBuffer, buffer);
    if whole {
        let size = e.get(file, NiFile::m_uiBufferAllocSize);
        e.set(file, NiFile::m_uiBufferReadSize, size);
        e.set(file, NiFile::m_uiPos, 0);
        let buffer = e.get(file, NiFile::m_pBuffer);
        let size = e.get(file, NiFile::m_uiBufferAllocSize);
        let read = e
            .call(NI_FILE_READ_DIRECT, &args![this, buffer, size])
            .u32();
        if read != e.get(file, NiFile::m_uiBufferAllocSize) {
            e.set(file, NiFile::m_bGood, 0);
        }
    }
    let context = e.tls() + ALLOCATION_CONTEXT;
    e.mem.set_u32(context, saved);
}

// Translated from 00aff300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Open` (Xbox PDB; vtable +0x20, two stack words, the first
/// unused): answers true at once when a `FILE` is already open. Otherwise
/// clears `m_uiPos`, `m_uiBufferReadSize` and `m_pBuffer` and opens
/// `pFileName` with `fopen_s` in a mode chosen by `m_eMode` (0 `"rb"`, 1
/// `"r+b"`, anything else `"a+b"`; the `t` forms when `text` is set). Mode 1
/// on a file that does not exist creates it first (`"wb"`/`"wt"`, closed at
/// once) and opens it again. Ends with [`bs_file_check_is_good`] and
/// answers `m_bGood`.
pub fn bs_file_open(e: &mut Engine, this: Ptr<BSFile>, _unused_1: u32, text: bool) -> bool {
    let file = ni_file(this);
    if e.get(file, NiFile::m_pFile) != 0 {
        return true;
    }
    e.set(file, NiFile::m_uiPos, 0);
    e.set(file, NiFile::m_uiBufferReadSize, 0);
    e.set(file, NiFile::m_pBuffer, 0);
    let mode = match (e.get(file, NiFile::m_eMode), text) {
        (0, true) => MODE_RT,
        (0, false) => MODE_RB,
        (1, true) => MODE_R_PLUS_T,
        (1, false) => MODE_R_PLUS_B,
        (_, true) => MODE_A_PLUS_T,
        (_, false) => MODE_A_PLUS_B,
    };
    let slot = this.addr() + NiFile::m_pFile.off;
    let name = this.addr() + BSFile::pFileName.off;
    e.set(file, NiFile::m_pFile, 0);
    e.call(FOPEN_S, &args![slot, name, mode]);
    if e.get(file, NiFile::m_pFile) == 0 && e.get(file, NiFile::m_eMode) == 1 {
        e.set(file, NiFile::m_pFile, 0);
        let create = if text { MODE_WT } else { MODE_WB };
        e.call(FOPEN_S, &args![slot, name, create]);
        if e.get(file, NiFile::m_pFile) != 0 {
            let handle = e.get(file, NiFile::m_pFile);
            e.call(FCLOSE, &args![handle]);
        }
        e.set(file, NiFile::m_pFile, 0);
        e.call(FOPEN_S, &args![slot, name, mode]);
    }
    bs_file_check_is_good(e, this);
    e.get(file, NiFile::m_bGood) != 0
}

/// `NiFile`'s base constructor (`00aa14b0`, `thiscall`), `strlen` (`00ec6130`), `strcpy_s`
/// (`00ec65a6`), `BSFile::Open` (`00aff300`, `thiscall`, two words) and `BSFile::Exist`
/// (`00affc60`, `thiscall`, answers `AL`).
const NI_FILE_CONSTRUCT: u32 = 0x00aa_14b0;
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const BS_FILE_OPEN: u32 = 0x00af_f300;
const BS_FILE_EXIST: u32 = 0x00af_fc60;

// Translated from 00b00260 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b00260 - the SEH frame (FS:[0] handler 00f23868, state 0 after the base constructor) is not translated.
/// `BSFile::BSFile` (engine map; unit `BSSystem/bsfile_xenon.cpp`; `thiscall`, four words,
/// `RET 0x10`: path, mode, buffer size, a byte for `Open`): `NiFile`'s constructor, the vtable
/// `010a4764`, no byte swapping (`00aff990(0)`), the mode at +0x28 and the buffer size at +0x10;
/// +0x14, +0x18, +0x20, +0x24, +0x150, +0x14c, +0x148, +0x154, +0x34, +0x3c and +0x40 cleared,
/// the byte +0x30 cleared and +0x38 set to -1. A path of 0x104 characters or more leaves the name
/// at +0x44 empty; a shorter one is copied there (`strcpy_s` into 0x104 bytes) and, for mode 1,
/// the file is opened (`BSFile::Open(0, byte)`). For mode 0 the byte +0x2c is set to whether
/// the file exists (`BSFile::Exist`). Answers `this`.
pub fn bs_file_bs_file(
    e: &mut Engine,
    this: Ptr<BSFile>,
    path: Ptr,
    mode: u32,
    buffer_size: u32,
    open_byte: u8,
) -> Ptr<BSFile> {
    let t = this.addr();
    e.call(NI_FILE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, BSFILE_VTABLE);
    e.call(0x00af_f990, &args![t, 0u32]);
    e.mem.set_u32(t + 0x28, mode);
    e.mem.set_u32(t + 0x10, buffer_size);
    for offset in [0x14u32, 0x18, 0x20, 0x24, 0x150, 0x14c, 0x148, 0x154] {
        e.mem.set_u32(t + offset, 0);
    }
    e.mem.set_u8(t + 0x30, 0);
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0xffff_ffff);
    e.mem.set_u32(t + 0x3c, 0);
    e.mem.set_u32(t + 0x40, 0);
    let length = e.call(STRLEN, &args![path]).u32();
    if length >= 0x104 {
        e.mem.set_u8(t + 0x44, 0);
    } else {
        e.call(STRCPY_S, &args![t + 0x44, 0x104u32, path]);
        if mode == 1 {
            e.call(BS_FILE_OPEN, &args![t, 0u32, u32::from(open_byte)]);
        }
    }
    if mode == 0 {
        let exists = e.call(BS_FILE_EXIST, &args![t]).u8();
        e.mem.set_u8(t + 0x2c, exists);
    }
    this
}

/// `BSFile` vtable +0x20 (`Open`, thiscall: two words).
const OPEN_SLOT: u32 = 0x20;
/// The `NiFile` read (`00aa1750`, thiscall: buffer, count) and seek (`00aa16d0`, thiscall:
/// offset, origin).
const NI_FILE_READ: u32 = 0x00aa_1750;
const NI_FILE_SEEK: u32 = 0x00aa_16d0;
/// `ftell` (`00ecb562`, cdecl: stream) and `fseek` (`00ecb65a`, cdecl: stream, offset, origin).
const FTELL: u32 = 0x00ec_b562;
const FSEEK: u32 = 0x00ec_b65a;
/// The three seek origins as `NiFile` keeps them (`010a2480`, `010a2484`, `010a2488`: the
/// set, current and end words `00b00590` compares with).
const SEEK_SET_WORD: u32 = 0x010a_2480;
const SEEK_CUR_WORD: u32 = 0x010a_2484;
const SEEK_END_WORD: u32 = 0x010a_2488;

// Translated from 00b00650 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSFile::ReadF` (Xbox PDB; `BSFile` vtable +0x44, `thiscall`, two words, `RET 8`): opens the
/// file first when it has no `FILE` (`m_pFile` +0x24; vtable +0x20 with `(0, 0)`), reads through
/// the `NiFile` buffer (`00aa1750(buffer, size)`) and moves `iTrueFilePos` (+0x150) on by what
/// that answered, which it answers.
pub fn bs_file_read_f(e: &mut Engine, this: Ptr<BSFile>, buffer: Ptr, size: u32) -> u32 {
    if e.get(ni_file(this), NiFile::m_pFile) == 0 {
        e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    }
    let read = e.call(NI_FILE_READ, &args![this, buffer, size]).u32();
    let position = e.get(this, BSFile::iTrueFilePos).wrapping_add(read);
    e.set(this, BSFile::iTrueFilePos, position);
    read
}

// Translated from 00b00590 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x14 (`thiscall`, two words, `RET 8`: offset,
/// origin): the seek. The target is the offset for the set origin (`010a2480`; a file in mode
/// 0, read, seeks by the distance from `iTrueFilePos` with the current origin `010a2484`),
/// `iTrueFilePos` plus the offset for the current origin, the size (vtable +0x28) less the
/// offset for the end origin (`010a2488`), else `iTrueFilePos`. When the target differs from
/// `iTrueFilePos` the `NiFile` seek runs (`00aa16d0(offset, origin)`, both as adjusted) and
/// `iTrueFilePos` becomes the target.
pub fn fn_00b00590(e: &mut Engine, this: Ptr<BSFile>, offset: u32, origin: u32) {
    let mut offset = offset;
    let mut origin = origin;
    let target = if origin == e.global::<u32>(SEEK_SET_WORD) {
        let target = offset;
        if e.get(ni_file(this), NiFile::m_eMode) == 0 {
            origin = e.global::<u32>(SEEK_CUR_WORD);
            offset = offset.wrapping_sub(e.get(this, BSFile::iTrueFilePos));
        }
        target
    } else if origin == e.global::<u32>(SEEK_CUR_WORD) {
        e.get(this, BSFile::iTrueFilePos).wrapping_add(offset)
    } else if origin == e.global::<u32>(SEEK_END_WORD) {
        e.vcall(this.addr(), GET_SIZE_SLOT, &args![])
            .u32()
            .wrapping_sub(offset)
    } else {
        e.get(this, BSFile::iTrueFilePos)
    };
    if target != e.get(this, BSFile::iTrueFilePos) {
        e.call(NI_FILE_SEEK, &args![this, offset, origin]);
        e.set(this, BSFile::iTrueFilePos, target);
    }
}

// Translated from 00b004e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x28 (`GetSize` by the PDB's slot order;
/// `thiscall`, no stack words, plain `RET`): clears `iFileSize` (+0x154), flushes the `NiFile`
/// buffer of a file in mode 1 (`00aa15a0`), opens the file (vtable +0x20 with `(0, 0)`), and
/// with a `FILE` measures it: `ftell`, `fseek(0, SEEK_END)`, `ftell` into `iFileSize`, and
/// `fseek` back to the first position. Answers `iFileSize`.
pub fn fn_00b004e0(e: &mut Engine, this: Ptr<BSFile>) -> u32 {
    e.set(this, BSFile::iFileSize, 0);
    if e.get(ni_file(this), NiFile::m_eMode) == 1 {
        e.call(NI_FILE_FLUSH, &args![this]);
    }
    e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    if e.get(ni_file(this), NiFile::m_pFile) != 0 {
        let file = e.get(ni_file(this), NiFile::m_pFile);
        let here = e.call(FTELL, &args![file]).u32();
        let file = e.get(ni_file(this), NiFile::m_pFile);
        e.call(FSEEK, &args![file, 0u32, 2u32]);
        let file = e.get(ni_file(this), NiFile::m_pFile);
        let size = e.call(FTELL, &args![file]).u32();
        e.set(this, BSFile::iFileSize, size);
        let file = e.get(ni_file(this), NiFile::m_pFile);
        e.call(FSEEK, &args![file, here, 0u32]);
    }
    e.get(this, BSFile::iFileSize)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0650, bs_file_read_f(Ptr<BSFile>, Ptr, u32) -> u32),
        entry!(0x00b0_0590, fn_00b00590(Ptr<BSFile>, u32, u32)),
        entry!(0x00b0_04e0, fn_00b004e0(Ptr<BSFile>) -> u32),
        entry!(0x00b0_0260, bs_file_bs_file(Ptr<BSFile>, Ptr, u32, u32, u8) -> Ptr<BSFile>),
        entry!(0x00af_f990, fn_00aff990(Ptr<BSFile>, bool)),
        entry!(0x00af_f9d0, fn_00aff9d0(Ptr, Ptr, u32) -> u32),
        entry!(0x00af_f9f0, fn_00aff9f0(Ptr, Ptr, u32) -> u32),
        entry!(0x00af_f610, fn_00aff610(Ptr<BSFile>) -> u32),
        entry!(0x00af_f950, fn_00aff950(Ptr<BSFile>, Ptr, u32) -> u32),
        entry!(0x00af_fc60, bs_file_exist(Ptr<BSFile>) -> bool),
        entry!(0x00af_fc90, fn_00affc90(Ptr<BSFile>, Ptr, u32, u16) -> u32),
        entry!(0x00af_fd10, bs_file_close(Ptr<BSFile>)),
        entry!(0x00af_f240, bs_file_destructor(Ptr<BSFile>)),
        entry!(0x00af_ff20, fn_00afff20(Ptr<BSFile>, u32) -> Ptr<BSFile>),
        entry!(0x00af_f490, bs_file_check_is_good(Ptr<BSFile>)),
        entry!(0x00af_f300, bs_file_open(Ptr<BSFile>, u32, bool) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const VTABLE: u32 = 0x0300_0000;
    const GET_FILE_SIZE: u32 = 0x0070_0000;
    const GET_SIZE: u32 = 0x0070_0010;
    const READ_F: u32 = 0x0070_0020;
    const WRITE_F: u32 = 0x0070_0030;

    /// A `BSFile` with a test vtable whose slots +0x1c, +0x28, +0x44 and
    /// +0x48 are the doubles above (they answer 0x40, 0, 7 and 9), and
    /// doubles for every callee outside this file.
    fn engine() -> (Engine, Ptr<BSFile>) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let mut slots = vec![0u32; 0x4c / 4 + 1];
        slots[GET_FILE_SIZE_SLOT as usize / 4] = GET_FILE_SIZE;
        slots[GET_SIZE_SLOT as usize / 4] = GET_SIZE;
        slots[READ_F_SLOT as usize / 4] = READ_F;
        slots[WRITE_F_SLOT as usize / 4] = WRITE_F;
        e.put_vtable(VTABLE, &slots);
        let this: Ptr<BSFile> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.register(GET_FILE_SIZE, |_, _| 0x40u32.into_ret());
        e.register(GET_SIZE, |_, _| Ret::default());
        e.register(READ_F, |_, _| 7u32.into_ret());
        e.register(WRITE_F, |_, _| 9u32.into_ret());
        for callee in [
            ACCESS,
            FOPEN_S,
            FCLOSE,
            NI_FILE_FLUSH,
            NI_FILE_READ_DIRECT,
            NI_FILE_WRITE,
            NI_FILE_DESTRUCTOR,
            MEMORY_MANAGER_ALLOCATE,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn set_endian_swap_installs_the_read_and_write_pair() {
        let (mut e, this) = engine();
        e.call(0x00af_f990, &args![this, true]);
        assert_eq!(e.mem.u32(this.addr() + 8), SWAPPING_READ);
        assert_eq!(e.mem.u32(this.addr() + 0xc), SWAPPING_WRITE);
        e.call(0x00af_f990, &args![this, false]);
        assert_eq!(e.mem.u32(this.addr() + 8), PLAIN_READ);
        assert_eq!(e.mem.u32(this.addr() + 0xc), PLAIN_WRITE);
    }

    #[test]
    fn the_plain_pair_calls_read_f_and_write_f() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00af_f9d0, &args![this, 0x900u32, 3u32]).u32(), 7);
        assert_eq!(e.call(0x00af_f9f0, &args![this, 0x900u32, 3u32]).u32(), 9);
        assert_eq!(calls_to(&e, READ_F), vec![vec![this.addr(), 0x900, 3]]);
        assert_eq!(calls_to(&e, WRITE_F), vec![vec![this.addr(), 0x900, 3]]);
    }

    #[test]
    fn get_file_size_asks_once_the_size_is_known() {
        let (mut e, this) = engine();
        e.register(GET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 0x154, 0x99);
            Ret::default()
        });
        assert_eq!(e.call(0x00af_f610, &args![this]).u32(), 0x99);
        assert_eq!(e.call(0x00af_f610, &args![this]).u32(), 0x99);
        assert_eq!(calls_to(&e, GET_SIZE).len(), 1);
    }

    #[test]
    fn write_f_counts_the_true_file_position() {
        let (mut e, this) = engine();
        e.register(NI_FILE_WRITE, |_, _| 5u32.into_ret());
        e.set(this, BSFile::iTrueFilePos, 10);
        assert_eq!(e.call(0x00af_f950, &args![this, 0x900u32, 6u32]).u32(), 5);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 15);
        assert_eq!(
            calls_to(&e, NI_FILE_WRITE),
            vec![vec![this.addr(), 0x900, 6]]
        );
    }

    #[test]
    fn exist_asks_access_for_the_file_name() {
        let (mut e, this) = engine();
        assert!(e.call(0x00af_fc60, &args![this]).bool());
        assert_eq!(calls_to(&e, ACCESS), vec![vec![this.addr() + 0x44, 0]]);
        e.register(ACCESS, |_, _| u32::MAX.into_ret());
        assert!(!e.call(0x00af_fc60, &args![this]).bool());
    }

    /// `ReadF` serving the bytes of `text` one at a time, then short reads.
    fn serve(e: &mut Engine, text: &'static [u8]) {
        let next = std::rc::Rc::new(std::cell::Cell::new(0usize));
        e.register_double(READ_F, move |e, a| {
            let at = next.get();
            if at >= text.len() {
                return 0u32.into_ret();
            }
            e.mem.set_u8(a[1], text[at]);
            next.set(at + 1);
            1u32.into_ret()
        });
    }

    #[test]
    fn get_line_stops_at_the_terminator() {
        let (mut e, this) = engine();
        serve(&mut e, b"ab\ncd");
        let buffer = e.mem.alloc(0x10);
        let read = e
            .call(0x00af_fc90, &args![this, buffer, 0x10u32, 0x0au16])
            .u32();
        assert_eq!(read, 3);
        assert_eq!(e.mem.cstr(buffer), b"ab");
    }

    #[test]
    fn get_line_stops_at_the_size_and_at_a_short_read() {
        let (mut e, this) = engine();
        serve(&mut e, b"abcdef");
        let buffer = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00af_fc90, &args![this, buffer, 4u32, 0x0au16])
                .u32(),
            3
        );
        assert_eq!(e.mem.cstr(buffer), b"abc");
        let (mut e, this) = engine();
        serve(&mut e, b"xy");
        let buffer = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00af_fc90, &args![this, buffer, 0x10u32, 0x0au16])
                .u32(),
            2
        );
        assert_eq!(e.mem.cstr(buffer), b"xy");
    }

    #[test]
    fn get_line_compares_a_signed_byte_with_the_terminator() {
        // 0xff sign-extends to -1, which never equals a 16-bit terminator.
        let (mut e, this) = engine();
        serve(&mut e, b"\xff\xff");
        let buffer = e.mem.alloc(0x10);
        e.call(0x00af_fc90, &args![this, buffer, 3u32, 0xffu16]);
        assert_eq!(e.mem.cstr(buffer), b"\xff\xff");
    }

    #[test]
    fn close_flushes_and_closes_a_good_file_then_frees_the_buffer() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_bGood, 1);
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_pBuffer, 0x6600);
        e.call(0x00af_fd10, &args![this]);
        assert_eq!(calls_to(&e, NI_FILE_FLUSH), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x6600]]
        );
        assert_eq!(e.get(file, NiFile::m_pFile), 0);
        assert_eq!(e.get(file, NiFile::m_pBuffer), 0);
        // A bad file is not flushed or closed.
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_pFile, 0x7700);
        e.call(0x00af_fd10, &args![this]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn destructor_closes_and_runs_the_ni_file_destructor() {
        let (mut e, this) = engine();
        e.call(0x00af_f240, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), BSFILE_VTABLE);
        assert_eq!(calls_to(&e, NI_FILE_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_zero() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00af_ff20, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        e.call(0x00af_ff20, &args![this, 1u32]);
        let frees = calls_to(&e, MEMORY_MANAGER_DEALLOCATE);
        assert_eq!(frees.len(), 3);
        assert_eq!(frees[2], vec![MEMORY_MANAGER, this.addr()]);
    }

    #[test]
    fn check_is_good_without_a_file_is_bad() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_bGood, 1);
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(this.cast::<NiFile>(), NiFile::m_bGood), 0);
    }

    #[test]
    fn check_is_good_allocates_the_buffer_in_context_0x17() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, 0x20);
        let context = e.tls() + ALLOCATION_CONTEXT;
        e.mem.set_u32(context, 5);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, _| {
            let context = e.tls() + ALLOCATION_CONTEXT;
            assert_eq!(e.mem.u32(context), FILE_BUFFER_CONTEXT);
            0x6600u32.into_ret()
        });
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_bGood), 1);
        assert_eq!(e.get(file, NiFile::m_pBuffer), 0x6600);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x20]]
        );
        assert_eq!(e.mem.u32(context), 5);
        assert!(calls_to(&e, NI_FILE_READ_DIRECT).is_empty());
        // A buffer already there, or no buffer size, allocates nothing.
        e.call(0x00af_f490, &args![this]);
        e.set(file, NiFile::m_pBuffer, 0);
        e.set(file, NiFile::m_uiBufferAllocSize, 0);
        e.call(0x00af_f490, &args![this]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
    }

    #[test]
    fn check_is_good_reads_the_whole_file_for_size_minus_one() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, u32::MAX);
        e.set(file, NiFile::m_uiPos, 3);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x6600u32.into_ret());
        e.register(NI_FILE_READ_DIRECT, |_, _| 0x40u32.into_ret());
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_uiBufferAllocSize), 0x40);
        assert_eq!(e.get(file, NiFile::m_uiBufferReadSize), 0x40);
        assert_eq!(e.get(file, NiFile::m_uiPos), 0);
        assert_eq!(
            calls_to(&e, NI_FILE_READ_DIRECT),
            vec![vec![this.addr(), 0x6600, 0x40]]
        );
        assert_eq!(e.get(file, NiFile::m_bGood), 1);
        // A short read makes the file bad.
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, u32::MAX);
        e.register(NI_FILE_READ_DIRECT, |_, _| 0x3fu32.into_ret());
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_bGood), 0);
    }

    /// `fopen_s` opening the given answers in turn (written to `*slot`).
    fn opens(e: &mut Engine, answers: &'static [u32]) {
        let next = std::rc::Rc::new(std::cell::Cell::new(0usize));
        e.register_double(FOPEN_S, move |e, a| {
            let at = next.get();
            e.mem.set_u32(a[0], answers[at]);
            next.set(at + 1);
            Ret::default()
        });
    }

    #[test]
    fn open_picks_the_mode_from_the_open_mode_and_text_flag() {
        for (mode, text, want) in [
            (0u32, false, MODE_RB),
            (0, true, MODE_RT),
            (1, false, MODE_R_PLUS_B),
            (1, true, MODE_R_PLUS_T),
            (2, false, MODE_A_PLUS_B),
            (5, true, MODE_A_PLUS_T),
        ] {
            let (mut e, this) = engine();
            let file = this.cast::<NiFile>();
            e.set(file, NiFile::m_eMode, mode);
            e.set(file, NiFile::m_uiPos, 3);
            e.set(file, NiFile::m_uiBufferReadSize, 4);
            opens(&mut e, &[0x7700]);
            assert!(e.call(0x00af_f300, &args![this, 0u32, text]).bool());
            assert_eq!(
                calls_to(&e, FOPEN_S),
                vec![vec![this.addr() + 0x24, this.addr() + 0x44, want]]
            );
            assert_eq!(e.get(file, NiFile::m_uiPos), 0);
            assert_eq!(e.get(file, NiFile::m_uiBufferReadSize), 0);
            assert_eq!(e.get(file, NiFile::m_bGood), 1);
        }
    }

    #[test]
    fn open_of_an_open_file_answers_true() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_pFile, 0x7700);
        assert!(e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        assert!(calls_to(&e, FOPEN_S).is_empty());
    }

    #[test]
    fn open_for_update_creates_a_missing_file() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_eMode, 1);
        opens(&mut e, &[0, 0x7100, 0x7700]);
        assert!(e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        let modes: Vec<u32> = calls_to(&e, FOPEN_S).iter().map(|c| c[2]).collect();
        assert_eq!(modes, vec![MODE_R_PLUS_B, MODE_WB, MODE_R_PLUS_B]);
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7100]]);
        assert_eq!(e.get(this.cast::<NiFile>(), NiFile::m_pFile), 0x7700);
        // Creation failing too: nothing to close, and the file stays bad.
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_eMode, 1);
        opens(&mut e, &[0, 0, 0]);
        assert!(!e.call(0x00af_f300, &args![this, 0u32, true]).bool());
        let modes: Vec<u32> = calls_to(&e, FOPEN_S).iter().map(|c| c[2]).collect();
        assert_eq!(modes, vec![MODE_R_PLUS_T, MODE_WT, MODE_R_PLUS_T]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        // Read mode does not retry.
        let (mut e, this) = engine();
        opens(&mut e, &[0]);
        assert!(!e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        assert_eq!(calls_to(&e, FOPEN_S).len(), 1);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        for f in [NI_FILE_CONSTRUCT, 0x00af_f990, BS_FILE_OPEN] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(BS_FILE_EXIST, |_, _| 1u32.into_ret());
        let file = e.mem.alloc(0x158);
        let path = e.mem.alloc(0x200);
        e.mem.set_cstr(path, b"Data.txt");
        (e, file, path)
    }

    #[test]
    fn mode_0_copies_the_name_and_asks_whether_it_exists() {
        let (mut e, file, path) = engine();
        e.call_log = Some(vec![]);
        let made = e.call(0x00b0_0260, &args![file, path, 0u32, 0x4000u32, 1u32]).u32();
        assert_eq!(made, file);
        assert_eq!(e.mem.u32(file), BSFILE_VTABLE);
        assert_eq!(e.mem.u32(file + 0x10), 0x4000);
        assert_eq!(e.mem.u32(file + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.cstr(file + 0x44), b"Data.txt".to_vec());
        assert_eq!(e.mem.u8(file + 0x2c), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != BS_FILE_OPEN));
    }

    #[test]
    fn mode_1_opens_and_a_long_name_stays_empty() {
        let (mut e, file, path) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00b0_0260, &args![file, path, 1u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        let opens: Vec<_> = log.iter().filter(|c| c.0 == BS_FILE_OPEN).collect();
        assert_eq!(opens[0].1, vec![file, 0, 1]);
        let long = vec![b'a'; 0x104];
        e.mem.set_cstr(path, &long);
        e.call(0x00b0_0260, &args![file, path, 1u32, 0u32, 1u32]);
        assert_eq!(e.mem.u8(file + 0x44), 0);
    }
}

#[cfg(test)]
mod tests_read_seek_size {
    use super::*;

    const VTABLE: u32 = 0x0300_0000;
    const OPEN_DOUBLE: u32 = 0x0e10_0000;
    const SIZE_DOUBLE: u32 = 0x0e10_0010;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A `BSFile` whose vtable's `Open` and `GetSize` are doubles (size 0x100), with the seek
    /// origin words 0, 1, 2 and the `NiFile` read answering its count.
    fn engine() -> (Engine, Ptr<BSFile>) {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SEEK_SET_WORD, 0u32);
        e.set_global(SEEK_CUR_WORD, 1u32);
        e.set_global(SEEK_END_WORD, 2u32);
        let mut slots = vec![0u32; 0x14];
        slots[(OPEN_SLOT / 4) as usize] = OPEN_DOUBLE;
        slots[(GET_SIZE_SLOT / 4) as usize] = SIZE_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        e.register(OPEN_DOUBLE, |_, _| 1u32.into_ret());
        e.register(SIZE_DOUBLE, |_, _| 0x100u32.into_ret());
        e.register(NI_FILE_READ, |_, a| a[2].into_ret());
        e.register(NI_FILE_SEEK, |_, _| Ret::default());
        e.register(NI_FILE_FLUSH, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this, VTABLE);
        e.call_log = Some(vec![]);
        (e, Ptr::new(this))
    }

    #[test]
    fn read_f_opens_a_closed_file_and_counts_the_position() {
        let (mut e, this) = engine();
        let buffer = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00b0_0650, &args![this, buffer, 8u32]).u32(), 8);
        assert_eq!(calls_to(&e, OPEN_DOUBLE), [vec![this.addr(), 0, 0]]);
        e.set(ni_file(this), NiFile::m_pFile, 0x7700);
        assert_eq!(e.call(0x00b0_0650, &args![this, buffer, 4u32]).u32(), 4);
        assert_eq!(calls_to(&e, OPEN_DOUBLE).len(), 1);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 12);
    }

    #[test]
    fn the_seek_moves_only_when_the_target_differs() {
        let (mut e, this) = engine();
        e.set(this, BSFile::iTrueFilePos, 0x10);
        // Set, read mode: by the distance with the current origin.
        e.call(0x00b0_0590, &args![this, 0x30u32, 0u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK), [vec![this.addr(), 0x20, 1]]);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 0x30);
        // The same place: nothing.
        e.call(0x00b0_0590, &args![this, 0x30u32, 0u32]);
        e.call(0x00b0_0590, &args![this, 0u32, 1u32]);
        e.call(0x00b0_0590, &args![this, 0u32, 7u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK).len(), 1);
        // From the end of 0x100 bytes; a write-mode set keeps its origin.
        e.call(0x00b0_0590, &args![this, 0x10u32, 2u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK)[1], vec![this.addr(), 0x10, 2]);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 0xf0);
        e.set(ni_file(this), NiFile::m_eMode, 1);
        e.call(0x00b0_0590, &args![this, 0x40u32, 0u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK)[2], vec![this.addr(), 0x40, 0]);
    }

    #[test]
    fn get_size_measures_the_file_and_goes_back() {
        let (mut e, this) = engine();
        // No FILE: 0, after trying to open.
        assert_eq!(e.call(0x00b0_04e0, &args![this]).u32(), 0);
        assert_eq!(calls_to(&e, OPEN_DOUBLE).len(), 1);
        e.set(ni_file(this), NiFile::m_pFile, 0x7700);
        e.set(ni_file(this), NiFile::m_eMode, 1);
        let mut tells = 0;
        e.register_double(FTELL, move |_, _| {
            tells += 1;
            (if tells == 1 { 0x24u32 } else { 0x5000 }).into_ret()
        });
        e.register(FSEEK, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b0_04e0, &args![this]).u32(), 0x5000);
        assert_eq!(calls_to(&e, NI_FILE_FLUSH), [vec![this.addr()]]);
        assert_eq!(
            calls_to(&e, FSEEK),
            [vec![0x7700, 0, 2], vec![0x7700, 0x24, 0]]
        );
    }
}
