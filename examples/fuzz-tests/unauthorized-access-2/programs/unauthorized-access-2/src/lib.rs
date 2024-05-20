pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("5XvBmfPNcHLCgbRK4nRYvfodAnhjArHSed2B3rhkF1Ug");

#[program]
pub mod unauthorized_access_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, receiver: Pubkey, amount: u64) -> Result<()> {
        _initialize(ctx, receiver, amount)
    }
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw(ctx)
    }
}
