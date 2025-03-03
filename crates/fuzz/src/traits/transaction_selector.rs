#![allow(dead_code)]

use crate::error::FuzzingError;
use crate::types::FuzzerData;

use super::FuzzClient;

pub trait TransactionSelector<T> {
    // This method is used with random selection callback
    #[doc(hidden)]
    fn transaction_selector(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut T,
    ) -> Result<(), FuzzingError>;

    // Select random Transaction and execute it
    fn select_n_execute(
        fuzzer_data: &mut FuzzerData,
        client: &mut impl FuzzClient,
        accounts: &mut T,
    ) -> Result<(), FuzzingError>;

    // Select random Transaction and execute it without hooks
    fn select_n_execute_no_hooks(
        fuzzer_data: &mut FuzzerData,
        client: &mut impl FuzzClient,
        accounts: &mut T,
    ) -> Result<(), FuzzingError>;
}
