use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::{AccountFieldVisibility, AccountQueryBuilder},
    display::DisplayFormat,
    output::get_account_string,
};

#[throws]
pub async fn view(pubkey: Pubkey) {
    let query = AccountQueryBuilder::with_pubkey(pubkey).build();
    let account = query.fetch_one().await?;
    let visibility = AccountFieldVisibility::new_all_enabled();
    let result = get_account_string(&account, &visibility, DisplayFormat::Trdelnik)?;
    println!("{}", result);
}
