#![allow(dead_code)]

use crate::error::*;
use crate::traits::FuzzClient;

use solana_sdk::transaction::TransactionError;

/// Trait providing custom methods for transactions
pub trait TransactionHooks {
    type IxAccounts;

    /// Method to perform custom actions before the transaction is executed
    #[allow(unused_variables)]
    fn pre_transaction(&self, client: &mut impl FuzzClient) {}

    /// Method to perform custom checks on the transaction
    #[allow(unused_variables)]
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        Ok(())
    }

    /// Method to handle transaction errors
    #[allow(unused_variables)]
    fn transaction_error_handler(&self, e: TransactionError) -> Result<(), TransactionError> {
        Err(e)
    }

    /// Method to perform custom actions after the transaction is executed
    #[allow(unused_variables)]
    fn post_transaction(&self, client: &mut impl FuzzClient) {}
}
