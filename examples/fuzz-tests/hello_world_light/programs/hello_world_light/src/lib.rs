use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

declare_id!("3GVPhWy5nmXk2WCWUhwtCXkT5ujK1zjASyWQV5TJYiMr");

#[program]
pub mod hello_world_light {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: u8) -> Result<()> {
        let hello_world_store = &mut ctx.accounts.hello_world_account;
        hello_world_store.input = input;
        Ok(())
    }
}

#[derive(Accounts, AccountsSnapshots)]
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
