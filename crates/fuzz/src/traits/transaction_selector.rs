#![allow(dead_code)]

use crate::error::FuzzingError;
use crate::fuzzing::{FuzzingStatistics, TridentRng};

use super::FuzzClient;

pub trait TransactionSelector<T> {
    // This method is used with random selection callback
    #[doc(hidden)]
    fn transaction_selector(
        &mut self,
        stats_logger: &mut FuzzingStatistics,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut T,
        rng: &mut TridentRng,
    ) -> Result<(), FuzzingError>;

    // Select random Transaction and execute it
    fn select_n_execute(
        stats_logger: &mut FuzzingStatistics,
        client: &mut impl FuzzClient,
        accounts: &mut T,
        rng: &mut TridentRng,
    ) -> Result<(), FuzzingError>;

    // Select random Transaction and execute it without hooks
    fn select_n_execute_no_hooks(
        client: &mut impl FuzzClient,
        accounts: &mut T,
        rng: &mut TridentRng,
    ) -> Result<(), FuzzingError>;
}
