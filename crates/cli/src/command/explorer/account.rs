use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;

#[throws]
pub async fn view(pubkey: Pubkey) {
    println!("Show account: {}", pubkey);
}
