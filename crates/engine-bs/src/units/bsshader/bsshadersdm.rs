//! `BSShader/bsshadersdm.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `BSShaderSDM::InitSDM` sets once it ran (`01201f70`).
const INITIALIZED: u32 = 0x0120_1f70;
/// `BSFadeNode::InitSDM` (engine map, `00b4e270`) and `BSShaderUtil::InitSDM` (`00b6ba70`), both
/// `cdecl` without words.
const FADE_NODE_INIT_SDM: u32 = 0x00b4_e270;
const SHADER_UTIL_INIT_SDM: u32 = 0x00b6_ba70;

// Translated from 00bc2940 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSShaderSDM::InitSDM` (engine map; `main` calls it at `0086aa0f`): once (the byte
/// `01201f70` clear), runs `BSFadeNode::InitSDM` and `BSShaderUtil::InitSDM` and sets the byte.
pub fn bs_shader_sdm_init_sdm(e: &mut Engine) {
    if e.mem.u8(INITIALIZED) != 0 {
        return;
    }
    e.call(FADE_NODE_INIT_SDM, &args![]);
    e.call(SHADER_UTIL_INIT_SDM, &args![]);
    e.mem.set_u8(INITIALIZED, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2940, bs_shader_sdm_init_sdm())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_sdm_runs_its_two_parts_once() {
        let mut e = Engine::new();
        e.map(INITIALIZED & !0xfff, 0x1000);
        e.register(FADE_NODE_INIT_SDM, |_, _| Ret::default());
        e.register(SHADER_UTIL_INIT_SDM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00bc_2940, &[]);
        e.call(0x00bc_2940, &[]);
        let log = e.call_log.take().unwrap();
        let parts: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == FADE_NODE_INIT_SDM || *a == SHADER_UTIL_INIT_SDM)
            .collect();
        assert_eq!(parts, [FADE_NODE_INIT_SDM, SHADER_UTIL_INIT_SDM]);
        assert_eq!(e.mem.u8(INITIALIZED), 1);
    }
}
