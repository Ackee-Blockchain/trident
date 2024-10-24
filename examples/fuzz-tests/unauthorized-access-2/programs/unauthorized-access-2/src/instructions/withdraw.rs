use crate::state::{Escrow, ESCROW_SEED};
use anchor_lang::prelude::*;

use trident_derive_accounts_snapshots::AccountsSnapshots;

pub fn _withdraw(_ctx: Context<Withdraw>) -> Result<()> {
    // close will transfer everything to the receiver
    Ok(())
}

#[derive(Accounts, AccountsSnapshots)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(
        mut,
        close = receiver,
        // INFO: There is a missing check to confirm that the 'receiver' matches the one specified in the Escrow.
        seeds = [escrow.author.key().as_ref(),escrow.receiver.as_ref(),ESCROW_SEED.as_bytes()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}
