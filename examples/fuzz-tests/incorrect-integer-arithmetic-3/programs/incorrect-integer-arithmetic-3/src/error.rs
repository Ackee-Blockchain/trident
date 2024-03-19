use anchor_lang::prelude::*;

#[error_code]
pub enum VestingError {
    InvalidAmount,
    InvalidTimeRange,
    InvalidInterval,
    Overflow,
    Underflow,
}
