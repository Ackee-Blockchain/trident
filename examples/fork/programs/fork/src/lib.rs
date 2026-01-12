use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

declare_id!("9nGj2iwXYhaQktGxiJgJ5aGBhfSb5ubYXxc1bZy4MD2k");

#[program]
pub mod fork {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_wallet.to_account_info(),
                to: ctx.accounts.receiver_wallet.to_account_info(),
            },
        );
        transfer(ctx, 1_000_000_000)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub sender_wallet: Signer<'info>,
    /// CHECK: recipient
    #[account(mut)]
    pub receiver_wallet: AccountInfo<'info>,
    #[account(address = wrapped_sol::id())]
    pub wrapped_sol: InterfaceAccount<'info, Mint>,
    pub metadao: Program<'info, MetaDAO>,
    pub kamino_lending: Program<'info, KaminoLending>,
    pub jupiter_aggregator_v6: Program<'info, JupiterAggregatorV6>,
    pub marinade_bond: Program<'info, MarinadeBond>,
    pub system_program: Program<'info, System>,
}

pub struct MetaDAO;

impl anchor_lang::Id for MetaDAO {
    fn id() -> Pubkey {
        pubkey!("moontUzsdepotRGe5xsfip7vLPTJnVuafqdUWexVnPM")
    }
}

pub struct KaminoLending;

impl anchor_lang::Id for KaminoLending {
    fn id() -> Pubkey {
        pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
    }
}

pub struct JupiterAggregatorV6;

impl anchor_lang::Id for JupiterAggregatorV6 {
    fn id() -> Pubkey {
        pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")
    }
}

pub struct MarinadeBond;

impl anchor_lang::Id for MarinadeBond {
    fn id() -> Pubkey {
        pubkey!("vBoNdEvzMrSai7is21XgVYik65mqtaKXuSdMBJ1xkW4")
    }
}

mod wrapped_sol {
    use super::*;
    declare_id!("So11111111111111111111111111111111111111112");
}
