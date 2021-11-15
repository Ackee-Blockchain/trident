use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn init() {
    println!("Init!");
}
