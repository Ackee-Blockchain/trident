#![allow(dead_code)]

use anchor_lang::solana_program::hash::Hash;

use solana_sdk::pubkey::Pubkey;

use std::cell::RefCell;
use std::collections::HashMap;

use crate::error::FuzzClientErrorWithOrigin;
use crate::fuzz_client::FuzzClient;

pub trait FuzzTestExecutor<T> {
    fn run_fuzzer(
        &self,
        program_id: Pubkey,
        accounts: &RefCell<T>,
        client: &mut impl FuzzClient,
        sent_txs: &mut HashMap<Hash, ()>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>;
}
