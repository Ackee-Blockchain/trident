use crate::{config::ExplorerConfig, error::Result, output::pretty_lamports_to_sol};
use console::style;
use serde::Serialize;
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::fmt;

#[derive(Serialize)]
pub struct KeyedAccount {
    pub pubkey: Pubkey,
    pub account: Account,
}

pub struct AccountQuery {
    pubkey: Pubkey,
    config: ExplorerConfig,
}

impl AccountQuery {
    pub async fn fetch_one(&self) -> Result<KeyedAccount> {
        let rpc_client = self.config.rpc_client();
        let account = rpc_client.get_account(&self.pubkey).await?;
        let keyed_account = KeyedAccount {
            pubkey: self.pubkey,
            account,
        };
        Ok(keyed_account)
    }
}

pub struct AccountQueryBuilder {
    pubkey: Pubkey,
    config: ExplorerConfig,
}

impl AccountQueryBuilder {
    pub fn with_pubkey(pubkey: &Pubkey) -> AccountQueryBuilder {
        AccountQueryBuilder {
            pubkey: *pubkey,
            config: ExplorerConfig::default(),
        }
    }

    pub fn pubkey(&mut self, pubkey: Pubkey) -> &mut AccountQueryBuilder {
        self.pubkey = pubkey;
        self
    }

    pub fn config(&mut self, config: ExplorerConfig) -> &mut AccountQueryBuilder {
        self.config = config;
        self
    }

    pub fn build(self) -> AccountQuery {
        AccountQuery {
            pubkey: self.pubkey,
            config: self.config,
        }
    }
}

pub struct AccountFieldVisibility {
    lamports: bool,
    data: bool,
    owner: bool,
    executable: bool,
    rent_epoch: bool,
}

impl AccountFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            lamports: true,
            data: true,
            owner: true,
            executable: true,
            rent_epoch: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            lamports: false,
            data: false,
            owner: false,
            executable: false,
            rent_epoch: false,
        }
    }

    pub fn lamports(&self) -> bool {
        self.lamports
    }

    pub fn enable_lamports(&mut self) -> &mut Self {
        self.lamports = true;
        self
    }

    pub fn disable_lamports(&mut self) -> &mut Self {
        self.lamports = false;
        self
    }

    pub fn data(&self) -> bool {
        self.data
    }

    pub fn enable_data(&mut self) -> &mut Self {
        self.data = true;
        self
    }

    pub fn disable_data(&mut self) -> &mut Self {
        self.data = false;
        self
    }

    pub fn owner(&self) -> bool {
        self.owner
    }

    pub fn enable_owner(&mut self) -> &mut Self {
        self.owner = true;
        self
    }

    pub fn disable_owner(&mut self) -> &mut Self {
        self.owner = false;
        self
    }

    pub fn executable(&self) -> bool {
        self.executable
    }

    pub fn enable_executable(&mut self) -> &mut Self {
        self.executable = true;
        self
    }

    pub fn disable_executable(&mut self) -> &mut Self {
        self.executable = false;
        self
    }

    pub fn rent_epoch(&self) -> bool {
        self.rent_epoch
    }

    pub fn enable_rent_epoch(&mut self) -> &mut Self {
        self.rent_epoch = true;
        self
    }

    pub fn disable_rent_epoch(&mut self) -> &mut Self {
        self.rent_epoch = false;
        self
    }
}

#[derive(Serialize)]
pub struct DisplayAccount {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize)]
pub struct DisplayKeyedAccount {
    pub pubkey: String,
    pub account: DisplayAccount,
}

impl DisplayKeyedAccount {
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Self {
        Self {
            pubkey: keyed_account.pubkey.to_string(),
            account: DisplayAccount {
                lamports: keyed_account.account.lamports,
                data: base64::encode(&keyed_account.account.data),
                owner: keyed_account.account.owner.to_string(),
                executable: keyed_account.account.executable,
                rent_epoch: keyed_account.account.rent_epoch,
            },
        }
    }
}

impl fmt::Display for DisplayKeyedAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "========================================================"
        )?;
        writeln!(f, "{} {}", style("Public Key:").bold(), self.pubkey)?;
        writeln!(
            f,
            "========================================================"
        )?;

        writeln!(f)?;

        writeln!(
            f,
            "{} {} (◎ {})",
            style("Lamports:").bold(),
            self.account.lamports,
            pretty_lamports_to_sol(self.account.lamports)
        )?;
        if self.account.data.is_empty() {
            writeln!(f, "{} [Empty]", style("Data:").bold())?;
        } else {
            writeln!(f, "{} [Hexdump below]", style("Data:").bold())?;
        }
        writeln!(f, "{} {}", style("Owner").bold(), &self.account.owner)?;
        writeln!(
            f,
            "{} {}",
            style("Executable:").bold(),
            self.account.executable
        )?;
        write!(
            f,
            "{} {}",
            style("Rent Epoch:").bold(),
            self.account.rent_epoch
        )?;

        Ok(())
    }
}
