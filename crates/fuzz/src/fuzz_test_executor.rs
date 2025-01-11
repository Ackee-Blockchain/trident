#![allow(dead_code)]

use std::cell::RefCell;

use crate::error::FuzzClientErrorWithOrigin;
use crate::fuzz_client::FuzzClient;
use trident_config::Config;

pub trait FuzzTestExecutor<T> {
    fn run_fuzzer(
        &self,
        accounts: &RefCell<T>,
        client: &mut impl FuzzClient,
        config: &Config,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>;
}
