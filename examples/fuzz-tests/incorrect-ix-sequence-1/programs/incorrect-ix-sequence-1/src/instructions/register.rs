pub use anchor_lang::prelude::*;

use crate::state::{Project, State, PROJECT_SEED, STATE_SEED};

pub fn _register(ctx: Context<Register>) -> Result<()> {
    let project = &mut ctx.accounts.project;

    project.project_author = ctx.accounts.project_author.key();
    project.invested_amount = 0;
    project.bump = ctx.bumps.project;

    Ok(())
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub project_author: Signer<'info>,
    #[account(
        init,
        payer = project_author,
        space = 8 + Project::LEN,
        seeds = [project_author.key().as_ref(), state.key().as_ref() ,PROJECT_SEED.as_ref()],
        bump
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
