#![allow(dead_code)]

use std::cell::RefCell;

use crate::error::FuzzingError;
use trident_config::TridentConfig;
use trident_svm::trident_svm::TridentSVM;

pub trait FuzzTestExecutor<T> {
    // Obtains the name of the transaction
    fn process_transaction(
        &mut self,
        client: &mut TridentSVM,
        config: &TridentConfig,
        fuzz_accounts: &RefCell<T>,
    ) -> Result<(), FuzzingError>;
}
