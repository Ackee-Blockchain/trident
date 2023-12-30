use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Cannot invested, amount overflow occured")]
    AmountOverflow,
    #[msg("Registration round is still open, cannot invest right now!")]
    RegistrationRoundOpen,
}
