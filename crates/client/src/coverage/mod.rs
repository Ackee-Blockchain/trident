pub mod constants;
use constants::*;
pub mod coverage_format;
use coverage_format::*;

use thiserror::Error;
use tokio::io::AsyncRead;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;

/// Errors that can occur during coverage operations.
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
    #[error("Failed to notify VSCode extension")]
    FailedToNotifyVSCodeExtension,
    #[error("Invalid report format")]
    InvalidReportFormat,
}

pub struct Coverage {
    profraw_file: String,
    report_path: String,
    target_dir: String,
    ignore_regex: String,
    rustflags: String,
    notify_extension: bool,
    format: CoverageFormat,
    loop_count: u64,
}

impl Coverage {
    pub fn new(
        cargo_target_dir: &str,
        target: &str,
        notify_extension: bool,
        format: String,
        loop_count: u64,
    ) -> Self {
        let report_format = CoverageFormat::from_str(&format)
            .expect(&format!("Invalid coverage format '{}'. Supported formats: json, html", format));

        Self {
            profraw_file: format!("{}/{}", cargo_target_dir, PROFRAW_FILENAME),
            report_path: format!("{}/../{}-{}", cargo_target_dir, target, report_format.get_report_filename()),
            target_dir: cargo_target_dir.to_string(),
            ignore_regex: COVERAGE_IGNORE_REGEX.to_string(),
            rustflags: COVERAGE_RUSTFLAGS.to_string(),
            notify_extension,
            format: report_format,
            loop_count: loop_count,
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
            Ok(_) => Ok(()),
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
            return result;
        }

        match result {
            Ok(_) => {
                println!("Report has been successfully generated.");
                Ok(())
            }
            Err(_) => {
                println!("Report generation failed, attempting to remove corrupted files.");
                self.remove_corrupted_files(&mut child).await?;
                Err(CoverageError::GeneratingReportFailed)
            }
        }
    }

