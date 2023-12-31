use fuzz_example2::state::Escrow;
use trdelnik_client::anchor_lang::solana_program::instruction::AccountMeta;
use trdelnik_client::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub author: Option<Signer<'info>>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub system_program: Option<Program<'info, System>>,
}
pub struct WithdrawSnapshot<'info> {
    pub receiver: Option<Signer<'info>>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub system_program: Option<Program<'info, System>>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let author: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        Ok(Self {
            author,
            escrow,
            system_program,
        })
    }
}
impl<'info> WithdrawSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let receiver: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        Ok(Self {
            receiver,
            escrow,
            system_program,
        })
    }
}
