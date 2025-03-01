use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use super::FuzzClient;

pub trait AccountsMethods {
    type IxAccounts;
    type IxData;

    #[doc(hidden)]
    #[allow(unused_variables)]
    fn resolve_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        ix_accounts: &mut Self::IxAccounts,
        _program_id: Pubkey,
        instruction_data: &Self::IxData,
    ) {
    }

    #[doc(hidden)]
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;

    #[doc(hidden)]
    fn capture_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
