use serde::Deserialize;

use crate::constants::*;

#[derive(Debug, Deserialize, Clone)]
pub struct AflArg {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    pub val: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Afl {
    pub fuzz_args: Vec<AflArg>,
    pub build_args: Vec<AflArg>,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct _Afl {
    #[serde(default)]
    pub cargo_target_dir: Option<String>,
    #[serde(default)]
    pub afl_workspace_in: Option<String>,
    #[serde(default)]
    pub afl_workspace_out: Option<String>,
}

impl From<_Afl> for Afl {
    fn from(_f: _Afl) -> Self {
        let mut _self = Self {
            fuzz_args: vec![],
            build_args: vec![],
        };

        // cargo_target_dir
        let cargo_target_dir = _f.cargo_target_dir.unwrap_or_default();
        if !cargo_target_dir.is_empty() {
            _self
                .build_args
                .push(AflArg::new("", "--target-dir", &cargo_target_dir));
        } else {
            _self.build_args.push(AflArg::new(
                "",
                "--target-dir",
                CARGO_TARGET_DIR_DEFAULT_AFL,
            ));
        }
        // afl_workspace_in
        let afl_workspace_in = _f.afl_workspace_in.unwrap_or_default();
        if !afl_workspace_in.is_empty() {
            _self
                .fuzz_args
                .push(AflArg::new("-i", "", &afl_workspace_in));
        } else {
            _self
                .fuzz_args
                .push(AflArg::new("-i", "", AFL_WORKSPACE_DEFAULT_IN));
        }

        // afl_workspace_ou
        let afl_workspace_out = _f.afl_workspace_out.unwrap_or_default();
        if !afl_workspace_out.is_empty() {
            _self
                .fuzz_args
                .push(AflArg::new("-o", "", &afl_workspace_out));
        } else {
            _self
                .fuzz_args
                .push(AflArg::new("-o", "", AFL_WORKSPACE_DEFAULT_OUT));
        }

        _self
    }
}

impl AflArg {
    pub fn new(short_opt: &str, long_opt: &str, val: &str) -> Self {
        let short_opt = if short_opt.is_empty() {
            None
        } else {
            Some(short_opt.to_owned())
        };
        let long_opt = if long_opt.is_empty() {
            None
        } else {
            Some(long_opt.to_owned())
        };
        let val = if val.is_empty() {
            None
        } else {
            Some(val.to_owned())
        };
        Self {
            short_opt,
            long_opt,
            val,
        }
    }
}
