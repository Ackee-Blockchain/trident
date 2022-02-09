use anchor_lang::{prelude::*, solana_program::system_program};

declare_id!("FZ2Q3Bpdg3mgoSjRi8xsPgycgVDgFNGQ77SErk8mCaki");

#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.locked = true;
        state.res = false;
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn coin(ctx: Context<UpdateState>, dummy_arg: String) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.locked = false;
        Ok(())
    }

    pub fn push(ctx: Context<UpdateState>) -> ProgramResult {
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
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
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
