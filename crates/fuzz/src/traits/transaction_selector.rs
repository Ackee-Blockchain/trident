#![allow(dead_code)]

use crate::error::FuzzingError;
use trident_config::TridentConfig;

use super::FuzzClient;

pub trait TransactionSelector<T> {
    fn transaction_selector(
        &mut self,
        client: &mut impl FuzzClient,
        config: &TridentConfig,
        fuzz_accounts: &mut T,
    ) -> Result<(), FuzzingError>;
}
