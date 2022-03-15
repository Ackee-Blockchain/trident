use crate::{
    account::KeyedAccount,
    error::{ExplorerError, Result},
};
use console::style;
use serde::Serialize;
use solana_sdk::native_token;
use std::fmt;

pub enum DisplayFormat {
    // Trdelnik-interpreted
    Trdelnik,
    TrdelnikJSON,
    TrdelnikJSONPretty,
    // Raw, same as it sits on-chain
    Raw,
    RawJSON,
    RawJSONPretty,
}

impl DisplayFormat {
    pub fn formatted_account_string(&self, item: &DisplayKeyedAccount) -> Result<String> {
        match self {
            DisplayFormat::Trdelnik => Ok(format!("{}", item)),
            DisplayFormat::RawJSON => Ok(serde_json::to_string(&item)?),
            DisplayFormat::RawJSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            _ => Err(ExplorerError::Custom(
                "unsupported display format".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DisplayAccount {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize, Debug)]
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

pub fn writeln_styled(f: &mut fmt::Formatter, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    writeln!(f, "{} {}", style(name).bold(), styled_value)
}

pub fn write_styled(f: &mut fmt::Formatter, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    write!(f, "{} {}", style(name).bold(), styled_value)
}

impl fmt::Display for DisplayKeyedAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln_styled(f, "Public Key:", &self.pubkey)?;
        writeln!(f, "-------------------------------------------------------")?;
        writeln!(f)?;
        writeln_styled(
            f,
            "Lamports:",
            &format!(
                "{} (◎ {})",
                self.account.lamports,
                pretty_lamports_to_sol(self.account.lamports)
            ),
        )?;
        if self.account.data.is_empty() {
            writeln_styled(f, "Data:", "[Empty]")?;
        } else {
            writeln_styled(f, "Data:", "[Hexdump below]")?;
        }
        writeln_styled(f, "Owner:", &self.account.owner)?;
        if self.account.executable {
            writeln_styled(
                f,
                "Executable:",
                &format!("{} (This account is a program!)", self.account.executable),
            )?;
        } else {
            writeln_styled(f, "Executable:", &self.account.executable.to_string())?;
        }
        write_styled(
            f,
            "Rent Epoch:",
            &format!(
                "{} (Irrelevant due to rent-exemption)",
                self.account.rent_epoch
            ),
        )?;
        Ok(())
    }
}

pub enum DisplayProgramFormat {
    Trdelnik,
    JSONPretty,
    JSON,
}

pub struct DisplayUpgradeableProgram {
    pub program_id: String,
    pub owner: String,
    pub programdata_address: String,
    pub authority: String,
    pub last_deploy_slot: u64,
    pub data_len: usize,
    pub lamports: u64,
}

impl fmt::Display for DisplayUpgradeableProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln_styled(f, "Program Id:", &self.program_id)?;
        writeln_styled(f, "Owner:", &self.owner)?;
        writeln_styled(f, "ProgramData Address:", &self.programdata_address)?;
        writeln_styled(f, "Authority:", &self.authority)?;
        writeln_styled(
            f,
            "Last Deployed In Slot:",
            &self.last_deploy_slot.to_string(),
        )?;
        writeln_styled(
            f,
            "Data Length:",
            &format!("{:?} ({:#x?}) bytes", self.data_len, self.data_len),
        )?;
        writeln_styled(
            f,
            "Lamports:",
            &format!(
                "{} (◎ {})",
                self.lamports,
                pretty_lamports_to_sol(self.lamports)
            ),
        )?;
        Ok(())
    }
}
