use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("7sP6fczLZFSxCYSMbVcHt35sMTMzbTS1oT9Qcsxc61Kt");

#[program]
pub mod fuzz_example1 {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        _initialize(ctx)
    }
    pub fn register(ctx: Context<Register>) -> Result<()> {
        _register(ctx)
    }
    pub fn end_registrations(ctx: Context<EndRegistration>) -> Result<()> {
        _end_registration(ctx)
    }
    pub fn invest(ctx: Context<Invest>, amount: u64) -> Result<()> {
        _invest(ctx, amount)
    }
}
