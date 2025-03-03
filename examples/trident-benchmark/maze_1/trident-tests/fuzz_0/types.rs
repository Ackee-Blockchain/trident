use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct State {
    x: u64,
    y: u64,
}
