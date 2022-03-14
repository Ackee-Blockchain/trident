use crate::account::KeyedAccount;
use console::style;
use serde::Serialize;
use solana_sdk::native_token;
use std::fmt;

pub enum DisplayFormat {
    Trdelnik,
    PrettyJSON,
    MinifiedJSON,
}

#[derive(Serialize)]
pub struct DisplayAccount {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize)]
pub struct DisplayKeyedAccount {
    pub pubkey: String,
    pub account: DisplayAccount,
}

impl DisplayKeyedAccount {
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Self {
        Self {
            pubkey: keyed_account.pubkey.to_string(),
            account: DisplayAccount {
                lamports: keyed_account.account.lamports,
                data: base64::encode(keyed_account.account.data.clone()),
                owner: keyed_account.account.owner.to_string(),
                executable: keyed_account.account.executable,
                rent_epoch: keyed_account.account.rent_epoch,
            },
        }
    }
}

pub fn pretty_lamports_to_sol(lamports: u64) -> String {
    let sol_str = format!("{:.9}", native_token::lamports_to_sol(lamports));
    sol_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn writeln_name_value(f: &mut fmt::Formatter, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    writeln!(f, "{}: {}", style(name).bold(), styled_value)
}

impl fmt::Display for DisplayKeyedAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln_name_value(f, "Public Key", &self.pubkey)?;
        writeln!(f, "-------------------------------------------------------")?;
        writeln!(f)?;
        writeln_name_value(
            f,
            "Lamports",
            &format!(
                "{} (◎{})",
                self.account.lamports,
                pretty_lamports_to_sol(self.account.lamports)
            ),
        )?;
        if self.account.data.is_empty() {
            writeln_name_value(f, "Data", "[Empty]")?;
        } else {
            writeln_name_value(f, "Data", "=> Data Dump")?;
        }
        writeln_name_value(f, "Owner", &self.account.owner)?;
        writeln_name_value(f, "Executable", &self.account.executable.to_string())?;
        writeln_name_value(f, "Rent Epoch", &self.account.rent_epoch.to_string())?;
        Ok(())
    }
}
