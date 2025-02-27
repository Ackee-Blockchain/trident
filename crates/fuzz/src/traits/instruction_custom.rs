use solana_sdk::pubkey::Pubkey;

use crate::traits::FuzzClient;

/// Trait implementing methods for the fuzzed instructions
pub trait InstructionCustomMethods {
    type IxAccounts;

    /// Set Instruction program id
    fn program_id(&self) -> Pubkey;

    /// Set Instruction discriminator
    fn discriminator(&self) -> Vec<u8>;

    #[doc(hidden)]
    /// Set Instruction data
    #[allow(unused_variables)]
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {}

    #[doc(hidden)]
    /// Set Instruction accounts
    #[allow(unused_variables)]
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
    }

    #[doc(hidden)]
    /// Set Instruction remaining accounts
    #[allow(unused_variables)]
    fn set_remaining_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
    }
}
