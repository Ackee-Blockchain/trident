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

pub mod program_client {
    pub use crate::client::Client;
    pub use anchor_client;
    pub use anchor_lang::{InstructionData, ToAccountMetas};
    pub use solana_sdk;
    pub use solana_transaction_status;
}

#[cfg(feature = "poctesting")]
pub mod poctesting {
    pub use crate::client::Client;
    pub use crate::error_reporter::*;
    pub use crate::TempClone;
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

// pub use futures::{self, FutureExt};
// pub use client::PrintableTransaction;
// pub use trdelnik_test::trdelnik_test;

// pub use rstest::*;
// pub use serial_test;
// pub use tokio;

mod config;
pub use config::Config;

mod client;
pub use client::Client;

mod reader;
pub use reader::Reader;

mod commander;
pub use commander::{Commander, LocalnetHandle};

mod tester;
pub use tester::Tester;

mod temp_clone;
pub use temp_clone::TempClone;

mod keys;
pub use keys::*;

mod idl;
pub use idl::{Idl, IdlError};

mod program_client_generator;
pub use program_client_generator::generate_source_code;

mod workspace_builder;
pub use workspace_builder::WorkspaceBuilder;

pub mod error_reporter;
pub use error_reporter::*;

pub mod constants {
    pub const PROGRAM_CLIENT_DIRECTORY: &str = ".program_client";
    pub const CARGO: &str = "Cargo.toml";
    pub const TRDELNIK: &str = "Trdelnik.toml";
    pub const ANCHOR: &str = "Anchor.toml";

    pub const LIB: &str = "lib.rs";
    pub const SRC: &str = "src";

    pub const TESTS_WORKSPACE_DIRECTORY: &str = "trdelnik-tests";
    pub const POC_TEST_DIRECTORY: &str = "poc_tests";
    pub const TESTS: &str = "tests";

    pub const FUZZ_TEST_DIRECTORY: &str = "fuzz_tests";

    pub const POC_TEST: &str = "test.rs";
    pub const FUZZ_TEST: &str = "test_fuzz.rs";

    pub const PROGRAM_STUBS: &str = "program_stubs.rs";

    pub const HFUZZ_TARGET: &str = "hfuzz_target";
    pub const HFUZZ_WORKSPACE: &str = "hfuzz_workspace";

    pub const GIT_IGNORE: &str = ".gitignore";

    pub const CLIENT_TOML_TEMPLATE: &str = "/src/templates/program_client/Cargo.toml.tmpl";

    pub const RETRY_LOCALNET_EVERY_MILLIS: u64 = 500;
    pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";
}
