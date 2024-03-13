use fuzz_example0::Counter;
use program_client::fuzz_example0_instruction::PROGRAM_ID;
use trident_client::anchor_lang::{self, prelude::*};
use trident_client::fuzzing::FuzzingError;
pub struct InitializeSnapshot<'info> {
    pub counter: Option<Account<'info, Counter>>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct UpdateSnapshot<'info> {
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let counter: Option<anchor_lang::accounts::account::Account<Counter>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("counter".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != PROGRAM_ID {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("counter".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "counter".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
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
            counter,
            user,
            system_program,
        })
    }
}
impl<'info> UpdateSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let counter: anchor_lang::accounts::account::Account<Counter> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("counter".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("counter".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("counter".to_string()))?;
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        Ok(Self { counter, authority })
    }
}
