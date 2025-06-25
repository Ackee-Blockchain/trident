use super::TransactionHooks;
use crate::{fuzzing::TridentRng, traits::FuzzClient};

#[doc(hidden)]
pub trait TransactionSetters: TransactionHooks {
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
