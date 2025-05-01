pub mod afl;
pub mod constants;
pub mod honggfuzz;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr};

#[derive(Error, Debug, Clone)]
pub enum CoverageError {
    #[error("Cleaning of coverage directory failed")]
    CleaningFailed,
    #[error("Coverage report generation failed")]
    GeneratingReportFailed,
    #[error("Found corrupted profraw files")]
    CorruptedProfrawFiles,
    #[error("Coverage generation failed due to process interruption")]
    CoverageGenerationInterrupted,
    #[error("Failed to extract corrupted files")]
    ExtractingCorruptedFilesFailed,
}

pub trait Coverage {
    fn new(cargo_target_dir: &str, fuzzer_loopcount: u64, target: &str) -> Self
    where
        Self: Sized;

    fn get_profraw_file(&self) -> String;
    fn get_coverage_file(&self) -> String;
    fn get_coverage_target_dir(&self) -> String;
    fn get_ignore_regex(&self) -> String;
    fn get_rustflags(&self) -> String;
    fn get_fuzzer_loopcount(&self) -> String;

    async fn handle_child(
        &self,
        child: &mut Child,
        error: CoverageError,
    ) -> Result<(), CoverageError> {
        tokio::select! {
            res = child.wait() => {
                if !res.map_err(|_| error.clone())?.success() {
                    return Err(error);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                if let Ok(status) = child.wait().await {
                    if !status.success() {
                        return Err(CoverageError::CoverageGenerationInterrupted);
                    }
                }
            }
        }
        Ok(())
    }

    async fn clean(&self) -> Result<(), CoverageError> {
        let profraw_list_path =
            std::path::Path::new(&self.get_coverage_target_dir()).join(constants::PROFRAW_LIST);
        if let Err(e) = tokio::fs::remove_file(profraw_list_path).await {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(CoverageError::CleaningFailed);
            }
        }

        let mut child = tokio::process::Command::new("cargo")
            .env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_coverage_target_dir())
            .arg("llvm-cov")
            .arg("clean")
            .arg("--workspace")
            .spawn()
            .map_err(|_| CoverageError::CleaningFailed)?;

        self.handle_child(&mut child, CoverageError::CleaningFailed)
            .await
    }

    async fn extract_corrupted_files(
        &self,
        reader: &mut BufReader<ChildStderr>,
    ) -> Result<Vec<String>, CoverageError> {
        let mut corrupted_files = Vec::new();
        let mut lines = reader.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|_| CoverageError::ExtractingCorruptedFilesFailed)?
        {
            if line.contains(".profraw: invalid instrumentation profile data") {
                if let Some(file_path) = line
                    .split("warning: ")
                    .nth(1)
                    .and_then(|s| s.split(':').next())
                {
                    corrupted_files.push(file_path.to_string());
                }
            }
        }
        Ok(corrupted_files)
    }

    async fn remove_files(&self, files: &[String]) {
        for file in files {
            let _ = tokio::fs::remove_file(file).await;
        }
    }

    fn build_coverage_command(&self, release: bool) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new("cargo");
        cmd.env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_coverage_target_dir())
            .arg("llvm-cov")
            .arg("report")
            .arg("--json")
            .arg("--skip-functions")
            .args(["--output-path", &self.get_coverage_file()])
            .args(["--ignore-filename-regex", &self.get_ignore_regex()]);

        if release {
            cmd.arg("--release");
        }

        cmd
    }

    async fn try_generate_report(&self, release: bool) -> Result<(), CoverageError> {
        let mut child = self
            .build_coverage_command(release)
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|_| CoverageError::GeneratingReportFailed)?;

        match self
            .handle_child(&mut child, CoverageError::GeneratingReportFailed)
            .await
        {
            Ok(_) => {
                println!("Report has been successfully generated.");
                Ok(())
            }
            Err(_) => {
                println!("Report generation failed, attempting to remove corrupted files.");
                self.remove_corrupted_files(&mut child).await?;
                Err(CoverageError::CorruptedProfrawFiles)
            }
        }
    }

    async fn remove_corrupted_files(&self, child: &mut Child) -> Result<(), CoverageError> {
        let stderr = child
            .stderr
            .take()
            .expect("Child did not have a handle to stderr.");
        let mut reader = BufReader::new(stderr);
        let corrupted_files = self.extract_corrupted_files(&mut reader).await?;

        if !corrupted_files.is_empty() {
            self.remove_files(&corrupted_files).await;
        }

        Ok(())
    }
}
