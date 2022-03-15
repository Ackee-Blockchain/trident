use serde_json::error::Error as SerdeError;
use solana_client::client_error::ClientError;
use std::fmt::Error as FmtError;
use solana_sdk::instruction::InstructionError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ExplorerError>;

#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("{0}")]
    SolanaClient(#[from] ClientError),
    #[error("{0}")]
    SerdeJson(#[from] SerdeError),
    #[error("{0}")]
    Fmt(#[from] FmtError),
    #[error("{0}")]
    Instruction(#[from] InstructionError),
    #[error("{0}")]
    Custom(String),
}
