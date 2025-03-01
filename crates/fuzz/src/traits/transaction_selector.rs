#![allow(dead_code)]

use crate::error::FuzzingError;

use super::FuzzClient;

pub trait TransactionSelector<T> {
    fn transaction_selector(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut T,
    ) -> Result<(), FuzzingError>;
}
