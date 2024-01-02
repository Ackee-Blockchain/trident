use anchor_spl::token::{Mint, Token, TokenAccount};
use fuzz_example3::state::Escrow;

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
        // eprintln!("deserializign sender");
        let sender: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign sender ata");
        let sender_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign escrow");
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign escrow ata");
        let escrow_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign mint");
        let mint: Option<anchor_lang::accounts::account::Account<Mint>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign token program");
        let token_program: Option<anchor_lang::accounts::program::Program<Token>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // eprintln!("deserializign system program");
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
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
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let recipient_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let escrow_token_account: Option<anchor_lang::accounts::account::Account<TokenAccount>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts)?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .transpose()
                .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let escrow_pda_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?;
        let mint: Option<anchor_lang::accounts::account::Account<Mint>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let token_program: Option<anchor_lang::accounts::program::Program<Token>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
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
