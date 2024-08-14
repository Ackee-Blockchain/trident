use anchor_lang::prelude::*;
use anchor_spl::token::{
    set_authority, transfer, Mint, SetAuthority, Token, TokenAccount, Transfer,
};
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::state::Escrow;
use crate::VestingError;

pub fn _init_vesting(
    ctx: Context<InitVesting>,
    recipient: Pubkey,
    amount: u64,
    start_at: u64,
    end_at: u64,
    interval: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(amount > 0, VestingError::InvalidAmount);
    // Validate timestamps order (overflow check)
    require!(end_at > start_at, VestingError::InvalidTimeRange);
    // Validate interval
    require!(end_at - start_at > interval, VestingError::InvalidInterval);
    require!(interval > 0, VestingError::InvalidInterval);

    escrow.amount = amount;
    escrow.start_time = start_at;
    escrow.end_time = end_at;
    escrow.interval = interval;
    escrow.recipient = recipient;
    escrow.bump = ctx.bumps.escrow;

    let (escrow_pda_authority, _) =
        Pubkey::find_program_address(&[b"ESCROW_PDA_AUTHORITY"], ctx.program_id);

    // Set escrow's token account authority to the program's PDA
    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.escrow_token_account.to_account_info(),
                current_authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner,
        Some(escrow_pda_authority),
    )?;

    // Transfer tokens from sender's token account to escrow's token account
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(AccountsSnapshots, Accounts)]
#[instruction(recipient: Pubkey)]
pub struct InitVesting<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut,
        token::authority = sender,
        token::mint = mint
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = sender,
        space = 8 + 1 + 32 + 5*8,
        seeds = [recipient.as_ref(),b"ESCROW_SEED"],
        bump
     )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        token::mint = mint
        // token account authority will be transfered to program PDA during instruction execution
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
