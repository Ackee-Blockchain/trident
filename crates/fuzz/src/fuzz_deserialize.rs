#![allow(dead_code)]

use anchor_lang::solana_program::account_info::AccountInfo;

use crate::error::FuzzingError;

pub trait FuzzDeserialize<'info>: Sized {
    // TODO return also remaining accounts

    fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &mut &'info [Option<AccountInfo<'info>>],
    ) -> Result<Self, FuzzingError>;
}
