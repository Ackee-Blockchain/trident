#![allow(dead_code)]

use anchor_lang::solana_program::hash::Hash;

use std::cell::RefCell;
use std::collections::HashMap;

use crate::config::Config;
use crate::error::FuzzClientErrorWithOrigin;
use crate::fuzz_client::FuzzClient;

pub trait FuzzTestExecutor<T> {
    fn run_fuzzer(
        &self,
        accounts: &RefCell<T>,
        client: &mut impl FuzzClient,
        sent_txs: &mut HashMap<Hash, ()>,
        config: &Config,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>;
}
