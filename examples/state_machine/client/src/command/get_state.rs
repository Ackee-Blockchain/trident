use anyhow::Error;
use fehler::throws;
use tokio::task;
use crate::anchor_helpers::{new_client, read_keypair, read_pubkey};

#[throws]
pub async fn get_state() {
    let client = new_client(read_keypair("id").await?);
    let program = client.program(read_pubkey("program").await?);

    let state = read_pubkey("state").await?;
    let pre_account = task::spawn_blocking(move || {
        program.account::<turnstile::State>(state)
    })
    .await??;

    println!("{}", pre_account.locked);
    println!("{}", pre_account.res);
}
