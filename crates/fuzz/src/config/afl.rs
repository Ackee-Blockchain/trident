use std::collections::HashMap;

use crate::config::constants::*;
use rand::RngCore;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum BuildArgument {
    CargoTargetDir,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FuzzArgument {
    AflWorkspaceIn,
    AflWorkspaceOut,
    Execs,
    Seconds,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Afl {
    pub build_args: HashMap<BuildArgument, AflArg>,
    pub fuzz_args: HashMap<FuzzArgument, AflArg>,
    pub seeds: Vec<AflSeed>,
}

impl Afl {
    pub fn get_cargo_build_dir(&self) -> Option<&AflArg> {
        self.build_args.get(&BuildArgument::CargoTargetDir)
    }
    pub fn get_workspace_in(&self) -> Option<&AflArg> {
        self.fuzz_args.get(&FuzzArgument::AflWorkspaceIn)
    }
    pub fn get_workspace_out(&self) -> Option<&AflArg> {
        self.fuzz_args.get(&FuzzArgument::AflWorkspaceOut)
    }
    pub fn get_execs(&self) -> Option<&AflArg> {
        self.fuzz_args.get(&FuzzArgument::Execs)
    }
    pub fn get_seconds(&self) -> Option<&AflArg> {
        self.fuzz_args.get(&FuzzArgument::Seconds)
    }
    pub fn get_collect_build_args(&self) -> Vec<String> {
        self.build_args
            .values()
            .flat_map(|arg| {
                let val = arg.val.clone().unwrap_or_default();
                if let Some(opt) = &arg.short_opt {
                    vec![opt.clone(), val]
                } else if let Some(opt) = &arg.long_opt {
                    vec![opt.clone(), val]
                } else {
                    vec![]
                }
            })
            .collect()
    }
    pub fn get_collect_fuzz_args(&self) -> Vec<String> {
        self.fuzz_args
            .values()
            .flat_map(|arg| {
                let val = arg.val.clone().unwrap_or_default();
                if let Some(opt) = &arg.short_opt {
                    vec![opt.clone(), val]
                } else if let Some(opt) = &arg.long_opt {
                    vec![opt.clone(), val]
                } else {
                    vec![]
                }
            })
            .collect()
    }
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct _Afl {
    #[serde(default)]
    pub cargo_target_dir: Option<String>,
    #[serde(default)]
    pub afl_workspace_in: Option<String>,
    #[serde(default)]
    pub afl_workspace_out: Option<String>,
    #[serde(default)]
    pub execs: Option<u64>,
    #[serde(default)]
    pub seconds: Option<u64>,
    #[serde(default)]
    pub seeds: Option<Vec<_AflSeed>>,
}

impl From<_Afl> for Afl {
    fn from(_f: _Afl) -> Self {
        let mut _self = Self {
            seeds: vec![],
            fuzz_args: HashMap::new(),
            build_args: HashMap::new(),
        };

        // cargo_target_dir
        let cargo_target_dir = _f
            .cargo_target_dir
            .and_then(|value| if value.is_empty() { None } else { Some(value) })
            .unwrap_or(CARGO_TARGET_DIR_DEFAULT_AFL.to_owned());

        _self.build_args.insert(
            BuildArgument::CargoTargetDir,
            AflArg::new("", "--target-dir", &cargo_target_dir),
        );

        // afl_workspace_in
        let afl_workspace_in = _f
            .afl_workspace_in
            .and_then(|value| if value.is_empty() { None } else { Some(value) })
            .unwrap_or(AFL_WORKSPACE_DEFAULT_IN.to_owned());

        _self.fuzz_args.insert(
            FuzzArgument::AflWorkspaceIn,
            AflArg::new("-i", "", &afl_workspace_in),
        );

        // afl_workspace_out
        let afl_workspace_out = _f
            .afl_workspace_out
            .and_then(|value| if value.is_empty() { None } else { Some(value) })
            .unwrap_or(AFL_WORKSPACE_DEFAULT_OUT.to_owned());

        _self.fuzz_args.insert(
            FuzzArgument::AflWorkspaceOut,
            AflArg::new("-o", "", &afl_workspace_out),
        );

        // execs
        let execs = _f.execs.unwrap_or(0);
        if execs > 0 {
            _self.fuzz_args.insert(
                FuzzArgument::Execs,
                AflArg::new("-E", "", &execs.to_string()),
            );
        }

        // seconds
        let seconds = _f.seconds.unwrap_or(0);
        if seconds > 0 {
            _self.fuzz_args.insert(
                FuzzArgument::Seconds,
                AflArg::new("-V", "", &seconds.to_string()),
            );
        }

        if let Some(seeds) = _f.seeds {
            for x in seeds {
                _self.seeds.push(x.into());
            }
        } else {
            _self.seeds.push(_AflSeed::default().into());
        }

        _self
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AflArg {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    pub val: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct _AflSeed {
    pub file_name: String,
    pub seed: Option<String>,
    pub override_file: Option<bool>,
    pub bytes_count: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AflSeed {
    pub file_name: String,
    pub seed: Vec<u8>,
    pub override_file: bool,
}

impl Default for _AflSeed {
    fn default() -> Self {
        Self {
            file_name: DEFAULT_SEED_FILENAME.to_string(),
            seed: Some(DEFAULT_SEED.to_string()),
            override_file: Some(false),
            bytes_count: None,
        }
    }
}

impl From<_AflSeed> for AflSeed {
    fn from(value: _AflSeed) -> Self {
        match value.bytes_count {
            Some(number_of_random_bytes) => {
                if number_of_random_bytes > 0 {
                    let mut rng = rand::rngs::OsRng;
                    let mut seed = vec![0u8; number_of_random_bytes];
                    rng.fill_bytes(&mut seed);
                    Self {
                        file_name: value.file_name,
                        seed,
                        override_file: value.override_file.unwrap_or_default(),
                    }
                } else {
                    let seed_as_bytes = value
                        .seed
                        .and_then(|value| if value.is_empty() { None } else { Some(value) })
                        .unwrap_or(DEFAULT_SEED.to_string())
                        .as_bytes()
                        .to_vec();
                    Self {
                        file_name: value.file_name,
                        seed: seed_as_bytes,
                        override_file: value.override_file.unwrap_or_default(),
                    }
                }
            }
            None => {
                let seed_as_bytes = value
                    .seed
                    .and_then(|value| if value.is_empty() { None } else { Some(value) })
                    .unwrap_or(DEFAULT_SEED.to_string())
                    .as_bytes()
                    .to_vec();
                Self {
                    file_name: value.file_name,
                    seed: seed_as_bytes,
                    override_file: value.override_file.unwrap_or_default(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Afl {
        fn clean() -> Self {
            Self {
                fuzz_args: HashMap::new(),
                build_args: HashMap::new(),
                seeds: vec![],
            }
        }
    }

    #[test]
    fn test_cargo_target_dir() {
        let mut afl = Afl::clean();

        afl.build_args.insert(
            BuildArgument::CargoTargetDir,
            AflArg::new("", "--target-dir", CARGO_TARGET_DIR_DEFAULT_AFL),
        );

        let arg = afl.get_collect_build_args();
        assert_eq!(arg, vec!["--target-dir", CARGO_TARGET_DIR_DEFAULT_AFL]);
    }
    #[test]
    fn test_workspace_in() {
        let mut afl = Afl::clean();

        // afl_workspace_in
        afl.fuzz_args.insert(
            FuzzArgument::AflWorkspaceIn,
            AflArg::new("-i", "", AFL_WORKSPACE_DEFAULT_IN),
        );

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-i", AFL_WORKSPACE_DEFAULT_IN]);
    }
    #[test]
    fn test_workspace_out() {
        let mut afl = Afl::clean();

        // afl_workspace_out
        afl.fuzz_args.insert(
            FuzzArgument::AflWorkspaceOut,
            AflArg::new("-o", "", AFL_WORKSPACE_DEFAULT_OUT),
        );

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-o", AFL_WORKSPACE_DEFAULT_OUT]);
    }
    #[test]
    fn test_execs() {
        let mut afl = Afl::clean();

        // execs
        afl.fuzz_args
            .insert(FuzzArgument::Execs, AflArg::new("-E", "", &555.to_string()));

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-E", "555"]);
    }
    #[test]
    fn test_seconds() {
        let mut afl = Afl::clean();

        // seconds
        afl.fuzz_args.insert(
            FuzzArgument::Seconds,
            AflArg::new("-V", "", &15.to_string()),
        );

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-V", "15"]);
    }
}
