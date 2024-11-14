#![allow(dead_code)]

use anchor_lang::solana_program::hash::Hash;

use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::transaction::VersionedTransaction;

use crate::error::*;

/// A trait providing methods to read and write (manipulate) accounts
pub trait FuzzClient {
    /// Get the cluster rent
    fn get_sysvar<T: Sysvar>(&mut self) -> T;

    /// Warp to specific epoch
    fn warp_to_epoch(&mut self, warp_epoch: u64);

    /// Warp to specific slot
    fn warp_to_slot(&mut self, warp_slot: u64);

    /// Forward in time by the desired number of seconds
    fn forward_in_time(&mut self, seconds: i64) -> Result<(), FuzzClientError>;

    /// Create or overwrite a custom account, subverting normal runtime checks.
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData);

    /// Get the Keypair of the client's payer account
    fn payer(&self) -> Keypair;

    /// Get the account at the given address
    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData;

    /// Get last blockhash
    fn get_last_blockhash(&self) -> Hash;

    /// Send a transaction and return until the transaction has been finalized or rejected.
    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError>;
}
