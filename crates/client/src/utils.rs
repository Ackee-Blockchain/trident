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
