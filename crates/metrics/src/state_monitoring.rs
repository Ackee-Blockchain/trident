use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::pubkey::Pubkey;
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AccountSnapshot {
    data_hash: String,
    capture_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StateMonitor {
    snapshots: BTreeMap<String, BTreeMap<String, Vec<AccountSnapshot>>>,
}

impl StateMonitor {
    pub(crate) fn monitor_account_state(
        &mut self,
        iteration_seed: &str,
        account_name: &str,
        address: &Pubkey,
        account_shared_data: &AccountSharedData,
    ) {
        // Hash the account data
        let mut hasher = Sha256::new();
        hasher.update(account_shared_data.data());
        let hash = hasher.finalize();
        let data_hash = hex::encode(hash);

        let snapshot = AccountSnapshot {
            data_hash,
            capture_name: account_name.to_string(),
        };

        self.snapshots
            .entry(iteration_seed.to_string())
            .or_default()
            .entry(address.to_string())
            .or_default()
            .push(snapshot);
    }

    pub(crate) fn merge_from(&mut self, other: &StateMonitor) {
        self.snapshots.extend(other.snapshots.clone());
    }

    /// Get the hash of the state monitor data
    pub(crate) fn get_state_hash(&self) -> std::io::Result<Option<String>> {
        if self.snapshots.is_empty() {
            return Ok(None);
        }

        let self_hash = serde_json::to_vec_pretty(&self)?;
        let mut hasher = Sha256::new();
        hasher.update(self_hash);
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);

        Ok(Some(hash_hex))
    }

    pub(crate) fn generate(&self, file_name: &str) -> std::io::Result<()> {
        if !self.snapshots.is_empty() {
            let file = File::create(file_name)?;
            serde_json::to_writer_pretty(file, &self)?;
        }

        Ok(())
    }
}
