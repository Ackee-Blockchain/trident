use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::AccountQueryBuilder, display::DisplayFormat, output::get_program_string,
    program::ProgramFieldVisibility,
};

#[throws]
pub async fn view(pubkey: Pubkey) {
    let query = AccountQueryBuilder::with_pubkey(pubkey).build();
    let account = query.fetch_one().await?;
    let visibility = ProgramFieldVisibility {};
    let result = get_program_string(&account, &visibility, DisplayFormat::Trdelnik).await?;
    print!("{}", result);
}
