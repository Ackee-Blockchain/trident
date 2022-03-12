use crate::config::ExplorerConfig;
use solana_sdk::signature::Signature;

pub struct TransactionQuery {
    signature: Signature,
    config: ExplorerConfig,
}

pub struct TransactionQueryBuilder {
    signature: Signature,
    config: ExplorerConfig,
}

impl TransactionQueryBuilder {
    pub fn with_signature(signature: Signature) -> TransactionQueryBuilder {
        TransactionQueryBuilder {
            signature,
            config: ExplorerConfig::default(),
        }
    }

    pub fn signature(&mut self, signature: Signature) -> &mut TransactionQueryBuilder {
        self.signature = signature;
        self
    }

    pub fn config(&mut self, config: ExplorerConfig) -> &mut TransactionQueryBuilder {
        self.config = config;
        self
    }

    pub fn build(self) -> TransactionQuery {
        TransactionQuery {
            signature: self.signature,
            config: self.config,
        }
    }
}
