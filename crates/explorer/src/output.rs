use crate::{
    account::{AccountFieldVisibility, KeyedAccount},
    display::{DisplayFormat, DisplayKeyedAccount},
    error::Result,
};
use console::style;
use pretty_hex::*;
use std::fmt::Write;

pub fn get_account_string(
    account: &KeyedAccount,
    visibility: &AccountFieldVisibility,
    format: DisplayFormat,
) -> Result<String> {
    let data = account.account.data.clone();
    let account = DisplayKeyedAccount::from_keyed_account(account);

    let account_string = match format {
        DisplayFormat::Trdelnik => {
            let mut account_string = format!("{}", account);
            if !data.is_empty() {
                writeln!(&mut account_string)?;
                writeln!(&mut account_string, "{}:", style("Data Dump").bold())?;
                writeln!(&mut account_string, "{:?}", data.hex_dump())?;
            }
            account_string
        }
        DisplayFormat::PrettyJSON => serde_json::to_string_pretty(&account)?,
        DisplayFormat::MinifiedJSON => serde_json::to_string(&account)?,
    };
    Ok(account_string)
}
