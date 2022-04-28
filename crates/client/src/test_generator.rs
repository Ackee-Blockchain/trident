use std::env::current_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;
use fehler::{throw, throws};
use tokio::fs;

const TESTS_DIRECTORY: &str = "tests";

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
}

pub struct TestGenerator {
    path: PathBuf
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            path: current_dir().unwrap()
        }
    }

    pub async fn generate(&self) {
        self.check_workspace().await;
        self.generate_test_files().await;
        self.add_dev_dependencies().await;
    }

    async fn generate_test_files(&self) {
        let path = Path::new(&self.path).join(TESTS_DIRECTORY);
        if fs::metadata(&path).await.is_ok() {
            return;
        }

        fs::create_dir(&path).await;
        fs::write(path.join("test.rs"), "").await;
    }

    async fn add_dev_dependencies(&self) {}

    #[throws]
    async fn check_workspace(&self) {
        // todo: throw error if the workspace is not valid
        throw!(Error::BadWorkspace);
    }
}