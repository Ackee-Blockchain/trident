pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("4VS6YQzYbdEVbt9iw1eoUnpNzqC5TJPoSeJH9qFfgm4m");

#[program]
pub mod incorrect_integer_arithmetic_3 {
    use super::*;

    pub fn init_vesting(
        ctx: Context<InitVesting>,
        recipient: Pubkey,
        amount: u64,
        start_at: u64,
        end_at: u64,
        interval: u64,
    ) -> Result<()> {
        _init_vesting(ctx, recipient, amount, start_at, end_at, interval)
    }

    pub fn withdraw_unlocked(ctx: Context<WithdrawUnlocked>) -> Result<()> {
        _withdraw_unlocked(ctx)
    }
}
