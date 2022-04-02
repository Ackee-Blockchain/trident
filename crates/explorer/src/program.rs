use crate::{account::KeyedAccount, output::pretty_lamports_to_sol};
use console::style;
use serde::Serialize;
use solana_sdk::{bpf_loader_upgradeable::UpgradeableLoaderState, pubkey::Pubkey};
use std::fmt;

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
    pub raw_program_data: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_account: Option<DisplayProgramAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programdata_account: Option<DisplayProgramDataAccount>,
}

impl DisplayUpgradeableProgram {
    pub fn from(
        program_account: &KeyedAccount,
        programdata_account: &KeyedAccount,
        slot: u64,
        upgrade_authority_address: &Option<Pubkey>,
        visibility: &ProgramFieldVisibility,
    ) -> Self {
        Self {
            program_id: program_account.pubkey.to_string(),
            program_account: if visibility.program_account {
                Some(DisplayProgramAccount {
                    lamports: program_account.account.lamports,
                    data: ProgramDeserialized {
                        programdata_address: programdata_account.pubkey.to_string(),
                    },
                    owner: program_account.account.owner.to_string(),
                    executable: program_account.account.executable,
                    rent_epoch: program_account.account.rent_epoch,
                })
            } else {
                None
            },
            programdata_account: if visibility.programdata_account {
                Some(DisplayProgramDataAccount {
                    lamports: programdata_account.account.lamports,
                    data: ProgramDataDeserialized {
                        slot,
                        upgrade_authority_address: upgrade_authority_address
                            .map(|pubkey| pubkey.to_string())
                            .unwrap_or_else(|| "none".to_string()),
                        raw_program_data: base64::encode(
                            &programdata_account.account.data
                                [UpgradeableLoaderState::programdata_data_offset().unwrap()..],
                        ),
                    },
                    owner: programdata_account.account.owner.to_string(),
                    executable: programdata_account.account.executable,
                    rent_epoch: programdata_account.account.rent_epoch,
                })
            } else {
                None
            },
        }
    }
}

impl fmt::Display for DisplayUpgradeableProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "========================================================"
        )?;
        writeln!(f, "{} {}", style("Program Id:").bold(), self.program_id)?;
        writeln!(
            f,
            "========================================================"
        )?;

        if let Some(program_account) = &self.program_account {
            writeln!(f)?;

            writeln!(f, "{}", style("--> Program Account").bold(),)?;

            writeln!(f)?;

            writeln!(
                f,
                "{} {} (◎ {})",
                style("Lamports:").bold(),
                program_account.lamports,
                pretty_lamports_to_sol(program_account.lamports)
            )?;
            writeln!(
                f,
                "{} [Deserialized and interpreted below]",
                style("Data:").bold()
            )?;
            writeln!(f, "{} {}", style("Owner").bold(), program_account.owner)?;
            writeln!(
                f,
                "{} {}",
                style("Executable:").bold(),
                program_account.executable
            )?;
            writeln!(
                f,
                "{} {}",
                style("Rent Epoch:").bold(),
                program_account.rent_epoch
            )?;

            writeln!(f)?;

            writeln!(f, "{}", style("Deserialized:").bold())?;
            write!(f, "  - ")?;
            write!(
                f,
                "{} {}",
                style("ProgramData Address:").bold(),
                program_account.data.programdata_address
            )?;

            if self.programdata_account.is_some() {
                writeln!(f)?;
            }
        }

        if let Some(programdata_account) = &self.programdata_account {
            writeln!(f)?;

            writeln!(f, "{}", style("--> ProgramData Account").bold())?;

            writeln!(f)?;

            writeln!(
                f,
                "{} {} (◎ {})",
                style("Lamports:").bold(),
                programdata_account.lamports,
                pretty_lamports_to_sol(programdata_account.lamports)
            )?;
            writeln!(
                f,
                "{} [Deserialized and interpreted below]",
                style("Data:").bold()
            )?;
            writeln!(f, "{} {}", style("Owner").bold(), programdata_account.owner)?;
            writeln!(
                f,
                "{} {}",
                style("Executable:").bold(),
                programdata_account.executable
            )?;
            writeln!(
                f,
                "{} {}",
                style("Rent Epoch:").bold(),
                programdata_account.rent_epoch
            )?;

            writeln!(f)?;

            writeln!(f, "{}", style("Deserialized:").bold())?;
            write!(f, "  - ")?;
            writeln!(
                f,
                "{} {}",
                style("Last Deployed Slot:").bold(),
                programdata_account.data.slot
            )?;
            write!(f, "  - ")?;
            write!(
                f,
                "{} {}",
                style("Upgrade Authority:").bold(),
                programdata_account.data.upgrade_authority_address
            )?;
        }

        Ok(())
    }
}
