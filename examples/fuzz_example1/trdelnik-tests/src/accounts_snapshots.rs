use fuzz_example1::state::{Project, State};
use trdelnik_client::anchor_lang::solana_program::instruction::AccountMeta;
use trdelnik_client::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
pub struct InitializeSnapshot<'info> {
    author: Option<Signer<'info>>,
    state: Option<Account<'info, State>>,
    system_program: Option<Program<'info, System>>,
}
pub struct RegisterSnapshot<'info> {
    project_author: Option<Signer<'info>>,
    project: Option<Account<'info, Project>>,
    state: Option<Account<'info, State>>,
    system_program: Option<Program<'info, System>>,
}
pub struct EndRegistrationsSnapshot<'info> {
    author: Option<Signer<'info>>,
    state: Option<Account<'info, State>>,
}
pub struct InvestSnapshot<'info> {
    investor: Option<Signer<'info>>,
    project: Option<Account<'info, Project>>,
    state: Option<Account<'info, State>>,
    system_program: Option<Program<'info, System>>,
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
        let state: Option<anchor_lang::accounts::account::Account<State>> = accounts_iter
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
            state,
            system_program,
        })
    }
}
impl<'info> RegisterSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let project_author: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let project: Option<anchor_lang::accounts::account::Account<Project>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let state: Option<anchor_lang::accounts::account::Account<State>> = accounts_iter
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
            project_author,
            project,
            state,
            system_program,
        })
    }
}
impl<'info> EndRegistrationsSnapshot<'info> {
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
        let state: Option<anchor_lang::accounts::account::Account<State>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        Ok(Self { author, state })
    }
}
impl<'info> InvestSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::CannotGetAccounts)?;
        let mut accounts_iter = accounts.into_iter();
        let investor: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let project: Option<anchor_lang::accounts::account::Account<Project>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts)?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .transpose()
            .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
        let state: Option<anchor_lang::accounts::account::Account<State>> = accounts_iter
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
            investor,
            project,
            state,
            system_program,
        })
    }
}
