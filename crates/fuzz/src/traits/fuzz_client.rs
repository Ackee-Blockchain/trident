#![allow(dead_code)]

use solana_sdk::account::AccountSharedData;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::transaction::TransactionError;

use trident_config::TridentConfig;
use trident_svm::trident_entrypoint::TridentEntrypoint;
use trident_svm::trident_program::TridentProgram;

/// A trait providing methods to read and write (manipulate) accounts
pub trait FuzzClient {
    /// Deploy program through its entrypoint
    fn deploy_entrypoint(&mut self, program: TridentEntrypoint);

    /// Deploy program as binary
    fn deploy_program(&mut self, program: TridentProgram);

    /// Create a new client
    fn new_client(config: &TridentConfig) -> Self;

    /// Get sysvar
    fn get_sysvar<T: Sysvar>(&self) -> T;

    /// Warp to specific epoch
    fn warp_to_epoch(&mut self, warp_epoch: u64);

    /// Warp to specific slot
    fn warp_to_slot(&mut self, warp_slot: u64);

    /// Warp to specific timestamp
    fn warp_to_timestamp(&mut self, warp_timestamp: i64);

    /// Forward in time by the desired number of seconds
    fn forward_in_time(&mut self, seconds: i64);

    /// Create or overwrite an account
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData);

    /// Get the Keypair of the client's payer account
    fn payer(&self) -> Keypair;

    /// Get the account at the given address
    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData;

    /// Get last blockhash
    fn get_last_blockhash(&self) -> Hash;

    /// Send a transaction and return until the transaction has been finalized or rejected.
    fn process_instructions(
        &mut self,
        _instructions: &[Instruction],
    ) -> Result<(), TransactionError>;

    // Clear Temp account created during fuzzing iteration
    fn clear_accounts(&mut self);
}
