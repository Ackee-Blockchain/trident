use std::fmt;

use console::style;
use serde::Serialize;
use solana_sdk::{pubkey::Pubkey, bpf_loader_upgradeable::UpgradeableLoaderState};

use crate::{output::{write_styled, writeln_styled, pretty_lamports_to_sol}, account::KeyedAccount};

pub struct ProgramFieldVisibility {
    program_account: bool,
    programdata_account: bool,
}

impl ProgramFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            program_account: true,
            programdata_account: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            program_account: false,
            programdata_account: false,
        }
    }

    pub fn program_account(&self) -> bool {
        self.program_account
    }

    pub fn enable_program_account(&mut self) -> &mut Self {
        self.program_account = true;
        self
    }

    pub fn disable_program_account(&mut self) -> &mut Self {
        self.program_account = false;
        self
    }

    pub fn programdata_account(&self) -> bool {
        self.program_account
    }

    pub fn enable_programdata_account(&mut self) -> &mut Self {
        self.programdata_account = true;
        self
    }

    pub fn disable_programdata_account(&mut self) -> &mut Self {
        self.programdata_account = false;
        self
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
