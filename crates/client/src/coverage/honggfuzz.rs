use super::{Coverage, CoverageError};
use crate::coverage::constants::*;
use std::process::Command;

pub struct HonggfuzzCoverage {
    profraw_file: String,
    coverage_file: String,
    coverage_target_dir: String,
    fuzzer_loopcount: String,
    ignore_regex: String,
    rustflags: String,
}

impl Coverage for HonggfuzzCoverage {
    fn new(cargo_target_dir: &str, fuzzer_loopcount: u64, target: &str) -> Self {
        let target_triple = HonggfuzzCoverage::target_triple();
        let cargo_target_dir = format!("{}/{}", cargo_target_dir, target_triple);

        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, HONGGFUZZ_PROFRAW_FILENAME),
            coverage_file: format!(
                "{}/{}-{}",
                cargo_target_dir, target, HONGGFUZZ_COVERAGE_FILENAME
            ),
            coverage_target_dir: cargo_target_dir.to_string(),
            fuzzer_loopcount: fuzzer_loopcount.to_string(),
            ignore_regex: COVERAGE_IGNORE_REGEX.to_string(),
            rustflags: HONGGFUZZ_COVERAGE_RUSTFLAGS.to_string(),
        }
    }

    fn get_profraw_file(&self) -> String {
        self.profraw_file.clone()
    }

    fn get_coverage_file(&self) -> String {
        self.coverage_file.clone()
    }

    fn get_coverage_target_dir(&self) -> String {
        self.coverage_target_dir.clone()
    }

    fn get_fuzzer_loopcount(&self) -> String {
        self.fuzzer_loopcount.clone()
    }

    fn get_ignore_regex(&self) -> String {
        self.ignore_regex.clone()
    }

    fn get_rustflags(&self) -> String {
        self.rustflags.clone()
    }
}

impl HonggfuzzCoverage {
    pub async fn generate_report(&self) -> Result<(), CoverageError> {
        let result = self.try_generate_report(true).await;
        match result {
            Ok(_) => Ok(()),
            Err(CoverageError::CorruptedProfrawFiles) => {
                self.try_generate_report(true).await
            }
            Err(e) => Err(e),
        }
    }

    fn target_triple() -> String {
        let output = Command::new("rustc").args(["-v", "-V"]).output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let triple = stdout
            .lines()
            .find(|l| l.starts_with("host: "))
            .unwrap()
            .get(6..)
            .unwrap();

        triple.into()
    }
}
