use anyhow::Error;
use fehler::throws;
use solana_sdk::signature::Signature;
use trdelnik_explorer::{
    display::{RawTransactionDisplayFormat, TransactionDisplayFormat},
    output::{get_transaction_string, get_transaction_string2},
    transaction::{RawTransactionFieldVisibility, TransactionFieldVisibility},
};

#[throws]
pub async fn view(signature: Signature, format: RawTransactionDisplayFormat) {
    let visibility = RawTransactionFieldVisibility {};
    let result = get_transaction_string(&signature, &visibility, format).await?;
    println!("{}", result);
}

#[throws]
pub async fn view2(signature: Signature, format: TransactionDisplayFormat) {
    let visibility = TransactionFieldVisibility {};
    let result = get_transaction_string2(&signature, &visibility, format).await?;
    println!("{}", result);
}
