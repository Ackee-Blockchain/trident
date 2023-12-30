use anchor_lang::{prelude::*, system_program};

use crate::{
    error::CustomError,
    state::{Project, State, PROJECT_SEED, STATE_SEED},
};
pub fn _invest(ctx: Context<Invest>, amount: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let state = &mut ctx.accounts.state;

    // INFO this bool is not set within the inicialization so it is
    // false from the start , so it is possible to call invest even
    // without closing registration round.
    require!(
        !state.registrations_round,
        CustomError::RegistrationRoundOpen
    );

    project.invested_amount = project
        .invested_amount
        .checked_add(amount)
        .ok_or(CustomError::AmountOverflow)?;

    state.total_invested = state
        .total_invested
        .checked_add(amount)
        .ok_or(CustomError::AmountOverflow)?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.investor.to_account_info(),
                to: ctx.accounts.project.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Invest<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,
    #[account(
        mut,
        seeds = [project.project_author.key().as_ref(), state.key().as_ref() ,PROJECT_SEED.as_ref()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [state.author.key().as_ref(), STATE_SEED.as_ref()],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}
