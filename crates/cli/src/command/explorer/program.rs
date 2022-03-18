use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::AccountQueryBuilder, display::ProgramDisplayFormat, output::get_program_string,
    program::ProgramFieldVisibility,
};

#[throws]
pub async fn view(pubkey: Pubkey, format: ProgramDisplayFormat) {
    let query = AccountQueryBuilder::with_pubkey(pubkey).build();
    let account = query.fetch_one().await?;
    let visibility = ProgramFieldVisibility {};
    let result = get_program_string(&account, &visibility, format).await?;
    println!("{}", result);
}
