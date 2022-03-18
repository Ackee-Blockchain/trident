use crate::{account::KeyedAccount, error::Result};
use console::style;
use serde::Serialize;
use solana_sdk::{bpf_loader_upgradeable::UpgradeableLoaderState, native_token, pubkey::Pubkey};
use std::fmt;

// Utility functions follow

pub fn pretty_lamports_to_sol(lamports: u64) -> String {
    let sol_str = format!("{:.9}", native_token::lamports_to_sol(lamports));
    sol_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn write_styled(f: &mut dyn fmt::Write, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    write!(f, "{} {}", style(name).bold(), styled_value)
}

pub fn writeln_styled(f: &mut dyn fmt::Write, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    writeln!(f, "{} {}", style(name).bold(), styled_value)
}

// Display formats follow

pub enum AccountDisplayFormat {
    Trdelnik,
    JSONPretty,
    JSON,
}

impl AccountDisplayFormat {
    pub fn formatted_account_string(&self, item: &DisplayKeyedAccount) -> Result<String> {
        match self {
            AccountDisplayFormat::Trdelnik => Ok(format!("{}", item)),
            AccountDisplayFormat::JSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            AccountDisplayFormat::JSON => Ok(serde_json::to_string(&item)?),
        }
    }
}

pub enum ProgramDisplayFormat {
    Trdelnik,
    JSONPretty,
    JSON,
}

impl ProgramDisplayFormat {
    pub fn formatted_program_string(&self, item: &DisplayUpgradeableProgram) -> Result<String> {
        match self {
            ProgramDisplayFormat::Trdelnik => Ok(format!("{}", item)),
            ProgramDisplayFormat::JSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            ProgramDisplayFormat::JSON => Ok(serde_json::to_string(&item)?),
        }
    }
}

// Structs needed for output, their constructors and fmt::Display trait implementations (always in this order)

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

impl fmt::Display for DisplayKeyedAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln_styled(f, "Public Key:", &self.pubkey)?;
        writeln!(
            f,
            "----------------------------------------------------------"
        )?;

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
                &format!("{} (implies account immutability)", self.account.executable),
            )?;
        } else {
            writeln_styled(f, "Executable:", &self.account.executable.to_string())?;
        }
        write_styled(
            f,
            "Rent Epoch:",
            &format!(
                "{} (irrelevant due to rent-exemption)",
                self.account.rent_epoch
            ),
        )?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ProgramDataDeserialized {
    pub slot: u64,
    pub upgrade_authority_address: String,
    pub raw_program_data_following_in_bytes: usize,
}

#[derive(Serialize)]
pub struct ProgramDeserialized {
    pub programdata_address: String,
}

#[derive(Serialize)]
pub struct DisplayProgramDataAccount {
    pub lamports: u64,
    pub data: ProgramDataDeserialized,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize)]
pub struct DisplayProgramAccount {
    pub lamports: u64,
    pub data: ProgramDeserialized,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize)]
pub struct DisplayUpgradeableProgram {
    pub program_id: String,
    pub program_account: DisplayProgramAccount,
    pub programdata_account: DisplayProgramDataAccount,
}

impl DisplayUpgradeableProgram {
    pub fn from(
        program_account: &KeyedAccount,
        programdata_account: &KeyedAccount,
        slot: u64,
        upgrade_authority_address: &Option<Pubkey>,
    ) -> Self {
        Self {
            program_id: program_account.pubkey.to_string(),
            program_account: DisplayProgramAccount {
                lamports: program_account.account.lamports,
                data: ProgramDeserialized {
                    programdata_address: programdata_account.pubkey.to_string(),
                },
                owner: program_account.account.owner.to_string(),
                executable: program_account.account.executable,
                rent_epoch: program_account.account.rent_epoch,
            },
            programdata_account: DisplayProgramDataAccount {
                lamports: programdata_account.account.lamports,
                data: ProgramDataDeserialized {
                    slot,
                    upgrade_authority_address: upgrade_authority_address
                        .map(|pubkey| pubkey.to_string())
                        .unwrap_or_else(|| "none".to_string()),
                    raw_program_data_following_in_bytes: programdata_account.account.data.len()
                        - UpgradeableLoaderState::programdata_data_offset().unwrap(),
                },

                owner: programdata_account.account.owner.to_string(),
                executable: programdata_account.account.executable,
                rent_epoch: programdata_account.account.rent_epoch,
            },
        }
    }
}

impl fmt::Display for DisplayUpgradeableProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln_styled(f, "Program Id:", &self.program_id)?;
        writeln!(
            f,
            "----------------------------------------------------------"
        )?;

        writeln!(f)?;

        writeln!(f, "{}", style("--> Program Account").bold())?;

        writeln!(f)?;

        writeln_styled(
            f,
            "Lamports:",
            &format!(
                "{} (◎ {})",
                self.program_account.lamports,
                pretty_lamports_to_sol(self.program_account.lamports)
            ),
        )?;
        writeln_styled(f, "Data:", "[Deserialized and interpreted below]")?;
        writeln_styled(f, "Owner:", &self.program_account.owner)?;
        if self.program_account.executable {
            writeln_styled(
                f,
                "Executable:",
                &format!(
                    "{} (implies account immutability)",
                    self.program_account.executable
                ),
            )?;
        } else {
            writeln_styled(
                f,
                "Executable:",
                &self.program_account.executable.to_string(),
            )?;
        }
        writeln_styled(
            f,
            "Rent Epoch:",
            &format!(
                "{} (irrelevant due to rent-exemption)",
                self.program_account.rent_epoch
            ),
        )?;

        writeln!(f)?;

        writeln!(f, "{}", style("Deserialized:").bold())?;
        write!(f, "  - ")?;
        writeln_styled(
            f,
            "ProgramData Address:",
            &self.program_account.data.programdata_address,
        )?;

        writeln!(f)?;

        writeln!(f, "{}", style("--> ProgramData Account").bold())?;

        writeln!(f)?;

        writeln_styled(
            f,
            "Lamports:",
            &format!(
                "{} (◎ {})",
                self.programdata_account.lamports,
                pretty_lamports_to_sol(self.programdata_account.lamports)
            ),
        )?;
        writeln_styled(f, "Data:", "[Deserialized and interpreted below]")?;
        writeln_styled(f, "Owner:", &self.programdata_account.owner)?;
        if self.programdata_account.executable {
            writeln_styled(
                f,
                "Executable:",
                &format!(
                    "{} (implies account immutability)",
                    self.programdata_account.executable
                ),
            )?;
        } else {
            writeln_styled(
                f,
                "Executable:",
                &self.programdata_account.executable.to_string(),
            )?;
        }
        writeln_styled(
            f,
            "Rent Epoch:",
            &format!(
                "{} (irrelevant due to rent-exemption)",
                self.programdata_account.rent_epoch
            ),
        )?;

        writeln!(f)?;

        writeln!(f, "{}", style("Deserialized:").bold())?;
        write!(f, "  - ")?;
        writeln_styled(
            f,
            "Last Deployed Slot:",
            &self.programdata_account.data.slot.to_string(),
        )?;
        write!(f, "  - ")?;
        write_styled(
            f,
            "Upgrade Authority:",
            &self.programdata_account.data.upgrade_authority_address,
        )?;

        Ok(())
    }
}
