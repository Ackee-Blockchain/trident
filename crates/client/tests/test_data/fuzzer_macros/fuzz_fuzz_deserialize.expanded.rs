use trdelnik_client::FuzzDeserialize;
pub enum FuzzInstruction {
    InitVesting(InitVesting),
    WithdrawUnlocked(WithdrawUnlocked),
}
impl<'info> FuzzDeserialize<'info> for InitVesting {
    type Ix = InitVestingSnapshot<'info>;
    fn deserialize_option(
        &self,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> Result<Self::Ix, FuzzingError> {
        Self::Ix::deserialize_option(accounts)
    }
}
impl<'info> FuzzDeserialize<'info> for WithdrawUnlocked {
    type Ix = WithdrawUnlockedSnapshot<'info>;
    fn deserialize_option(
        &self,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> Result<Self::Ix, FuzzingError> {
        Self::Ix::deserialize_option(accounts)
    }
}
pub struct InitVesting {
    pub accounts: InitVestingAccounts,
    pub data: InitVestingData,
}
pub struct InitVestingAccounts {
    pub sender: AccountId,
    pub sender_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub mint: AccountId,
    pub token_program: AccountId,
    pub system_program: AccountId,
}
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
pub struct WithdrawUnlocked {
    pub accounts: WithdrawUnlockedAccounts,
    pub data: WithdrawUnlockedData,
}
pub struct WithdrawUnlockedAccounts {
    pub recipient: AccountId,
    pub recipient_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub escrow_pda_authority: AccountId,
    pub mint: AccountId,
    pub token_program: AccountId,
    pub system_program: AccountId,
}
pub struct WithdrawUnlockedData {}
