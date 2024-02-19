use anchor_lang::{prelude::*, system_program};

use crate::state::{Escrow, ESCROW_SEED};

pub fn _initialize(ctx: Context<Initialize>, receiver: Pubkey, amount: u64) -> Result<()> {
    let escorw = &mut ctx.accounts.escrow;

    escorw.author = ctx.accounts.author.key();
    escorw.amount = amount;
    escorw.receiver = receiver;
    escorw.bump = ctx.bumps.escrow;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.author.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(receiver: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + Escrow::LEN,
        seeds = [author.key().as_ref(),receiver.as_ref(),ESCROW_SEED.as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}
