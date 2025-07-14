use fehler::throw;
use fehler::throws;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::string::FromUtf8Error;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;
use tokio::process::Command;
use tokio::signal;

use crate::constants::TESTS_WORKSPACE_DIRECTORY;

mod fuzz;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Utf8(#[from] FromUtf8Error),
    #[error("build programs failed")]
    BuildProgramsFailed,
    #[error("fuzzing failed")]
    FuzzingFailed,
    // #[error("Coverage error: {0}")]
    // Coverage(#[from] crate::coverage::CoverageError),
}

/// `Commander` allows you to start localnet, build programs,
/// run tests and do other useful operations.
#[derive(Default)]
pub struct Commander {
    root: PathBuf,
}

impl Commander {
    pub fn new(root: &str) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
        }
    }

    #[throws]
    pub async fn build_anchor_project(root: &Path, program_name: Option<String>) {
        let mut cmd = Command::new("anchor");
        cmd.arg("build");
        cmd.current_dir(root);

        if let Some(name) = program_name {
            cmd.args(["-p", name.as_str()]);
        }

        let success = cmd.spawn()?.wait().await?.success();
        if !success {
            throw!(Error::BuildProgramsFailed);
        }
    }

    /// Formats program code.
    #[throws]
    pub async fn format_program_code(code: &str) -> String {
        let mut rustfmt = Command::new("rustfmt")
            .args(["--edition", "2018"])
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        if let Some(stdio) = &mut rustfmt.stdin {
            stdio.write_all(code.as_bytes()).await?;
        }
        let output = rustfmt.wait_with_output().await?;
        String::from_utf8(output.stdout)?
    }

    /// Formats program code - nightly.
    #[throws]
    pub async fn format_program_code_nightly(code: &str) -> String {
        let mut rustfmt = Command::new("rustfmt")
            .arg("+nightly")
            .arg("--config")
            .arg(
                "\
            edition=2021,\
            wrap_comments=true,\
            normalize_doc_attributes=true",
            )
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        if let Some(stdio) = &mut rustfmt.stdin {
            stdio.write_all(code.as_bytes()).await?;
        }
        let output = rustfmt.wait_with_output().await?;
        String::from_utf8(output.stdout)?
    }

    /// Manages a child process in an async context, specifically for monitoring fuzzing tasks.
    /// Waits for the process to exit or a Ctrl+C signal. Prints an error message if the process
    /// exits with an error, and sleeps briefly on Ctrl+C. Throws `Error::FuzzingFailed` on errors.
    ///
    /// # Arguments
    /// * `child` - A mutable reference to a `Child` process.
    ///
    /// # Errors
    /// * Throws `Error::FuzzingFailed` if waiting on the child process fails.
    #[throws]
    async fn handle_child(child: &mut Child) {
        tokio::select! {
            res = child.wait() =>
                match res {
                    Ok(status) => if !status.success() {
                        throw!(Error::FuzzingFailed);
                    },
                    Err(e) => throw!(e),
            },
            _ = signal::ctrl_c() => {
                let _res = child.wait().await?;

                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            },
        }
    }
    #[throws]
    pub async fn clean_target(&self) {
        self.clean_anchor_target().await?;
        self.clean_fuzz_target().await?;
    }

    #[throws]
    async fn clean_anchor_target(&self) {
        Command::new("anchor").arg("clean").spawn()?.wait().await?;
    }

    #[throws]
    #[allow(dead_code)]
    async fn clean_fuzz_target(&self) {
        let trident_tests_dir = self.root.join(TESTS_WORKSPACE_DIRECTORY);
        Command::new("cargo")
            .arg("clean")
            .current_dir(trident_tests_dir)
            .spawn()?
            .wait()
            .await?;
    }
}
