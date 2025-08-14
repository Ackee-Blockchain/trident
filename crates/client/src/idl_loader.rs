use heck::ToSnakeCase;
use std::fmt;
use std::fs::File;
use std::fs::{self};
use std::io::Read;
use std::path::PathBuf;

use trident_idl_spec::Idl;

/// Custom error type for IDL loading operations
#[derive(Debug)]
pub enum IdlError {
    IoError {
        source: std::io::Error,
        path: PathBuf,
        operation: &'static str,
    },
    ParseError {
        source: serde_json::Error,
        path: PathBuf,
    },
    NoIdlsFound {
        path: String,
    },
}

impl fmt::Display for IdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdlError::IoError {
                source,
                path,
                operation,
            } => {
                write!(
                    f,
                    "Failed to {} '{}': {}",
                    operation,
                    path.display(),
                    source
                )
            }
            IdlError::ParseError { source, path } => {
                write!(
                    f,
                    "Failed to parse IDL file '{}': {}",
                    path.display(),
                    source
                )
            }
            IdlError::NoIdlsFound { path } => {
                write!(f, "No IDL files found in {}", path)
            }
        }
    }
}

impl std::error::Error for IdlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IdlError::IoError { source, .. } => Some(source),
            IdlError::ParseError { source, .. } => Some(source),
            IdlError::NoIdlsFound { .. } => None,
        }
    }
}

/// Loads IDL files from a directory
///
/// # Arguments
///
/// * `dir_path` - Path to the directory containing IDL files
/// * `program_name` - Optional program name to filter IDL files
///
/// # Returns
///
/// A Result containing a vector of parsed IDL files or an error with context
pub fn load_idls(dir_path: PathBuf, program_name: Option<String>) -> Result<Vec<Idl>, IdlError> {
    let mut idls = Vec::new();

    // Read the directory and iterate over each entry
    let read_dir = fs::read_dir(&dir_path).map_err(|e| IdlError::IoError {
        source: e,
        path: dir_path.clone(),
        operation: "read directory",
    })?;

    for entry_result in read_dir {
        let entry = entry_result.map_err(|e| IdlError::IoError {
            source: e,
            path: dir_path.clone(),
            operation: "read directory entry",
        })?;

        let path = entry.path();

        if let Some(ref program_name) = program_name {
            if path.is_file()
                && !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    // convert program_name to match case of IDL names
                    .map(|name| name.trim_end_matches(".json") == program_name.to_snake_case())
                    .unwrap_or(false)
            {
                continue;
            }
        }

        // Only process .json files
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Open the file in read-only mode
            let mut file = File::open(&path).map_err(|e| IdlError::IoError {
                source: e,
                path: path.clone(),
                operation: "open file",
            })?;

            // Read the file contents into a string
            let mut json_content = String::new();
            file.read_to_string(&mut json_content)
                .map_err(|e| IdlError::IoError {
                    source: e,
                    path: path.clone(),
                    operation: "read file",
                })?;

            // Parse the string of data into an Idl struct
            match serde_json::from_str::<Idl>(&json_content) {
                Ok(parsed_idl) => {
                    idls.push(parsed_idl);
                }
                Err(e) => {
                    // Instead of just printing the error, collect it in our custom error type
                    return Err(IdlError::ParseError {
                        source: e,
                        path: path.clone(),
                    });
                }
            }
        }
    }

    if idls.is_empty() {
        return Err(IdlError::NoIdlsFound {
            path: dir_path.to_string_lossy().to_string(),
        });
    }

    Ok(idls)
}
