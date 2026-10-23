//! `BSMovie/movieplayer.obj` (Xbox PDB source unit), subsystem `BSMovie`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The movie object's destructor (`00ec1f40`, `thiscall`), `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on the manager `011f6238`, one word) and the player's own call after
/// the stop (`00ec2a60`, `thiscall`).
const MOVIE_DESTRUCT: u32 = 0x00ec_1f40;
const DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const AFTER_STOP: u32 = 0x00ec_2a60;
/// The flag the inlined deleting destructor tests (`MOV ECX,1; AND ECX,1; JZ`): the constant 1,
/// so the free always runs.
const DELETE_FLAG: u32 = 1;

// Translated from 00ec25e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::Stop` (engine map; `thiscall`, no stack words, plain `RET`): a movie at `+0xc`
/// is destroyed (`00ec1f40`), its memory given back (`MemoryManager::Deallocate` on
/// `011f6238`), and the word cleared; then `00ec2a60`.
pub fn movie_player_stop(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + 0xc) != 0 {
        let movie = e.mem.u32(t + 0xc);
        if movie != 0 {
            e.call(MOVIE_DESTRUCT, &args![movie]);
            if DELETE_FLAG & 1 != 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, movie]);
            }
        }
        e.mem.set_u32(t + 0xc, 0);
    }
    e.call(AFTER_STOP, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec_25e0, movie_player_stop(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn stop_destroys_and_frees_the_movie_then_runs_00ec2a60() {
        let mut e = Engine::new();
        for addr in [MOVIE_DESTRUCT, DEALLOCATE, AFTER_STOP] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xc, 0x0500_0000);
        e.call(0x00ec_25e0, &args![player]);
        assert_eq!(calls(&e, MOVIE_DESTRUCT), vec![vec![0x0500_0000]]);
        assert_eq!(
            calls(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0500_0000]]
        );
        assert_eq!(e.mem.u32(player + 0xc), 0);
        assert_eq!(calls(&e, AFTER_STOP), vec![vec![player]]);
        // Without a movie only 00ec2a60 runs.
        e.call(0x00ec_25e0, &args![player]);
        assert_eq!(calls(&e, MOVIE_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, DEALLOCATE).len(), 1);
        assert_eq!(calls(&e, AFTER_STOP).len(), 2);
    }
}
