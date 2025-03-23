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
    fn new(cargo_target_dir: &str, fuzzer_loopcount: u64) -> Self {
        let target_triple = HonggfuzzCoverage::target_triple();
        let cargo_target_dir = format!("{}/{}", cargo_target_dir, target_triple);

        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, HONGGFUZZ_PROFRAW_FILENAME),
            coverage_file: format!("{}/{}", cargo_target_dir, HONGGFUZZ_COVERAGE_FILENAME),
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
        let mut child = tokio::process::Command::new("cargo")
            .env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_coverage_target_dir())
            .arg("llvm-cov")
            .arg("report")
            .arg("--json")
            .arg("--skip-functions")
            .arg("--release")
            .args(["--output-path", &self.get_coverage_file()])
            .args(["--ignore-filename-regex", &self.get_ignore_regex()])
            .spawn()
            .map_err(|_| CoverageError::GeneratingReportFailed)?;

        Self::handle_child(&mut child, CoverageError::GeneratingReportFailed).await
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
