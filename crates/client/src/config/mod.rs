use anyhow::Context;
use fehler::throw;
use serde::Deserialize;
use std::{env, fs, io, path::PathBuf};
use thiserror::Error;

use crate::constants::*;

mod fuzz;
use fuzz::*;
mod honggfuzz;
use honggfuzz::*;
mod afl;
use afl::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub honggfuzz: HonggFuzz,
    pub afl: Afl,
    pub fuzz: Fuzz,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Config {
    #[serde(default)]
    pub honggfuzz: Option<_HonggFuzz>,
    #[serde(default)]
    pub afl: Option<_Afl>,
    #[serde(default)]
    pub fuzz: Option<_Fuzz>,
}

impl From<_Config> for Config {
    fn from(_c: _Config) -> Self {
        Self {
            honggfuzz: _c.honggfuzz.unwrap_or_default().into(),
            afl: _c.afl.unwrap_or_default().into(),
            fuzz: _c.fuzz.unwrap_or_default().into(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let root = discover_root().expect("failed to find the root folder");
        let s = fs::read_to_string(root.join(TRIDENT_TOML).as_path())
            .expect("failed to read the Trident config file");
        let _config: _Config = toml::from_str(&s).expect("failed to parse the Trident config file");
        _config.into()
    }
    pub fn get_honggfuzz_args(&self, cli_input: String) -> String {
        // Tested on a few examples, HFUZZ_RUN_ARGS give precedence to the later arguments.
        // so if HFUZZ_RUN_ARGS="-t 10 -t 15" -> timeout 15s is applied.
        // That means we do not need to parse the arguments from the CLI;
        // thus, we can simply append them at the end, and the CLI will have precedence.

        let mut args: Vec<String> = self
            .honggfuzz
            .fuzz_args
            .iter()
            .map(|a| {
                let val = a.val.to_owned().unwrap_or("".to_string());
                if let Some(o) = &a.short_opt {
                    format!("{} {}", o, val)
                } else if let Some(o) = &a.long_opt {
                    format!("{} {}", o, val)
                } else {
                    "".to_string()
                }
            })
            .collect();
        args.push(cli_input);
        args.join(" ")
    }
    pub fn get_afl_build_args(&self) -> Vec<String> {
        // Tested on a few examples, HFUZZ_RUN_ARGS give precedence to the later arguments.
        // so if HFUZZ_RUN_ARGS="-t 10 -t 15" -> timeout 15s is applied.
        // That means we do not need to parse the arguments from the CLI;
        // thus, we can simply append them at the end, and the CLI will have precedence.

        self.afl
            .build_args
            .iter()
            .flat_map(|a| {
                let val = a.val.to_owned().unwrap_or("".to_string());
                if let Some(o) = &a.short_opt {
                    vec![o.clone(), val] // Return as two separate elements
                } else if let Some(o) = &a.long_opt {
                    vec![o.clone(), val] // Return as two separate elements
                } else {
                    vec![] // Return an empty vector if no option is found
                }
            })
            .collect()
    }
    pub fn get_afl_fuzz_args(&self) -> Vec<String> {
        // Tested on a few examples, HFUZZ_RUN_ARGS give precedence to the later arguments.
        // so if HFUZZ_RUN_ARGS="-t 10 -t 15" -> timeout 15s is applied.
        // That means we do not need to parse the arguments from the CLI;
        // thus, we can simply append them at the end, and the CLI will have precedence.

        self.afl
            .fuzz_args
            .iter()
            .flat_map(|a| {
                let val = a.val.to_owned().unwrap_or("".to_string());
                if let Some(o) = &a.short_opt {
                    vec![o.clone(), val] // Return as two separate elements
                } else if let Some(o) = &a.long_opt {
                    vec![o.clone(), val] // Return as two separate elements
                } else {
                    vec![] // Return an empty vector if no option is found
                }
            })
            .collect()
    }
    pub fn get_target_path(&self) -> String {
        // Tested on a few examples, HFUZZ_RUN_ARGS give precedence to the later arguments.
        // so if HFUZZ_RUN_ARGS="-t 10 -t 15" -> timeout 15s is applied.
        // That means we do not need to parse the arguments from the CLI;
        // thus, we can simply append them at the end, and the CLI will have precedence.

        let target_dir_arg = match self.afl.build_args.iter().find(|a| {
            if let Some(o) = &a.long_opt {
                return o == "--target-dir";
            }
            false
        }) {
            Some(target_dir) => target_dir
                .val
                .clone()
                .unwrap_or(CARGO_TARGET_DIR_DEFAULT_AFL.to_owned()),
            None => CARGO_TARGET_DIR_DEFAULT_AFL.to_owned(),
        };
        target_dir_arg
    }
    pub fn get_rustflags_args(&self, cli_input: String) -> String {
        let mut args: Vec<String> = self
            .fuzz
            .rust_flags
            .iter()
            .map(|arg| {
                if arg.val {
                    format!("--cfg {}", arg.cfg_identifier)
                } else {
                    "".to_string()
                }
            })
            .collect();
        args.push(cli_input);
        args.join(" ")
    }
    pub fn get_env_arg(&self, env_variable: &str) -> String {
        let expect = format!("{env_variable} not found");
        self.honggfuzz
            .env_variables
            .get(env_variable)
            .expect(&expect)
            .to_string()
    }
}

/// Tries to find the root directory with the `Anchor.toml` file.
/// Throws an error when there is no directory with the `Anchor.toml` file
pub fn discover_root() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir()?;
    let mut dir = Some(current_dir.as_path());
    while let Some(cwd) = dir {
        for file in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let path = file
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = path.file_name() {
                if filename.to_str() == Some(ANCHOR_TOML) {
                    return Ok(PathBuf::from(cwd));
                }
            }
        }
        dir = cwd.parent();
    }
    throw!(Error::BadWorkspace)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    impl Default for HonggFuzz {
        fn default() -> Self {
            let mut env_variables: HashMap<String, String> = HashMap::default();
            env_variables.insert(
                HFUZZ_WORKSPACE_ENV.to_owned(),
                HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_owned(),
            );
            env_variables.insert(
                CARGO_TARGET_DIR_ENV.to_owned(),
                HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_owned(),
            );
            Self {
                fuzz_args: vec![
                    HonggFuzzArg::new("-t", "--timeout", &10.to_string()),
                    HonggFuzzArg::new("-N", "--iterations", &0.to_string()),
                    HonggFuzzArg::new("-r", "--mutations_per_run", &6.to_string()),
                    HonggFuzzArg::new("-e", "--extension", "fuzz"),
                    HonggFuzzArg::new("", "--run_time", &0.to_string()),
                    HonggFuzzArg::new("-F", "--max_file_size", &1_048_576.to_string()),
                ],
                env_variables,
            }
        }
    }

