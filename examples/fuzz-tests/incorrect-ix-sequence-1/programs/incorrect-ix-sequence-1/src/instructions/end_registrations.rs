pub use anchor_lang::prelude::*;

use crate::state::{State, STATE_SEED};

pub fn _end_registration(ctx: Context<EndRegistration>) -> Result<()> {
    let state = &mut ctx.accounts.state;

    state.registrations_round = false;
    Ok(())
}

#[derive(Accounts)]
pub struct EndRegistration<'info> {
    pub author: Signer<'info>,
    #[account(
        mut,
        has_one = author,
        seeds = [author.key().as_ref(), STATE_SEED.as_ref()],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
}
