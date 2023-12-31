use anchor_lang::prelude::*;

pub const ESCROW_SEED: &str = "escrow_seed";

#[account]
pub struct Escrow {
    pub author: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 8 + 1;
}
