pub mod constants;
use constants::*;
pub mod coverage_format;
use coverage_format::*;

use thiserror::Error;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::BufReader;
use tokio::process::Child;

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
    #[error("Invalid report format")]
    InvalidReportFormat,
    #[error("llvm-tools-preview is not installed. Please install it with: rustup component add llvm-tools-preview")]
    LlvmToolsNotInstalled,
}

#[derive(PartialEq)]
pub enum NotificationType {
    Setup,
    DisplayFinalReport,
}

impl NotificationType {
    pub fn endpoint(&self) -> &str {
        match self {
            NotificationType::Setup => SETUP_DYNAMIC_COVERAGE,
            NotificationType::DisplayFinalReport => DISPLAY_FINAL_REPORT,
        }
    }
}

pub struct Coverage {
    profraw_file: String,
    report_path: String,
    target_dir: String,
    target: String,
    ignore_regex: String,
    rustflags: String,
    notify_extension: bool,
    format: CoverageFormat,
    loop_count: u64,
    coverage_server_port: u16,
}

impl Coverage {
    pub fn new(
        cargo_target_dir: &str,
        target: &str,
        notify_extension: bool,
        format: String,
        loop_count: u64,
        coverage_server_port: u16,
    ) -> Self {
        let report_format = CoverageFormat::from_str(&format).expect(&format!(
            "Invalid coverage format '{}'. Supported formats: json, html",
            format
        ));

        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, PROFRAW_FILENAME),
            report_path: format!(
                "{}/../{}-{}",
                cargo_target_dir,
                target,
                report_format.get_report_filename()
            ),
            target_dir: cargo_target_dir.to_string(),
            target: target.to_string(),
            ignore_regex: COVERAGE_IGNORE_REGEX.to_string(),
            rustflags: COVERAGE_RUSTFLAGS.to_string(),
            notify_extension,
            format: report_format,
            loop_count: loop_count,
            coverage_server_port: coverage_server_port,
        }
    }

    pub fn get_profraw_file(&self) -> String {
        self.profraw_file.clone()
    }

    pub fn get_report_path(&self) -> String {
        self.report_path.clone()
    }

    pub fn get_target_dir(&self) -> String {
        self.target_dir.clone()
    }

    pub fn get_ignore_regex(&self) -> String {
        self.ignore_regex.clone()
    }

    pub fn get_rustflags(&self) -> String {
        self.rustflags.clone()
    }

    pub fn get_notify_extension(&self) -> bool {
        self.notify_extension
    }

    pub fn get_loop_count(&self) -> u64 {
        self.loop_count
    }

    pub async fn generate_report(&self) -> Result<(), CoverageError> {
        let result = self.try_generate_report(false).await;
        match result {
            Ok(_) => self.clean().await,
            Err(_) => self.try_generate_report(true).await,
        }
    }

    async fn try_generate_report(&self, is_retry: bool) -> Result<(), CoverageError> {
        let mut child = self
            .build_generate_report_command()
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|_| CoverageError::GeneratingReportFailed)?;

        let result = self
            .handle_child(&mut child, CoverageError::GeneratingReportFailed)
            .await;

        if is_retry && result.is_err() {
            self.clean().await?;
            return result;
        }

        match result {
            Ok(_) => {
                println!("Report has been successfully generated.");
                self.notify_extension(NotificationType::DisplayFinalReport)
                    .await?;
                Ok(())
            }
            Err(_) => {
                println!("Report generation failed, attempting to remove corrupted files.");
                self.remove_corrupted_files(&mut child).await?;
                Err(CoverageError::GeneratingReportFailed)
            }
        }
    }

    pub async fn notify_extension(
        &self,
        notification_type: NotificationType,
    ) -> Result<(), CoverageError> {
        if !self.get_notify_extension() && notification_type == NotificationType::Setup {
            return Ok(());
        }

        let url = format!(
            "http://localhost:{}{}",
            self.coverage_server_port,
            notification_type.endpoint()
        );

        let json_content = format!("{{\"target\":\"{}\"}}", self.target);

        let client = reqwest::Client::new();
        tokio::spawn(async move {
            let _ = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(json_content)
                .send()
                .await;
        });

        Ok(())
    }

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

    pub async fn clean(&self) -> Result<(), CoverageError> {
        if !tokio::fs::try_exists(self.get_target_dir())
            .await
            .unwrap_or(false)
        {
            return Ok(());
        }

        // Remove profraw-list file, there should only be one in the target directory
        self.remove_profraw_list().await?;

        let mut child = tokio::process::Command::new("cargo")
            .env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_target_dir())
            .arg("llvm-cov")
            .arg("clean")
            .arg("--workspace")
            .spawn()
            .map_err(|_| CoverageError::CleaningFailed)?;

        self.handle_child(&mut child, CoverageError::CleaningFailed)
            .await
    }

    async fn remove_profraw_list(&self) -> Result<(), CoverageError> {
        let target_dir_str = self.get_target_dir();
        let target_dir = std::path::Path::new(&target_dir_str);
        let mut entries = tokio::fs::read_dir(target_dir)
            .await
            .map_err(|_| CoverageError::CleaningFailed)?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            match entry.file_name().to_str() {
                Some(name) if name.ends_with("profraw-list") => name,
                _ => continue,
            };

            let path = match entry.path().to_str() {
                Some(path) => path.to_string(),
                _ => continue,
            };

            self.remove_files(&[path]).await;
        }

        Ok(())
    }

    async fn extract_corrupted_files<R>(
        &self,
        reader: &mut BufReader<R>,
    ) -> Result<Vec<String>, CoverageError>
    where
        R: AsyncRead + Unpin,
    {
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

    fn build_generate_report_command(&self) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new("cargo");
        cmd.env("CARGO_LLVM_COV_TARGET_DIR", self.get_target_dir())
            .arg("llvm-cov")
            .arg("report")
            .arg("--release")
            .arg(self.format.get_cargo_arg())
            .args(["--ignore-filename-regex", &self.get_ignore_regex()]);

        match self.format {
            CoverageFormat::Json => {
                cmd.args(["--output-path", &self.get_report_path()]);
                cmd.arg("--skip-functions");
            }
            CoverageFormat::Html => {
                cmd.args(["--output-dir", &self.get_report_path()]);
            }
        }

        cmd
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

    pub async fn check_llvm_tools_installed(&self) -> Result<(), CoverageError> {
        let output = tokio::process::Command::new("rustup")
            .args(["component", "list", "--installed"])
            .output()
            .await
            .map_err(|_| CoverageError::LlvmToolsNotInstalled)?;

        let installed_components = String::from_utf8_lossy(&output.stdout);

        if installed_components.contains("llvm-tools") {
            Ok(())
        } else {
            Err(CoverageError::LlvmToolsNotInstalled)
        }
    }

    pub async fn prompt_and_install_llvm_tools(&self) -> Result<(), CoverageError> {
        println!(
            "llvm-tools-preview is required for coverage report generation but is not installed."
        );

        println!("\nWould you like to install it now? (y/n): ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|_| CoverageError::LlvmToolsNotInstalled)?;

        let input = input.trim().to_lowercase();
        if input == "y" || input == "yes" {
            let status = tokio::process::Command::new("rustup")
                .args(["component", "add", "llvm-tools-preview"])
                .status()
                .await
                .map_err(|_| CoverageError::LlvmToolsNotInstalled)?;

            if status.success() {
                println!("llvm-tools-preview installed successfully!");
                Ok(())
            } else {
                println!("Failed to install llvm-tools-preview");
                Err(CoverageError::LlvmToolsNotInstalled)
            }
        } else {
            Err(CoverageError::LlvmToolsNotInstalled)
        }
    }
}

#[cfg(test)]
mod tests;
