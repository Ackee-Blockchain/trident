use crate::coverage::Coverage;
use crate::metrics::Metrics;
use crate::regression::Regression;
use crate::utils::resolve_path;
use crate::Error;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::WritableAccount;
use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Fuzz {
    metrics: Option<Metrics>,
    regression: Option<Regression>,
    pub programs: Option<Vec<_FuzzProgram>>,
    pub accounts: Option<Vec<_FuzzAccount>>,
    pub coverage: Option<Coverage>,
    pub fork: Option<Vec<_FuzzFork>>,
}

impl Fuzz {
    pub fn get_metrics(&self) -> bool {
        match self.metrics.as_ref() {
            Some(metrics) => metrics.enabled.unwrap_or(false),
            None => false,
        }
    }

    pub fn get_metrics_json(&self) -> bool {
        match self.metrics.as_ref() {
            Some(metrics) => metrics.json.unwrap_or(false),
            None => false,
        }
    }

    pub fn get_metrics_dashboard(&self) -> bool {
        match self.metrics.as_ref() {
            Some(metrics) => metrics.dashboard.unwrap_or(false),
            None => false,
        }
    }

    pub fn get_regression(&self) -> bool {
        match self.regression.as_ref() {
            Some(regression) => regression.enabled.unwrap_or(false),
            None => false,
        }
    }

    pub fn get_coverage(&self) -> Coverage {
        self.coverage.clone().unwrap_or_default()
    }

    pub fn get_forks(&self) -> Result<Vec<FuzzFork>, Error> {
        match self.fork.as_ref() {
            Some(forks) => forks.iter().map(FuzzFork::try_from_raw).collect(),
            None => Ok(Vec::default()),
        }
    }

