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
        anchor_lang::InstructionData, anchor_lang::ToAccountMetas, fuzz_trd,
        solana_sdk::account::Account, solana_sdk::transaction::Transaction, Instruction, Keypair,
        Pubkey, Signer, TempClone,
    };
    pub use anchor_client::anchor_lang::solana_program::hash::Hash;
    pub use anchor_lang::solana_program::instruction::AccountMeta;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz;
    // TODO add optional feature gated dependency
    pub use solana_program_test::{
        processor, tokio::runtime::Runtime, BanksClient, BanksClientError, ProgramTest,
        ProgramTestContext,
    };

    pub use super::fuzzer::accounts_storage::*;
    pub use super::fuzzer::data_builder::build_ix_fuzz_data;
    pub use super::fuzzer::data_builder::*;

    pub use super::fuzzer::program_test_client_blocking::ProgramTestClientBlocking;
    pub use super::fuzzer::snapshot::Snapshot;
    pub use super::fuzzer::*;
    pub use std::cell::RefCell;
    pub use trdelnik_derive_displayix::DisplayIx;
    pub use trdelnik_derive_fuzz_deserialize::FuzzDeserialize;
    pub use trdelnik_derive_fuzz_test_executor::FuzzTestExecutor;
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

mod fuzzer;
pub use fuzzer::*;
pub mod idl;
pub mod program_client_generator;

pub mod test_generator;
pub use test_generator::TestGenerator;

pub mod error_reporter;
pub use error_reporter::*;

pub mod cleaner;
pub use cleaner::*;

// This is a workaround for tests: https://github.com/Ackee-Blockchain/trdelnik/pull/112#issuecomment-1924920952
pub use trdelnik_derive_displayix::DisplayIx;
pub use trdelnik_derive_fuzz_deserialize::FuzzDeserialize;
pub use trdelnik_derive_fuzz_test_executor::FuzzTestExecutor;
