use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::AccountFieldVisibility, config::ExplorerConfig, display::DisplayFormat,
    output::print_account,
};

#[throws]
pub async fn view(pubkey: Pubkey, format: DisplayFormat) {
    let visibility = AccountFieldVisibility::new_all_enabled();
    let config = ExplorerConfig::default();
    print_account(&pubkey, &visibility, format, &config).await?;
}
