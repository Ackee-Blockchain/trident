use trdelnik_client::anchor_lang::solana_program::instruction::AccountMeta;
use trdelnik_client::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
pub struct InitVestingSnapshot<'info> {
    pub sender: Option<Signer<'info>>,
    pub sender_token_account: Option<Account<'info, TokenAccount>>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub escrow_token_account: Option<Account<'info, TokenAccount>>,
    pub mint: Option<Account<'info, Mint>>,
    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Option<Program<'info, System>>,
}
pub struct WithdrawUnlockedSnapshot<'info> {
    pub recipient: Option<Signer<'info>>,
    pub recipient_token_account: Option<Account<'info, TokenAccount>>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub escrow_token_account: Option<Account<'info, TokenAccount>>,
    pub escrow_pda_authority: Option<AccountInfo<'info>>,
    pub mint: Option<Account<'info, Mint>>,
    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Option<Program<'info, System>>,
}
impl<'info> InitVestingSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let sender: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let sender_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .unwrap_or(None);
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let escrow_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .unwrap_or(None);
        let mint: Option<anchor_lang::accounts::account::Account<Mint>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let token_program: Option<anchor_lang::accounts::program::Program<Token>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        Ok(Self {
            sender,
            sender_token_account,
            escrow,
            escrow_token_account,
            mint,
            token_program,
            system_program,
        })
    }
}
impl<'info> WithdrawUnlockedSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let recipient: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let recipient_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .unwrap_or(None);
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let escrow_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .unwrap_or(None);
        let escrow_pda_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?;
        let mint: Option<anchor_lang::accounts::account::Account<Mint>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let token_program: Option<anchor_lang::accounts::program::Program<Token>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        Ok(Self {
            recipient,
            recipient_token_account,
            escrow,
            escrow_token_account,
            escrow_pda_authority,
            mint,
            token_program,
            system_program,
        })
    }
}
