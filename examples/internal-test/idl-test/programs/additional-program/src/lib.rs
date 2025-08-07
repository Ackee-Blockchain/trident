use anchor_lang::prelude::*;

declare_id!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB");

#[program]
pub mod additional_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
