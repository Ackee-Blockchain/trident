#![allow(dead_code)]

use anchor_lang::prelude::Rent;
use anchor_lang::solana_program::hash::Hash;

use solana_sdk::account::{Account, AccountSharedData};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::VersionedTransaction;

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

    /// Send a transaction and return until the transaction has been finalized or rejected.
    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError>;
}
