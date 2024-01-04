use crate::state::{Escrow, ESCROW_SEED};
use anchor_lang::prelude::*;

pub fn _withdraw(ctx: Context<Withdraw>) -> Result<()> {
    // close will transfer everything to the receiver
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(
        mut,
        close = receiver,
        // INFO we have missing check here that receiver is actually receiver from
        // the Escrow, same goes for receiver PKey seed which is taken from the Escrow
        // Account not from the Signer
        seeds = [escrow.author.key().as_ref(),escrow.receiver.as_ref(),ESCROW_SEED.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}
