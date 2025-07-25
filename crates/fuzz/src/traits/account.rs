use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

use super::FuzzClient;

pub trait AccountsMethods {
    type IxAccounts;
    type IxData;

    /// Resolve Instruction accounts based on the attributes specified within macro
    #[doc(hidden)]
    #[allow(unused_variables)]
    fn resolve_accounts(
        &mut self,
        trident: &mut Trident,
        ix_accounts: &mut Self::IxAccounts,
        _program_id: Pubkey,
        instruction_data: &Self::IxData,
    ) {
    }

    /// Convert instruction accounts to AccountMeta
    #[doc(hidden)]
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;

    /// Capture the state of accounts before transaction execution
    #[doc(hidden)]
    fn capture_before(&mut self, client: &mut impl FuzzClient);

    /// Capture the state of accounts after transaction execution
    #[doc(hidden)]
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
