use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used in transactions and
/// instructions
/// or invariant checks.
///
/// You can create your own types here and use them in transactions and
/// instructions.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Project {
    pub project_author: TridentPubkey,
    pub invested_amount: u64,
    pub bump: u8,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct State {
    pub author: TridentPubkey,
    pub registrations_round: bool,
    pub total_invested: u64,
    pub bump: u8,
}
