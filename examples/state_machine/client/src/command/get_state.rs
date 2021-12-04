use anyhow::Error;
use fehler::throws;
use trdelnik::*;

#[throws]
pub async fn get_state() {
    let reader = Reader::new();
    let account_pubkey = reader.pubkey("state").await?;
    let client = Client::new(reader.keypair("id").await?);
    let state: turnstile::State = client.account_data(account_pubkey).await?;
    println!("{}", state.locked);
    println!("{}", state.res);
}
