use anyhow::Error;
use fehler::throws;
use solana_sdk::signature::Signature;

#[throws]
pub async fn view(_signature: Signature) {}
