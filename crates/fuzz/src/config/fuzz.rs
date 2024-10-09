use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use super::discover_root;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Fuzz {
    pub fuzzing_with_stats: bool,
    pub allow_duplicate_txs: bool,
    pub programs: Vec<FuzzProgram>,
    pub accounts: Vec<FuzzAccount>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct _Fuzz {
    #[serde(default)]
    pub fuzzing_with_stats: Option<bool>,
    #[serde(default)]
    pub allow_duplicate_txs: Option<bool>,
    #[serde(default)]
    pub programs: Option<Vec<_FuzzProgram>>,
    #[serde(default)]
    pub accounts: Option<Vec<_FuzzAccount>>,
}
impl From<_Fuzz> for Fuzz {
    fn from(_f: _Fuzz) -> Self {
        let mut _self = Self {
            fuzzing_with_stats: _f.fuzzing_with_stats.unwrap_or_default(),
            allow_duplicate_txs: _f.allow_duplicate_txs.unwrap_or_default(),
            programs: vec![],
            accounts: vec![],
        };

        if let Some(accounts) = _f.accounts {
            for account in accounts {
                _self
                    .accounts
                    .push(read_and_parse_account(&account.filename));
            }
        }
        if let Some(programs) = _f.programs {
            for account in programs {
                _self
                    .programs
                    .push(read_and_parse_program(&account.program, &account.address));
            }
        }

        _self
    }
}

impl Fuzz {
    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzzing_with_stats
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        self.allow_duplicate_txs
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

#[derive(Debug, Deserialize, Clone)]
pub struct FuzzAccount {
    pub pubkey: Pubkey,
    pub account: Account,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub lamports: u64,
    pub data: String,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
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

fn read_and_parse_program(filename: &str, program_address: &str) -> FuzzProgram {
    let path = resolve_path(filename);

    let program_data =
        fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let pubkey = Pubkey::from_str(program_address)
        .unwrap_or_else(|_| panic!("Cannot parse the program address: {}", program_address));

    FuzzProgram {
        address: pubkey,
        data: program_data,
    }
}

fn read_and_parse_account(filename: &str) -> FuzzAccount {
    let path = resolve_path(filename);

    let file_content =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let account_raw: FuzzAccountRaw = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to parse JSON from file: {}", filename));

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

    let account = Account {
        lamports: account_raw.account.lamports,
        data: data_base_64.to_string(),
        owner: owner_address,
        executable: account_raw.account.executable,
        rent_epoch: account_raw.account.rent_epoch,
    };

    FuzzAccount { pubkey, account }
}

fn resolve_path(filename: &str) -> PathBuf {
    let path = Path::new(filename);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        discover_root()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| panic!("Failed to resolve relative path: {}", path.display()))
    }
}
