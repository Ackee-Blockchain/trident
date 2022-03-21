use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::AccountFieldVisibility, display::DisplayFormat, output::print_account,
};

#[throws]
pub async fn view(pubkey: Pubkey, format: DisplayFormat) {
    let visibility = AccountFieldVisibility::new_all_enabled();
    print_account(&pubkey, &visibility, format).await?;
}
