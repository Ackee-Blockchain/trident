use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::regression::regression::AccountSnapshot;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegressionFile {
    pub state_hash: Option<String>,
    pub snapshots: BTreeMap<String, BTreeMap<String, Vec<AccountSnapshot>>>,
}

#[derive(Debug)]
pub struct ComparisonResult {
    pub identical: bool,
    pub differing_seeds: Vec<String>,
    pub only_in_first: Vec<String>,
    pub only_in_second: Vec<String>,
}

impl ComparisonResult {
    pub fn has_differences(&self) -> bool {
        !self.differing_seeds.is_empty()
            || !self.only_in_first.is_empty()
            || !self.only_in_second.is_empty()
    }
}

/// Compare two regression JSON files and return which iteration seeds differ
pub fn compare_regression_files(
    file1_path: &str,
    file2_path: &str,
) -> std::io::Result<ComparisonResult> {
    // Read and parse both files
    let file1_content = std::fs::read_to_string(file1_path)?;
    let file2_content = std::fs::read_to_string(file2_path)?;

    let regression1: RegressionFile = serde_json::from_str(&file1_content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse {}: {}", file1_path, e),
        )
    })?;

    let regression2: RegressionFile = serde_json::from_str(&file2_content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse {}: {}", file2_path, e),
        )
    })?;

    compare_regression_data(&regression1, &regression2)
}

/// Compare two RegressionFile structures
pub fn compare_regression_data(
    regression1: &RegressionFile,
    regression2: &RegressionFile,
) -> std::io::Result<ComparisonResult> {
    let mut differing_seeds = Vec::new();
    let mut only_in_first = Vec::new();
    let mut only_in_second = Vec::new();

    // Get all unique iteration seeds from both files
    let seeds1: HashSet<_> = regression1.snapshots.keys().collect();
    let seeds2: HashSet<_> = regression2.snapshots.keys().collect();

    // Find seeds only in first file
    for seed in seeds1.difference(&seeds2) {
        only_in_first.push((*seed).clone());
    }

    // Find seeds only in second file
    for seed in seeds2.difference(&seeds1) {
        only_in_second.push((*seed).clone());
    }

    // Compare seeds that exist in both files
    for seed in seeds1.intersection(&seeds2) {
        let snapshots1 = &regression1.snapshots[*seed];
        let snapshots2 = &regression2.snapshots[*seed];

        // Compare the snapshots for this seed
        if !snapshots_equal(snapshots1, snapshots2) {
            differing_seeds.push((*seed).clone());
        }
    }

    // Sort for consistent output
    differing_seeds.sort();
    only_in_first.sort();
    only_in_second.sort();

    let identical =
        differing_seeds.is_empty() && only_in_first.is_empty() && only_in_second.is_empty();

    Ok(ComparisonResult {
        identical,
        differing_seeds,
        only_in_first,
        only_in_second,
    })
}

/// Compare two sets of snapshots for equality
fn snapshots_equal(
    snapshots1: &BTreeMap<String, Vec<AccountSnapshot>>,
    snapshots2: &BTreeMap<String, Vec<AccountSnapshot>>,
) -> bool {
    if snapshots1.len() != snapshots2.len() {
        return false;
    }

    for (account_name, account_snapshots1) in snapshots1 {
        if let Some(account_snapshots2) = snapshots2.get(account_name) {
            if account_snapshots1.len() != account_snapshots2.len() {
                return false;
            }

            // Compare each snapshot
            for (snapshot1, snapshot2) in account_snapshots1.iter().zip(account_snapshots2.iter()) {
                if snapshot1.data_hash != snapshot2.data_hash {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    true
}
