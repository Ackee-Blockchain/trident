use crate::{argument::Argument, constants::*, utils::arg_to_string};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Afl {
    // cargo_target_dir
    // --target-dir
    pub cargo_target_dir: Option<String>,
    // afl_workspace_in
    // -i
    pub afl_workspace_in: Option<String>,
    // afl_workspace_out
    // -o
    pub afl_workspace_out: Option<String>,
    // execs
    // -E
    pub iterations: Option<u64>,
    // seconds
    // -V
    pub run_time: Option<u64>,
    // seeds
    // -s
    pub seeds: Option<Vec<AflSeed>>,
}

impl Afl {
    pub fn get_cargo_target_dir(&self) -> Argument {
        // cargo_target_dir
        if let Some(cargo_target_dir) = &self.cargo_target_dir {
            Argument::new("", "--target-dir", Some(cargo_target_dir))
        } else {
            Argument::new("", "--target-dir", Some(CARGO_TARGET_DIR_DEFAULT_AFL))
        }
    }
    pub fn get_workspace_in(&self) -> Argument {
        // afl_workspace_in
        if let Some(afl_workspace_in) = &self.afl_workspace_in {
            Argument::new("-i", "", Some(afl_workspace_in))
        } else {
            Argument::new("-i", "", Some(AFL_WORKSPACE_DEFAULT_IN))
        }
    }
    pub fn get_workspace_out(&self) -> Argument {
        // afl_workspace_out
        if let Some(afl_workspace_out) = &self.afl_workspace_out {
            Argument::new("-o", "", Some(afl_workspace_out))
        } else {
            Argument::new("-o", "", Some(AFL_WORKSPACE_DEFAULT_OUT))
        }
    }
    pub fn get_iterations(&self) -> Option<Argument> {
        // execs
        self.iterations
            .as_ref()
            .filter(|&iterations| *iterations > 0)
            .map(|iterations| Argument::new("-E", "", Some(&iterations.to_string())))
    }
    pub fn get_run_time(&self) -> Option<Argument> {
        // seconds
        self.run_time
            .as_ref()
            .filter(|&run_time| *run_time > 0)
            .map(|run_time| Argument::new("-V", "", Some(&run_time.to_string())))
    }
    pub fn get_seeds(&self) -> Vec<AflSeed> {
        // seeds
        if let Some(seeds) = &self.seeds {
            seeds.clone()
        } else {
            vec![AflSeed::default()]
        }
    }
    pub fn get_collect_build_args(&self) -> Vec<String> {
        #[allow(unused_mut)]
        let mut result = vec![];
        // we do not have any build arguments so far.
        result
    }
    pub fn get_collect_fuzz_args(&self) -> Vec<String> {
        let mut result = vec![];

        if let Some(execs) = self.get_iterations() {
            result.extend(arg_to_string(&execs));
        }
        if let Some(seconds) = self.get_run_time() {
            result.extend(arg_to_string(&seconds));
        }
        result
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AflSeed {
    pub file_name: String,
    pub seed: Option<String>,
    pub override_file: Option<bool>,
    pub bytes_count: Option<usize>,
}

impl Default for AflSeed {
    fn default() -> Self {
        Self {
            file_name: DEFAULT_SEED_FILENAME.to_string(),
            seed: Some(DEFAULT_SEED.to_string()),
            override_file: Some(false),
            bytes_count: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Afl {
        fn clean() -> Self {
            Self {
                cargo_target_dir: None,
                afl_workspace_in: None,
                afl_workspace_out: None,
                iterations: None,
                run_time: None,
                seeds: None,
            }
        }
    }

    #[test]
    fn test_cargo_target_dir() {
        let mut afl = Afl::clean();

        let target_dir = "/foo/bar".to_string();

        afl.cargo_target_dir = Some(target_dir);

        let arg = afl.get_cargo_target_dir();
        assert_eq!(arg, Argument::new("", "--target-dir", Some("/foo/bar")));
    }
    #[test]
    fn test_workspace_in() {
        let mut afl = Afl::clean();

        // afl_workspace_in
        afl.afl_workspace_in = Some("/foo/bar/dead/beef".to_string());

        let arg = afl.get_workspace_in();
        assert_eq!(arg, Argument::new("-i", "", Some("/foo/bar/dead/beef")));
    }
    #[test]
    fn test_workspace_out() {
        let mut afl = Afl::clean();

        // afl_workspace_out
        afl.afl_workspace_out = Some("/foo/bar/dead/beef/out".to_string());

        let arg = afl.get_workspace_out();
        assert_eq!(arg, Argument::new("-o", "", Some("/foo/bar/dead/beef/out")));
    }
    #[test]
    fn test_iterations() {
        let mut afl = Afl::clean();

        // execs
        afl.iterations = Some(555);

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-E", "555"]);
    }
    #[test]
    fn test_timeout() {
        let mut afl = Afl::clean();

        // seconds
        afl.run_time = Some(15);

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(arg, vec!["-V", "15"]);
    }
}
