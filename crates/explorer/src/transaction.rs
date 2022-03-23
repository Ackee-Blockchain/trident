use crate::{
    error::ExplorerError,
    error::Result,
    output::{change_in_sol, classify_account, pretty_lamports_to_sol, status_to_string},
};
use chrono::{TimeZone, Utc};
use console::style;
use serde::Serialize;

use solana_sdk::message::VersionedMessage;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransactionWithStatusMeta, TransactionStatus,
};

use std::fmt;

pub struct RawTransactionFieldVisibility {
    slot: bool,
    block_time: bool,
    status: bool,
    raw_content: bool,
}

impl RawTransactionFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            slot: true,
            block_time: true,
            status: true,
            raw_content: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            slot: false,
            block_time: false,
            status: false,
            raw_content: false,
        }
    }

    pub fn slot(&self) -> bool {
        self.slot
    }

    pub fn enable_slot(&mut self) -> &mut Self {
        self.slot = true;
        self
    }

    pub fn disable_slot(&mut self) -> &mut Self {
        self.slot = false;
        self
    }

    pub fn block_time(&self) -> bool {
        self.block_time
    }

    pub fn enable_block_time(&mut self) -> &mut Self {
        self.block_time = true;
        self
    }

    pub fn disable_block_time(&mut self) -> &mut Self {
        self.block_time = false;
        self
    }

    pub fn status(&self) -> bool {
        self.status
    }

    pub fn enable_status(&mut self) -> &mut Self {
        self.status = true;
        self
    }

    pub fn disable_status(&mut self) -> &mut Self {
        self.status = false;
        self
    }

    pub fn raw_content(&self) -> bool {
        self.raw_content
    }

    pub fn enable_raw_content(&mut self) -> &mut Self {
        self.raw_content = true;
        self
    }

    pub fn disable_raw_content(&mut self) -> &mut Self {
        self.raw_content = false;
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
    pub overview: DisplayRawTransactionOverview,
    pub transaction: DisplayRawTransactionContent,
}

