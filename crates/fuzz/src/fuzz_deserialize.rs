#![allow(dead_code)]

use anchor_lang::solana_program::account_info::AccountInfo;

use crate::error::FuzzingError;

pub trait FuzzDeserialize<'info> {
    type Ix;
    // TODO return also remaining accounts

    fn deserialize_option(
        &self,
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> Result<Self::Ix, FuzzingError>;
}
