use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub recipient: Pubkey,
    pub amount: u64,
    pub withdrawal: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
    pub bump: u8,
}

impl Escrow {
    pub fn amount_unlocked(&self, now: u64) -> Option<u64> {
        let time = if now < self.end_time {
            now
        } else {
            self.end_time
        };

        let duration = self.end_time.checked_sub(self.start_time)?;

        let interval_amount = self
            .amount
            .checked_mul(self.interval)?
            .checked_div(duration)?;

        let nr_intervals = time
            .checked_sub(self.start_time)?
            .checked_div(self.interval)?
            .checked_add(1)?;

        nr_intervals
            .checked_mul(interval_amount)?
            .checked_sub(self.withdrawal)
    }
}
