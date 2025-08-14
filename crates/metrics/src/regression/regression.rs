use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AccountSnapshot {
    pub(crate) data_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FuzzingRegression {
    // iteration seed | account name | account snapshots
    pub(crate) snapshots: BTreeMap<String, BTreeMap<String, Vec<AccountSnapshot>>>,
}

impl FuzzingRegression {
    pub(crate) fn add_to_regression(
        &mut self,
        iteration_seed: &str,
        account_name: &str,
        account_shared_data: &AccountSharedData,
    ) {
        let mut hasher_account = Sha256::new();
        hasher_account.update(account_shared_data.data());

        let hash = hasher_account.finalize();

        let data_hash = hex::encode(hash);

        let snapshot = AccountSnapshot { data_hash };

        self.snapshots
            .entry(iteration_seed.to_string())
            .or_default()
            .entry(account_name.to_string())
            .or_default()
            .push(snapshot);
    }

    pub(crate) fn merge_from(&mut self, other: &FuzzingRegression) {
        self.snapshots.extend(other.snapshots.clone());
    }

    pub(crate) fn generate(&self, file_name: &str) -> std::io::Result<()> {
        // Create JSON structure with state hash at the top
        let mut regression_data = serde_json::Map::new();

        // Add state hash at the top - hash of just the snapshots data
        if !self.snapshots.is_empty() {
            let snapshots_hash = self.get_snapshots_hash()?;
            regression_data.insert("state_hash".to_string(), snapshots_hash.into());
        }

        // Add the regression snapshots data
        regression_data.insert(
            "snapshots".to_string(),
            serde_json::to_value(&self.snapshots)?,
        );

        let file = File::create(file_name)?;
        serde_json::to_writer_pretty(file, &regression_data)?;

        Ok(())
    }

    /// Get the hash of just the snapshots data
    pub(crate) fn get_snapshots_hash(&self) -> std::io::Result<String> {
        if self.snapshots.is_empty() {
            return Ok("".to_string());
        }

        let snapshots_json = serde_json::to_vec_pretty(&self.snapshots)?;
        let mut hasher = Sha256::new();
        hasher.update(snapshots_json);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }
}
