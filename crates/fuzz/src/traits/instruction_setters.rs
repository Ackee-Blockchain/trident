use crate::traits::FuzzClient;

use super::InstructionHooks;

pub trait InstructionSetters: InstructionHooks {
    #[doc(hidden)]
    /// Set snapshot of accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set snapshot of accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Resolve accounts
    fn resolve_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    );
}
