use crate::{config::ExplorerConfig, error::Result};
use solana_sdk::{account::Account, pubkey::Pubkey};

pub struct AccountQuery {
    pubkey: Pubkey,
    config: ExplorerConfig,
}

impl AccountQuery {
    pub async fn fetch_one(&self) -> Result<Account> {
        let rpc_client = self.config.rpc_client();
        let account = rpc_client.get_account(&self.pubkey).await?;
        Ok(account)
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
