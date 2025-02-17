use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used in transactions and
/// instructions
/// or invariant checks.
///
/// You can create your own types here and use them in transactions and
/// instructions.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Escrow {
    pub author: TridentPubkey,
    pub receiver: TridentPubkey,
    pub amount: u64,
    pub bump: u8,
}
