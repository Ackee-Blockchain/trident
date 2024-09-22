use serde::Deserialize;
use std::collections::HashMap;

use crate::config::constants::*;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FuzzArgument {
    Timeout,
    Iterations,
    Threads,
    KeepOutput,
    Verbose,
    ExitUponCrash,
    MutationsPerRun,
    CargoTargetDir,
    HfuzzWorkspace,
    Crashdir,
    Extension,
    RunTime,
    MaxFileSize,
    SaveAll,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum EnvVariable {
    CargoTargetDir,
    HfuzzWorkspace,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HonggFuzzArg {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    pub val: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct HonggFuzz {
    pub fuzz_args: HashMap<FuzzArgument, HonggFuzzArg>,
    pub env_variables: HashMap<EnvVariable, String>,
}

impl HonggFuzz {
    pub fn get_timeout(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Timeout)
    }
    pub fn get_iterations(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Iterations)
    }
    pub fn get_threads(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Threads)
    }
    pub fn get_keep_output(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::KeepOutput)
    }
    pub fn get_verbose(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Verbose)
    }
    pub fn get_exit_upon_crash(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::ExitUponCrash)
    }
    pub fn get_mutations_per_run(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::MutationsPerRun)
    }
    pub fn get_crashdir(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Crashdir)
    }
    pub fn get_extension(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::Extension)
    }
    pub fn get_run_time(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::RunTime)
    }
    pub fn get_max_file_size(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::MaxFileSize)
    }
    pub fn get_save_all(&self) -> Option<&HonggFuzzArg> {
        self.fuzz_args.get(&FuzzArgument::SaveAll)
    }
    pub fn get_cargo_target_dir(&self) -> Option<&String> {
        self.env_variables.get(&EnvVariable::CargoTargetDir)
    }
    pub fn get_hfuzz_workspace(&self) -> Option<&String> {
        self.env_variables.get(&EnvVariable::HfuzzWorkspace)
    }

    pub fn get_collect_fuzz_args(&self) -> Vec<String> {
        self.fuzz_args
            .values()
            .map(|arg| {
                if let Some(opt) = &arg.short_opt {
                    match &arg.val {
                        Some(value) => format!("{} {}", opt, value),
                        None => opt.to_string(),
                    }
                } else if let Some(opt) = &arg.long_opt {
                    match &arg.val {
                        Some(value) => format!("{} {}", opt, value),
                        None => opt.to_string(),
                    }
                } else {
                    "".to_string()
                }
            })
            .collect()
    }
    pub fn get_env_variable(&self, key: &EnvVariable) -> Option<String> {
        self.env_variables.get(key).cloned()
    }
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
            fuzz_args: HashMap::new(),
            env_variables: HashMap::new(),
        };

        // timeout
        let timeout = _f.timeout.unwrap_or(10);
        _self.fuzz_args.insert(
            FuzzArgument::Timeout,
            HonggFuzzArg::new("-t", "--timeout", &timeout.to_string()),
        );

        // iterations
        let iterations = _f.iterations.unwrap_or(0);
        _self.fuzz_args.insert(
            FuzzArgument::Iterations,
            HonggFuzzArg::new("-N", "--iterations", &iterations.to_string()),
        );

        // threads
        let threads = _f.threads.unwrap_or(0);
        if threads > 0 {
            _self.fuzz_args.insert(
                FuzzArgument::Threads,
                HonggFuzzArg::new("-n", "--threads", &threads.to_string()),
            );
        }

        // keep_output
        let keep_output = _f.keep_output.unwrap_or(false);
        if keep_output {
            _self.fuzz_args.insert(
                FuzzArgument::KeepOutput,
                HonggFuzzArg::new("-Q", "--keep_output", ""),
            );
        }
        // verbose
        let verbose = _f.verbose.unwrap_or(false);
        if verbose {
            _self.fuzz_args.insert(
                FuzzArgument::Verbose,
                HonggFuzzArg::new("-v", "--verbose", ""),
            );
        }

        // exit_upon_crash
        let exit_upon_crash = _f.exit_upon_crash.unwrap_or(false);
        if exit_upon_crash {
            _self.fuzz_args.insert(
                FuzzArgument::ExitUponCrash,
                HonggFuzzArg::new("", "--exit_upon_crash", ""),
            );
        }
        // mutations_per_run
        let mutations_per_run = _f.mutations_per_run.unwrap_or(6);
        _self.fuzz_args.insert(
            FuzzArgument::MutationsPerRun,
            HonggFuzzArg::new("-r", "--mutations_per_run", &mutations_per_run.to_string()),
        );
        // cargo_target_dir
        let cargo_target_dir = _f
            .cargo_target_dir
            .and_then(|value| if value.is_empty() { None } else { Some(value) })
            .unwrap_or(CARGO_TARGET_DIR_DEFAULT_HFUZZ.to_owned());

        _self
            .env_variables
            .insert(EnvVariable::CargoTargetDir, cargo_target_dir);

        // hfuzz_workspace
        let hfuzz_workspace = _f
            .hfuzz_workspace
            .and_then(|value| if value.is_empty() { None } else { Some(value) })
            .unwrap_or(HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_owned());

        _self
            .env_variables
            .insert(EnvVariable::HfuzzWorkspace, hfuzz_workspace);

        // crashdir
        let crash_dir = _f.crashdir.unwrap_or_default();
        if !crash_dir.is_empty() {
            _self.fuzz_args.insert(
                FuzzArgument::Crashdir,
                HonggFuzzArg::new("", "--crashdir", &crash_dir),
            );
        }
        // extension
        let extension = _f.extension.unwrap_or_default();
        if !extension.is_empty() {
            _self.fuzz_args.insert(
                FuzzArgument::Extension,
                HonggFuzzArg::new("-e", "--extension", &extension),
            );
        }
        // run_time
        let run_time = _f.run_time.unwrap_or(0);
        _self.fuzz_args.insert(
            FuzzArgument::RunTime,
            HonggFuzzArg::new("", "--run_time", &run_time.to_string()),
        );

        // max_file_size
        let max_file_size = _f.max_file_size.unwrap_or(1_048_576);
        _self.fuzz_args.insert(
            FuzzArgument::MaxFileSize,
            HonggFuzzArg::new("-F", "--max_file_size", &max_file_size.to_string()),
        );
        // save_all
        let save_all = _f.save_all.unwrap_or_default();
        if save_all {
            _self.fuzz_args.insert(
                FuzzArgument::SaveAll,
                HonggFuzzArg::new("-u", "--save_all", ""),
            );
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

#[cfg(test)]
mod tests {
    use super::*;

    impl HonggFuzz {
        fn clean() -> Self {
            Self {
                fuzz_args: HashMap::new(),
                env_variables: HashMap::new(),
            }
        }
    }

    #[test]
    fn test_timeout() {
        let mut honggfuzz = HonggFuzz::clean();

        // timeout
        let timeout = 10;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Timeout,
            HonggFuzzArg::new("-t", "--timeout", &timeout.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-t 10"]);
    }
    #[test]
    fn test_iterations() {
        let mut honggfuzz = HonggFuzz::clean();

        // iterations
        let iterations = 1000;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Iterations,
            HonggFuzzArg::new("-N", "--iterations", &iterations.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-N 1000"]);
    }
    #[test]
    fn test_threads() {
        let mut honggfuzz = HonggFuzz::clean();

        // threads
        let threads = 15;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Threads,
            HonggFuzzArg::new("-n", "--threads", &threads.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-n 15"]);
    }
    #[test]
    fn test_keep_output() {
        let mut honggfuzz = HonggFuzz::clean();

        // keep_output
        honggfuzz.fuzz_args.insert(
            FuzzArgument::KeepOutput,
            HonggFuzzArg::new("-Q", "--keep_output", ""),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-Q"]);
    }
    #[test]
    fn test_verbose() {
        let mut honggfuzz = HonggFuzz::clean();

        // verbose
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Verbose,
            HonggFuzzArg::new("-v", "--verbose", ""),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-v"]);
    }
    #[test]
    fn test_exit_upon_crash() {
        let mut honggfuzz = HonggFuzz::clean();

        // exit_upon_crash
        honggfuzz.fuzz_args.insert(
            FuzzArgument::ExitUponCrash,
            HonggFuzzArg::new("", "--exit_upon_crash", ""),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--exit_upon_crash"]);
    }
    #[test]
    fn test_mutations_per_run() {
        let mut honggfuzz = HonggFuzz::clean();

        // mutations_per_run
        let mutations_per_run = 33;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::MutationsPerRun,
            HonggFuzzArg::new("-r", "--mutations_per_run", &mutations_per_run.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-r 33"]);
    }
    #[test]
    fn test_crashdir() {
        let mut honggfuzz = HonggFuzz::clean();

        let crash_dir = "crashdir1";
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Crashdir,
            HonggFuzzArg::new("", "--crashdir", crash_dir),
        );
        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--crashdir crashdir1"]);
    }
    #[test]
    fn test_extension() {
        let mut honggfuzz = HonggFuzz::clean();

        // extension
        let extension = "sol";
        honggfuzz.fuzz_args.insert(
            FuzzArgument::Extension,
            HonggFuzzArg::new("-e", "--extension", extension),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-e sol"]);
    }
    #[test]
    fn test_run_time() {
        let mut honggfuzz = HonggFuzz::clean();

        // run_time
        let run_time = 13;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::RunTime,
            HonggFuzzArg::new("", "--run_time", &run_time.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--run_time 13"]);
    }
    #[test]
    fn test_max_file_size() {
        let mut honggfuzz = HonggFuzz::clean();

        // max_file_size
        let max_file_size = 500;
        honggfuzz.fuzz_args.insert(
            FuzzArgument::MaxFileSize,
            HonggFuzzArg::new("-F", "--max_file_size", &max_file_size.to_string()),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-F 500"]);
    }
    #[test]
    fn test_save_all() {
        let mut honggfuzz = HonggFuzz::clean();

        // save_all
        honggfuzz.fuzz_args.insert(
            FuzzArgument::SaveAll,
            HonggFuzzArg::new("-u", "--save_all", ""),
        );

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-u"]);
    }
    #[test]
    fn test_cargo_target_dir() {
        let mut honggfuzz = HonggFuzz::clean();

        // cargo_target_dir
        honggfuzz.env_variables.insert(
            EnvVariable::CargoTargetDir,
            CARGO_TARGET_DIR_DEFAULT_HFUZZ.to_owned(),
        );

        let arg = honggfuzz.get_cargo_target_dir().unwrap();
        assert_eq!(arg, CARGO_TARGET_DIR_DEFAULT_HFUZZ);
    }
    #[test]
    fn test_hfuzz_workspace() {
        let mut honggfuzz = HonggFuzz::clean();

        // hfuzz_workspace
        honggfuzz.env_variables.insert(
            EnvVariable::HfuzzWorkspace,
            HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_owned(),
        );

        let arg = honggfuzz.get_hfuzz_workspace().unwrap();
        assert_eq!(arg, HFUZZ_WORKSPACE_DEFAULT_HFUZZ);
    }
}
