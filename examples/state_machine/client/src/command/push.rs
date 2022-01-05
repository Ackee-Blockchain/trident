use anyhow::Error;
use fehler::throws;
use trdelnik::*;

#[throws]
pub async fn push() {
    let reader = Reader::new();
    let payer = reader.keypair("id").await?;
    Client::new(payer).send_instruction(
        reader.pubkey("program").await?,
        turnstile::instruction::Push,
        turnstile::accounts::UpdateState { 
            state: reader.pubkey("state").await?
        },
        None,
    ).await?;
}
