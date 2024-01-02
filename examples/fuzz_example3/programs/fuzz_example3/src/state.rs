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
        // take minimum from now and end time
        let time = if now < self.end_time {
            now
        } else {
            self.end_time
        };

        // end_at - start_at, difference of timestamps in second
        let duration = self.end_time.checked_sub(self.start_time)?;
        msg!("duration {}", duration);
        msg!("amount {}", self.amount);
        msg!("interval {}", self.interval);
        // amount * interval / duration
        let interval_amount = self
            .amount
            .checked_mul(self.interval)?
            .checked_div(duration)?;
        msg!("interval_amount {}", interval_amount);

        // (time - self.start_at) / self.interval + 1, current and passed intervals are unlocked
        let nr_intervals = time
            .checked_sub(self.start_time)?
            .checked_div(self.interval)?
            .checked_add(1)?;
        msg!("nr_intervals {}", nr_intervals);

        // nr_intervals * interval_amount - self.withdrawal
        nr_intervals
            .checked_mul(interval_amount)?
            .checked_sub(self.withdrawal)
    }
}
