use crate::{
    account::KeyedAccount,
    error::{ExplorerError, Result},
};
use chrono::{TimeZone, Utc};
use console::style;
use serde::Serialize;
use solana_sdk::{
    bpf_loader_upgradeable::UpgradeableLoaderState,
    clock::Slot,
    message::{Message, VersionedMessage},
    native_token,
    program_utils::limited_deserialize,
    pubkey::Pubkey,
    signature::Signature,
    stake, system_instruction, system_program,
    transaction::VersionedTransaction,
};
use solana_transaction_status::{
    Encodable, EncodableWithMeta, EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    UiCompiledInstruction, UiMessage, UiTransaction, UiTransactionEncoding,
    UiTransactionStatusMeta,
};
use spl_memo::{id as spl_memo_id, v1::id as spl_memo_v1_id};
use std::fmt;

// Utility functions follow

pub fn pretty_lamports_to_sol(lamports: u64) -> String {
    let sol_str = format!("{:.9}", native_token::lamports_to_sol(lamports));
    sol_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn classify_account(message: &Message, index: usize) -> String {
    let mut account_type = String::new();
    let mut started = false;
    if index == 0 {
        account_type.push_str("[Fee Payer]");
        started = true;
    }
    if message.is_writable(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Writable]");
        started = true;
    }
    if message.is_signer(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Signer]");
        started = true;
    }
    if message.maybe_executable(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Program]");
    }
    account_type
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

// Display formats for items follow
// item: Account | Program

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

pub enum RawTransactionDisplayFormat {
    Trdelnik,
    JSONPretty,
    JSON,
}

impl RawTransactionDisplayFormat {
    pub fn formatted_transaction_string(&self, item: &DisplayRawTransaction) -> Result<String> {
        match self {
            RawTransactionDisplayFormat::Trdelnik => Ok(format!("{}", item)),
            RawTransactionDisplayFormat::JSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            RawTransactionDisplayFormat::JSON => Ok(serde_json::to_string(&item)?),
        }
    }
}

pub enum TransactionDisplayFormat {
    Trdelnik,
    JSONPretty,
    JSON,
}

impl TransactionDisplayFormat {
    pub fn formatted_transaction_string(&self, item: &DisplayTransaction) -> Result<String> {
        match self {
            TransactionDisplayFormat::Trdelnik => Ok(format!("{}", item)),
            TransactionDisplayFormat::JSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            TransactionDisplayFormat::JSON => Ok(serde_json::to_string(&item)?),
        }
    }
}

// Structs needed for items output, their constructors and fmt::Display trait implementations
// item: Account | Program

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawTransaction {
    pub transaction_id: String,
    pub slot: Slot,
    pub block_time: String,
    pub status: String,
    pub raw_content: UiTransaction,
}

impl DisplayRawTransaction {
    pub fn from(
        signature: &Signature,
        confirmed_transaction: &EncodedConfirmedTransactionWithStatusMeta,
    ) -> Result<Self> {
        let EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction: transaction_with_status_meta,
            block_time,
        } = confirmed_transaction;
        if let EncodedTransaction::Json(ui_transaction) = &transaction_with_status_meta.transaction
        {
            Ok(Self {
                transaction_id: signature.to_string(),
                slot: *slot,
                block_time: Utc.timestamp(block_time.unwrap(), 0).to_string(),
                status: transaction_with_status_meta
                    .meta
                    .clone()
                    .unwrap()
                    .err
                    .map(|err| format!("{}", err))
                    .unwrap_or_else(|| "SUCCESS".to_string()),
                raw_content: ui_transaction.clone(),
            })
        } else {
            Err(ExplorerError::Custom(
                "transaction decode failed".to_string(),
            ))
        }
    }
}

