use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{state::Escrow, VestingError};

pub fn _withdraw_unlocked(ctx: Context<WithdrawUnlocked>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    let current_time = Clock::get()?.unix_timestamp as u64;
    let unlocked_amount = escrow
        .amount_unlocked(current_time)
        .ok_or(VestingError::InvalidAmount)?;

    let bump = ctx.bumps.escrow_pda_authority;
    let seeds = &[b"ESCROW_PDA_AUTHORITY".as_ref(), &[bump]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.escrow_pda_authority.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        unlocked_amount,
    )?;

    escrow.withdrawal += unlocked_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawUnlocked<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(mut,
        token::mint = mint,
        token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        has_one = recipient,
        close = recipient,
        seeds = [escrow.recipient.key().as_ref(),b"ESCROW_SEED"],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = escrow_pda_authority  // only the program has the authority as this is a PDA
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    /// CHECK: we do not read or write to this account
    #[account(
        seeds = [b"ESCROW_PDA_AUTHORITY"],
        bump
    )]
    pub escrow_pda_authority: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
