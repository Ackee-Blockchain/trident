use super::TransactionHooks;
use crate::traits::FuzzClient;
use crate::trident::Trident;

#[doc(hidden)]
pub trait TransactionSetters: TransactionHooks {
    /// Builds a transaction instance from fuzzer data
    ///
    /// Creates a new transaction with the necessary data from the fuzzer
    /// and prepares any accounts needed for execution.
    fn build(trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) -> Self
    where
        Self: Default,
    {
        let mut tx = Self::default();
        tx.set_instructions(trident, fuzz_accounts);

        tx
    }
    #[doc(hidden)]
    /// Set accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set instructions for the transaction
    fn set_instructions(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts);
}