impl fmt::Display for DisplayRawTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "================================================================================"
        )?;
        writeln!(f, "{:^80}", style("On-chain State").bold())?;
        writeln!(
            f,
            "================================================================================"
        )?;

        writeln!(f)?;

        writeln_styled(f, "Transaction Id:", &self.transaction_id)?;
        writeln_styled(f, "Slot:", &self.slot.to_string())?;
        writeln_styled(f, "Timestamp:", &self.block_time)?;
        writeln_styled(f, "Status:", &self.status)?;

        writeln!(f)?;

        writeln!(
            f,
            "================================================================================"
        )?;
        writeln!(f, "{:^80}", style("Raw Transaction").bold())?;
        writeln!(
            f,
            "================================================================================"
        )?;

        writeln!(f)?;

        writeln!(
            f,
            "{}",
            style(format!(
                "Signatures ({}):",
                self.raw_content.signatures.len()
            ))
            .bold()
        )?;

        for (index, signature) in self.raw_content.signatures.iter().enumerate() {
            writeln_styled(f, &format!("  {:>2}", index), signature)?;
        }

        writeln!(f)?;

        if let UiMessage::Raw(ui_raw_message) = &self.raw_content.message {
            writeln!(f, "{}", style("Message:").bold())?;

            writeln!(f, "  {}", style("Header:").bold())?;

            writeln_styled(
                f,
                "    # of required signatures:",
                &ui_raw_message.header.num_required_signatures.to_string(),
            )?;
            writeln_styled(
                f,
                "    # of read-only signed accounts:",
                &ui_raw_message
                    .header
                    .num_readonly_signed_accounts
                    .to_string(),
            )?;
            writeln_styled(
                f,
                "    # of read-only unsigned accounts:",
                &ui_raw_message
                    .header
                    .num_readonly_unsigned_accounts
                    .to_string(),
            )?;

            writeln!(
                f,
                "  {}",
                style(format!(
                    "Account Keys ({}):",
                    ui_raw_message.account_keys.len()
                ))
                .bold()
            )?;

            for (index, account_key) in ui_raw_message.account_keys.iter().enumerate() {
                writeln_styled(f, &format!("   {:>2}", index), account_key)?;
            }

            writeln!(f, "  {}", style("Recent Blockhash:").bold())?;

            writeln!(f, "    {}", ui_raw_message.recent_blockhash)?;

            write!(
                f,
                "  {}",
                style(format!(
                    "Instructions ({}):",
                    ui_raw_message.instructions.len()
                ))
                .bold()
            )?;

            for (
                index,
                UiCompiledInstruction {
                    program_id_index,
                    accounts,
                    data,
                },
            ) in ui_raw_message.instructions.iter().enumerate()
            {
                writeln!(f)?;
                writeln_styled(
                    f,
                    &format!("    {:>2} Program Id Index:", index),
                    &program_id_index.to_string(),
                )?;
                writeln_styled(f, "      Account Indices:", &format!("{:?}", accounts))?;
                write_styled(f, "      Data:", data)?;
            }
        }

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayTransaction {
    pub transaction_id: String,
    pub slot: Slot,
    pub block_time: String,
    pub transaction: UiTransaction,
    pub meta: UiTransactionStatusMeta,
    #[serde(skip_serializing)]
    pub decoded: VersionedTransaction,
}

impl DisplayTransaction {
    pub fn from(
        signature: &Signature,
        confirmed_transaction: &EncodedConfirmedTransactionWithStatusMeta,
    ) -> Result<Self> {
        let EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction: transaction_with_status_meta,
            block_time,
        } = confirmed_transaction;
        let decoded = transaction_with_status_meta.transaction.decode().unwrap();
        let temp = decoded.clone();
        let transaction = if let VersionedMessage::Legacy(message) = temp.message {
            EncodedTransaction::Json(UiTransaction {
                signatures: temp.signatures.iter().map(ToString::to_string).collect(),
                message: message.encode(UiTransactionEncoding::JsonParsed),
            })
        } else {
            return Err(ExplorerError::Custom(
                "transaction back encode failed".to_string(),
            ));
        };

        if let EncodedTransaction::Json(ui_transaction) = &transaction {
            Ok(Self {
                transaction_id: signature.to_string(),
                slot: *slot,
                block_time: Utc.timestamp(block_time.unwrap(), 0).to_string(),
                transaction: ui_transaction.clone(),
                meta: transaction_with_status_meta.meta.clone().unwrap(),
                decoded,
            })
        } else {
            Err(ExplorerError::Custom(
                "transaction decode failed".to_string(),
            ))
        }
    }
}

