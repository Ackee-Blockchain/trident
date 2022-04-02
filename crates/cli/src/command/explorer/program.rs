use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    config::ExplorerConfig, display::DisplayFormat, output::print_program,
    program::ProgramFieldVisibility,
};

#[throws]
pub async fn view(
    pubkey: Pubkey,
    hideprogramaccount: bool,
    hideprogramdataaccount: bool,
    format: DisplayFormat,
) {
    let mut visibility = ProgramFieldVisibility::new_all_enabled();
    if hideprogramaccount {
        visibility.disable_program_account();
    }
    if hideprogramdataaccount {
        visibility.disable_programdata_account();
    }
    let config = ExplorerConfig::default();
    print_program(&pubkey, &visibility, format, &config).await?;
}
