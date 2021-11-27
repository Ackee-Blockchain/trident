use anchor_client::{Client, Cluster};
use anyhow::Error;
use fehler::throws;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::{
        keypair::Keypair,
    },
};
use tokio::fs;
use std::str::FromStr;

pub fn new_client(payer: Keypair) -> Client {
    Client::new_with_options(
        Cluster::Localnet,
        payer,
        CommitmentConfig::confirmed(),
    )
}

#[throws]
pub async fn read_pubkey(name: &str) -> Pubkey {
    let path = format!("./keys/{}_pub.json", name);
    let key: String = serde_json::from_str(&fs::read_to_string(path).await?)?;
    Pubkey::from_str(&key)?
}

#[throws]
pub async fn read_keypair(name: &str) -> Keypair {
    let path = format!("./keys/{}.json", name);
    let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).await?)?;
    Keypair::from_bytes(&bytes)?
}
