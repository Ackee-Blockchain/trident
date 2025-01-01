use crate::{
    argument::Argument,
    constants::*,
    utils::{arg_to_string_afl, resolve_path},
};
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
    pub timeout: Option<u64>,
    // seeds
    // -s
    pub seeds: Option<Vec<AflSeed>>,
}

impl Afl {
    pub fn get_cargo_build_dir(&self) -> Option<Argument> {
        // cargo_target_dir
        if let Some(cargo_target_dir) = &self.cargo_target_dir {
            let cargo_target_dir_full_path = resolve_path(cargo_target_dir);
            Some(Argument::new(
                "",
                "--target-dir",
                Some(cargo_target_dir_full_path.to_str().unwrap()),
            ))
        } else {
            Some(Argument::new(
                "",
                "--target-dir",
                Some(CARGO_TARGET_DIR_DEFAULT_AFL),
            ))
        }
    }
    pub fn get_workspace_in(&self) -> Option<Argument> {
        // afl_workspace_in
        if let Some(afl_workspace_in) = &self.afl_workspace_in {
            let afl_workspace_in_full_path = resolve_path(afl_workspace_in);
            Some(Argument::new(
                "-i",
                "",
                Some(afl_workspace_in_full_path.to_str().unwrap()),
            ))
        } else {
            Some(Argument::new("-i", "", Some(AFL_WORKSPACE_DEFAULT_IN)))
        }
    }
    pub fn get_workspace_out(&self) -> Option<Argument> {
        // afl_workspace_out
        if let Some(afl_workspace_out) = &self.afl_workspace_out {
            let afl_workspace_out_full_path = resolve_path(afl_workspace_out);
            Some(Argument::new(
                "-o",
                "",
                Some(afl_workspace_out_full_path.to_str().unwrap()),
            ))
        } else {
            Some(Argument::new("-o", "", Some(AFL_WORKSPACE_DEFAULT_OUT)))
        }
    }
    pub fn get_execs(&self) -> Option<Argument> {
        // execs
        self.iterations
            .as_ref()
            .map(|iterations| Argument::new("-E", "", Some(&iterations.to_string())))
    }
    pub fn get_seconds(&self) -> Option<Argument> {
        // seconds
        self.timeout
            .as_ref()
            .map(|timeout| Argument::new("-V", "", Some(&timeout.to_string())))
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
        let mut result = vec![];

        if let Some(cargo_target_dir) = self.get_cargo_build_dir() {
            result.extend(arg_to_string_afl(&cargo_target_dir));
        }
        result
    }
    pub fn get_collect_fuzz_args(&self) -> Vec<String> {
        let mut result = vec![];

        if let Some(afl_workspace_in) = self.get_workspace_in() {
            result.extend(arg_to_string_afl(&afl_workspace_in));
        }
        if let Some(afl_workspace_out) = self.get_workspace_out() {
            result.extend(arg_to_string_afl(&afl_workspace_out));
        }
        if let Some(execs) = self.get_execs() {
            result.extend(arg_to_string_afl(&execs));
        }
        if let Some(seconds) = self.get_seconds() {
            result.extend(arg_to_string_afl(&seconds));
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
                timeout: None,
                seeds: None,
            }
        }
    }

    #[test]
    fn test_cargo_target_dir() {
        let mut afl = Afl::clean();

        let target_dir = "/foo/bar".to_string();

        afl.cargo_target_dir = Some(target_dir);

        let arg = afl.get_collect_build_args();
        assert_eq!(arg, vec!["--target-dir", "/foo/bar"]);
    }
    #[test]
    fn test_workspace_in() {
        let mut afl = Afl::clean();

        // afl_workspace_in
        afl.afl_workspace_in = Some("/foo/bar/dead/beef".to_string());

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(
            arg,
            vec![
                "-i",
                "/foo/bar/dead/beef",
                "-o",
                "trident-tests/fuzzing/afl/afl_workspace/out"
            ]
        );
    }
    #[test]
    fn test_workspace_out() {
        let mut afl = Afl::clean();

        // afl_workspace_out
        afl.afl_workspace_out = Some("/foo/bar/dead/beef/out".to_string());

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(
            arg,
            vec![
                "-i",
                "trident-tests/fuzzing/afl/afl_workspace/in",
                "-o",
                "/foo/bar/dead/beef/out"
            ]
        );
    }
    #[test]
    fn test_execs() {
        let mut afl = Afl::clean();

        // execs
        afl.iterations = Some(555);

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(
            arg,
            vec![
                "-i",
                "trident-tests/fuzzing/afl/afl_workspace/in",
                "-o",
                "trident-tests/fuzzing/afl/afl_workspace/out",
                "-E",
                "555"
            ]
        );
    }
    #[test]
    fn test_seconds() {
        let mut afl = Afl::clean();

        // seconds
        afl.timeout = Some(15);

        let arg = afl.get_collect_fuzz_args();
        assert_eq!(
            arg,
            vec![
                "-i",
                "trident-tests/fuzzing/afl/afl_workspace/in",
                "-o",
                "trident-tests/fuzzing/afl/afl_workspace/out",
                "-V",
                "15"
            ]
        );
    }
}
