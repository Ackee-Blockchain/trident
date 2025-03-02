use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    InitVestingTransaction(InitVestingTransaction),
    WithdrawUnlockedTransaction(WithdrawUnlockedTransaction),
}
/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub system_program: AccountsStorage,
    pub escrow: AccountsStorage,
    pub sender: AccountsStorage,
    pub token_program: AccountsStorage,
    pub escrow_token_account: AccountsStorage,
    pub mint: AccountsStorage,
    pub recipient: AccountsStorage,
    pub sender_token_account: AccountsStorage,
    pub recipient_token_account: AccountsStorage,
    pub escrow_pda_authority: AccountsStorage,
}
