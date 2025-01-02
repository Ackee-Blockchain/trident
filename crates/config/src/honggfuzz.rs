use serde::Deserialize;

use crate::{
    argument::{Argument, EnvironmentVariable},
    constants::*,
    utils::arg_to_string,
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct HonggFuzz {
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
    /// Target compilation directory, defaults to "trident-tests/fuzzing/hfuzz_target" to not clash with cargo build's default target directory.
    /// CARGO_TARGET_DIR env variable
    pub cargo_target_dir: Option<String>,
    #[serde(default)]
    /// Honggfuzz working directory, defaults to "trident-tests/fuzzing/hfuzz_workspace".
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

impl HonggFuzz {
    pub fn get_timeout(&self) -> Option<Argument> {
        // timeout
        self.timeout
            .map(|timeout| Argument::new("-t", "--timeout", Some(&timeout.to_string())))
    }
    pub fn get_iterations(&self) -> Option<Argument> {
        // iterations
        self.iterations
            .map(|iterations| Argument::new("-N", "--iterations", Some(&iterations.to_string())))
    }
    pub fn get_threads(&self) -> Option<Argument> {
        // threads
        self.threads
            .map(|threads| Argument::new("-n", "--threads", Some(&threads.to_string())))
    }
    pub fn get_keep_output(&self) -> Option<Argument> {
        // keep_output
        self.keep_output.and_then(|keep_output| {
            if keep_output {
                Some(Argument::new("-Q", "--keep_output", None))
            } else {
                None
            }
        })
    }
    pub fn get_verbose(&self) -> Option<Argument> {
        // verbose
        self.verbose.and_then(|verbose| {
            if verbose {
                Some(Argument::new("-v", "--verbose", None))
            } else {
                None
            }
        })
    }
    pub fn get_exit_upon_crash(&self) -> Option<Argument> {
        // exit_upon_crash
        self.exit_upon_crash.and_then(|exit_upon_crash| {
            if exit_upon_crash {
                Some(Argument::new("", "--exit_upon_crash", None))
            } else {
                None
            }
        })
    }
    pub fn get_mutations_per_run(&self) -> Option<Argument> {
        // mutations_per_run
        self.mutations_per_run.map(|mutations_per_run| {
            Argument::new(
                "-r",
                "--mutations_per_run",
                Some(&mutations_per_run.to_string()),
            )
        })
    }
    pub fn get_crashdir(&self) -> Option<Argument> {
        // crashdir
        self.crashdir
            .as_ref()
            .map(|crashdir| Argument::new("", "--crashdir", Some(crashdir)))
    }
    pub fn get_extension(&self) -> Option<Argument> {
        // extension
        self.extension
            .as_ref()
            .map(|extension| Argument::new("-e", "--extension", Some(extension)))
    }
    pub fn get_run_time(&self) -> Option<Argument> {
        // run_time
        self.run_time
            .map(|run_time| Argument::new("", "--run_time", Some(&run_time.to_string())))
    }
    pub fn get_max_file_size(&self) -> Option<Argument> {
        // max_file_size
        self.max_file_size.map(|max_file_size| {
            Argument::new("-F", "--max_file_size", Some(&max_file_size.to_string()))
        })
    }
    pub fn get_save_all(&self) -> Option<Argument> {
        // save_all
        self.save_all.and_then(|save_all| {
            if save_all {
                Some(Argument::new("-u", "--save_all", None))
            } else {
                None
            }
        })
    }
    pub fn get_cargo_target_dir(&self) -> EnvironmentVariable {
        // cargo_target_dir
        if let Some(cargo_target_dir) = &self.cargo_target_dir {
            EnvironmentVariable::new(
                CARGO_TARGET_DIR_ENV.to_string(),
                cargo_target_dir.to_string(),
            )
        } else {
            EnvironmentVariable::new(
                CARGO_TARGET_DIR_ENV.to_string(),
                CARGO_TARGET_DIR_DEFAULT_HFUZZ.to_string(),
            )
        }
    }
    pub fn get_hfuzz_workspace(&self) -> EnvironmentVariable {
        // hfuzz_workspace
        if let Some(hfuzz_workspace) = &self.hfuzz_workspace {
            EnvironmentVariable::new(HFUZZ_WORKSPACE_ENV.to_string(), hfuzz_workspace.to_string())
        } else {
            EnvironmentVariable::new(
                HFUZZ_WORKSPACE_ENV.to_string(),
                HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_string(),
            )
        }
    }
    pub fn get_collect_fuzz_args(&self) -> Vec<String> {
        let mut result = vec![];

        if let Some(timeout) = self.get_timeout() {
            result.extend(arg_to_string(&timeout));
        }
        if let Some(iterations) = self.get_iterations() {
            result.extend(arg_to_string(&iterations));
        }
        if let Some(threads) = self.get_threads() {
            result.extend(arg_to_string(&threads));
        }
        if let Some(keep_output) = self.get_keep_output() {
            result.extend(arg_to_string(&keep_output));
        }
        if let Some(verbose) = self.get_verbose() {
            result.extend(arg_to_string(&verbose));
        }
        if let Some(exit_upon_crash) = self.get_exit_upon_crash() {
            result.extend(arg_to_string(&exit_upon_crash));
        }
        if let Some(mutations_per_run) = self.get_mutations_per_run() {
            result.extend(arg_to_string(&mutations_per_run));
        }
        if let Some(crashdir) = self.get_crashdir() {
            result.extend(arg_to_string(&crashdir));
        }
        if let Some(extension) = self.get_extension() {
            result.extend(arg_to_string(&extension));
        }
        if let Some(run_time) = self.get_run_time() {
            result.extend(arg_to_string(&run_time));
        }
        if let Some(max_file_size) = self.get_max_file_size() {
            result.extend(arg_to_string(&max_file_size));
        }
        if let Some(save_all) = self.get_save_all() {
            result.extend(arg_to_string(&save_all));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl HonggFuzz {
        fn clean() -> Self {
            Self {
                timeout: None,
                iterations: None,
                threads: None,
                keep_output: None,
                verbose: None,
                exit_upon_crash: None,
                mutations_per_run: None,
                cargo_target_dir: None,
                hfuzz_workspace: None,
                crashdir: None,
                extension: None,
                run_time: None,
                max_file_size: None,
                save_all: None,
            }
        }
    }

    #[test]
    fn test_timeout() {
        let mut honggfuzz = HonggFuzz::clean();

        // timeout
        let timeout = 10;

        honggfuzz.timeout = Some(timeout);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-t", "10"]);
    }
    #[test]
    fn test_iterations() {
        let mut honggfuzz = HonggFuzz::clean();

        // iterations
        let iterations = 1000;
        honggfuzz.iterations = Some(iterations);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-N", "1000"]);
    }
    #[test]
    fn test_threads() {
        let mut honggfuzz = HonggFuzz::clean();

        // threads
        let threads = 15;
        honggfuzz.threads = Some(threads);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-n", "15"]);
    }
    #[test]
    fn test_keep_output() {
        let mut honggfuzz = HonggFuzz::clean();

        // keep_output
        honggfuzz.keep_output = Some(true);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-Q", ""]);
    }
    #[test]
    fn test_verbose() {
        let mut honggfuzz = HonggFuzz::clean();

        // verbose
        honggfuzz.verbose = Some(true);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-v", ""]);
    }
    #[test]
    fn test_exit_upon_crash() {
        let mut honggfuzz = HonggFuzz::clean();

        // exit_upon_crash
        honggfuzz.exit_upon_crash = Some(true);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--exit_upon_crash", ""]);
    }
    #[test]
    fn test_mutations_per_run() {
        let mut honggfuzz = HonggFuzz::clean();

        // mutations_per_run
        let mutations_per_run = 33;
        honggfuzz.mutations_per_run = Some(mutations_per_run);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-r", "33"]);
    }
    #[test]
    fn test_crashdir() {
        let mut honggfuzz = HonggFuzz::clean();

        let crash_dir = "crashdir1";
        honggfuzz.crashdir = Some(crash_dir.to_string());

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--crashdir", "crashdir1"]);
    }
    #[test]
    fn test_extension() {
        let mut honggfuzz = HonggFuzz::clean();

        // extension
        let extension = "sol";
        honggfuzz.extension = Some(extension.to_string());

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-e", "sol"]);
    }
    #[test]
    fn test_run_time() {
        let mut honggfuzz = HonggFuzz::clean();

        // run_time
        let run_time = 13;
        honggfuzz.run_time = Some(run_time);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["--run_time", "13"]);
    }
    #[test]
    fn test_max_file_size() {
        let mut honggfuzz = HonggFuzz::clean();

        // max_file_size
        let max_file_size = 500;
        honggfuzz.max_file_size = Some(max_file_size);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-F", "500"]);
    }
    #[test]
    fn test_save_all() {
        let mut honggfuzz = HonggFuzz::clean();

        // save_all
        honggfuzz.save_all = Some(true);

        let arg = honggfuzz.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-u", ""]);
    }
    #[test]
    fn test_cargo_target_dir() {
        let mut honggfuzz = HonggFuzz::clean();

        // cargo_target_dir
        honggfuzz.cargo_target_dir = Some("/foo/bar/target".to_string());

        let arg = honggfuzz.get_cargo_target_dir().value;
        assert_eq!(arg, "/foo/bar/target");
    }
    #[test]
    fn test_hfuzz_workspace() {
        let mut honggfuzz = HonggFuzz::clean();

        // hfuzz_workspace
        honggfuzz.hfuzz_workspace = Some("/foo/bar/workspace".to_string());

        let arg = honggfuzz.get_hfuzz_workspace().value;
        assert_eq!(arg, "/foo/bar/workspace");
    }
}
