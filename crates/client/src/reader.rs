use anchor_client::solana_sdk::{
    pubkey::{ParsePubkeyError, Pubkey},
    signer::keypair::Keypair,
};
use ed25519_dalek::SignatureError;
use fehler::throws;
use std::{borrow::Cow, io, str::FromStr};
use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot read the file")]
    Io(#[from] io::Error),
    #[error("deserialization failed")]
    SerdeJson(#[from] serde_json::Error),
    #[error("pubkey parsing failed")]
    Pubkey(#[from] ParsePubkeyError),
    #[error("keypair parsing failed")]
    Keypair(#[from] SignatureError),
}

/// `Reader` allows you to read [Pubkey], [Keypair] and other entities from files.
pub struct Reader {
    root: Cow<'static, str>,
}

impl Reader {
    /// Creates a new `Reader` instance with the default root `"../../"`.
    pub fn new() -> Self {
        Self {
            root: "../../".into(),
        }
    }

    /// Creates a new `Reader` instance with the provided `root`.
    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self { root: root.into() }
    }

    /// Reads [Pubkey] from `[root]keys/[name]_pub.json`.
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the requested file does not exist or it is not readable.
    /// - [Pubkey] cannot be parsed from the file content.
    #[throws]
    pub async fn pubkey(&self, name: &str) -> Pubkey {
        let path = format!("{}keys/{}_pub.json", self.root, name);
        let key: String = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Pubkey::from_str(&key)?
    }

    /// Reads [Keypair] from `[root]keys/[name].json`.
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the requested file does not exist or it is not readable.
    /// - [Keypair] cannot be parsed from the file content.
    #[throws]
    pub async fn keypair(&self, name: &str) -> Keypair {
        let path = format!("{}keys/{}.json", self.root, name);
        let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).await?)?;
        Keypair::from_bytes(&bytes)?
    }

    /// Reads program data from `[root]target/deploy/[name].so`.
    ///
    /// # Errors
    ///
    /// It fails when the requested file does not exist or it is not readable.
    #[throws]
    pub async fn program_data(&self, name: &str) -> Vec<u8> {
        fs::read(format!("{}target/deploy/{}.so", self.root, name)).await?
    }
}

impl Default for Reader {
    /// Creates a new `Reader` instance with the default root `"../../"`.
    fn default() -> Self {
        Self::new()
    }
}
