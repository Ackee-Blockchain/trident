use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used in transactions and
/// instructions
/// or invariant checks.
///
/// You can create your own types here and use them in transactions and
/// instructions.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Counter {
    authority: TridentPubkey,
    count: u64,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InputUpdatePrameters {
    input1: u8,
    input2: u8,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum InputUpdateVariant {
    UpdateVariant1,
    UpdateVariant2,
}
