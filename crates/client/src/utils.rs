use crate::error::Error;

use crate::constants::*;
use fehler::throws;
use serde_json::Value;
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

/// Merges JSON values: objects recursively, arrays by adding unique items, primitives by replacing
fn merge_json(existing: &mut Value, new: &Value) {
    match (existing, new) {
        (Value::Object(existing_map), Value::Object(new_map)) => {
            for (key, new_val) in new_map {
                existing_map
                    .entry(key.clone())
                    .and_modify(|existing_val| merge_json(existing_val, new_val))
                    .or_insert_with(|| new_val.clone());
            }
        }
        (Value::Array(existing_arr), Value::Array(new_arr)) => {
            // Add unique items only
            for item in new_arr {
                if !existing_arr.contains(item) {
                    existing_arr.push(item.clone());
                }
            }
        }
        (existing_val, new_val) => {
            *existing_val = new_val.clone();
        }
    }
}

/// Strips trailing commas from JSON (common in VSCode settings files)
fn strip_trailing_commas(json_str: &str) -> String {
    let chars: Vec<char> = json_str.chars().collect();
    let mut result = String::with_capacity(json_str.len());

    for i in 0..chars.len() {
        if chars[i] == ',' {
            // Look ahead: skip comma if only whitespace before } or ]
            let remaining = &chars[i + 1..];
            if remaining.iter().take_while(|c| c.is_whitespace()).count() == remaining.len()
                || remaining
                    .iter()
                    .find(|c| !c.is_whitespace())
                    .is_some_and(|c| *c == '}' || *c == ']')
            {
                continue;
            }
        }
        result.push(chars[i]);
    }
    result
}

