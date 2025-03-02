use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    InitializeCalleeTransaction(InitializeCalleeTransaction),
    InitializeCallerTransaction(InitializeCallerTransaction),
}
/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub signer: AccountsStorage,
    pub program: AccountsStorage,
}
