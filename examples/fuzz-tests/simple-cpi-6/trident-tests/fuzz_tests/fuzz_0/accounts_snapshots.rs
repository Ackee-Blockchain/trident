use anchor_lang::prelude::*;
use callee::program::Callee;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
// pub struct InitializeCalleeSnapshot<'info> {
//     pub signer: Signer<'info>,
// }
// impl<'info> InitializeCalleeSnapshot<'info> {
//     pub fn deserialize_option(
//         _program_id: &anchor_lang::prelude::Pubkey,
//         accounts: &'info mut [Option<AccountInfo<'info>>],
//     ) -> core::result::Result<Self, FuzzingError> {
//         let mut accounts_iter = accounts.iter();
//         let signer: Signer<'_> = accounts_iter
//             .next()
//             .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
//             .as_ref()
//             .map(anchor_lang::accounts::signer::Signer::try_from)
//             .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
//             .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
//         Ok(Self { signer })
//     }
// }
pub struct InitializeCallerSnapshot<'info> {
    pub signer: Signer<'info>,
    pub program: Program<'info, Callee>,
}
impl<'info> InitializeCallerSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let program: anchor_lang::accounts::program::Program<Callee> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("program".to_string()))?;
        Ok(Self { signer, program })
    }
}