/// Creates or updates a JSON file with intelligent merging
#[throws]
pub async fn create_or_update_json_file(root: &PathBuf, path: &PathBuf, content: &str) {
    let file = path.strip_prefix(root)?.to_str().unwrap_or_default();

    if !path.exists() {
        fs::write(path, content).await?;
        println!("{FINISH} [{file}] created");
        return;
    }

    let existing_content = fs::read_to_string(path).await?;

    // Empty file - just write new content
    if existing_content.trim().is_empty() {
        fs::write(path, content).await?;
        println!("{FINISH} [{file}] created (was empty)");
        return;
    }

    // Try to parse, fixing trailing commas if needed
    let cleaned = strip_trailing_commas(&existing_content);
    let mut existing_json: Value = match serde_json::from_str(&cleaned) {
        Ok(json) => json,
        Err(e) => {
            // Invalid JSON - backup and replace
            eprintln!("Warning: Invalid JSON in {}: {}", file, e);
            let backup_path = path.with_extension("json.backup");
            fs::write(&backup_path, &existing_content).await?;
            fs::write(path, content).await?;
            println!("{UPDATED} [{file}] (backed up invalid JSON)");
            return;
        }
    };

    // Merge and write
    let new_json: Value = serde_json::from_str(content)?;
    merge_json(&mut existing_json, &new_json);
    let merged = serde_json::to_string_pretty(&existing_json)?;
    fs::write(path, merged).await?;
    println!("{UPDATED} [{file}] merged with existing settings");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_json_objects() {
        let mut existing = json!({
            "key1": "value1",
            "key2": {
                "nested": "old"
            }
        });

        let new = json!({
            "key2": {
                "nested": "new",
                "added": "value"
            },
            "key3": "value3"
        });

        merge_json(&mut existing, &new);

        assert_eq!(existing["key1"], "value1");
        assert_eq!(existing["key2"]["nested"], "new");
        assert_eq!(existing["key2"]["added"], "value");
        assert_eq!(existing["key3"], "value3");
    }

    #[test]
    fn test_merge_json_arrays() {
        let mut existing = json!({
            "linkedProjects": ["./Cargo.toml"]
        });

        let new = json!({
            "linkedProjects": ["./trident-tests/Cargo.toml"]
        });

        merge_json(&mut existing, &new);

        let projects = existing["linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&json!("./Cargo.toml")));
        assert!(projects.contains(&json!("./trident-tests/Cargo.toml")));
    }

    #[test]
    fn test_merge_json_arrays_no_duplicates() {
        let mut existing = json!({
            "linkedProjects": ["./Cargo.toml", "./trident-tests/Cargo.toml"]
        });

        let new = json!({
            "linkedProjects": ["./Cargo.toml", "./trident-tests/Cargo.toml"]
        });

        merge_json(&mut existing, &new);

        let projects = existing["linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn test_merge_json_primitive_override() {
        let mut existing = json!({
            "setting": "old_value"
        });

        let new = json!({
            "setting": "new_value"
        });

        merge_json(&mut existing, &new);

        assert_eq!(existing["setting"], "new_value");
    }

    #[test]
    fn test_merge_json_complex() {
        let mut existing = json!({
            "rust-analyzer.linkedProjects": ["./Cargo.toml"],
            "editor.formatOnSave": true,
            "custom": {
                "nested": "value"
            }
        });

        let new = json!({
            "rust-analyzer.linkedProjects": ["./trident-tests/Cargo.toml"],
            "editor.rulers": [80, 120]
        });

        merge_json(&mut existing, &new);

        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(existing["editor.formatOnSave"], true);
        assert_eq!(existing["editor.rulers"], json!([80, 120]));
        assert_eq!(existing["custom"]["nested"], "value");
    }

    #[test]
    fn test_strip_trailing_commas_simple() {
        let input = r#"{
  "key": "value",
}"#;
        let expected = r#"{
  "key": "value"
}"#;
        assert_eq!(strip_trailing_commas(input), expected);
    }

    #[test]
    fn test_strip_trailing_commas_array() {
        let input = r#"{
  "items": [
    "item1",
    "item2",
  ]
}"#;
        let expected = r#"{
  "items": [
    "item1",
    "item2"
  ]
}"#;
        assert_eq!(strip_trailing_commas(input), expected);
    }

    #[test]
    fn test_strip_trailing_commas_nested() {
        let input = r#"{
  "outer": {
    "inner": "value",
  },
  "array": [1, 2, 3,],
}"#;
        let expected = r#"{
  "outer": {
    "inner": "value"
  },
  "array": [1, 2, 3]
}"#;
        assert_eq!(strip_trailing_commas(input), expected);
    }

    #[test]
    fn test_strip_trailing_commas_preserves_valid_commas() {
        let input = r#"{
  "key1": "value1",
  "key2": "value2"
}"#;
        // Should not change anything
        assert_eq!(strip_trailing_commas(input), input);
    }

    #[test]
    fn test_strip_trailing_commas_vscode_settings() {
        let input = r#"{
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml",
  ],
  "editor.formatOnSave": true,
}"#;
        let cleaned = strip_trailing_commas(input);
        // Should be valid JSON now
        let result: Result<Value, _> = serde_json::from_str(&cleaned);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["editor.formatOnSave"], true);
        let projects = json["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 1);
    }

    #[test]
    fn test_merge_linked_projects_when_cargo_toml_exists() {
        // When ./Cargo.toml already exists, only add ./trident-tests/Cargo.toml
        let mut existing = json!({
            "rust-analyzer.linkedProjects": ["./Cargo.toml"],
            "editor.formatOnSave": true
        });

        let new = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        merge_json(&mut existing, &new);

        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0], "./Cargo.toml");
        assert_eq!(projects[1], "./trident-tests/Cargo.toml");
        assert_eq!(existing["editor.formatOnSave"], true);
    }

    #[test]
    fn test_merge_linked_projects_when_cargo_toml_missing() {
        // When ./Cargo.toml doesn't exist, add both paths
        let mut existing = json!({
            "rust-analyzer.linkedProjects": [],
            "editor.formatOnSave": true
        });

        let new = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        merge_json(&mut existing, &new);

        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&json!("./Cargo.toml")));
        assert!(projects.contains(&json!("./trident-tests/Cargo.toml")));
        assert_eq!(existing["editor.formatOnSave"], true);
    }

    #[test]
    fn test_merge_linked_projects_when_both_exist() {
        // When both paths already exist, don't add duplicates
        let mut existing = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        let new = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        merge_json(&mut existing, &new);

        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn test_merge_linked_projects_with_other_paths() {
        // When other paths exist, preserve them and add trident-tests
        let mut existing = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./other-project/Cargo.toml"
            ]
        });

        let new = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        merge_json(&mut existing, &new);

        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 3);
        assert!(projects.contains(&json!("./Cargo.toml")));
        assert!(projects.contains(&json!("./other-project/Cargo.toml")));
        assert!(projects.contains(&json!("./trident-tests/Cargo.toml")));
    }

    #[test]
    fn test_merge_creates_linked_projects_when_missing() {
        // When rust-analyzer.linkedProjects doesn't exist, create it with both paths
        let mut existing = json!({
            "editor.formatOnSave": true
        });

        let new = json!({
            "rust-analyzer.linkedProjects": [
                "./Cargo.toml",
                "./trident-tests/Cargo.toml"
            ]
        });

        merge_json(&mut existing, &new);

        assert!(existing.get("rust-analyzer.linkedProjects").is_some());
        let projects = existing["rust-analyzer.linkedProjects"].as_array().unwrap();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&json!("./Cargo.toml")));
        assert!(projects.contains(&json!("./trident-tests/Cargo.toml")));
        assert_eq!(existing["editor.formatOnSave"], true);
    }

    #[tokio::test]
    async fn test_create_or_update_json_file_empty_file() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let vscode_dir = root.join(".vscode");
        std::fs::create_dir_all(&vscode_dir).unwrap();

        let settings_path = vscode_dir.join("settings.json");

        // Create an empty file
        std::fs::write(&settings_path, "").unwrap();

        let new_content = r#"{
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml",
    "./trident-tests/Cargo.toml"
  ]
}"#;

        // Should not create a backup for empty file
        create_or_update_json_file(&root, &settings_path, new_content)
            .await
            .unwrap();

        // Verify no backup was created
        let backup_path = settings_path.with_extension("json.backup");
        assert!(!backup_path.exists());

        // Verify the new content was written
        let content = std::fs::read_to_string(&settings_path).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();
        assert!(json.get("rust-analyzer.linkedProjects").is_some());
    }

    #[tokio::test]
    async fn test_create_or_update_json_file_whitespace_only() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let vscode_dir = root.join(".vscode");
        std::fs::create_dir_all(&vscode_dir).unwrap();

        let settings_path = vscode_dir.join("settings.json");

        // Create a file with only whitespace
        std::fs::write(&settings_path, "   \n\t  \n  ").unwrap();

        let new_content = r#"{
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml",
    "./trident-tests/Cargo.toml"
  ]
}"#;

        // Should not create a backup for whitespace-only file
        create_or_update_json_file(&root, &settings_path, new_content)
            .await
            .unwrap();

        // Verify no backup was created
        let backup_path = settings_path.with_extension("json.backup");
        assert!(!backup_path.exists());

        // Verify the new content was written
        let content = std::fs::read_to_string(&settings_path).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();
        assert!(json.get("rust-analyzer.linkedProjects").is_some());
    }
}
