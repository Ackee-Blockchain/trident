use anyhow::Error;
use fehler::throws;
use tokio::task;
use crate::anchor_helpers::{new_client, read_keypair, read_pubkey};
use anchor_client::{
    anchor_lang::{System, Id},
    solana_sdk::signer::Signer,
};

#[throws]
pub async fn init() {
    let client = new_client(read_keypair("id").await?);
    let program = client.program(read_pubkey("program").await?);

    let state = read_keypair("state").await?;
    task::spawn_blocking(move || {
        program
            .request()
            .args(turnstile::instruction::Initialize)
            .accounts(turnstile::accounts::Initialize { 
                state: state.pubkey(),
                user: program.payer(),
                system_program: System::id()
            })
            .signer(&state)
            .send()
    }).await??;

    println!("Initialized");
}