impl fmt::Display for DisplayTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "================================================================================"
        )?;
        writeln!(f, "{:^80}", style("On-chain State").bold())?;
        writeln!(
            f,
            "================================================================================"
        )?;

        writeln!(f)?;

        writeln_styled(f, "Transaction Id:", &self.transaction_id)?;
        writeln_styled(f, "Slot:", &self.slot.to_string())?;
        writeln_styled(f, "Timestamp:", &self.block_time)?;
        writeln_styled(
            f,
            "Status:",
            &self
                .meta
                .clone()
                .err
                .map(|err| format!("{}", err))
                .unwrap_or_else(|| "SUCCESS".to_string()),
        )?;

        writeln!(f)?;

        writeln!(
            f,
            "================================================================================"
        )?;
        writeln!(f, "{:^80}", style("Transaction").bold())?;
        writeln!(
            f,
            "================================================================================"
        )?;

        writeln!(f)?;

        writeln!(
            f,
            "{}",
            style(format!("Signatures ({}):", self.decoded.signatures.len())).bold()
        )?;

        for (index, signature) in self.decoded.signatures.iter().enumerate() {
            writeln_styled(f, &format!("{:>3}", index), &signature.to_string())?;
        }

        writeln!(f)?;

        if let VersionedMessage::Legacy(message) = &self.decoded.message {
            writeln!(
                f,
                "{}",
                style(format!("Accounts ({}):", message.account_keys.len())).bold()
            )?;

            for (index, account_key) in message.account_keys.iter().enumerate() {
                let account_type = classify_account(message, index);
                writeln_styled(
                    f,
                    &format!("{:>3}", index),
                    &format!("{:<44}  {}", account_key.to_string(), account_type),
                )?;
            }

            writeln!(f)?;

            writeln!(
                f,
                "{}",
                style(format!("Instructions ({}):", message.instructions.len())).bold()
            )?;

            for (index, instruction) in message.instructions.iter().enumerate() {
                let program_id = message.account_keys[instruction.program_id_index as usize];
                writeln_styled(
                    f,
                    &format!("{:>3} Program Id:", index),
                    &format!(
                        "{:<44} ({})",
                        program_id.to_string(),
                        instruction.program_id_index
                    ),
                )?;
                for (account_index, account) in instruction.accounts.iter().enumerate() {
                    let account_id = message.account_keys[*account as usize];
                    writeln_styled(
                        f,
                        &format!("    Account {:>2}:", account_index),
                        &format!("{:<44} ({})", account_id.to_string(), account),
                    )?;
                }

                let mut raw = true;
                if program_id == solana_vote_program::id() {
                    if let Ok(vote_instruction) = limited_deserialize::<
                        solana_vote_program::vote_instruction::VoteInstruction,
                    >(&instruction.data)
                    {
                        writeln_styled(f, "    Data:", &format!("{:?}", vote_instruction))?;
                        raw = false;
                    }
                } else if program_id == stake::program::id() {
                    if let Ok(stake_instruction) = limited_deserialize::<
                        stake::instruction::StakeInstruction,
                    >(&instruction.data)
                    {
                        writeln_styled(f, "    Data:", &format!("{:?}", stake_instruction))?;
                        raw = false;
                    }
                } else if program_id == system_program::id() {
                    if let Ok(system_instruction) = limited_deserialize::<
                        system_instruction::SystemInstruction,
                    >(&instruction.data)
                    {
                        writeln_styled(f, "    Data:", &format!("{:?}", system_instruction))?;
                        raw = false;
                    }
                } else if program_id == spl_memo_v1_id() || program_id == spl_memo_id() {
                    if let Ok(s) = std::str::from_utf8(&instruction.data) {
                        writeln_styled(f, "    Data:", &format!("\"{}\"", s))?;
                        raw = false;
                    }
                }

                if raw {
                    writeln_styled(f, "    Data:", &format!("{:?}", instruction.data))?;
                }
            }
        }
        Ok(())
    }
}
