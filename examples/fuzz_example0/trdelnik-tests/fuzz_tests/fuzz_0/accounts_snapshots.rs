use fuzzer::Counter;
use trdelnik_client::fuzzing::anchor_lang::solana_program::instruction::AccountMeta;
use trdelnik_client::fuzzing::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub counter: Option<Account<'info, Counter>>,
    pub user: Option<Signer<'info>>,
    pub system_program: Option<Program<'info, System>>,
}
pub struct UpdateSnapshot<'info> {
    pub counter: Option<Account<'info, Counter>>,
    pub authority: Option<Signer<'info>>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::fuzzing::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        // -----------------------------
        let counter: Option<anchor_lang::accounts::account::Account<Counter>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        // let counter: Option<anchor_lang::accounts::account::Account<Counter>> = accounts_iter
        //     .next()
        //     .ok_or(FuzzingError::NotEnoughAccounts)?
        //     .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
        //     .transpose()
        //     .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // -----------------------------

        let user: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        Ok(Self {
            counter,
            user,
            system_program,
        })
    }
}
impl<'info> UpdateSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::fuzzing::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        // -----------------------------
        let counter: Option<anchor_lang::accounts::account::Account<Counter>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .unwrap_or(None);
        // let counter: Option<anchor_lang::accounts::account::Account<Counter>> = accounts_iter
        //     .next()
        //     .ok_or(FuzzingError::NotEnoughAccounts)?
        //     .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
        //     .transpose()
        //     .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        // -----------------------------

        let authority: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        Ok(Self { counter, authority })
    }
}
