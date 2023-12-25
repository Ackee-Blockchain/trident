//! Trdelnik is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trdelnik could be useful for writing Rust dApps, too.

#[cfg(feature = "fuzzing")]
pub mod fuzzing {
    pub use anchor_lang;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz;
    pub use solana_program_test;
    pub use solana_sdk;
    pub use solana_sdk::signer::Signer;
}

pub mod prelude {
    pub use crate::client::Client;
    pub use anchor_client;
    pub use anchor_client::ClientError;
    pub use anchor_lang;
    pub use anchor_lang::{InstructionData, ToAccountMetas};
    pub use solana_sdk;
    pub use solana_sdk::instruction::Instruction;
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signer::keypair::Keypair;
    pub use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
}

#[doc(hidden)]
pub mod __private {
    pub use crate::commander::Commander;
    pub use crate::idl::Idl;
    pub use crate::idl::IdlError;
    pub use crate::program_client_generator::ProgramCLientGenerator;
    pub use crate::workspace_builder::WorkspaceBuilder;
}

#[cfg(feature = "poctesting")]
pub mod poctesting {
    pub use crate::client::Client;
    pub use crate::error_reporter;
    pub use crate::temp_clone;
    pub use crate::tester::Tester;
    pub use anchor_lang;
    pub use anyhow::{Error, Result};

    pub use fehler::throws;

    pub use futures::FutureExt;
    pub use rstest::*;
    pub use serial_test;
    pub use solana_sdk;
    pub use solana_sdk::signer::Signer;
    pub use tokio;
    pub use trdelnik_test::trdelnik_test;

    pub use crate::keys::*;
}

mod config;
use config::Config;

mod client;
use client::Client;

mod reader;
use reader::Reader;

mod commander;
use commander::Commander;

mod idl;
use idl::{Idl, IdlError};

mod program_client_generator;

mod workspace_builder;

pub mod error_reporter;
pub mod keys;
pub mod temp_clone;
pub mod tester;

mod constants {
    pub const PROGRAM_CLIENT_DIRECTORY: &str = ".program_client";
    pub const CARGO: &str = "Cargo.toml";
    pub const TRDELNIK: &str = "Trdelnik.toml";

    pub const LIB: &str = "lib.rs";
    pub const SRC: &str = "src";

    pub const TESTS_WORKSPACE_DIRECTORY: &str = "trdelnik-tests";
    pub const POC_TEST_DIRECTORY: &str = "poc_tests";
    pub const TESTS: &str = "tests";

    pub const FUZZ_TEST_DIRECTORY: &str = "fuzz_tests";

    pub const POC_TEST: &str = "test.rs";
    pub const FUZZ_TEST: &str = "test_fuzz.rs";

    // pub const PROGRAM_STUBS: &str = "program_stubs.rs";

    pub const HFUZZ_TARGET: &str = "hfuzz_target";
    pub const HFUZZ_WORKSPACE: &str = "hfuzz_workspace";

    pub const GIT_IGNORE: &str = ".gitignore";

    pub const RETRY_LOCALNET_EVERY_MILLIS: u64 = 500;
    pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";
}
