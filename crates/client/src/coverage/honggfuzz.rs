//! Honggfuzz-specific implementation of code coverage collection and reporting.
//!
//! This module provides functionality to generate and manage code coverage reports
//! when using the Honggfuzz fuzzer. It handles profraw file management and report generation
//! using LLVM coverage tools.

use super::{Coverage, CoverageError};
use crate::coverage::constants::*;
use std::process::Command;

/// Coverage implementation specific to Honggfuzz fuzzing operations.
///
/// This struct manages coverage data collection and reporting for Honggfuzz fuzzing sessions,
/// including handling of profraw files and coverage report generation.
///
/// # Fields
/// * `profraw_file` - Path to the LLVM profile raw data file
/// * `coverage_file` - Path where the generated coverage report will be saved
/// * `coverage_target_dir` - Directory containing build artifacts and coverage data
/// * `fuzzer_loopcount` - Number of iterations each process must execute before finishing and writing gathered profraw data
/// * `ignore_regex` - Pattern for files to exclude from coverage analysis
/// * `rustflags` - Rust compiler flags for coverage instrumentation
pub struct HonggfuzzCoverage {
    profraw_file: String,
    coverage_file: String,
    coverage_target_dir: String,
    fuzzer_loopcount: String,
    ignore_regex: String,
    rustflags: String,
}

impl Coverage for HonggfuzzCoverage {
    /// Creates a new instance of HonggfuzzCoverage.
    ///
    /// # Arguments
    /// * `cargo_target_dir` - Base directory for build artifacts
    /// * `fuzzer_loopcount` - Number of iterations each process must execute before finishing and writing gathered profraw data
    /// * `target` - Name of the target being fuzzed
    ///
    /// # Returns
    /// A new HonggfuzzCoverage instance configured for the specified target
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
    /// Generates a coverage report for the current fuzzing session.
    ///
    /// This method attempts to generate a coverage report from the collected profraw data.
    /// If corrupted profraw files are encountered, it will attempt to clean them up and
    /// retry the report generation once.
    ///
    /// # Returns
    /// * `Ok(())` if the report was generated successfully
    /// * `Err(CoverageError)` if report generation failed
    ///
    /// # Errors
    /// Can return various `CoverageError` variants depending on the type of failure:
    /// * `CoverageError::GeneratingReportFailed` - If the initial report generation fails
    /// * `CoverageError::CorruptedProfrawFiles` - If corrupted profraw files are detected
    pub async fn generate_report(&self) -> Result<(), CoverageError> {
        let result = self.try_generate_report(true).await;
        match result {
            Ok(_) => Ok(()),
            Err(CoverageError::CorruptedProfrawFiles) => self.try_generate_report(true).await,
            Err(e) => Err(e),
        }
    }

    /// Determines the target triple for the current build environment.
    ///
    /// Executes `rustc -v -V` to get the host target triple, which is used to
    /// properly locate build artifacts and coverage data.
    ///
    /// # Returns
    /// The host target triple as a String (e.g., "x86_64-unknown-linux-gnu")
    ///
    /// # Panics
    /// Panics if unable to execute rustc or parse its output
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    async fn cleanup_test_dir() {
        let _ = fs::remove_dir_all("/tmp/test").await;
    }

    #[tokio::test]
    async fn test_honggfuzz_coverage_new() {
        let target_triple = HonggfuzzCoverage::target_triple();
        let coverage = HonggfuzzCoverage::new("/tmp/test", 1000, "test-target");

        assert_eq!(
            coverage.get_profraw_file(),
            format!("/tmp/test/{}/{}", target_triple, HONGGFUZZ_PROFRAW_FILENAME)
        );
        assert_eq!(
            coverage.get_coverage_file(),
            format!(
                "/tmp/test/{}/test-target-{}",
                target_triple, HONGGFUZZ_COVERAGE_FILENAME
            )
        );
        assert_eq!(
            coverage.get_coverage_target_dir(),
            format!("/tmp/test/{}", target_triple)
        );
        assert_eq!(coverage.get_fuzzer_loopcount(), "1000");
        assert_eq!(coverage.get_ignore_regex(), COVERAGE_IGNORE_REGEX);
        assert_eq!(coverage.get_rustflags(), HONGGFUZZ_COVERAGE_RUSTFLAGS);
    }

    #[test]
    fn test_target_triple_format() {
        let triple = HonggfuzzCoverage::target_triple();

        assert!(triple.contains('-'));
        assert!(!triple.contains("host: "));
        assert!(!triple.is_empty());
    }

    #[tokio::test]
    async fn test_generate_report_from_non_existing_target() {
        let coverage = HonggfuzzCoverage::new("/tmp/test", 1000, "test-target");
        let result = coverage.generate_report().await;
        assert!(result.is_err());

        cleanup_test_dir().await;
    }

    #[test]
    fn test_build_coverage_command_contains_required_args() {
        let coverage = HonggfuzzCoverage::new("/tmp/test", 100, "test");
        let cmd = coverage.build_coverage_command(true);

        let cmd_str = format!("{:?}", cmd);
        // Basic command structure
        assert!(cmd_str.contains("cargo"));
        assert!(cmd_str.contains("llvm-cov"));
        assert!(cmd_str.contains("report"));

        // Required flags
        assert!(cmd_str.contains("--release"));
        assert!(cmd_str.contains("--json"));
        assert!(cmd_str.contains("--skip-functions"));

        // Output configuration
        let output_path = format!("\"--output-path\" \"{}\"", coverage.get_coverage_file());
        assert!(cmd_str.contains(output_path.as_str()));

        // Ignore patterns
        let ignore_regex = format!(
            "\"--ignore-filename-regex\" \"{}\"",
            coverage.get_ignore_regex()
        );
        assert!(cmd_str.contains(ignore_regex.as_str()));

        // Environment variables
        let env_vars = format!("LLVM_PROFILE_FILE=\"{}\"", coverage.get_profraw_file());
        assert!(cmd_str.contains(env_vars.as_str()));
        let env_vars = format!(
            "CARGO_LLVM_COV_TARGET_DIR=\"{}\"",
            coverage.get_coverage_target_dir()
        );
        assert!(cmd_str.contains(env_vars.as_str()));
    }
}
