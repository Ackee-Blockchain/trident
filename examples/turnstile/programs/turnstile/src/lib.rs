use anchor_lang::prelude::*;

declare_id!("Po1RaS8BEDbNcn5oXsFryAeQ6Wn8fvmE111DJaKCgPC");

#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = true;
        state.res = false;
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn coin(ctx: Context<UpdateState>, dummy_arg: String) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = false;
        Ok(())
    }

    pub fn push(ctx: Context<UpdateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        if state.locked {
            state.res = false;
        } else {
            state.locked = true;
            state.res = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 2
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[account]
pub struct State {
    pub locked: bool,
    pub res: bool,
}
