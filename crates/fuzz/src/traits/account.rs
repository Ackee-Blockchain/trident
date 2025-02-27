use solana_sdk::instruction::AccountMeta;

use super::FuzzClient;

/// Trait for managing Solana account operations during fuzzing
pub trait AccountsMethods {
    /// The instruction accounts structure
    type IxAccounts;

    #[doc(hidden)]
    /// Sets up accounts needed for the instruction
    #[allow(unused_variables)]
    fn resolve_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        ix_accounts: &mut Self::IxAccounts,
    ) {
    }

    #[doc(hidden)]
    /// Converts to AccountMeta format for instruction construction
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;

    #[doc(hidden)]
    /// Captures account state before instruction execution
    fn capture_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Captures account state after instruction execution
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
