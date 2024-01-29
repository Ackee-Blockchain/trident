use trdelnik_client::__private::DisplayIx;
pub enum FuzzInstruction {
    InitVesting(InitVesting),
    WithdrawUnlocked(WithdrawUnlocked),
}
impl std::fmt::Display for FuzzInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuzzInstruction::InitVesting(_) => f.write_fmt(format_args!("InitVesting")),
            FuzzInstruction::WithdrawUnlocked(_) => {
                f.write_fmt(format_args!("WithdrawUnlocked"))
            }
        }
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
