use anyhow::Error;
use fehler::throws;
use tokio::{task, fs};
use anchor_client::{
    anchor_lang::{System, Id},
    solana_sdk::signer::Signer, 
};
use trdelnik_client::{TrdelnikClient, read_keypair};

#[throws]
pub async fn init() {
    let id_keypair = read_keypair("id").await?;
    let id_pubkey = id_keypair.pubkey();

    let trdelnik = TrdelnikClient::new(id_keypair);

    let program_keypair = read_keypair("program").await?;
    let program_pubkey = program_keypair.pubkey();
    let program_data = fs::read("./target/deploy/turnstile.so").await?;
    let program = trdelnik.program(program_pubkey);

    let state = read_keypair("state").await?;

    // --

    println!("AIRDROP");
    trdelnik.airdrop(id_pubkey, 5_000_000_000).await?;

    println!("DEPLOY");
    trdelnik.deploy(program_keypair, program_data).await?;

    println!("INIT STATE");
    // @TODO design better and async API for `Program` and `Request(Builder)`
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
