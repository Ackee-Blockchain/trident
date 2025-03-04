use fehler::{throw, throws};
use std::path::{Path, PathBuf};
use std::{io, process::Stdio, string::FromUtf8Error};
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
    signal,
};

mod afl;
mod honggfuzz;

use tokio::io::AsyncBufReadExt;
use trident_fuzz::fuzz_stats::FuzzingStatistics;

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
    #[error("Trident it not correctly initialized! The trident-tests folder in the root of your project does not exist")]
    NotInitialized,
    #[error("the crash file does not exist")]
    CrashFileNotFound,
    #[error("The Solana project does not contain any programs")]
    NoProgramsFound,
    #[error("Incorrect AFL workspace provided")]
    BadAFLWorkspace,
}

/// `Commander` allows you to start localnet, build programs,
/// run tests and do other useful operations.
#[derive(Default)]
pub struct Commander {
    root: PathBuf,
}

impl Commander {
    /// Creates a new `Commander` instance with the provided `root`.
    pub fn with_root(root: &PathBuf) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
        }
    }
    pub fn get_anchor_version() -> Result<String, std::io::Error> {
        let output = std::process::Command::new("anchor")
            .arg("--version")
            .output()?;

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(version)
    }

    #[throws]
    pub async fn build_anchor_project(program_name: Option<String>) {
        let mut cmd = Command::new("anchor");
        cmd.arg("build");

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
                    Err(_) => throw!(Error::FuzzingFailed),
            },
            _ = signal::ctrl_c() => {
                let _res = child.wait().await?;

                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            },
        }
    }
    /// Asynchronously manages a child fuzzing process, collecting and logging its statistics.
    /// This function spawns a new task dedicated to reading the process's standard output and logging the fuzzing statistics.
    /// It waits for either the child process to exit or a Ctrl+C signal to be received. Upon process exit or Ctrl+C signal,
    /// it stops the logging task and displays the collected statistics in a table format.
    ///
    /// The implementation ensures that the statistics logging task only stops after receiving a signal indicating the end of the fuzzing process
    /// or an interrupt from the user, preventing premature termination of the logging task if scenarios where reading is faster than fuzzing,
    /// which should not be common.
    ///
    /// # Arguments
    /// * `child` - A mutable reference to a `Child` process, representing the child fuzzing process.
    ///
    /// # Errors
    /// * `Error::FuzzingFailed` - Thrown if there's an issue with managing the child process, such as failing to wait on the child process.
    #[throws]
    async fn handle_child_with_stats(child: &mut Child) {
        let stdout = child
            .stdout
            .take()
            .expect("child did not have a handle to stdout");

        let reader = tokio::io::BufReader::new(stdout);

        let fuzz_end = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let fuzz_end_clone = std::sync::Arc::clone(&fuzz_end);

        let stats_handle: tokio::task::JoinHandle<Result<FuzzingStatistics, std::io::Error>> =
            tokio::spawn(async move {
                let mut stats_logger = FuzzingStatistics::new();

                let mut lines = reader.lines();
                loop {
                    let _line = lines.next_line().await;
                    match _line {
                        Ok(__line) => match __line {
                            Some(content) => {
                                stats_logger.insert_serialized(&content);
                            }
                            None => {
                                if fuzz_end_clone.load(std::sync::atomic::Ordering::SeqCst) {
                                    break;
                                }
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(stats_logger)
            });

        tokio::select! {
            res = child.wait() =>{
                fuzz_end.store(true, std::sync::atomic::Ordering::SeqCst);

                match res {
                    Ok(status) => {
                        if !status.success() {
                            throw!(Error::FuzzingFailed);
                        }
                    },
                    Err(_) => throw!(Error::FuzzingFailed),
                }
            },
            _ = signal::ctrl_c() => {
                fuzz_end.store(true, std::sync::atomic::Ordering::SeqCst);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            },
        }
        let stats_result = stats_handle
            .await
            .expect("Unable to obtain Statistics Handle");
        match stats_result {
            Ok(stats_result) => {
                stats_result.show_table();
            }
            Err(e) => {
                println!("Statistics thread exited with the Error: {}", e);
            }
        }
    }
}

fn get_crash_dir_and_ext(
    root: &Path,
    target: &str,
    hfuzz_run_args: &str,
    hfuzz_workspace: &str,
) -> (PathBuf, String) {
    // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed
    let hfuzz_run_args = hfuzz_run_args.split_whitespace();

    let extension =
        get_cmd_option_value(hfuzz_run_args.clone(), "-e", "--ext").unwrap_or("fuzz".to_string());

    // If we run fuzzer like:
    // HFUZZ_WORKSPACE="./new_hfuzz_workspace" HFUZZ_RUN_ARGS="--crashdir ./new_crash_dir -W ./new_workspace" cargo hfuzz run
    // The structure will be as follows:
    // ./new_hfuzz_workspace - will contain inputs
    // ./new_crash_dir - will contain crashes
    // ./new_workspace - will contain report
    // So finally , we have to give precedence:
    // --crashdir > --workspace > HFUZZ_WORKSPACE
    let crash_dir = get_cmd_option_value(hfuzz_run_args.clone(), "", "--cr")
        .or_else(|| get_cmd_option_value(hfuzz_run_args.clone(), "-W", "--w"));

    let crash_path = if let Some(dir) = crash_dir {
        // INFO If path is absolute, it replaces the current path.
        root.join(dir)
    } else {
        std::path::Path::new(hfuzz_workspace).join(target)
    };

    (crash_path, extension)
}

fn get_cmd_option_value<'a>(
    hfuzz_run_args: impl Iterator<Item = &'a str>,
    short_opt: &str,
    long_opt: &str,
) -> Option<String> {
    let mut args_iter = hfuzz_run_args;
    let mut value: Option<String> = None;

    // ensure short option starts with one dash and long option with two dashes
    let short_opt = format!("-{}", short_opt.trim_start_matches('-'));
    let long_opt = format!("--{}", long_opt.trim_start_matches('-'));

    while let Some(arg) = args_iter.next() {
        match arg.strip_prefix(&short_opt) {
            Some(val) if short_opt.len() > 1 => {
                if !val.is_empty() {
                    // -ecrash for crash extension with no space
                    value = Some(val.to_string());
                } else if let Some(next_arg) = args_iter.next() {
                    // -e crash for crash extension with space
                    value = Some(next_arg.to_string());
                } else {
                    value = None;
                }
            }
            _ => {
                if arg.starts_with(&long_opt) && long_opt.len() > 2 {
                    value = args_iter.next().map(|a| a.to_string());
                }
            }
        }
    }

    value
}

fn get_crash_files(
    dir: &PathBuf,
    extension: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `extension`
        .filter_map(|path| {
            if path.extension().is_some_and(|ext| ext == extension) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}
