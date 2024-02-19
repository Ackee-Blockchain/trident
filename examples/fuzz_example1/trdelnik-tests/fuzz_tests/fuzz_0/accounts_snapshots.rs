use fuzz_example1::state::{Project, State};
use program_client::fuzz_example1_instruction::PROGRAM_ID;
use trdelnik_client::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::FuzzingError;

pub struct InitializeSnapshot<'info> {
    pub author: Signer<'info>,
    pub state: Option<Account<'info, State>>,
    pub system_program: Program<'info, System>,
}
pub struct RegisterSnapshot<'info> {
    pub project_author: Signer<'info>,
    pub project: Option<Account<'info, Project>>,
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}
pub struct EndRegistrationsSnapshot<'info> {
    pub author: Signer<'info>,
    pub state: Account<'info, State>,
}
pub struct InvestSnapshot<'info> {
    pub investor: Signer<'info>,
    pub project: Account<'info, Project>,
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let author: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("author".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("author".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("author".to_string()))?;
        let state: Option<anchor_lang::accounts::account::Account<State>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("state".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != PROGRAM_ID {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("state".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "state".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            author,
            state,
            system_program,
        })
    }
}
impl<'info> RegisterSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let project_author: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "project_author".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("project_author".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("project_author".to_string()))?;
        let project: Option<anchor_lang::accounts::account::Account<Project>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("project".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != PROGRAM_ID {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("project".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "project".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let state: anchor_lang::accounts::account::Account<State> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("state".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
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
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let author: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("author".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("author".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("author".to_string()))?;
        let state: anchor_lang::accounts::account::Account<State> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("state".to_string()))?;
        Ok(Self { author, state })
    }
}
impl<'info> InvestSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let investor: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("investor".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("investor".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("investor".to_string()))?;
        let project: anchor_lang::accounts::account::Account<Project> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("project".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("project".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("project".to_string()))?;
        let state: anchor_lang::accounts::account::Account<State> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("state".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            investor,
            project,
            state,
            system_program,
        })
    }
}
