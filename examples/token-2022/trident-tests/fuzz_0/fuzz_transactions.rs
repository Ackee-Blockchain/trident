use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    CreateMintAccountTransaction(CreateMintAccountTransaction),
}
/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub receiver: AccountsStorage,
    pub mint_token_account: AccountsStorage,
    pub associated_token_program: AccountsStorage,
    pub payer: AccountsStorage,
    pub token_program: AccountsStorage,
    pub authority: AccountsStorage,
    pub mint: AccountsStorage,
    pub system_program: AccountsStorage,
    pub extra_metas_account: AccountsStorage,
}
