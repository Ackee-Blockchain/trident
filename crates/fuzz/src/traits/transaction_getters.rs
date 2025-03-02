use super::TransactionHooks;
use crate::traits::FuzzClient;
use solana_sdk::instruction::AccountMeta;

#[doc(hidden)]
pub trait TransactionGetters: TransactionHooks {
    #[doc(hidden)]
    /// Get transaction name
    fn get_transaction_name(&self) -> String;

    #[doc(hidden)]
    /// Get instruction discriminators
    fn get_instruction_discriminators(&self) -> Vec<Vec<u8>>;

    #[doc(hidden)]
    /// Get instruction program ids
    fn get_instruction_program_ids(&self) -> Vec<solana_sdk::pubkey::Pubkey>;

    #[doc(hidden)]
    fn get_instruction_data(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<u8>>;

    #[doc(hidden)]
    /// Get instruction accounts
    fn get_instruction_accounts(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<AccountMeta>>;
}
