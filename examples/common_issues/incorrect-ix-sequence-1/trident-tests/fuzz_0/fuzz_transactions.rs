#![allow(clippy::enum_variant_names)]
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
    EndRegistrationsTransaction(EndRegistrationsTransaction),
    InitializeTransaction(InitializeTransaction),
    InvestTransaction(InvestTransaction),
    RegisterTransaction(RegisterTransaction),
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    pub author: AccountsStorage<KeypairStore>,
    pub investor: AccountsStorage<KeypairStore>,
    pub project: AccountsStorage<PdaStore>,
    pub project_author: AccountsStorage<KeypairStore>,
    pub state: AccountsStorage<PdaStore>,
}
