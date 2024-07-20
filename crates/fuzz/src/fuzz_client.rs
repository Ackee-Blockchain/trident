#![allow(dead_code)]

use anchor_lang::prelude::Rent;
use anchor_lang::solana_program::hash::Hash;

use solana_sdk::account::{Account, AccountSharedData};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::error::*;

/// A trait providing methods to read and write (manipulate) accounts
pub trait FuzzClient {
    /// Create an empty account and add lamports to it
    fn set_account(&mut self, lamports: u64) -> Keypair;

    /// Create or overwrite a custom account, subverting normal runtime checks.
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData);

    /// Create an SPL token account
    #[allow(clippy::too_many_arguments)]
    fn set_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey;

    /// Create an SPL mint account
    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey;

    /// Get the Keypair of the client's payer account
    fn payer(&self) -> Keypair;

    /// Get the account at the given address
    fn get_account(&mut self, key: &Pubkey) -> Result<Option<Account>, FuzzClientError>;

    /// Get accounts based on the supplied meta information
    fn get_accounts(
        &mut self,
        metas: &[AccountMeta],
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin>;

    /// Get last blockhash
    fn get_last_blockhash(&self) -> Hash;

    /// Get the cluster rent
    fn get_rent(&mut self) -> Result<Rent, FuzzClientError>;

    /// Process instruction
    fn process(
        &mut self,
        instruction: Instruction,
        signers: Vec<Keypair>,
    ) -> Result<(), FuzzClientError>;
}

/// Converts Anchor 0.29.0 and higher entrypoint into the runtime's entrypoint style
///
/// Starting Anchor 0.29.0 the accounts are passed by reference https://github.com/coral-xyz/anchor/pull/2656
/// and the lifetime requirements are `accounts: &'a [AccountInfo<'a>]` instead of `accounts: &'a [AccountInfo<'b>]`.
/// The new requirements require the slice of AccountInfos and the contained Accounts to have the same lifetime but
/// the previous version is more general. The compiler implies that `'b` must live at least as long as `'a` or longer.
///
/// The transaction data is serialized and again deserialized to the `&[AccountInfo<_>]` slice just before invoking
/// the entry point and the modified account data is copied to the original accounts just after the the entry point.
/// After that the `&[AccountInfo<_>]` slice goes out of scope entirely and therefore `'a` == `'b`. So it _SHOULD_ be
/// safe to do this conversion in this testing scenario.
///
/// Do not use this conversion in any on-chain programs!
#[macro_export]
macro_rules! convert_entry {
    ($entry:expr) => {
        unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
    };
}

pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;
