use crate::{
    error::ExplorerError,
    error::Result,
    output::{change_in_sol, classify_account, pretty_lamports_to_sol, status_to_string},
    parse::{parse, partially_parse},
};
use chrono::{TimeZone, Utc};
use console::style;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::{instruction::CompiledInstruction, message::VersionedMessage, pubkey::Pubkey};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransactionWithStatusMeta, TransactionStatus,
};
use std::fmt;

pub struct RawTransactionFieldVisibility {
    overview: bool,
    transaction: bool,
}

impl RawTransactionFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            overview: true,
            transaction: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            overview: false,
            transaction: false,
        }
    }

    pub fn overview(&self) -> bool {
        self.overview
    }

    pub fn enable_overview(mut self) -> Self {
        self.overview = true;
        self
    }

    pub fn disable_overview(mut self) -> Self {
        self.overview = false;
        self
    }

    pub fn transaction(&self) -> bool {
        self.transaction
    }

    pub fn enable_transaction(mut self) -> Self {
        self.transaction = true;
        self
    }

    pub fn disable_transaction(mut self) -> Self {
        self.transaction = false;
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawMessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawMessage {
    pub header: DisplayRawMessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<DisplayRawInstruction>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawTransactionContent {
    pub signatures: Vec<String>,
    pub message: DisplayRawMessage,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawTransactionOverview {
    pub signature: String,
    pub result: String,
    pub timestamp: String,
    pub confirmation_status: String,
    pub confirmations: String,
    pub slot: u64,
    pub recent_blockhash: String,
    pub fee: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRawTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<DisplayRawTransactionOverview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<DisplayRawTransactionContent>,
}

impl DisplayRawTransaction {
    pub fn from(
        transaction: &EncodedConfirmedTransactionWithStatusMeta,
        transaction_status: &TransactionStatus,
        visibility: &RawTransactionFieldVisibility,
    ) -> Result<Self> {
        let EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction,
            block_time,
        } = transaction;

        let EncodedTransactionWithStatusMeta {
            transaction,
            meta,
            version: _version,
        } = transaction;

        let versioned_transaction = transaction.decode().unwrap();

        if let VersionedMessage::Legacy(message) = versioned_transaction.message {
            let overview = if visibility.overview {
                Some(DisplayRawTransactionOverview {
                    signature: versioned_transaction.signatures[0].to_string(),
                    result: meta
                        .as_ref()
                        .unwrap()
                        .err
                        .as_ref()
                        .map(|err| err.to_string())
                        .unwrap_or_else(|| "Success".to_string()),
                    timestamp: Utc.timestamp(block_time.unwrap(), 0).to_string(),
                    confirmation_status: status_to_string(
                        transaction_status.confirmation_status.as_ref().unwrap(),
                    ),
                    confirmations: transaction_status
                        .confirmations
                        .map_or_else(|| "MAX (32)".to_string(), |n| n.to_string()),
                    slot: *slot,
                    recent_blockhash: message.recent_blockhash.to_string(),
                    fee: format!("◎ {}", pretty_lamports_to_sol(meta.as_ref().unwrap().fee)),
                })
            } else {
                None
            };

            let transaction = if visibility.transaction {
                Some(DisplayRawTransactionContent {
                    signatures: versioned_transaction
                        .signatures
                        .into_iter()
                        .map(|sig| sig.to_string())
                        .collect(),
                    message: DisplayRawMessage {
                        header: DisplayRawMessageHeader {
                            num_required_signatures: message.header.num_required_signatures,
                            num_readonly_signed_accounts: message
                                .header
                                .num_readonly_signed_accounts,
                            num_readonly_unsigned_accounts: message
                                .header
                                .num_readonly_unsigned_accounts,
                        },
                        account_keys: message
                            .account_keys
                            .into_iter()
                            .map(|key| key.to_string())
                            .collect(),
                        recent_blockhash: message.recent_blockhash.to_string(),
                        instructions: message
                            .instructions
                            .into_iter()
                            .map(|instruction| DisplayRawInstruction {
                                program_id_index: instruction.program_id_index,
                                accounts: instruction.accounts,
                                data: bs58::encode(instruction.data).into_string(),
                            })
                            .collect(),
                    },
                })
            } else {
                None
            };

            Ok(DisplayRawTransaction {
                overview,
                transaction,
            })
        } else {
            Err(ExplorerError::Custom("message in wrong format".to_string()))
        }
    }
}

impl fmt::Display for DisplayRawTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(overview) = &self.overview {
            writeln!(
                f,
                "================================================================================"
            )?;
            writeln!(f, "{:^80}", style("Overview").bold())?;
            writeln!(
                f,
                "================================================================================"
            )?;

            writeln!(f)?;

            writeln!(f, "{} {}", style("Signature:").bold(), overview.signature)?;
            writeln!(f, "{} {}", style("Result:").bold(), overview.result)?;
            writeln!(f, "{} {}", style("Timestamp:").bold(), overview.timestamp)?;
            writeln!(
                f,
                "{} {}",
                style("Confirmation Status:").bold(),
                overview.confirmation_status
            )?;
            writeln!(
                f,
                "{} {}",
                style("Confirmations:").bold(),
                overview.confirmations
            )?;
            writeln!(f, "{} {}", style("Slot:").bold(), overview.slot)?;
            writeln!(
                f,
                "{} {}",
                style("Recent Blockhash:").bold(),
                overview.recent_blockhash
            )?;
            write!(f, "{} {}", style("Fee:").bold(), overview.fee)?;
        }

        if self.overview.is_some() && self.transaction.is_some() {
            writeln!(f)?;
            writeln!(f)?;
        }

        if let Some(transaction) = &self.transaction {
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
                style(format!("Signatures ({}):", transaction.signatures.len())).bold()
            )?;

            for (index, signature) in transaction.signatures.iter().enumerate() {
                writeln!(f, "  {:>2} {}", style(index).bold(), signature)?;
            }

            writeln!(f)?;

            writeln!(f, "{}", style("Message:").bold())?;

            writeln!(f, "  {}", style("Header:").bold())?;

            writeln!(
                f,
                "    {} {}",
                style("# of required signatures:").bold(),
                transaction.message.header.num_required_signatures
            )?;

            writeln!(
                f,
                "    {} {}",
                style("# of read-only signed accounts:").bold(),
                transaction.message.header.num_readonly_signed_accounts
            )?;

            writeln!(
                f,
                "    {} {}",
                style("# of read-only unsigned accounts:").bold(),
                transaction.message.header.num_readonly_unsigned_accounts
            )?;

            writeln!(
                f,
                "  {}",
                style(format!(
                    "Account Keys ({}):",
                    transaction.message.account_keys.len()
                ))
                .bold()
            )?;

            for (index, account_key) in transaction.message.account_keys.iter().enumerate() {
                writeln!(f, "   {:>2} {}", style(index).bold(), account_key)?;
            }

            writeln!(f, "  {}", style("Recent Blockhash:").bold())?;

            writeln!(f, "    {}", transaction.message.recent_blockhash)?;

            write!(
                f,
                "  {}",
                style(format!(
                    "Instructions ({}):",
                    transaction.message.instructions.len()
                ))
                .bold()
            )?;

            for (
                index,
                DisplayRawInstruction {
                    program_id_index,
                    accounts,
                    data,
                },
            ) in transaction.message.instructions.iter().enumerate()
            {
                writeln!(f)?;
                writeln!(
                    f,
                    "   {:>2} {} {}",
                    style(index).bold(),
                    style("Program Id Index:").bold(),
                    program_id_index
                )?;
                writeln!(
                    f,
                    "      {} {:?}",
                    style("Account Indices:").bold(),
                    accounts
                )?;
                write!(f, "      {} {:?}", style("Data:").bold(), data)?;
            }
        }

        Ok(())
    }
}

