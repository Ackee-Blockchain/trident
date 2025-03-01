use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    InitializeTransaction(InitializeTransaction),
    ProcessCustomTypesTransaction(ProcessCustomTypesTransaction),
    ProcessRustTypesTransaction(ProcessRustTypesTransaction),
}
/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub data_account_6: AccountsStorage,
    pub system_program: AccountsStorage,
    pub data_account_1: AccountsStorage,
    pub data_account_2: AccountsStorage,
    pub some_account: AccountsStorage,
    pub data_account_3: AccountsStorage,
    pub data_account_4: AccountsStorage,
    pub signer: AccountsStorage,
    pub data_account_5: AccountsStorage,
}
