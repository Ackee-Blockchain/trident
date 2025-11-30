use crate::fuzz::AccountRaw;
use crate::fuzz::FuzzCluster;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedForkAccount {
    pub address: String,
    pub cluster: String,
    pub account: AccountRaw,
}

impl CachedForkAccount {
    pub fn new(address: Pubkey, cluster: &FuzzCluster, account: &AccountSharedData) -> Self {
        let data_base64 = BASE64_STANDARD.encode(account.data());

        CachedForkAccount {
            address: address.to_string(),
            cluster: cluster.as_str().to_string(),
            account: AccountRaw {
                lamports: account.lamports(),
                data: vec![data_base64],
                owner: account.owner().to_string(),
                executable: account.executable(),
                rent_epoch: account.rent_epoch(),
            },
        }
    }

    pub fn to_account_shared_data(&self) -> Result<AccountSharedData, Box<dyn std::error::Error>> {
        use std::str::FromStr;

        let owner = Pubkey::from_str(&self.account.owner)?;
        let data_base64 = self.account.data.first().ok_or("Missing account data")?;
        let data = BASE64_STANDARD.decode(data_base64)?;

        Ok(AccountSharedData::create(
            self.account.lamports,
            data,
            owner,
            self.account.executable,
            self.account.rent_epoch,
        ))
    }
}
