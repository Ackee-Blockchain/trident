use anchor_lang::prelude::*;

const MAGIC_NUMBER: u8 = 254;
declare_id!("CTb5cKBLEGgFEw3jDGYCHCiHJ2PY2LFoxXhEPRhFFL4h");

#[program]
pub mod fuzzer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count = 0;
        counter.authority = ctx.accounts.user.key();

        Ok(())
    }

    pub fn update(ctx: Context<Update>, input1: u8, input2: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("input1 = {}, input2 = {}", input1, input2);

        // comment this to fix the black magic panic
        if input1 == MAGIC_NUMBER {
            panic!("Black magic not supported!");
        }

        counter.count = buggy_math_function(input1, input2).into();
        Ok(())
    }
}

pub fn buggy_math_function(input1: u8, input2: u8) -> u8 {
    // comment the if statement to cause div-by-zero and subtract with overflow panic
    if input2 >= MAGIC_NUMBER {
        return 0;
    }
    let divisor = MAGIC_NUMBER - input2;
    input1 / divisor
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
