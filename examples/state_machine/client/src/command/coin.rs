use anyhow::Error;
use fehler::throws;
use tokio::task;
use trdelnik_client::{TrdelnikClient, read_keypair, read_pubkey};

#[throws]
pub async fn coin() {
    let trdelnik = TrdelnikClient::new(read_keypair("id").await?);
    let program = trdelnik.program(read_pubkey("program").await?);
    let state = read_pubkey("state").await?;
    
    task::spawn_blocking(move || {
        program
            .request()
            .args(turnstile::instruction::Coin)
            .accounts(turnstile::accounts::UpdateState { state })
            .send()
    }).await??
}
