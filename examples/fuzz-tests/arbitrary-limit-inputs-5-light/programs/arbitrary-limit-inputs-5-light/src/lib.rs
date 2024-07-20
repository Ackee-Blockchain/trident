pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("GY4giB9emc5zuJJYzwQx2pepYVpCwruqbAKDmSTibKBB");

#[program]
pub mod arbitrary_limit_inputs_5_light {
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
