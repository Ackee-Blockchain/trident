use crate::state::{Escrow, ESCROW_SEED};
use anchor_lang::prelude::*;

pub fn _withdraw(_ctx: Context<Withdraw>) -> Result<()> {
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
        // seed is derived from saved receiver not from context receiver
        seeds = [escrow.author.key().as_ref(),escrow.receiver.as_ref(),ESCROW_SEED.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]

pub struct MyStruct {
    pub input1: u64,
    pub input2: u64,
    pub input3: u64,
}
