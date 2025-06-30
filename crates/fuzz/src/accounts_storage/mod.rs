pub mod account_storage;

#[cfg(feature = "token")]
mod mint_account;

#[cfg(feature = "stake")]
mod stake_account;

#[cfg(feature = "token")]
mod token_account;

#[cfg(feature = "token2022")]
mod token_2022_account;

#[cfg(feature = "token2022")]
mod token_2022_mint;

#[cfg(feature = "token2022")]
pub mod extensions;

#[cfg(feature = "vote")]
mod vote_account;

#[cfg(feature = "token2022")]
pub use extensions::*;

use solana_sdk::pubkey::Pubkey;

#[cfg(feature = "token2022")]
use solana_zk_sdk::encryption::pod::{
    auth_encryption::PodAeCiphertext,
    elgamal::{PodElGamalCiphertext, PodElGamalPubkey},
};

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

// Define enum outside the impl block
#[cfg(feature = "token2022")]
pub enum ParamValue {
    // Token 2022 Mint Extensions
    MetadataPointer(extensions::MetadataPointer),
    GroupMemberPointer(extensions::GroupMemberPointer),
    GroupPointer(extensions::GroupPointer),
    TransferHook(extensions::TransferHook),
    MintCloseAuthority(extensions::MintCloseAuthority),
    PermanentDelegate(extensions::PermanentDelegate),
    DefaultAccountState(extensions::DefaultAccountState),
    InterestBearingConfig(extensions::InterestBearingConfig),
    NonTransferable(extensions::NonTransferable),
    TransferFeeConfig(extensions::TransferFeeConfig),
    ConfidentialTransferMint(extensions::ConfidentialTransferMint),
    ConfidentialTransferFeeConfig(extensions::ConfidentialTransferFeeConfig),
    ConfidentialMintBurn(extensions::ConfidentialMintBurn),
    // Token 2022 Account Extensions
    ConfidentialTransferAccount(extensions::ConfidentialTransferAccount),
    TransferHookAccount(extensions::TransferHookAccount),
    CpiGuard(extensions::CpiGuard),
    MemoTransfer(extensions::MemoTransfer),
    TransferFeeAmount(extensions::TransferFeeAmount),
    ImmutableOwner(extensions::ImmutableOwner),
    NonTransferableAccount(extensions::NonTransferableAccount),
    ConfidentialTransferFeeAmount(extensions::ConfidentialTransferFeeAmount),
    // Fin
}
