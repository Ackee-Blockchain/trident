use anyhow::Error;
use fehler::throws;
use trdelnik_client::*;

#[throws]
pub async fn init() {
    let reader = TrdelnikReader::new();

    let payer = reader.keypair("id").await?;
    let payer_pubkey = payer.pubkey();

    let program_keypair = reader.keypair("program").await?;
    let program_pubkey = program_keypair.pubkey();
    let program_data = reader.program_data("turnstile").await?;

    let client = TrdelnikClient::new(payer);

    println!("AIRDROP");
    client.airdrop(payer_pubkey, 5_000_000_000).await?;

    println!("DEPLOY");
    client.deploy(program_keypair, program_data).await?;

    println!("INIT STATE");
    let state = reader.keypair("state").await?;
    client.send_instruction(
        program_pubkey,
        turnstile::instruction::Initialize,
        turnstile::accounts::Initialize { 
            state: state.pubkey(),
            user: payer_pubkey,
            system_program: System::id()
        },
        Some(state),
    ).await?;

    println!("Initialized");
}
