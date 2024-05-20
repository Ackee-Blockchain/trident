pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU");

#[program]
pub mod incorrect_ix_sequence_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        _initialize(ctx)
    }
    pub fn register(ctx: Context<Register>) -> Result<()> {
        _register(ctx)
    }
    pub fn end_registrations(ctx: Context<EndRegistration>) -> Result<()> {
        _end_registration(ctx)
    }
    pub fn invest(ctx: Context<Invest>, amount: u64) -> Result<()> {
        _invest(ctx, amount)
    }
}