pub struct TransactionFieldVisibility {
    overview: bool,
    transaction: bool,
    log_messages: bool,
}

impl TransactionFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            overview: true,
            transaction: true,
            log_messages: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            overview: false,
            transaction: false,
            log_messages: false,
        }
    }

    pub fn overview(&self) -> bool {
        self.overview
    }

    pub fn enable_overview(mut self) -> Self {
        self.overview = true;
        self
    }

    pub fn disable_overview(mut self) -> Self {
        self.overview = false;
        self
    }

    pub fn transaction(&self) -> bool {
        self.transaction
    }

    pub fn enable_transaction(mut self) -> Self {
        self.transaction = true;
        self
    }

    pub fn disable_transaction(mut self) -> Self {
        self.transaction = false;
        self
    }

    pub fn log_messages(&self) -> bool {
        self.log_messages
    }

    pub fn enable_log_messages(mut self) -> Self {
        self.log_messages = true;
        self
    }

    pub fn disable_log_messages(mut self) -> Self {
        self.log_messages = false;
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPartiallyParsedInstruction {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub data: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayParsedInstruction {
    pub program: String,
    pub program_id: String,
    pub parsed: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayInstruction {
    Parsed(DisplayParsedInstruction),
    PartiallyParsed(DisplayPartiallyParsedInstruction),
}

impl DisplayInstruction {
    fn parse(instruction: &CompiledInstruction, account_keys: &[Pubkey]) -> Self {
        let program_id = &account_keys[instruction.program_id_index as usize];
        if let Ok(parsed_instruction) = parse(program_id, instruction, account_keys) {
            DisplayInstruction::Parsed(parsed_instruction)
        } else {
            DisplayInstruction::PartiallyParsed(partially_parse(
                program_id,
                instruction,
                account_keys,
            ))
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayInputAccount {
    pub pubkey: String,
    pub fee_payer: bool,
    pub writable: bool,
    pub signer: bool,
    pub program: bool,
    pub post_balance_in_sol: String,
    pub balance_change_in_sol: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayTransactionContent {
    pub accounts: Vec<DisplayInputAccount>,
    pub instructions: Vec<DisplayInstruction>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayTransactionOverview {
    pub signature: String,
    pub result: String,
    pub timestamp: String,
    pub confirmation_status: String,
    pub confirmations: String,
    pub slot: u64,
    pub recent_blockhash: String,
    pub fee: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<DisplayTransactionOverview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<DisplayTransactionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_messages: Option<Option<Vec<String>>>,
}

impl DisplayTransaction {
    pub fn from(
        transaction: &EncodedConfirmedTransactionWithStatusMeta,
        transaction_status: &TransactionStatus,
        visibility: &TransactionFieldVisibility,
    ) -> Result<Self> {
        let EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction,
            block_time,
        } = transaction;

        let EncodedTransactionWithStatusMeta {
            transaction,
            meta,
            version: _version,
        } = transaction;

        let versioned_transaction = transaction.decode().unwrap();

        if let VersionedMessage::Legacy(message) = versioned_transaction.message {
            let overview = if visibility.overview {
                Some(DisplayTransactionOverview {
                    signature: versioned_transaction.signatures[0].to_string(),
                    result: meta
                        .as_ref()
                        .unwrap()
                        .err
                        .as_ref()
                        .map(|err| err.to_string())
                        .unwrap_or_else(|| "Success".to_string()),
                    timestamp: Utc.timestamp(block_time.unwrap(), 0).to_string(),
                    confirmation_status: status_to_string(
                        transaction_status.confirmation_status.as_ref().unwrap(),
                    ),
                    confirmations: transaction_status
                        .confirmations
                        .map_or_else(|| "MAX (32)".to_string(), |n| n.to_string()),
                    slot: *slot,
                    recent_blockhash: message.recent_blockhash.to_string(),
                    fee: format!("◎ {}", pretty_lamports_to_sol(meta.as_ref().unwrap().fee)),
                })
            } else {
                None
            };

            let mut fee_payer_found = false; // always first account
            let transaction = if visibility.transaction {
                Some(DisplayTransactionContent {
                    accounts: message
                        .account_keys
                        .iter()
                        .enumerate()
                        .map(|(index, account_key)| DisplayInputAccount {
                            pubkey: account_key.to_string(),
                            fee_payer: if !fee_payer_found {
                                fee_payer_found = true;
                                true
                            } else {
                                false
                            },
                            writable: message.is_writable(index),
                            signer: message.is_signer(index),
                            program: message.maybe_executable(index),
                            post_balance_in_sol: pretty_lamports_to_sol(
                                meta.as_ref().unwrap().post_balances[index],
                            ),
                            balance_change_in_sol: change_in_sol(
                                meta.as_ref().unwrap().post_balances[index],
                                meta.as_ref().unwrap().pre_balances[index],
                            ),
                        })
                        .collect(),
                    instructions: message
                        .instructions
                        .iter()
                        .map(|instruction| {
                            DisplayInstruction::parse(instruction, &message.account_keys)
                        })
                        .collect(),
                })
            } else {
                None
            };

            let log_messages = if visibility.log_messages {
                Some(meta.as_ref().unwrap().log_messages.clone())
            } else {
                None
            };

            Ok(DisplayTransaction {
                overview,
                transaction,
                log_messages,
            })
        } else {
            Err(ExplorerError::Custom(
                "message in a new unsupported format".to_string(),
            ))
        }
    }
}

impl fmt::Display for DisplayTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(overview) = &self.overview {
            writeln!(
                f,
                "================================================================================"
            )?;
            writeln!(f, "{:^80}", style("Overview").bold())?;
            writeln!(
                f,
                "================================================================================"
            )?;

            writeln!(f)?;

            writeln!(f, "{} {}", style("Signature:").bold(), overview.signature)?;
            writeln!(f, "{} {}", style("Result:").bold(), overview.result)?;
            writeln!(f, "{} {}", style("Timestamp:").bold(), overview.timestamp)?;
            writeln!(
                f,
                "{} {}",
                style("Confirmation Status:").bold(),
                overview.confirmation_status
            )?;
            writeln!(
                f,
                "{} {}",
                style("Confirmations:").bold(),
                overview.confirmations
            )?;
            writeln!(f, "{} {}", style("Slot:").bold(), overview.slot)?;
            writeln!(
                f,
                "{} {}",
                style("Recent Blockhash:").bold(),
                overview.recent_blockhash
            )?;
            write!(f, "{} {}", style("Fee:").bold(), overview.fee)?;
        }

        if self.overview.is_some() && self.transaction.is_some() {
            writeln!(f)?;
            writeln!(f)?;
        }

        if let Some(transaction) = &self.transaction {
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
                style(format!("Accounts ({}):", transaction.accounts.len())).bold()
            )?;

            for (index, account) in transaction.accounts.iter().enumerate() {
                let account_type_string = classify_account(
                    account.fee_payer,
                    account.writable,
                    account.signer,
                    account.program,
                );

                let balance_information_string = if account.balance_change_in_sol != "0" {
                    format!(
                        "◎ {} (◎ {})",
                        account.post_balance_in_sol, account.balance_change_in_sol
                    )
                } else {
                    format!("◎ {}", account.post_balance_in_sol)
                };

                writeln!(
                    f,
                    " {:>2} {:<44} {:31} {}",
                    style(index).bold(),
                    account.pubkey,
                    account_type_string,
                    balance_information_string
                )?;
            }

            writeln!(f)?;

            writeln!(
                f,
                "{}",
                style(format!(
                    "Instructions ({}):",
                    transaction.instructions.len()
                ))
                .bold()
            )?;

            for (index, instruction) in transaction.instructions.iter().enumerate() {
                if let DisplayInstruction::Parsed(instruction) = instruction {
                    writeln!(
                        f,
                        " {:>2} {} {} {}",
                        style(index).bold(),
                        style(&instruction.program).bold(),
                        style("Program:").bold(),
                        instruction.parsed["type"].to_string().trim_matches('"')
                    )?;
                    writeln!(f, "    [{}]", instruction.program_id)?;
                    for (name, value) in instruction.parsed["info"].as_object().unwrap() {
                        writeln!(
                            f,
                            "    {}{} {}",
                            style(name).bold(),
                            style(":").bold(),
                            value
                        )?;
                    }
                } else if let DisplayInstruction::PartiallyParsed(instruction) = instruction {
                    writeln!(
                        f,
                        " {:>2} {} Unknown Instruction",
                        style(index).bold(),
                        style("Unknown Program:").bold(),
                    )?;
                    writeln!(f, "    [{}]", instruction.program_id)?;
                    for (index, account) in instruction.accounts.iter().enumerate() {
                        writeln!(
                            f,
                            "    {} {}{} {:<44}",
                            style("Account").bold(),
                            style(index).bold(),
                            style(":").bold(),
                            account,
                        )?;
                    }
                    writeln!(
                        f,
                        "    {} {:?}",
                        style("Data:").bold(),
                        bs58::encode(instruction.data.clone()).into_string()
                    )?;
                }
                writeln!(f)?;
            }
        }

        if self.overview.is_some() && self.transaction.is_none() {
            writeln!(f)?;
            writeln!(f)?;
        }

        if let Some(Some(log_messages)) = &self.log_messages {
            write!(
                f,
                "{}",
                style(format!("Log Messages ({}):", log_messages.len())).bold()
            )?;

            for (log_message_index, log_message) in log_messages.iter().enumerate() {
                writeln!(f)?;
                write!(f, " {:>2} {}", style(log_message_index).bold(), log_message)?;
            }
        }

        Ok(())
    }
}
