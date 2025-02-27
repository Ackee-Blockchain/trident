#![allow(dead_code)]

use crate::error::FuzzingError;
use crate::traits::FuzzClient;

pub trait Selector {
    type IxAccounts: Default;

    // Obtains the name of the transaction
    fn execute_with_hooks(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Result<(), FuzzingError>;
}
