//! AFL-specific implementation of code coverage collection and reporting.
//!
//! This module provides functionality to generate and manage code coverage reports
//! when using the AFL fuzzer. It handles profraw file management and report generation
//! using LLVM coverage tools.

use super::{Coverage, CoverageError};
use crate::coverage::constants::*;
use std::path::PathBuf;
/// Coverage implementation specific to AFL fuzzing operations.
///
/// This struct manages coverage data collection and reporting for AFL fuzzing sessions,
/// including handling of profraw files and coverage report generation.
///
/// # Fields
/// * `profraw_file` - Path to the LLVM profile raw data file
/// * `coverage_file` - Path where the generated coverage report will be saved
/// * `coverage_target_dir` - Directory containing build artifacts and coverage data
/// * `fuzzer_loopcount` - Number of iterations each process must execute before finishing and writing gathered profraw data
/// * `ignore_regex` - Pattern for files to exclude from coverage analysis
/// * `rustflags` - Rust compiler flags for coverage instrumentation
/// * `dynamic_coverage` - Whether to use dynamic coverage collection

pub struct AflCoverage {
    profraw_file: String,
    coverage_file: String,
    coverage_target_dir: String,
    fuzzer_loopcount: String,
    ignore_regex: String,
    rustflags: String,
    dynamic_coverage: bool,
}

impl Coverage for AflCoverage {
    /// Creates a new instance of AflCoverage.
    ///
    /// # Arguments
    /// * `cargo_target_dir` - Base directory for build artifacts
    /// * `fuzzer_loopcount` - Number of iterations each process must execute before finishing and writing gathered profraw data
    /// * `target` - Name of the target being fuzzed
    /// * `dynamic_coverage` - Whether to use dynamic coverage collection
    ///
    /// # Returns
    /// A new AflCoverage instance configured for the specified target
    fn new(
        cargo_target_dir: &str,
        fuzzer_loopcount: u64,
        target: &str,
        dynamic_coverage: bool,
    ) -> Self {
        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, AFL_PROFRAW_FILENAME),
            coverage_file: format!("{}/{}-{}", cargo_target_dir, target, AFL_COVERAGE_FILENAME),
            coverage_target_dir: cargo_target_dir.to_string(),
            fuzzer_loopcount: fuzzer_loopcount.to_string(),
            ignore_regex: COVERAGE_IGNORE_REGEX.to_string(),
            rustflags: AFL_COVERAGE_RUSTFLAGS.to_string(),
            dynamic_coverage,
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

    fn get_dynamic_coverage(&self) -> bool {
        self.dynamic_coverage.clone()
    }

    /// Returns the root fuzzing directory by traversing up from the coverage target directory.
    ///
    /// This function navigates up two directory levels from the coverage target directory
    /// to locate the main fuzzing folder where AFL's input/output directories and other
    /// fuzzing-related data are stored.
    ///
    /// # Returns
    /// * `PathBuf` - Path to the fuzzing directory
    fn get_fuzzing_folder(&self) -> PathBuf {
        let target_dir = self.get_coverage_target_dir();
        let mut path = std::path::PathBuf::from(&target_dir);

        // Need to go up 2 levels to get to the fuzzing folder
        let levels = 2;
        for _ in 0..levels {
            if let Some(parent) = path.parent() {
                path = parent.to_path_buf();
            }
        }

        path
    }
}

impl AflCoverage {
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
        let result = self.try_generate_report(false).await;
        match result {
            Ok(_) => Ok(()),
            Err(CoverageError::CorruptedProfrawFiles) => self.try_generate_report(false).await,
            Err(e) => Err(e),
        }
    }

    /// Initializes and notifies the start of dynamic coverage collection for AFL fuzzing.
    ///
    /// This method sets up the necessary environment for dynamic coverage collection
    /// and signals that coverage tracking should begin. It is called just before
    /// the AFL fuzzing process starts when dynamic coverage is enabled.
    ///
    /// # Returns
    /// * `Ok(())` if dynamic coverage setup succeeds
    /// * `Err(CoverageError)` if setup fails
    pub async fn notify_dynamic_coverage_start(&self) -> Result<(), CoverageError> {
        self.setup_dynamic_coverage("AFL").await
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
        let coverage = AflCoverage::new(target_dir, loopcount, target, false);

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
        let coverage = AflCoverage::new("/tmp/test", 1000, "test-target", false);

        let result = coverage.generate_report().await;
        assert!(result.is_err());

        cleanup_test_dir().await;
    }

    #[test]
    fn test_build_coverage_command_contains_required_args() {
        let coverage = AflCoverage::new("/tmp", 100, "test", false);
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
        let ignore_regex = format!(
            "\"--ignore-filename-regex\" \"{}\"",
            coverage.get_ignore_regex()
        );
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
