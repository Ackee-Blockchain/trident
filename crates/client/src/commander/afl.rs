use fehler::{throw, throws};
use rand::RngCore;
use std::io::{Read, Write};
use std::process::Stdio;
use std::{fs::File, path::Path};
use tokio::{io::AsyncWriteExt, process::Command};

use trident_config::afl::AflSeed;
use trident_config::TridentConfig;

use crate::commander::{Commander, Error};
use crate::constants::*;

impl Commander {
    /// Runs AFL (American Fuzzy Lop) fuzzer on the specified target binary.
    ///
    /// # Arguments
    /// * `target` - The name of the target binary to fuzz
    ///
    /// # Errors
    /// Returns an error if:
    /// * Failed to create AFL workspace directories
    /// * Failed to create seed files
    /// * Failed to build or run the fuzzer
    ///
    /// # Example
    /// ```no_run
    /// # use fehler::throws;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let commander = Commander::new();
    /// commander.run_afl("my_target".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[throws]
    pub async fn run_afl(&self, target: String) {
        let config = TridentConfig::new();

        // Build args without cargo target dir
        let build_args = config.get_afl_build_args();
        // Fuzz args without afl workspace in and out
        let fuzz_args = config.get_afl_fuzz_args();

        // Cargo target directory
        let cargo_target_dir = config.get_afl_target_dir();

        // AFL workspace in and out
        let afl_workspace_in = config.get_afl_workspace_in();
        let afl_workspace_out = config.get_afl_workspace_out();

        let full_target_path = config.get_afl_target_path(&target);

        let afl_workspace_in_path = Path::new(&afl_workspace_in);
        let initial_seeds = config.get_initial_seed();

        if !afl_workspace_in_path.exists() {
            std::fs::create_dir_all(afl_workspace_in_path)?;

            for x in initial_seeds {
                create_seed_file(afl_workspace_in_path, &x)?;
            }
        } else if afl_workspace_in_path.is_dir() {
            for x in initial_seeds {
                create_seed_file(afl_workspace_in_path, &x)?;
            }
        } else {
            throw!(Error::BadAFLWorkspace)
        }

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg afl");

        let mut child = Command::new("cargo")
            .env("RUSTFLAGS", rustflags)
            .arg("afl")
            .arg("build")
            .args(["--target-dir", &cargo_target_dir])
            .args(build_args)
            .args(["--bin", &target])
            .spawn()?;
        Self::handle_child(&mut child).await?;

        let mut child = Command::new("cargo")
            .arg("afl")
            .arg("fuzz")
            .args(["-i", &afl_workspace_in])
            .args(["-o", &afl_workspace_out])
            .args(fuzz_args)
            .arg(&full_target_path)
            .spawn()?;

        Self::handle_child(&mut child).await?;
    }

    /// Runs AFL fuzzer in debug mode on the given target with a specified crash file.
    ///
    /// # Arguments
    /// * `target` - The name of the target binary to fuzz
    /// * `crash_file` - The path to the crash file to use for fuzzing
    ///
    /// # Errors
    /// Returns an error if:
    /// * The crash file does not exist
    /// * Failed to build or run the fuzzer
    ///
    /// # Example
    /// ```no_run
    /// # use fehler::throws;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let commander = Commander::new();
    /// commander.run_afl_debug("my_target".to_string(), "crash_file.txt".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[throws]
    pub async fn run_afl_debug(&self, target: String, crash_file: String) {
        let config = TridentConfig::new();

        let crash_file_path = Path::new(&crash_file);

        let crash_file = if crash_file_path.is_absolute() {
            crash_file_path
        } else {
            let cwd = std::env::current_dir()?;

            &cwd.join(crash_file_path)
        };

        if !crash_file.try_exists()? {
            println!("{ERROR} The crash file [{:?}] not found", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str("--cfg afl --cfg fuzzing_debug");

        let mut file = File::open(crash_file)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        let cargo_target_directory = config.get_afl_target_dir();

        // Using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        let mut child = Command::new("cargo")
            .env("TRIDENT_LOG", "1")
            .env("RUSTFLAGS", rustflags)
            .arg("afl")
            .arg("run")
            .args(["--target-dir", &cargo_target_directory])
            .args(["--bin", &target])
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&file_contents).await?;
        }
        child.wait().await?;
    }
}

/// Creates a seed file for AFL fuzzing in the specified directory
///
/// # Arguments
///
/// * `path` - The directory path where the seed file will be created
/// * `seed` - The AFL seed configuration containing file name and content details
///
/// # Returns
///
/// Returns `Ok(())` on successful file creation, or an IO error if file operations fail
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use trident_config::afl::AflSeed;
///
/// let path = Path::new("seeds");
/// let seed = AflSeed {
///     file_name: "test.txt".into(),
///     bytes_count: Some(16),
///     override_file: Some(true),
///     seed: None
/// };
/// create_seed_file(path, &seed)?;
/// ```
fn create_seed_file(path: &Path, seed: &AflSeed) -> std::io::Result<()> {
    let (bytes, override_file) = obtain_seed(seed);

    let file_path = path.join(&seed.file_name);

    if file_path.exists() {
        if override_file {
            let mut file = File::create(file_path)?;
            file.write_all(&bytes)?;
        }
    } else {
        let mut file = File::create(file_path)?;
        file.write_all(&bytes)?;
    }

    Ok(())
}

/// Helper function to obtain seed bytes and override flag from an AFL seed configuration
///
/// # Arguments
///
/// * `value` - The AFL seed configuration
///
/// # Returns
///
/// Returns a tuple containing:
/// - A vector of bytes representing the seed data
/// - A boolean indicating whether to override existing files
///
/// # Examples
///
/// ```
/// let seed = AflSeed {
///     file_name: "test_seed".into(),
///     bytes_count: Some(32),
///     override_file: Some(true),
///     seed: None
/// };
/// let (bytes, override_flag) = obtain_seed(&seed);
/// assert_eq!(bytes.len(), 32);
/// assert!(override_flag);
/// ```
///
/// # Panics
///
/// Panics if seed value is empty when bytes_count is None or 0
///
fn obtain_seed(value: &AflSeed) -> (Vec<u8>, bool) {
    match value.bytes_count {
        Some(number_of_random_bytes) => {
            if number_of_random_bytes > 0 {
                let mut rng = rand::rngs::OsRng;
                let mut seed = vec![0u8; number_of_random_bytes];
                rng.fill_bytes(&mut seed);
                (seed, value.override_file.unwrap_or_default())
            } else {
                let seed_as_bytes = value
                    .seed
                    .clone()
                    .unwrap_or_else(|| panic!("Seed value is empty for seed {}", value.file_name));

                (
                    seed_as_bytes.as_bytes().to_vec(),
                    value.override_file.unwrap_or_default(),
                )
            }
        }
        None => {
            let seed_as_bytes = value
                .seed
                .clone()
                .unwrap_or_else(|| panic!("Seed value is empty for seed {}", value.file_name));
            (
                seed_as_bytes.as_bytes().to_vec(),
                value.override_file.unwrap_or_default(),
            )
        }
    }
}
