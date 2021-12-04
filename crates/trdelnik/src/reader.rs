use anchor_client::{
    solana_sdk::{
        signer::keypair::Keypair,
        pubkey::{ParsePubkeyError, Pubkey},
    }
};
use tokio::fs;
use std::{io, str::FromStr};
use fehler::throws;
use thiserror::Error;
use ed25519_dalek::SignatureError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot read the file")]
    IoError(#[from] io::Error),
    #[error("deserialization failed")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("pubkey parsing failed")]
    PubkeyError(#[from] ParsePubkeyError),
    #[error("pubkey parsing failed")]
    KeypairError(#[from] SignatureError),
}

#[derive(Default)]
pub struct Reader;

impl Reader {
    pub fn new() -> Self {
        Self::default()
    }

    #[throws]
    pub async fn pubkey(&self, name: &str) -> Pubkey {
        let path = format!("./keys/{}_pub.json", name);
        let key: String = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Pubkey::from_str(&key)?
    }

    #[throws]
    pub async fn keypair(&self, name: &str) -> Keypair {
        let path = format!("./keys/{}.json", name);
        let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Keypair::from_bytes(&bytes)?
    }

    #[throws]
    pub async fn program_data(&self, name: &str) -> Vec<u8> {
        fs::read(format!("./target/deploy/{}.so", name)).await?
    }
}
