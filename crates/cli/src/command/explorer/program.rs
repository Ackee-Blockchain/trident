use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    config::ExplorerConfig, display::DisplayFormat, output::print_program,
    program::ProgramFieldVisibility,
};

#[throws]
pub async fn view(pubkey: Pubkey, format: DisplayFormat) {
    let visibility = ProgramFieldVisibility::new_all_enabled();
    let config = ExplorerConfig::default();
    print_program(&pubkey, &visibility, format, &config).await?;
}
