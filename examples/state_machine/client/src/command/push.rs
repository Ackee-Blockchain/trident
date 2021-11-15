use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn push() {
    println!("Push!");
}
