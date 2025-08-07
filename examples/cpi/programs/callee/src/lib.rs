use anchor_lang::prelude::*;

declare_id!("CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4");

#[program]
pub mod callee {
    use super::*;

    pub fn initialize_callee(_ctx: Context<InitializeCallee>, input: u16) -> Result<()> {
        msg!("Greetings from Callee");

        if input > 12589 && input < 13458 {
            panic!("This number is pure magic");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCallee<'info> {
    pub signer: Signer<'info>,
}
