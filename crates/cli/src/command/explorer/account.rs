use anyhow::Error;
use fehler::throws;
use solana_sdk::pubkey::Pubkey;
use trdelnik_explorer::{
    account::AccountFieldVisibility, config::ExplorerConfig, display::DisplayFormat,
    output::print_account,
};

#[throws]
pub async fn view(
    pubkey: Pubkey,
    hidelamports: bool,
    hidedata: bool,
    hideowner: bool,
    hideexecutable: bool,
    hiderentepoch: bool,
    format: DisplayFormat,
) {
    let mut visibility = AccountFieldVisibility::new_all_enabled();
    if hidelamports {
        visibility.disable_lamports();
    }
    if hidedata {
        visibility.disable_data();
    }
    if hideowner {
        visibility.disable_owner();
    }
    if hideexecutable {
        visibility.disable_executable();
    }
    if hiderentepoch {
        visibility.disable_rent_epoch();
    }
    let config = ExplorerConfig::default();
    print_account(&pubkey, &visibility, format, &config).await?;
}
