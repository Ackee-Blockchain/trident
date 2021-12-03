use anyhow::Error;
use fehler::throws;
use trdelnik_client::*;

#[throws]
pub async fn get_state() {
    let reader = TrdelnikReader::new();
    let account_pubkey = reader.pubkey("state").await?;
    let trdelnik = TrdelnikClient::new(reader.keypair("id").await?);
    let state: turnstile::State = trdelnik.account_data(account_pubkey).await?;
    println!("{}", state.locked);
    println!("{}", state.res);
}
