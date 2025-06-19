use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;


declare_id!("7mic9LMCr7wpHeixUpEwQ9pVYa9HB2wQ5Jb47no9yXvx");


#[program]
pub mod token_extensions {
    use super::*;

    pub fn create_mint_account(
        ctx: Context<CreateMintAccount>,
    ) -> Result<()> {
        instructions::handler(ctx)
    }

}
