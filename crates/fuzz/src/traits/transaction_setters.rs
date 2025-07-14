use super::TransactionHooks;
use crate::fuzzing::TridentRng;
use crate::traits::FuzzClient;

#[doc(hidden)]
pub trait TransactionSetters: TransactionHooks {
    /// Builds a transaction instance from fuzzer data
    ///
    /// Creates a new transaction with the necessary data from the fuzzer
    /// and prepares any accounts needed for execution.
    fn build(
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) -> Self;
    #[doc(hidden)]
    /// Set accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set instructions for the transaction
    fn set_instructions(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    );
}
