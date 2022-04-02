use crate::output::pretty_lamports_to_sol;
use console::style;
use serde::Serialize;
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::fmt;

#[derive(Serialize)]
pub struct KeyedAccount {
    pub pubkey: Pubkey,
    pub account: Account,
}

pub struct AccountFieldVisibility {
    lamports: bool,
    data: bool,
    owner: bool,
    executable: bool,
    rent_epoch: bool,
}

impl AccountFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            lamports: true,
            data: true,
            owner: true,
            executable: true,
            rent_epoch: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            lamports: false,
            data: false,
            owner: false,
            executable: false,
            rent_epoch: false,
        }
    }

    pub fn lamports(&self) -> bool {
        self.lamports
    }

    pub fn enable_lamports(mut self) -> Self {
        self.lamports = true;
        self
    }

    pub fn disable_lamports(mut self) -> Self {
        self.lamports = false;
        self
    }

    pub fn data(&self) -> bool {
        self.data
    }

    pub fn enable_data(mut self) -> Self {
        self.data = true;
        self
    }

    pub fn disable_data(mut self) -> Self {
        self.data = false;
        self
    }

    pub fn owner(&self) -> bool {
        self.owner
    }

    pub fn enable_owner(mut self) -> Self {
        self.owner = true;
        self
    }

    pub fn disable_owner(mut self) -> Self {
        self.owner = false;
        self
    }

    pub fn executable(&self) -> bool {
        self.executable
    }

    pub fn enable_executable(mut self) -> Self {
        self.executable = true;
        self
    }

    pub fn disable_executable(mut self) -> Self {
        self.executable = false;
        self
    }

    pub fn rent_epoch(&self) -> bool {
        self.rent_epoch
    }

    pub fn enable_rent_epoch(mut self) -> Self {
        self.rent_epoch = true;
        self
    }

    pub fn disable_rent_epoch(mut self) -> Self {
        self.rent_epoch = false;
        self
    }
}

#[derive(Serialize)]
pub struct DisplayAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rent_epoch: Option<u64>,
}

#[derive(Serialize)]
pub struct DisplayKeyedAccount {
    pub pubkey: String,
    pub account: DisplayAccount,
}

impl DisplayKeyedAccount {
    pub fn from_keyed_account(
        keyed_account: &KeyedAccount,
        visibility: &AccountFieldVisibility,
    ) -> Self {
        Self {
            pubkey: keyed_account.pubkey.to_string(),
            account: DisplayAccount {
                lamports: if visibility.lamports {
                    Some(keyed_account.account.lamports)
                } else {
                    None
                },
                data: if visibility.data {
                    Some(base64::encode(&keyed_account.account.data))
                } else {
                    None
                },
                owner: if visibility.owner {
                    Some(keyed_account.account.owner.to_string())
                } else {
                    None
                },
                executable: if visibility.executable {
                    Some(keyed_account.account.executable)
                } else {
                    None
                },
                rent_epoch: if visibility.rent_epoch {
                    Some(keyed_account.account.rent_epoch)
                } else {
                    None
                },
            },
        }
    }
}

impl fmt::Display for DisplayKeyedAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "========================================================"
        )?;
        writeln!(f, "{} {}", style("Public Key:").bold(), self.pubkey)?;
        writeln!(
            f,
            "========================================================"
        )?;

        if let Some(lamports) = self.account.lamports {
            writeln!(f)?;
            write!(
                f,
                "{} {} (◎ {})",
                style("Lamports:").bold(),
                lamports,
                pretty_lamports_to_sol(lamports)
            )?;
        }
        if let Some(data) = &self.account.data {
            writeln!(f)?;
            if data.is_empty() {
                write!(f, "{} [Empty]", style("Data:").bold())?;
            } else {
                write!(f, "{} [Hexdump below]", style("Data:").bold())?;
            }
        }
        if let Some(owner) = &self.account.owner {
            writeln!(f)?;
            write!(f, "{} {}", style("Owner").bold(), owner)?;
        }
        if let Some(executable) = self.account.executable {
            writeln!(f)?;
            write!(f, "{} {}", style("Executable:").bold(), executable)?;
        }
        if let Some(rent_epoch) = self.account.rent_epoch {
            writeln!(f)?;
            write!(f, "{} {}", style("Rent Epoch:").bold(), rent_epoch)?;
        }

        Ok(())
    }
}
