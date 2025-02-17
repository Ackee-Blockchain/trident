use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// Below, the transaction variants are defined.
/// Each variant contains a transaction struct.
/// The transaction struct contains the instruction and the accounts and data.
///
/// You can create your own transactions by adding new variants to the enum.
#[derive(Arbitrary, FuzzTestExecutor)]
pub enum FuzzTransactions {
    InitVestingTransaction(InitVestingTransaction),
    WithdrawUnlockedTransaction(WithdrawUnlockedTransaction),
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    pub sender: AccountsStorage<KeypairStore>,
    pub escrow_token_account: AccountsStorage<KeypairStore>,
    pub mint: AccountsStorage<KeypairStore>,
    pub recipient_token_account: AccountsStorage<KeypairStore>,
    pub recipient: AccountsStorage<KeypairStore>,
    pub escrow_pda_authority: AccountsStorage<PdaStore>,
    pub sender_token_account: AccountsStorage<KeypairStore>,
    pub escrow: AccountsStorage<PdaStore>,
}
