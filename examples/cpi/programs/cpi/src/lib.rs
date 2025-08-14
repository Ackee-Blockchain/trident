use anchor_lang::prelude::*;
use callee::program::Callee;

declare_id!("77skervubsozZaRdojomG7FK8T2QQppxtSqG8ag9D4qV");

#[program]
pub mod cpi {
    use super::*;

    pub fn initialize_caller(ctx: Context<InitializeCaller>, input: u16) -> Result<()> {
        msg!("Greetings from Caller");

        let cpi_context = CpiContext::new(
            ctx.accounts.program.to_account_info(),
            callee::cpi::accounts::InitializeCallee {
                signer: ctx.accounts.signer.to_account_info(),
            },
        );

        callee::cpi::initialize_callee(cpi_context, input)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCaller<'info> {
    pub signer: Signer<'info>,
    pub program: Program<'info, Callee>,
}
