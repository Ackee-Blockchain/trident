use anyhow::Error;
use fehler::throws;
use solana_sdk::signature::Signature;
use trident_explorer::{
    config::ExplorerConfig,
    display::DisplayFormat,
    output::{print_raw_transaction, print_transaction},
    transaction::{RawTransactionFieldVisibility, TransactionFieldVisibility},
};

#[throws]
pub async fn view(
    signature: Signature,
    raw: bool,
    hideoverview: bool,
    hidetransaction: bool,
    hidelogmessages: bool,
    format: DisplayFormat,
) {
    let config = ExplorerConfig::default();
    if raw {
        let mut visibility = RawTransactionFieldVisibility::new_all_enabled();
        if hideoverview {
            visibility.disable_overview();
        }
        if hidetransaction {
            visibility.disable_transaction();
        }
        print_raw_transaction(&signature, &visibility, format, &config).await?
    } else {
        let mut visibility = TransactionFieldVisibility::new_all_enabled();
        if hideoverview {
            visibility.disable_overview();
        }
        if hidetransaction {
            visibility.disable_transaction();
        }
        if hidelogmessages {
            visibility.disable_log_messages();
        }
        print_transaction(&signature, &visibility, format, &config).await?
    };
}
