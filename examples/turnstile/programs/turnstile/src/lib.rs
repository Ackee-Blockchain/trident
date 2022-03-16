use anchor_lang::{
    prelude::*,
    solana_program::{native_token::sol_to_lamports, program::invoke, system_instruction},
};
use anchor_spl::token::{
    burn, initialize_mint, mint_to, Burn, InitializeMint, Mint, MintTo, Token, TokenAccount,
};

declare_id!("Po1RaS8BEDbNcn5oXsFryAeQ6Wn8fvmE111DJaKCgPC");

#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_mint(
            ctx.accounts.init_context_mint(),
            0,
            ctx.accounts.mint_authority.key,
            None,
        )?;

        let state = &mut ctx.accounts.state;

        state.bump = *ctx.bumps.get("mint_authority").unwrap();
        state.mint = ctx.accounts.mint.key();
        state.treasury = ctx.accounts.treasury.key();
        state.locked = true;
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.treasury.key(),
                sol_to_lamports(1.0),
            ),
            &[
                ctx.accounts.user.to_account_info().clone(),
                ctx.accounts.treasury.clone(),
            ],
        )?;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info().clone(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let seeds = &[
            ctx.accounts.mint.to_account_info().key.as_ref(),
            &[ctx.accounts.state.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        mint_to(cpi_ctx, 5)?;

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn coin(ctx: Context<Coin>) -> Result<()> {
        burn(ctx.accounts.init_burn_context(), 5)?;
        let state = &mut ctx.accounts.state;
        state.locked = false;
        Ok(())
    }

    pub fn push(ctx: Context<UpdateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = Mint::LEN
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        space = 8 + 1 + 1 + 32 + 32
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [mint.key().as_ref()], bump)]
    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}

impl<'info> Initialize<'info> {
    pub fn init_context_mint(&self) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.clone(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct Coin<'info> {
    #[account(has_one = mint)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Coin<'info> {
    pub fn init_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(has_one = treasury, has_one = mint)]
    pub state: Account<'info, State>,
    // #[account(
    //     constraint = state.mint == user_token_account.mint,
    // )]
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK:
    #[account(seeds = [mint.key().as_ref()], bump = state.bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub locked: bool,
    pub mint: Pubkey,
    pub treasury: Pubkey,
    pub bump: u8,
}