    /// Lightweight preflight validation used by CLI startup checks.
    /// This validates addresses and file path existence without loading
    /// full program binaries/account payloads into memory.
    pub fn validate_preflight(&self) -> Result<(), Error> {
        if let Some(programs) = &self.programs {
            for program in programs {
                let _ = Pubkey::from_str(&program.address).map_err(|_| {
                    Error::Anyhow(anyhow::anyhow!(
                        "Cannot parse the program address: {}",
                        program.address
                    ))
                })?;
                if let Some(authority) = program.upgrade_authority.as_ref() {
                    let _ = Pubkey::from_str(authority).map_err(|_| {
                        Error::Anyhow(anyhow::anyhow!(
                            "Cannot parse upgrade authority: {}",
                            authority
                        ))
                    })?;
                }

                let path = resolve_path(&program.program)?;
                if !path.exists() {
                    return Err(Error::Anyhow(anyhow::anyhow!(
                        "Failed to read file: {}",
                        program.program
                    )));
                }
            }
        }

        if let Some(accounts) = &self.accounts {
            for account in accounts {
                let _ = Pubkey::from_str(&account.address).map_err(|_| {
                    Error::Anyhow(anyhow::anyhow!(
                        "Cannot parse account address: {}",
                        account.address
                    ))
                })?;
                let path = resolve_path(&account.filename)?;
                if !path.exists() {
                    return Err(Error::Anyhow(anyhow::anyhow!(
                        "Failed to read file: {}",
                        account.filename
                    )));
                }
            }
        }

        if let Some(forks) = &self.fork {
            for fork in forks {
                let _ = Pubkey::from_str(&fork.address).map_err(|_| {
                    Error::Anyhow(anyhow::anyhow!(
                        "Cannot parse fork address: {}",
                        fork.address
                    ))
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct _FuzzProgram {
    pub address: String,
    pub upgrade_authority: Option<String>,
    pub program: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct _FuzzAccount {
    pub address: String,
    pub filename: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct _FuzzFork {
    pub address: String,
    pub cluster: String,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone)]
pub struct FuzzFork {
    pub address: Pubkey,
    pub cluster: FuzzCluster,
    pub overwrite: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzCluster {
    Mainnet,
    Devnet,
    Testnet,
    Custom(String),
}

impl FuzzCluster {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "mainnet" | "m" => FuzzCluster::Mainnet,
            "devnet" | "d" => FuzzCluster::Devnet,
            "testnet" | "t" => FuzzCluster::Testnet,
            custom => FuzzCluster::Custom(custom.to_string()),
        }
    }

    pub fn rpc_url(&self) -> String {
        match self {
            FuzzCluster::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            FuzzCluster::Devnet => "https://api.devnet.solana.com".to_string(),
            FuzzCluster::Testnet => "https://api.testnet.solana.com".to_string(),
            FuzzCluster::Custom(url) => url.clone(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FuzzCluster::Mainnet => "mainnet-beta",
            FuzzCluster::Devnet => "devnet",
            FuzzCluster::Testnet => "testnet",
            FuzzCluster::Custom(url) => url.as_str(),
        }
    }
}

impl From<&_FuzzFork> for FuzzFork {
    fn from(_f: &_FuzzFork) -> Self {
        let address = Pubkey::from_str(&_f.address)
            .unwrap_or_else(|_| panic!("Cannot parse fork address: {}", _f.address));

        let cluster = FuzzCluster::parse(&_f.cluster);

        FuzzFork {
            address,
            cluster,
            overwrite: _f.overwrite,
        }
    }
}
impl FuzzFork {
    pub fn try_from_raw(value: &_FuzzFork) -> Result<Self, Error> {
        let address = Pubkey::from_str(&value.address).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Cannot parse fork address: {}",
                value.address
            ))
        })?;

        Ok(FuzzFork {
            address,
            cluster: FuzzCluster::parse(&value.cluster),
            overwrite: value.overwrite,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FuzzProgram {
    pub address: Pubkey,
    pub upgrade_authority: Option<Pubkey>,
    pub data: Vec<u8>,
}

impl From<&_FuzzProgram> for FuzzProgram {
    fn from(_f: &_FuzzProgram) -> Self {
        let program_path = &_f.program;
        let program_address = &_f.address;

        let upgrade_authority = _f
            .upgrade_authority
            .as_ref()
            .map(|upgrade_authority| Pubkey::from_str(upgrade_authority).unwrap());

        let path = resolve_path(program_path)
            .unwrap_or_else(|_| panic!("Failed to resolve path: {}", program_path));

        let program_data =
            fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", program_path));

        let pubkey = Pubkey::from_str(program_address)
            .unwrap_or_else(|_| panic!("Cannot parse the program address: {}", program_address));

        FuzzProgram {
            address: pubkey,
            upgrade_authority,
            data: program_data,
        }
    }
}
impl FuzzProgram {
    pub fn try_from_raw(value: &_FuzzProgram) -> Result<Self, Error> {
        let path = resolve_path(&value.program)?;
        let program_data = fs::read(path).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!("Failed to read file: {}", value.program))
        })?;

        let address = Pubkey::from_str(&value.address).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Cannot parse the program address: {}",
                value.address
            ))
        })?;

        let upgrade_authority = match value.upgrade_authority.as_ref() {
            Some(authority) => Some(Pubkey::from_str(authority).map_err(|_| {
                Error::Anyhow(anyhow::anyhow!(
                    "Cannot parse upgrade authority: {}",
                    authority
                ))
            })?),
            None => None,
        };

        Ok(FuzzProgram {
            address,
            upgrade_authority,
            data: program_data,
        })
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

        let path = resolve_path(account_path)
            .unwrap_or_else(|_| panic!("Failed to resolve path: {}", account_path));

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
impl FuzzAccount {
    pub fn try_from_raw(value: &_FuzzAccount) -> Result<Self, Error> {
        let path = resolve_path(&value.filename)?;
        let file_content = fs::read_to_string(path).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!("Failed to read file: {}", value.filename))
        })?;

        let account_raw: FuzzAccountRaw = serde_json::from_str(&file_content).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Failed to parse JSON from file: {}",
                value.filename
            ))
        })?;

        let pubkey = Pubkey::from_str(&account_raw.pubkey).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Cannot convert address for: {}",
                account_raw.pubkey
            ))
        })?;

        let owner_address = Pubkey::from_str(&account_raw.account.owner).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Cannot convert address for owner: {}",
                account_raw.account.owner
            ))
        })?;

        let data_base_64 = account_raw.account.data.first().ok_or_else(|| {
            Error::Anyhow(anyhow::anyhow!(
                "Cannot read base64 data for account: {}",
                account_raw.pubkey
            ))
        })?;

        let data = BASE64_STANDARD.decode(data_base_64).map_err(|_| {
            Error::Anyhow(anyhow::anyhow!(
                "Failed to decode base64 data of {}",
                value.filename
            ))
        })?;

        let account = AccountSharedData::create(
            account_raw.account.lamports,
            data,
            owner_address,
            account_raw.account.executable,
            account_raw.account.rent_epoch,
        );

        Ok(FuzzAccount { pubkey, account })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FuzzAccountRaw {
    pub pubkey: String,
    pub account: AccountRaw,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AccountRaw {
    pub lamports: u64,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
}
