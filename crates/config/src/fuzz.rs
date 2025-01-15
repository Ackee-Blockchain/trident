use crate::utils::resolve_path;
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    pubkey::Pubkey,
};
use std::{fs, str::FromStr};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Fuzz {
    pub fuzzing_with_stats: Option<bool>,
    pub allow_duplicate_txs: Option<bool>,
    pub programs: Option<Vec<_FuzzProgram>>,
    pub accounts: Option<Vec<_FuzzAccount>>,
}

impl Fuzz {
    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzzing_with_stats.unwrap_or(false)
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        self.allow_duplicate_txs.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct _FuzzProgram {
    pub address: String,
    pub program: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct _FuzzAccount {
    pub address: String,
    pub filename: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FuzzProgram {
    pub address: Pubkey,
    pub data: Vec<u8>,
}

impl From<&_FuzzProgram> for FuzzProgram {
    fn from(_f: &_FuzzProgram) -> Self {
        let program_path = &_f.program;
        let program_address = &_f.address;

        let path = resolve_path(program_path);

        let program_data =
            fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", program_path));

        let pubkey = Pubkey::from_str(program_address)
            .unwrap_or_else(|_| panic!("Cannot parse the program address: {}", program_address));

        FuzzProgram {
            address: pubkey,
            data: program_data,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FuzzAccount {
    pub pubkey: Pubkey,
    pub account: AccountSharedData,
}

impl From<&_FuzzAccount> for FuzzAccount {
    fn from(_f: &_FuzzAccount) -> Self {
        let account_path = &_f.filename;

        let path = resolve_path(account_path);

        let file_content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", account_path));

        let account_raw: FuzzAccountRaw = serde_json::from_str(&file_content)
            .unwrap_or_else(|_| panic!("Failed to parse JSON from file: {}", account_path));

        let pubkey = Pubkey::from_str(&account_raw.pubkey)
            .unwrap_or_else(|_| panic!("Cannot convert address for: {}", account_raw.pubkey));

        let owner_address = Pubkey::from_str(&account_raw.account.owner).unwrap_or_else(|_| {
            panic!(
                "Cannot convert address for owner: {}",
                account_raw.account.owner
            )
        });

        let data_base_64 = account_raw.account.data.first().unwrap_or_else(|| {
            panic!(
                "Cannot read base64 data for account: {}",
                account_raw.pubkey
            )
        });

        let account = AccountSharedData::create(
            account_raw.account.lamports,
            BASE64_STANDARD
                .decode(data_base_64)
                .unwrap_or_else(|_| panic!("Failed to decode base64 data of {}", account_path)),
            owner_address,
            account_raw.account.executable,
            account_raw.account.rent_epoch,
        );

        FuzzAccount { pubkey, account }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FuzzAccountRaw {
    pub pubkey: String,
    pub account: AccountRaw,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountRaw {
    pub lamports: u64,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
}
