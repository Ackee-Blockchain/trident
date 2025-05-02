use super::{Coverage, CoverageError};
use crate::coverage::constants::*;

pub struct AflCoverage {
    profraw_file: String,
    coverage_file: String,
    coverage_target_dir: String,
    fuzzer_loopcount: String,
    ignore_regex: String,
    rustflags: String,
}

impl Coverage for AflCoverage {
    fn new(cargo_target_dir: &str, fuzzer_loopcount: u64, target: &str) -> Self {
        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, AFL_PROFRAW_FILENAME),
            coverage_file: format!("{}/{}-{}", cargo_target_dir, target, AFL_COVERAGE_FILENAME),
            coverage_target_dir: cargo_target_dir.to_string(),
            fuzzer_loopcount: fuzzer_loopcount.to_string(),
            ignore_regex: COVERAGE_IGNORE_REGEX.to_string(),
            rustflags: AFL_COVERAGE_RUSTFLAGS.to_string(),
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

impl AflCoverage {
    pub async fn generate_report(&self) -> Result<(), CoverageError> {
        let result = self.try_generate_report(false).await;
        match result {
            Ok(_) => Ok(()),
            Err(CoverageError::CorruptedProfrawFiles) => self.try_generate_report(false).await,
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    async fn cleanup_test_dir() {
        let _ = fs::remove_dir_all("/tmp/test").await;
    }

    #[test]
    fn test_afl_coverage_new() {
        let target_dir = "/tmp/test";
        let loopcount = 1000;
        let target = "test-target";
        let coverage = AflCoverage::new(target_dir, loopcount, target);

        assert_eq!(
            coverage.get_profraw_file(),
            format!("{}/{}", target_dir, AFL_PROFRAW_FILENAME)
        );
        assert_eq!(
            coverage.get_coverage_file(),
            format!("{}/{}-{}", target_dir, target, AFL_COVERAGE_FILENAME)
        );
        assert_eq!(coverage.get_coverage_target_dir(), target_dir);
        assert_eq!(coverage.get_fuzzer_loopcount(), loopcount.to_string());
        assert_eq!(coverage.get_ignore_regex(), COVERAGE_IGNORE_REGEX);
        assert_eq!(coverage.get_rustflags(), AFL_COVERAGE_RUSTFLAGS);
    }

    #[tokio::test]
    async fn test_generate_report_from_non_existing_target() {
        let coverage = AflCoverage::new("/tmp/test", 1000, "test-target");

        let result = coverage.generate_report().await;
        assert!(result.is_err());

        cleanup_test_dir().await;
    }

    #[test]
    fn test_build_coverage_command_contains_required_args() {
        let coverage = AflCoverage::new("/tmp", 100, "test");
        let cmd = coverage.build_coverage_command(false);

        let cmd_str = format!("{:?}", cmd);

        // Basic command structure
        assert!(cmd_str.contains("cargo"));
        assert!(cmd_str.contains("llvm-cov"));
        assert!(cmd_str.contains("report"));

        // Required flags
        assert!(!cmd_str.contains("--release"));
        assert!(cmd_str.contains("--json"));
        assert!(cmd_str.contains("--skip-functions"));

        // Output configuration
        let output_path = format!("\"--output-path\" \"{}\"", coverage.get_coverage_file());
        assert!(cmd_str.contains(&output_path));

        // Ignore patterns
        let ignore_regex = format!("\"--ignore-filename-regex\" \"{}\"", coverage.get_ignore_regex());
        assert!(cmd_str.contains(&ignore_regex));

        // Environment variables
        let env_vars = format!("LLVM_PROFILE_FILE=\"{}\"", coverage.get_profraw_file());
        assert!(cmd_str.contains(&env_vars));
        let env_vars = format!(
            "CARGO_LLVM_COV_TARGET_DIR=\"{}\"",
            coverage.get_coverage_target_dir()
        );
        assert!(cmd_str.contains(&env_vars));
    }
}
