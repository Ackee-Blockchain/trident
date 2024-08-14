use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub author: Signer<'info>,
    pub hello_world_account: Option<Account<'info, hello_world::StoreHelloWorld>>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
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
        let hello_world_account: Option<
            anchor_lang::accounts::account::Account<hello_world::StoreHelloWorld>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "hello_world_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("hello_world_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "hello_world_account".to_string(),
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
            hello_world_account,
            system_program,
        })
    }
}
