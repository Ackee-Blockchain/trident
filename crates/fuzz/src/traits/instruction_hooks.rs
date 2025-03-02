use crate::traits::FuzzClient;

/// Trait implementing methods for the fuzzed instructions
pub trait InstructionHooks {
    type IxAccounts;
    /// Set Instruction data
    #[allow(unused_variables)]
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {}

    /// Set Instruction accounts
    #[allow(unused_variables)]
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
    }

    /// Set Instruction remaining accounts
    #[allow(unused_variables)]
    fn set_remaining_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
    }
}
