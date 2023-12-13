//! Trdelnik is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trdelnik could be useful for writing Rust dApps, too.

pub use anchor_client::{
    self,
    anchor_lang::{self, prelude::System, Id, InstructionData, ToAccountMetas},
    solana_sdk::{
        self,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::Signature,
        signer::{keypair::Keypair, Signer},
    },
    ClientError,
};
pub use anyhow::{self, Error};

#[cfg(feature = "fuzzing")]
pub mod fuzzing {
    pub use super::{
        anchor_lang, anchor_lang::system_program::ID as SYSTEM_PROGRAM_ID,
        solana_sdk::transaction::Transaction, Instruction, Keypair, Pubkey, Signer,
    };
    pub use anchor_client::anchor_lang::solana_program::hash::Hash;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz;
    pub use solana_program_test::{
        processor, tokio::runtime::Runtime, BanksClient, BanksClientError, ProgramTest,
    };
}

pub use futures::{self, FutureExt};
pub use rstest::*;
pub use serial_test;
pub use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
pub use tokio;

pub use trdelnik_test::trdelnik_test;

mod config;

mod client;
pub use client::Client;
pub use client::PrintableTransaction;

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

pub mod idl;
pub mod program_client_generator;

pub mod workspace_builder;
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
    pub const TEST_DIRECTORY: &str = "tests";
    pub const TEST: &str = "test.rs";

    pub const FUZZ_DIRECTORY: &str = "fuzz_0";
    pub const FUZZ: &str = "fuzz_target.rs";
    pub const PROGRAM_STUBS: &str = "program_stubs.rs";

    //pub const PROGRAM_STUBS_ENTRIES: &str = "// ### \"Entrypoints go above\" ###";
    pub const HFUZZ_TARGET: &str = "hfuzz_target";
    pub const HFUZZ_WORKSPACE: &str = "hfuzz_workspace";

    pub const GIT_IGNORE: &str = ".gitignore";

    pub const CLIENT_TOML_TEMPLATE: &str = "/src/templates/program_client/Cargo.toml.tmpl";
}
