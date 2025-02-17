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
    InitializeTransaction(InitializeTransaction),
    ProcessCustomTypesTransaction(ProcessCustomTypesTransaction),
    ProcessRustTypesTransaction(ProcessRustTypesTransaction),
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
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
