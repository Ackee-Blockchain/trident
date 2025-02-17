#![allow(clippy::large_enum_variant)]
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
    InitializeTransaction(InitializeTransaction),
    MoveEastTransaction(MoveEastTransaction),
    MoveNorthTransaction(MoveNorthTransaction),
    MoveSouthTransaction(MoveSouthTransaction),
    MoveWestTransaction(MoveWestTransaction),
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    pub state: AccountsStorage<PdaStore>,
    pub state_author: AccountsStorage<KeypairStore>,
}
