use solana_sdk::instruction::AccountMeta;

pub trait InstructionGetters {
    #[doc(hidden)]
    /// Convert accounts to account metas
    fn to_account_metas(&mut self) -> Vec<AccountMeta>;
}
