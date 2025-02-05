#![allow(dead_code)]

use std::cell::RefCell;

use crate::error::FuzzingError;
use crate::fuzz_client::FuzzClient;
use crate::snapshot::TransactionSnapshot;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::TransactionError;

pub trait FuzzTestExecutor<T> {
    fn instruction_name(&self) -> String;
    fn get_discriminator(&self) -> Vec<u8>;
    fn get_program_id(&self) -> Pubkey;
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        accounts: &RefCell<T>,
    ) -> Result<Vec<u8>, FuzzingError>;
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        accounts: &RefCell<T>,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError>;
    fn transaction_invariant_check(
        &self,
        pre_tx: &TransactionSnapshot,
        post_tx: &TransactionSnapshot,
    ) -> Result<(), FuzzingError>;
    fn transaction_error_handler(
        &self,
        e: TransactionError,
        pre_tx: &TransactionSnapshot,
    ) -> Result<(), TransactionError>;
    fn post_transaction(&self, client: &mut impl FuzzClient, post_tx: &TransactionSnapshot);
}
