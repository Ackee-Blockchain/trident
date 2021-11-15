use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn coin() {
    println!("Coin!");
}
