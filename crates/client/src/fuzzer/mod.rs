pub mod fuzzer_generator;
pub mod data_builder;
#[cfg(feature = "fuzzing")]
pub mod program_test_client_blocking;
pub mod snapshot;
pub mod accounts_storage;

pub type AccountId = u8;
