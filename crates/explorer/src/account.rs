use crate::{config::ExplorerConfig, error::Result};
use solana_sdk::{account::Account, pubkey::Pubkey};

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
    pub fn with_pubkey(pubkey: Pubkey) -> AccountQueryBuilder {
        AccountQueryBuilder {
            pubkey,
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
