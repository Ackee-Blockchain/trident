use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn get_state() {
    println!("GetState!");
}
