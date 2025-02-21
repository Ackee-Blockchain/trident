pub mod account_storage;

#[cfg(feature = "token")]
mod mint_account;

#[cfg(feature = "stake")]
mod stake_account;

#[cfg(feature = "token")]
mod token_account;

#[cfg(feature = "vote")]
mod vote_account;

use solana_sdk::pubkey::Pubkey;

pub struct AccountMetadata {
    pub lamports: u64,
    pub space: usize,
    pub owner: Pubkey,
}

impl AccountMetadata {
    pub fn new(lamports: u64, space: usize, owner: Pubkey) -> Self {
        Self {
            lamports,
            space,
            owner,
        }
    }
}

pub struct PdaSeeds<'a> {
    pub seeds: &'a [&'a [u8]],
    pub program_id: Pubkey,
}

impl<'a> PdaSeeds<'a> {
    pub fn new(seeds: &'a [&'a [u8]], program_id: Pubkey) -> Self {
        Self { seeds, program_id }
    }
}

fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Option<Pubkey> {
    if let Some((address, _)) = Pubkey::try_find_program_address(seeds, program_id) {
        Some(address)
    } else {
        None
    }
}
