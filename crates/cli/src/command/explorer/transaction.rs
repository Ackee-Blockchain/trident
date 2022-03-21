use anyhow::Error;
use fehler::throws;
use solana_sdk::signature::Signature;
use trdelnik_explorer::{
    display::DisplayFormat,
    output::{print_raw_transaction, print_transaction},
    transaction::{RawTransactionFieldVisibility, TransactionFieldVisibility},
};

#[throws]
pub async fn view(signature: Signature, raw: bool, format: DisplayFormat) {
    if raw {
        let visibility = RawTransactionFieldVisibility::new_all_enabled();
        print_raw_transaction(&signature, &visibility, format).await?
    } else {
        let visibility = TransactionFieldVisibility::new_all_enabled();
        print_transaction(&signature, &visibility, format).await?
    };
}