impl DisplayRawTransaction {
    pub fn from(
        transaction: &EncodedConfirmedTransactionWithStatusMeta,
        transaction_status: &TransactionStatus,
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
            let overview = DisplayRawTransactionOverview {
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
            };

            let transaction = DisplayRawTransactionContent {
                signatures: versioned_transaction
                    .signatures
                    .into_iter()
                    .map(|sig| sig.to_string())
                    .collect(),
                message: DisplayRawMessage {
                    header: DisplayRawMessageHeader {
                        num_required_signatures: message.header.num_required_signatures,
                        num_readonly_signed_accounts: message.header.num_readonly_signed_accounts,
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

        writeln!(
            f,
            "{} {}",
            style("Signature:").bold(),
            self.overview.signature
        )?;
        writeln!(f, "{} {}", style("Result:").bold(), self.overview.result)?;
        writeln!(
            f,
            "{} {}",
            style("Timestamp:").bold(),
            self.overview.timestamp
        )?;
        writeln!(
            f,
            "{} {}",
            style("Confirmation Status:").bold(),
            self.overview.confirmation_status
        )?;
        writeln!(
            f,
            "{} {}",
            style("Confirmations:").bold(),
            self.overview.confirmations
        )?;
        writeln!(f, "{} {}", style("Slot:").bold(), self.overview.slot)?;
        writeln!(
            f,
            "{} {}",
            style("Recent Blockhash:").bold(),
            self.overview.recent_blockhash
        )?;
        writeln!(f, "{} {}", style("Fee:").bold(), self.overview.fee)?;

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
                self.transaction.signatures.len()
            ))
            .bold()
        )?;

        for (index, signature) in self.transaction.signatures.iter().enumerate() {
            writeln!(f, "  {:>2} {}", style(index).bold(), signature)?;
        }

        writeln!(f)?;

        writeln!(f, "{}", style("Message:").bold())?;

        writeln!(f, "  {}", style("Header:").bold())?;

        writeln!(
            f,
            "    {} {}",
            style("# of required signatures:").bold(),
            self.transaction.message.header.num_required_signatures
        )?;

        writeln!(
            f,
            "    {} {}",
            style("# of read-only signed accounts:").bold(),
            self.transaction.message.header.num_readonly_signed_accounts
        )?;

        writeln!(
            f,
            "    {} {}",
            style("# of read-only unsigned accounts:").bold(),
            self.transaction
                .message
                .header
                .num_readonly_unsigned_accounts
        )?;

        writeln!(
            f,
            "  {}",
            style(format!(
                "Account Keys ({}):",
                self.transaction.message.account_keys.len()
            ))
            .bold()
        )?;

        for (index, account_key) in self.transaction.message.account_keys.iter().enumerate() {
            writeln!(f, "   {:>2} {}", style(index).bold(), account_key)?;
        }

        writeln!(f, "  {}", style("Recent Blockhash:").bold())?;

        writeln!(f, "    {}", self.transaction.message.recent_blockhash)?;

        write!(
            f,
            "  {}",
            style(format!(
                "Instructions ({}):",
                self.transaction.message.instructions.len()
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
        ) in self.transaction.message.instructions.iter().enumerate()
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

        Ok(())
    }
}

pub struct TransactionFieldVisibility {
    slot: bool,
    block_time: bool,
    transaction: bool,
    meta: bool,
}

impl TransactionFieldVisibility {
    pub fn new_all_enabled() -> Self {
        Self {
            slot: true,
            block_time: true,
            transaction: true,
            meta: true,
        }
    }

    pub fn new_all_disabled() -> Self {
        Self {
            slot: false,
            block_time: false,
            transaction: false,
            meta: false,
        }
    }

    pub fn slot(&self) -> bool {
        self.slot
    }

    pub fn enable_slot(&mut self) -> &mut Self {
        self.slot = true;
        self
    }

    pub fn disable_slot(&mut self) -> &mut Self {
        self.slot = false;
        self
    }

    pub fn block_time(&self) -> bool {
        self.block_time
    }

    pub fn enable_block_time(&mut self) -> &mut Self {
        self.block_time = true;
        self
    }

    pub fn disable_block_time(&mut self) -> &mut Self {
        self.block_time = false;
        self
    }

    pub fn transaction(&self) -> bool {
        self.transaction
    }

    pub fn enable_transaction(&mut self) -> &mut Self {
        self.transaction = true;
        self
    }

    pub fn disable_transaction(&mut self) -> &mut Self {
        self.transaction = false;
        self
    }

    pub fn meta(&self) -> bool {
        self.meta
    }

    pub fn enable_meta(&mut self) -> &mut Self {
        self.meta = true;
        self
    }

    pub fn disable_meta(&mut self) -> &mut Self {
        self.meta = false;
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayInstruction {
    pub program: String,
    pub program_id: String,
    pub data: String,
    pub inner_instructions: Option<Vec<DisplayInstruction>>,
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
    pub overview: DisplayTransactionOverview,
    pub transaction: DisplayTransactionContent,
    pub log_messages: Option<Vec<String>>,
}

impl DisplayTransaction {
    pub fn from(
        transaction: &EncodedConfirmedTransactionWithStatusMeta,
        transaction_status: &TransactionStatus,
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
            let overview = DisplayTransactionOverview {
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
            };

            let mut fee_payer_found = false; // always first account
            let transaction = DisplayTransactionContent {
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
                    .enumerate()
                    .map(|(index, instruction)| DisplayInstruction {
                        program: "Unknown".to_string(),
                        program_id: message.account_keys[instruction.program_id_index as usize]
                            .to_string(),
                        data: bs58::encode(instruction.data.clone()).into_string(),
                        inner_instructions: None,
                    })
                    .collect(),
            };

            let log_messages = meta.as_ref().unwrap().log_messages.clone();

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

        writeln!(
            f,
            "{} {}",
            style("Signature:").bold(),
            self.overview.signature
        )?;
        writeln!(f, "{} {}", style("Result:").bold(), self.overview.result)?;
        writeln!(
            f,
            "{} {}",
            style("Timestamp:").bold(),
            self.overview.timestamp
        )?;
        writeln!(
            f,
            "{} {}",
            style("Confirmation Status:").bold(),
            self.overview.confirmation_status
        )?;
        writeln!(
            f,
            "{} {}",
            style("Confirmations:").bold(),
            self.overview.confirmations
        )?;
        writeln!(f, "{} {}", style("Slot:").bold(), self.overview.slot)?;
        writeln!(
            f,
            "{} {}",
            style("Recent Blockhash:").bold(),
            self.overview.recent_blockhash
        )?;
        writeln!(f, "{} {}", style("Fee:").bold(), self.overview.fee)?;

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
            style(format!("Accounts ({}):", self.transaction.accounts.len())).bold()
        )?;

        for (index, account) in self.transaction.accounts.iter().enumerate() {
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

        write!(
            f,
            "{}",
            style(format!(
                "Instructions ({}):",
                self.transaction.instructions.len()
            ))
            .bold()
        )?;

        for (index, instruction) in self.transaction.instructions.iter().enumerate() {
            writeln!(f)?;
            writeln!(
                f,
                " {:>2} {} {:<44}",
                style(index).bold(),
                style(&instruction.program).bold(),
                instruction.program_id
            )?;
        }

        //                 let mut raw = true;
        //                 if program_id == solana_vote_program::id() {
        //                     if let Ok(vote_instruction) = limited_deserialize::<
        //                         solana_vote_program::vote_instruction::VoteInstruction,
        //                     >(&instruction.data)
        //                     {
        //                         write!(f, "    {} {:?}", style("Data:").bold(), vote_instruction)?;
        //                         raw = false;
        //                     }
        //                 } else if program_id == stake::program::id() {
        //                     if let Ok(stake_instruction) = limited_deserialize::<
        //                         stake::instruction::StakeInstruction,
        //                     >(&instruction.data)
        //                     {
        //                         write!(f, "    {} {:?}", style("Data:").bold(), stake_instruction)?;
        //                         raw = false;
        //                     }
        //                 } else if program_id == system_program::id() {
        //                     if let Ok(system_instruction) = limited_deserialize::<
        //                         system_instruction::SystemInstruction,
        //                     >(&instruction.data)
        //                     {
        //                         write!(f, "    {} {:?}", style("Data:").bold(), system_instruction)?;
        //                         raw = false;
        //                     }
        //                 } else if program_id == spl_memo_v1_id() || program_id == spl_memo_id() {
        //                     if let Ok(s) = std::str::from_utf8(&instruction.data) {
        //                         write!(f, "    {} \"{}\"", style("Data:").bold(), s)?;
        //                         raw = false;
        //                     }
        //                 }

        //                 if raw {
        //                     write!(
        //                         f,
        //                         "    {} {:?}",
        //                         style("Data:").bold(),
        //                         bs58::encode(instruction.data.clone()).into_string()
        //                     )?;
        //                 }
        //             }

        if let Some(log_messages) = &self.log_messages {
            writeln!(f)?;
            writeln!(f)?;

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
