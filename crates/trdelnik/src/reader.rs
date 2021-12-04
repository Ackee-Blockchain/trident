use anchor_client::{
    solana_sdk::{
        signer::keypair::Keypair,
        pubkey::{ParsePubkeyError, Pubkey},
    }
};
use tokio::fs;
use std::{io, str::FromStr, borrow::Cow};
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

pub struct Reader {
    root: Cow<'static, str>
}

impl Reader {
    pub fn new() -> Self {
        Self {
            root: "./".into()
        }
    }

    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self {
            root: root.into()
        }
    }

    #[throws]
    pub async fn pubkey(&self, name: &str) -> Pubkey {
        let path = format!("{}keys/{}_pub.json", self.root, name);
        let key: String = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Pubkey::from_str(&key)?
    }

    #[throws]
    pub async fn keypair(&self, name: &str) -> Keypair {
        let path = format!("{}keys/{}.json", self.root, name);
        let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Keypair::from_bytes(&bytes)?
    }

    #[throws]
    pub async fn program_data(&self, name: &str) -> Vec<u8> {
        fs::read(format!("{}target/deploy/{}.so", self.root, name)).await?
    }
}
