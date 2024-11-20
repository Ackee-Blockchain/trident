#![allow(dead_code)]

use std::cell::RefCell;

use crate::config::Config;
use crate::error::FuzzClientErrorWithOrigin;
use crate::fuzz_client::FuzzClient;

pub trait FuzzTestExecutor<T> {
    fn run_fuzzer(
        &self,
        accounts: &RefCell<T>,
        client: &mut impl FuzzClient,
        config: &Config,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>;
}
