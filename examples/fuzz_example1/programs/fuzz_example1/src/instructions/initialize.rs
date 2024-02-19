pub use anchor_lang::prelude::*;

use crate::state::{State, STATE_SEED};

pub fn _initialize(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;

    state.author = ctx.accounts.author.key();
    state.total_invested = 0;
    state.bump = ctx.bumps.state;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + State::LEN,
        seeds = [author.key().as_ref(), STATE_SEED.as_ref()],
        bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}
