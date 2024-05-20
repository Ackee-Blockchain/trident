use anchor_lang::prelude::*;

pub const STATE_SEED: &str = "state_seed";
pub const PROJECT_SEED: &str = "project_seed";

#[account]
pub struct State {
    pub author: Pubkey,
    pub registrations_round: bool,
    pub total_invested: u64,
    pub bump: u8,
}

impl State {
    pub const LEN: usize = 32 + 1 + 8 + 1;
}

#[account]
pub struct Project {
    pub project_author: Pubkey,
    pub invested_amount: u64,
    pub bump: u8,
}

impl Project {
    pub const LEN: usize = 32 + 8 + 1;
}
