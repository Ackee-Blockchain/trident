use anchor_lang::prelude::*;

declare_id!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL");

#[program]
pub mod callee {
    use super::*;

    pub fn initialize_callee(_ctx: Context<InitializeCallee>, input: u8) -> Result<()> {
        msg!("Greetings from Callee");

        if input >= 15 {
            panic!();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCallee<'info> {
    pub signer: Signer<'info>,
}
