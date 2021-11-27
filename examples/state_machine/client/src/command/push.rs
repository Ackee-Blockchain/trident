use anyhow::Error;
use fehler::throws;
use tokio::task;
use crate::anchor_helpers::{new_client, read_keypair, read_pubkey};

#[throws]
pub async fn push() {
    let client = new_client(read_keypair("id").await?);
    let program = client.program(read_pubkey("program").await?);

    let state = read_pubkey("state").await?;
    task::spawn_blocking(move || {
        program
            .request()
            .args(turnstile::instruction::Push)
            .accounts(turnstile::accounts::UpdateState { state })
            .send()
    }).await??
}
