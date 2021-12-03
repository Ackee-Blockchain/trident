use anyhow::Error;
use fehler::throws;
use trdelnik_client::*;

#[throws]
pub async fn coin() {
    let reader = TrdelnikReader::new();
    let payer = reader.keypair("id").await?;
    TrdelnikClient::new(payer).send_instruction(
        reader.pubkey("program").await?,
        turnstile::instruction::Coin,
        turnstile::accounts::UpdateState { 
            state: reader.pubkey("state").await?
        },
        None,
    ).await?;
}