    pub async fn notify_extension(&self) -> Result<(), CoverageError> {
        if !self.get_notify_extension() {
            return Ok(());
        }

        let target_dir = self.get_target_dir();
        let notification_file = format!("{}/{}", target_dir, constants::EXTENSION_NOTIFICATION_FILE);
        
        //TODO: Add metadata
        let json_content = format!("{{\"metadata\":\"{}\"}}", "TODO");

        tokio::fs::write(&notification_file, json_content)
            .await
            .map_err(|_| CoverageError::FailedToNotifyVSCodeExtension)?;

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
        // Remove notification file if it exists
        self.remove_notification_file().await?;

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
        cmd.env("LLVM_PROFILE_FILE", self.get_profraw_file())
            .env("CARGO_LLVM_COV_TARGET_DIR", self.get_target_dir())
            .arg("llvm-cov")
            .arg("report")
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

    async fn remove_notification_file(&self) -> Result<(), CoverageError> {
        let target_dir = self.get_target_dir();
        let notification_file = format!("{}/{}", target_dir, constants::EXTENSION_NOTIFICATION_FILE);
        self.remove_files(&[notification_file])
            .await;

        Ok(())
    }
}

// mod tests {
//     #![allow(unused_imports)]
//     #![allow(dead_code)]
//     use super::*;
//     use std::path::PathBuf;
//     use std::pin::Pin;
//     use std::task::{Context, Poll};
//     use tokio::fs;
//     use tokio::io::{AsyncRead, ReadBuf};

//     struct MockStderr {
//         data: Vec<u8>,
//         position: usize,
//     }

//     impl MockStderr {
//         fn new(data: &str) -> Self {
//             Self {
//                 data: data.as_bytes().to_vec(),
//                 position: 0,
//             }
//         }
//     }

//     impl AsyncRead for MockStderr {
//         fn poll_read(
//             mut self: Pin<&mut Self>,
//             _: &mut Context<'_>,
//             buf: &mut ReadBuf<'_>,
//         ) -> Poll<std::io::Result<()>> {
//             let remaining = &self.data[self.position..];
//             let amount = std::cmp::min(remaining.len(), buf.remaining());
//             buf.put_slice(&remaining[..amount]);
//             self.position += amount;
//             Poll::Ready(Ok(()))
//         }
//     }

//     struct MockCoverage {
//         profraw_file: String,
//         coverage_file: String,
//         coverage_target_dir: String,
//         fuzzer_loopcount: String,
//         ignore_regex: String,
//         rustflags: String,
//         dynamic_coverage: bool,
//     }

//     impl MockCoverage {
//         fn new(
//             cargo_target_dir: &str,
//             fuzzer_loopcount: u64,
//             _target: &str,
//             _dynamic_coverage: bool,
//         ) -> Self {
//             Self {
//                 profraw_file: format!("{}/mock.profraw", cargo_target_dir),
//                 coverage_file: format!("{}/mock-coverage.json", cargo_target_dir),
//                 coverage_target_dir: cargo_target_dir.to_string(),
//                 fuzzer_loopcount: fuzzer_loopcount.to_string(),
//                 ignore_regex: "test-ignore".to_string(),
//                 rustflags: "-test-flags".to_string(),
//                 dynamic_coverage: false,
//             }
//         }

//         fn get_profraw_file(&self) -> String {
//             self.profraw_file.clone()
//         }

//         fn get_coverage_file(&self) -> String {
//             self.coverage_file.clone()
//         }

//         fn get_target_dir(&self) -> String {
//             self.coverage_target_dir.clone()
//         }

//         fn get_fuzzer_loopcount(&self) -> String {
//             self.fuzzer_loopcount.clone()
//         }

//         fn get_ignore_regex(&self) -> String {
//             self.ignore_regex.clone()
//         }

//         fn get_rustflags(&self) -> String {
//             self.rustflags.clone()
//         }

//         fn get_dynamic_coverage(&self) -> bool {
//             self.dynamic_coverage
//         }

//         fn get_fuzzing_folder(&self) -> PathBuf {
//             PathBuf::from("/dummy/path") // Dummy function to satisfy the trait
//         }
//     }

//     #[tokio::test]
//     async fn test_clean_removes_profraw_list() {
//         let temp_dir = std::env::temp_dir();
//         let target_dir = format!("{}/test", temp_dir.to_str().unwrap());
//         let coverage = MockCoverage::new(&target_dir, 100, "test", false);

//         let _ = fs::create_dir_all(&target_dir).await;

//         let profraw_list = PathBuf::from(&coverage.get_target_dir()).join("profraw-list");
//         fs::write(&profraw_list, "test").await.unwrap();

//         assert!(coverage.clean().await.is_ok());
//         assert!(!profraw_list.exists());

//         let _ = fs::remove_dir_all(&target_dir).await;
//     }

//     #[tokio::test]
//     async fn test_clean_succeeds_with_non_existent_file() {
//         let temp_dir = std::env::temp_dir();
//         let coverage = MockCoverage::new(temp_dir.to_str().unwrap(), 100, "test", false);

//         let profraw_list = PathBuf::from(&coverage.get_target_dir()).join("profraw-list");

//         assert!(!profraw_list.exists());
//         assert!(coverage.clean().await.is_ok());
//     }

//     #[test]
//     fn test_mock_coverage_new() {
//         let coverage = MockCoverage::new("/tmp/test", 100, "test", false);
//         assert_eq!(coverage.get_profraw_file(), "/tmp/test/mock.profraw");
//         assert_eq!(coverage.get_coverage_file(), "/tmp/test/mock-coverage.json");
//         assert_eq!(coverage.get_target_dir(), "/tmp/test");
//         assert_eq!(coverage.get_fuzzer_loopcount(), "100");
//         assert_eq!(coverage.get_ignore_regex(), "test-ignore");
//         assert_eq!(coverage.get_rustflags(), "-test-flags");
//     }

//     #[tokio::test]
//     async fn test_extract_corrupted_files_from_stderr() {
//         let coverage = MockCoverage::new("/tmp/test", 100, "test", false);

//         let test_data = "warning: /path/to/file1.profraw: invalid instrumentation profile data\n\
//                         some other warning\n\
//                         warning: /path/to/file2.profraw: invalid instrumentation profile data\n";

//         let stderr = MockStderr::new(test_data);
//         let mut reader = BufReader::new(stderr);

//         let corrupted_files = coverage.extract_corrupted_files(&mut reader).await.unwrap();

//         assert_eq!(corrupted_files.len(), 2);
//         assert_eq!(corrupted_files[0], "/path/to/file1.profraw");
//         assert_eq!(corrupted_files[1], "/path/to/file2.profraw");
//     }

//     #[tokio::test]
//     async fn test_remove_corrupted_files() {
//         let temp_dir = std::env::temp_dir();
//         let target_dir = temp_dir.to_str().unwrap().to_string();
//         let coverage = MockCoverage::new(&target_dir, 100, "test", false);

//         let _ = fs::create_dir_all(&target_dir).await;

//         let file1 = PathBuf::from(&coverage.get_target_dir()).join("test1.profraw");
//         let file2 = PathBuf::from(&coverage.get_target_dir()).join("test2.profraw");

//         fs::write(&file1, "test data 1").await.unwrap();
//         fs::write(&file2, "test data 2").await.unwrap();

//         let files_to_remove = vec![
//             file1.to_str().unwrap().to_string(),
//             file2.to_str().unwrap().to_string(),
//         ];

//         coverage.remove_files(&files_to_remove).await;

//         assert!(!file1.exists());
//         assert!(!file2.exists());

//         let _ = fs::remove_dir_all(&target_dir).await;
//     }

//     #[tokio::test]
//     async fn test_extract_corrupted_files_empty_stderr() {
//         let coverage = MockCoverage::new("/tmp/test", 100, "test", false);

//         let test_data = "some other warning\nsome other message\n";
//         let stderr = MockStderr::new(test_data);
//         let mut reader = BufReader::new(stderr);

//         let corrupted_files = coverage.extract_corrupted_files(&mut reader).await.unwrap();
//         assert!(corrupted_files.is_empty());
//     }
// }
