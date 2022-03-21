use crate::{
    error::ExplorerError,
    error::Result,
    output::{classify_account, write_styled, writeln_styled},
};
use chrono::{TimeZone, Utc};
use console::style;
use serde::Serialize;
use solana_sdk::{
    clock::Slot, message::VersionedMessage, program_utils::limited_deserialize,
    signature::Signature, stake, system_program, transaction::VersionedTransaction, system_instruction,
};
use solana_transaction_status::{
    Encodable, EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    UiCompiledInstruction, UiMessage, UiTransaction, UiTransactionEncoding,
    UiTransactionStatusMeta,
};
use spl_memo::{id as spl_memo_id, v1::id as spl_memo_v1_id};
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
