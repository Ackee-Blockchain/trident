use anchor_lang::prelude::*;

declare_id!("icbQHAzqtdyR4yaGvNQYfceoBKJKQmMqTxscqSSdgS3");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
