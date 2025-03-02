use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Escrow {
    pub recipient: TridentPubkey,
    pub amount: u64,
    pub withdrawal: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
    pub bump: u8,
}
