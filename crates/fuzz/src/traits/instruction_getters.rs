use solana_sdk::instruction::AccountMeta;

use super::InstructionHooks;

pub trait InstructionGetters: InstructionHooks {
    #[doc(hidden)]
    /// Get Instruction discriminator
    fn get_discriminator(&self) -> Vec<u8>;

    #[doc(hidden)]
    /// Get Instruction program id
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey;

    #[doc(hidden)]
    /// Convert accounts to account metas
    fn to_account_metas(&mut self) -> Vec<AccountMeta>;
}
