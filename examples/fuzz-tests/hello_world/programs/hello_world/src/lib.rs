use anchor_lang::prelude::*;

declare_id!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: u8) -> Result<()> {
        let hello_world_store = &mut ctx.accounts.hello_world_account;
        hello_world_store.input = input;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer=author,
        space=8+100,
        seeds=[b"hello_world_seed"],
        bump
    )]
    pub hello_world_account: Account<'info, StoreHelloWorld>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StoreHelloWorld {
    pub input: u8,
}
