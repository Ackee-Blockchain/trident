#![allow(dead_code)]

use solana_sdk::transaction::TransactionError;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FuzzingError {
    #[error("Custom fuzzing error: {0}\n")]
    Custom(u32),
    #[error("Fuzzing error with Custom Message: {0}\n")]
    CustomMessage(String),
    #[error("Transaction failed: {0}")]
    TransactionFailed(#[from] TransactionError),
    #[error("Arbitrary error: {0}")]
    ArbitraryError(#[from] arbitrary::Error),
}

impl FuzzingError {
    pub fn with_message(message: &str) -> Self {
        Self::CustomMessage(message.to_string())
    }
}
