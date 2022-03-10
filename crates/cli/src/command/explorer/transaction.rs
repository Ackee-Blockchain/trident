use anyhow::Error;
use fehler::throws;
use solana_sdk::signature::Signature;

#[throws]
pub async fn view(signature: Signature) {
    println!("Show transaction: {}", signature);
}
