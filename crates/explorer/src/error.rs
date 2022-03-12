use solana_client::client_error::ClientError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ExplorerError>;

#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("Solana Client failed: {0}")]
    SolanaClient(#[from] ClientError),
    #[error("{0}")]
    Custom(String),
}