    impl Default for Afl {
        fn default() -> Self {
            Self {
                build_args: vec![],
                fuzz_args: vec![
                    AflArg::new("", "--target-dir", CARGO_TARGET_DIR_DEFAULT_AFL),
                    AflArg::new("-i", "", AFL_WORKSPACE_DEFAULT_IN),
                    AflArg::new("-o", "", AFL_WORKSPACE_DEFAULT_OUT),
                ],
            }
        }
    }

    impl Default for Fuzz {
        fn default() -> Self {
            let rust_flags = vec![
                Cfg {
                    cfg_identifier: "allow_duplicate_txs".to_string(),
                    val: false,
                },
                Cfg {
                    cfg_identifier: "fuzzing_with_stats".to_string(),
                    val: false,
                },
            ];

            Self { rust_flags }
        }
    }

    use super::*;
    #[test]
    fn test_merge_and_precedence1() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_honggfuzz_args(String::default());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -r 6 -e fuzz --run_time 0 -F 1048576 "
        );
    }
    #[test]
    fn test_merge_and_precedence2() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_honggfuzz_args("-t 0 -N10 --exit_upon_crash".to_string());

        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -r 6 -e fuzz --run_time 0 -F 1048576 -t 0 -N10 --exit_upon_crash"
        );
    }
    #[test]
    fn test_merge_and_precedence3() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };
        let env_var_string =
            config.get_honggfuzz_args("-t 100 -N 5000 -Q -v --exit_upon_crash".to_string());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -r 6 -e fuzz --run_time 0 -F 1048576 -t 100 -N 5000 -Q -v --exit_upon_crash"
        );
    }
    #[test]
    fn test_merge_and_precedence4() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_honggfuzz_args("-t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666".to_string());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -r 6 -e fuzz --run_time 0 -F 1048576 -t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
    #[test]
    fn test_merge_and_precedence5() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_honggfuzz_args("-t 10 -N 500 -Q -v --exit_upon_crash -n 15 --verifier -W random_dir --crashdir random_dir5 --run_time 666".to_string());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -r 6 -e fuzz --run_time 0 -F 1048576 -t 10 -N 500 -Q -v --exit_upon_crash -n 15 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
    #[test]
    fn test_obtain_env_variables() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let cargo_target_dir = config.get_env_arg(CARGO_TARGET_DIR_ENV);

        assert_eq!(cargo_target_dir, CARGO_TARGET_DIR_DEFAULT_HFUZZ);
        let hfuzz_workspace = config.get_env_arg(HFUZZ_WORKSPACE_ENV);
        assert_eq!(hfuzz_workspace, HFUZZ_WORKSPACE_DEFAULT_HFUZZ);
    }
    #[test]
    fn test_obtain_env_variables2() {
        let mut config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        config
            .honggfuzz
            .env_variables
            .insert(CARGO_TARGET_DIR_ENV.to_owned(), "new_value_x".to_owned());

        config
            .honggfuzz
            .env_variables
            .insert(HFUZZ_WORKSPACE_ENV.to_owned(), "new_value_y".to_owned());

        let cargo_target_dir = config.get_env_arg(CARGO_TARGET_DIR_ENV);

        assert_eq!(cargo_target_dir, "new_value_x");
        let hfuzz_workspace = config.get_env_arg(HFUZZ_WORKSPACE_ENV);
        assert_eq!(hfuzz_workspace, "new_value_y");
    }

    #[test]
    fn test_obtain_rustflags_variable1() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz::default(),
        };

        let rustflags = config.get_rustflags_args("".to_string());
        let default_rustflags = "  ";

        assert_eq!(rustflags, default_rustflags);
    }
    #[test]
    fn test_obtain_rustflags_variable2() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz {
                rust_flags: vec![Cfg {
                    cfg_identifier: "fuzzing_with_stats".to_string(),
                    val: true,
                }],
            },
        };

        let rustflags = config.get_rustflags_args("".to_string());
        let reference_rustflags = "--cfg fuzzing_with_stats ";

        assert_eq!(rustflags, reference_rustflags);
    }
    #[test]
    fn test_obtain_rustflags_variable3() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz {
                rust_flags: vec![
                    Cfg {
                        cfg_identifier: "allow_duplicate_txs".to_string(),
                        val: true,
                    },
                    Cfg {
                        cfg_identifier: "fuzzing_with_stats".to_string(),
                        val: false,
                    },
                ],
            },
        };

        let rustflags = config.get_rustflags_args("".to_string());
        let reference_rustflags = "--cfg allow_duplicate_txs  ";

        assert_eq!(rustflags, reference_rustflags);
    }
    #[test]
    fn test_obtain_rustflags_variable4() {
        let config = Config {
            honggfuzz: HonggFuzz::default(),
            afl: Afl::default(),
            fuzz: Fuzz {
                rust_flags: vec![Cfg {
                    cfg_identifier: "allow_duplicate_txs".to_string(),
                    val: true,
                }],
            },
        };

        let rustflags = config.get_rustflags_args("--cfg fuzzing_with_stats".to_string());
        let reference_rustflags = "--cfg allow_duplicate_txs --cfg fuzzing_with_stats";

        assert_eq!(rustflags, reference_rustflags);
    }
}
