use serde::Deserialize;
use std::collections::HashMap;

use crate::config::constants::*;

#[derive(Debug, Deserialize, Clone)]
pub struct HonggFuzzArg {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    pub val: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct HonggFuzz {
    pub fuzz_args: Vec<HonggFuzzArg>,
    pub env_variables: HashMap<String, String>,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct _HonggFuzz {
    #[serde(default)]
    /// Timeout in seconds (default: 10)
    /// -t
    /// --timeout
    pub timeout: Option<u16>,
    #[serde(default)]
    /// Number of fuzzing iterations (default: 0 [no limit])
    /// -N
    /// --iterations
    pub iterations: Option<u64>,
    #[serde(default)]
    /// Number of concurrent fuzzing threads (default: number of CPUs / 2)
    /// -n
    /// --threads
    pub threads: Option<u16>,
    #[serde(default)]
    /// Don't close children's stdin, stdout, stderr; can be noisy
    /// -Q
    /// --keep_output
    pub keep_output: Option<bool>,
    #[serde(default)]
    /// Disable ANSI console; use simple log output
    /// -v
    /// --verbose
    pub verbose: Option<bool>,
    #[serde(default)]
    /// Exit upon seeing the first crash (default: false)
    /// --exit_upon_crash
    pub exit_upon_crash: Option<bool>,
    #[serde(default)]
    /// Maximal number of mutations per one run (default: 6)
    /// -r
    /// --mutations_per_run
    pub mutations_per_run: Option<u16>,
    #[serde(default)]
    /// Target compilation directory, defaults to "trident-tests/fuzz_tests/fuzzing/hfuzz_target" to not clash with cargo build's default target directory.
    /// CARGO_TARGET_DIR env variable
    pub cargo_target_dir: Option<String>,
    #[serde(default)]
    /// Honggfuzz working directory, defaults to "trident-tests/fuzz_tests/fuzzing/hfuzz_workspace".
    /// HFUZZ_WORKSPACE env variable
    pub hfuzz_workspace: Option<String>,
    #[serde(default)]
    /// Directory where crashes are saved to (default: workspace directory)
    /// --crashdir
    pub crashdir: Option<String>,
    #[serde(default)]
    /// Input file extension (e.g. 'swf'), (default: 'fuzz')
    /// -e
    /// --extension
    pub extension: Option<String>,
    #[serde(default)]
    /// Number of seconds this fuzzing session will last (default: 0 [no limit])
    /// --run_time
    pub run_time: Option<u32>,
    #[serde(default)]
    /// Maximal size of files processed by the fuzzer in bytes (default: 1048576 = 1MB)
    /// -F
    /// --max_file_size
    pub max_file_size: Option<u32>,
    #[serde(default)]
    /// Save all test-cases (not only the unique ones) by appending the current time-stamp to the filenames (default: false)
    /// -u
    /// --save_all
    pub save_all: Option<bool>,
}

impl From<_HonggFuzz> for HonggFuzz {
    fn from(_f: _HonggFuzz) -> Self {
        let mut _self = Self {
            fuzz_args: vec![],
            env_variables: HashMap::default(),
        };

        // timeout
        let timeout = _f.timeout.unwrap_or(10);
        _self
            .fuzz_args
            .push(HonggFuzzArg::new("-t", "--timeout", &timeout.to_string()));

        // iterations
        let iterations = _f.iterations.unwrap_or(0);
        _self.fuzz_args.push(HonggFuzzArg::new(
            "-N",
            "--iterations",
            &iterations.to_string(),
        ));

        // threads
        let threads = _f.threads.unwrap_or(0);
        if threads > 0 {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("-n", "--threads", &threads.to_string()));
        }

        // keep_output
        let keep_output = _f.keep_output.unwrap_or(false);
        if keep_output {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("-Q", "--keep_output", ""));
        }
        // verbose
        let verbose = _f.verbose.unwrap_or(false);
        if verbose {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("-v", "--verbose", ""));
        }

        // exit_upon_crash
        let exit_upon_crash = _f.exit_upon_crash.unwrap_or(false);
        if exit_upon_crash {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("", "--exit_upon_crash", ""));
        }
        // mutations_per_run
        let mutations_per_run = _f.mutations_per_run.unwrap_or(6);
        _self.fuzz_args.push(HonggFuzzArg::new(
            "-r",
            "--mutations_per_run",
            &mutations_per_run.to_string(),
        ));
        // cargo_target_dir
        let cargo_target_dir = _f.cargo_target_dir.unwrap_or_default();
        if !cargo_target_dir.is_empty() {
            _self
                .env_variables
                .insert(CARGO_TARGET_DIR_ENV.to_owned(), cargo_target_dir);
        } else {
            _self.env_variables.insert(
                CARGO_TARGET_DIR_ENV.to_owned(),
                CARGO_TARGET_DIR_DEFAULT.to_owned(),
            );
        }
        // hfuzz_workspace
        let hfuzz_workspace = _f.hfuzz_workspace.unwrap_or_default();
        if !hfuzz_workspace.is_empty() {
            _self
                .env_variables
                .insert(HFUZZ_WORKSPACE_ENV.to_owned(), hfuzz_workspace);
        } else {
            _self.env_variables.insert(
                HFUZZ_WORKSPACE_ENV.to_owned(),
                HFUZZ_WORKSPACE_DEFAULT.to_owned(),
            );
        }
        // crashdir
        let crash_dir = _f.crashdir.unwrap_or_default();
        if !crash_dir.is_empty() {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("", "--crashdir", &crash_dir));
        }
        // extension
        let extension = _f.extension.unwrap_or_default();
        if !extension.is_empty() {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("-e", "--extension", &extension));
        }
        // run_time
        let run_time = _f.run_time.unwrap_or(0);
        _self
            .fuzz_args
            .push(HonggFuzzArg::new("", "--run_time", &run_time.to_string()));

        // max_file_size
        let max_file_size = _f.max_file_size.unwrap_or(1_048_576);
        _self.fuzz_args.push(HonggFuzzArg::new(
            "-F",
            "--max_file_size",
            &max_file_size.to_string(),
        ));
        // save_all
        let save_all = _f.save_all.unwrap_or(false);
        if save_all {
            _self
                .fuzz_args
                .push(HonggFuzzArg::new("-u", "--save_all", ""));
        }
        _self
    }
}

impl HonggFuzzArg {
    pub(crate) fn new(short_opt: &str, long_opt: &str, val: &str) -> Self {
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
