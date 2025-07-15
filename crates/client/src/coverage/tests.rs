#![allow(unused_imports)]
#![allow(dead_code)]
use super::*;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs;
use tokio::io::{AsyncRead, ReadBuf};

struct MockStderr {
    data: Vec<u8>,
    position: usize,
}

impl MockStderr {
    fn new(data: &str) -> Self {
        Self {
            data: data.as_bytes().to_vec(),
            position: 0,
        }
    }
}

impl AsyncRead for MockStderr {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let remaining = &self.data[self.position..];
        let amount = std::cmp::min(remaining.len(), buf.remaining());
        buf.put_slice(&remaining[..amount]);
        self.position += amount;
        Poll::Ready(Ok(()))
    }
}

struct MockCoverage {
    profraw_file: String,
    coverage_file: String,
    coverage_target_dir: String,
    fuzzer_loopcount: String,
    ignore_regex: String,
    rustflags: String,
    dynamic_coverage: bool,
}

impl MockCoverage {
    fn new(
        cargo_target_dir: &str,
        fuzzer_loopcount: u64,
        _target: &str,
        _dynamic_coverage: bool,
    ) -> Self {
        Self {
            profraw_file: format!("{}/mock.profraw", cargo_target_dir),
            coverage_file: format!("{}/mock-coverage.json", cargo_target_dir),
            coverage_target_dir: cargo_target_dir.to_string(),
            fuzzer_loopcount: fuzzer_loopcount.to_string(),
            ignore_regex: "test-ignore".to_string(),
            rustflags: "-test-flags".to_string(),
            dynamic_coverage: false,
        }
    }

    fn get_profraw_file(&self) -> String {
        self.profraw_file.clone()
    }

    fn get_coverage_file(&self) -> String {
        self.coverage_file.clone()
    }

    fn get_target_dir(&self) -> String {
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
        self.dynamic_coverage
    }

    fn get_fuzzing_folder(&self) -> PathBuf {
        PathBuf::from("/dummy/path") // Dummy function to satisfy the trait
    }

    async fn clean(&self) -> Result<(), CoverageError> {
        let profraw_list = PathBuf::from(&self.get_target_dir()).join("profraw-list");
        if profraw_list.exists() {
            tokio::fs::remove_file(&profraw_list)
                .await
                .map_err(|_| CoverageError::CleaningFailed)?;
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
        let mut line = String::new();

        while reader
            .read_line(&mut line)
            .await
            .map_err(|_| CoverageError::ExtractingCorruptedFilesFailed)?
            > 0
        {
            if line.contains("invalid instrumentation profile data") {
                if let Some(start) = line.find("warning: ") {
                    if let Some(end) = line.find(": invalid instrumentation profile data") {
                        let file_path = &line[start + 9..end];
                        corrupted_files.push(file_path.to_string());
                    }
                }
            }
            line.clear();
        }

        Ok(corrupted_files)
    }

    async fn remove_files(&self, files: &[String]) {
        for file in files {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}

#[tokio::test]
async fn test_clean_removes_profraw_list() {
    let temp_dir = std::env::temp_dir();
    let target_dir = format!("{}/test", temp_dir.to_str().unwrap());
    let coverage = MockCoverage::new(&target_dir, 100, "test", false);

    let _ = fs::create_dir_all(&target_dir).await;

    let profraw_list = PathBuf::from(&coverage.get_target_dir()).join("profraw-list");
    fs::write(&profraw_list, "test").await.unwrap();

    assert!(coverage.clean().await.is_ok());
    assert!(!profraw_list.exists());

    let _ = fs::remove_dir_all(&target_dir).await;
}

#[tokio::test]
async fn test_clean_succeeds_with_non_existent_file() {
    let temp_dir = std::env::temp_dir();
    let coverage = MockCoverage::new(temp_dir.to_str().unwrap(), 100, "test", false);

    let profraw_list = PathBuf::from(&coverage.get_target_dir()).join("profraw-list");

    assert!(!profraw_list.exists());
    assert!(coverage.clean().await.is_ok());
}

#[test]
fn test_mock_coverage_new() {
    let coverage = MockCoverage::new("/tmp/test", 100, "test", false);
    assert_eq!(coverage.get_profraw_file(), "/tmp/test/mock.profraw");
    assert_eq!(coverage.get_coverage_file(), "/tmp/test/mock-coverage.json");
    assert_eq!(coverage.get_target_dir(), "/tmp/test");
    assert_eq!(coverage.get_fuzzer_loopcount(), "100");
    assert_eq!(coverage.get_ignore_regex(), "test-ignore");
    assert_eq!(coverage.get_rustflags(), "-test-flags");
}

#[tokio::test]
async fn test_extract_corrupted_files_from_stderr() {
    let coverage = MockCoverage::new("/tmp/test", 100, "test", false);

    let test_data = "warning: /path/to/file1.profraw: invalid instrumentation profile data\n\
                    some other warning\n\
                    warning: /path/to/file2.profraw: invalid instrumentation profile data\n";

    let stderr = MockStderr::new(test_data);
    let mut reader = BufReader::new(stderr);

    let corrupted_files = coverage.extract_corrupted_files(&mut reader).await.unwrap();

    assert_eq!(corrupted_files.len(), 2);
    assert_eq!(corrupted_files[0], "/path/to/file1.profraw");
    assert_eq!(corrupted_files[1], "/path/to/file2.profraw");
}

#[tokio::test]
async fn test_remove_corrupted_files() {
    let temp_dir = std::env::temp_dir();
    let target_dir = temp_dir.to_str().unwrap().to_string();
    let coverage = MockCoverage::new(&target_dir, 100, "test", false);

    let _ = fs::create_dir_all(&target_dir).await;

    let file1 = PathBuf::from(&coverage.get_target_dir()).join("test1.profraw");
    let file2 = PathBuf::from(&coverage.get_target_dir()).join("test2.profraw");

    fs::write(&file1, "test data 1").await.unwrap();
    fs::write(&file2, "test data 2").await.unwrap();

    let files_to_remove = vec![
        file1.to_str().unwrap().to_string(),
        file2.to_str().unwrap().to_string(),
    ];

    coverage.remove_files(&files_to_remove).await;

    assert!(!file1.exists());
    assert!(!file2.exists());

    let _ = fs::remove_dir_all(&target_dir).await;
}

#[tokio::test]
async fn test_extract_corrupted_files_empty_stderr() {
    let coverage = MockCoverage::new("/tmp/test", 100, "test", false);

    let test_data = "some other warning\nsome other message\n";
    let stderr = MockStderr::new(test_data);
    let mut reader = BufReader::new(stderr);

    let corrupted_files = coverage.extract_corrupted_files(&mut reader).await.unwrap();
    assert!(corrupted_files.is_empty());
}