use crate::error::Error;

use crate::constants::*;
use fehler::throws;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;

#[macro_export]
macro_rules! construct_path {
    ($root:expr, $($component:expr),*) => {
        {
            let mut path = $root.to_owned();
            $(path = path.join($component);)*
            path
        }
    };
}
#[macro_export]
macro_rules! load_template {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file))
    };
}

#[throws]
pub async fn create_directory_all(path: &PathBuf) {
    match path.exists() {
        true => {}
        false => {
            fs::create_dir_all(path).await?;
        }
    };
}

#[throws]
pub async fn create_file(root: &PathBuf, path: &PathBuf, content: &str) {
    let file = path.strip_prefix(root)?.to_str().unwrap_or_default();

    match path.exists() {
        true => {
            println!("{SKIP} [{file}] already exists")
        }
        false => {
            fs::write(path, content).await?;
            println!("{FINISH} [{file}] created");
        }
    };
}

#[throws]
pub fn get_fuzz_id(fuzz_dir_path: &Path) -> i32 {
    if fuzz_dir_path.exists() {
        if fuzz_dir_path.read_dir()?.next().is_none() {
            0
        } else {
            let entries = fuzz_dir_path.read_dir()?;
            let mut max_num = -1;
            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if file_name.starts_with("fuzz_") {
                    let stripped = file_name.strip_prefix("fuzz_").unwrap_or_default();
                    let num = stripped.parse::<i32>()?;
                    max_num = max_num.max(num);
                }
            }
            max_num + 1
        }
    } else {
        0
    }
}

/// Creates .fuzz-artifacts directory if it doesn't exist
#[throws]
pub async fn ensure_fuzz_artifacts_dir() -> PathBuf {
    let artifacts_dir = PathBuf::from(".fuzz-artifacts");
    create_directory_all(&artifacts_dir).await?;
    artifacts_dir
}

/// Generates a unique filename in .fuzz-artifacts directory
/// If the base filename already exists, appends a readable timestamp to make it unique
#[throws]
pub async fn generate_unique_fuzz_filename(
    base_name: &str,
    fuzz_test_name: &str,
    extension: &str,
) -> PathBuf {
    let artifacts_dir = ensure_fuzz_artifacts_dir().await?;
    let base_filename = format!("{}_{}.{}", base_name, fuzz_test_name, extension);
    let mut target_path = artifacts_dir.join(&base_filename);

    // If file already exists, append a readable timestamp to make it unique
    if target_path.exists() {
        use chrono::DateTime;
        use chrono::Local;

        // Try different timestamp formats until we find a unique one
        let now: DateTime<Local> = Local::now();

        // First try: YYYY-MM-DD_HH-MM-SS format
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let unique_filename = format!(
            "{}_{}-{}.{}",
            base_name, fuzz_test_name, timestamp, extension
        );
        target_path = artifacts_dir.join(&unique_filename);

        // If that still exists (very unlikely), add milliseconds
        if target_path.exists() {
            let timestamp_with_ms = now.format("%Y-%m-%d_%H-%M-%S-%3f").to_string();
            let unique_filename = format!(
                "{}_{}-{}.{}",
                base_name, fuzz_test_name, timestamp_with_ms, extension
            );
            target_path = artifacts_dir.join(&unique_filename);
        }
    }

    target_path
}
