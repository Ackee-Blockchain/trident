use std::{
    env,
    path::{Path, PathBuf},
    process,
};

use anyhow::{bail, Context, Error, Result};

use clap::Subcommand;
use fehler::throws;
use trdelnik_client::Commander;

pub const TESTS_WORKSPACE: &str = "trdelnik-tests";
pub const HFUZZ_WORKSPACE: &str = "hfuzz_workspace";

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub enum FuzzCommand {
    /// Run fuzz target
    Run {
        /// Name of the fuzz target
        target: String,
        /// Trdelnik will return exit code 1 in case of found crash files in the crash folder. This is checked before and after the fuzz test run.
        #[arg(short, long)]
        with_exit_code: bool,
    },
    /// Debug fuzz target with crash file
    Run_Debug {
        /// Name of the fuzz target
        target: String,
        /// Path to the crash file
        crash_file_path: String,
    },
}

#[throws]
pub async fn fuzz(root: Option<String>, subcmd: FuzzCommand) {
    let root = match root {
        Some(r) => r,
        _ => {
            let root = _discover()?;
            if let Some(r) = root {
                r
            } else {
                bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!");
            }
        }
    };

    let mut commander = Commander::with_root(root.clone());

    match subcmd {
        FuzzCommand::Run {
            target,
            with_exit_code,
        } => {
            if with_exit_code {
                // Parse the HFUZZ run arguments to find out if the crash folder and crash files extension was modified.
                let hfuzz_run_args = env::var("HFUZZ_RUN_ARGS").unwrap_or_default();
                let (crash_dir, ext) = get_crash_dir_and_ext(&root, &target, &hfuzz_run_args);

                if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
                    if !crash_files.is_empty() {
                        println!("Error: The crash directory {} already contains crash files from previous runs. \n\nTo run Trdelnik fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                        process::exit(1);
                    }
                }
                commander.run_fuzzer(target).await?;
                if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
                    if !crash_files.is_empty() {
                        println!(
                            "The crash directory {} contains new fuzz test crashes. Exiting!",
                            crash_dir.to_string_lossy()
                        );
                        process::exit(1);
                    }
                }
            } else {
                commander.run_fuzzer(target).await?;
            }
        }
        FuzzCommand::Run_Debug {
            target,
            crash_file_path,
        } => {
            commander.run_fuzzer_debug(target, crash_file_path).await?;
        }
    };
}

// Climbs each parent directory until we find Trdelnik.toml.
fn _discover() -> Result<Option<String>> {
    let _cwd = std::env::current_dir()?;
    let mut cwd_opt = Some(_cwd.as_path());

    while let Some(cwd) = cwd_opt {
        for f in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let p = f
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = p.file_name() {
                if filename.to_str() == Some("Trdelnik.toml") {
                    return Ok(Some(cwd.to_string_lossy().to_string()));
                }
            }
        }

        cwd_opt = cwd.parent();
    }

    Ok(None)
}

fn get_crash_dir_and_ext(root: &str, target: &str, hfuzz_run_args: &str) -> (PathBuf, String) {
    // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed
    let hfuzz_run_args = hfuzz_run_args.split_whitespace();

    let extension =
        get_cmd_option_value(hfuzz_run_args.clone(), "-e", "--ext").unwrap_or("fuzz".to_string());

    let crash_dir = get_cmd_option_value(hfuzz_run_args.clone(), "", "--cr")
        .or_else(|| get_cmd_option_value(hfuzz_run_args.clone(), "-W", "--w"));

    let crash_path = if let Some(dir) = crash_dir {
        Path::new(root).join(TESTS_WORKSPACE).join(dir)
    } else {
        Path::new(root)
            .join(TESTS_WORKSPACE)
            .join(HFUZZ_WORKSPACE)
            .join(target)
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
            if path.extension().map_or(false, |ext| ext == extension) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cmd_options_parsing() {
        let mut command = String::from("-Q -v --extension fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -e fuzz -v");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v --extension ");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q --extension fuzz -v -e ");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        let mut command = String::from("--extension buzz -e fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -e fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -efuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --extfuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --workspace");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v -e");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "");
        assert_eq!(extension, None);
    }

    #[test]
    fn test_get_crash_dir_and_ext() {
        let root = "/home/fuzz";
        let target = "target";
        let default_crash_path = Path::new(root)
            .join(TESTS_WORKSPACE)
            .join(HFUZZ_WORKSPACE)
            .join(target);

        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "");

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "fuzz");

        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "-Q -e");

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "fuzz");

        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "-Q -e crash");

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "-Q -W /home/crash -e crash");

        let expected_crash_path = Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        let (crash_dir, ext) =
            get_crash_dir_and_ext(root, target, "-Q --crash /home/crash -e crash");

        let expected_crash_path = Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "-Q -W ../crash -e crash");

        let expected_crash_path = Path::new(root).join(TESTS_WORKSPACE).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(root, target, "-Q --crash ../crash -e crash");

        let expected_crash_path = Path::new(root).join(TESTS_WORKSPACE).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace
        let (crash_dir, ext) =
            get_crash_dir_and_ext(root, target, "-Q --crash ../crash -W /workspace -e crash");

        let expected_crash_path = Path::new(root).join(TESTS_WORKSPACE).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");
    }
}
