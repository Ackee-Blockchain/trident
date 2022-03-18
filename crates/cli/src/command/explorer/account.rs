use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::{AccountFieldVisibility, AccountQueryBuilder},
    display::AccountDisplayFormat,
    output::get_account_string,
};

#[throws]
pub async fn view(pubkey: Pubkey, format: AccountDisplayFormat) {
    let query = AccountQueryBuilder::with_pubkey(pubkey).build();
    let account = query.fetch_one().await?;
    let visibility = AccountFieldVisibility::new_all_enabled();
    let result = get_account_string(&account, &visibility, format)?;
    println!("{}", result);
}
