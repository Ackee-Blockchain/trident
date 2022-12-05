use self::{
    associated_token_account::parse_associated_token_account, bpf_loader::parse_bpf_loader,
    bpf_upgradeable_loader::parse_bpf_upgradeable_loader, memo::parse_memo, stake::parse_stake,
    system::parse_system, token::parse_token, vote::parse_vote,
};
use crate::transaction::{DisplayParsedInstruction, DisplayPartiallyParsedInstruction};
use phf::phf_map;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};
use thiserror::Error;

mod associated_token_account;
mod bpf_loader;
mod bpf_upgradeable_loader;
mod memo;
mod stake;
mod system;
mod token;
mod vote;

#[derive(Clone, Debug)]
pub enum ParsableProgram {
    System,
    BPFLoaderDeprecated,
    BPFLoader,
    BPFLoaderUpgradeable,
    Stake,
    Vote,
    SPLMemoV1,
    SPLMemo,
    SPLToken,
    SPLAssociatedTokenAccount,
}

static PARSABLE_PROGRAM_IDS: phf::Map<&'static str, ParsableProgram> = phf_map! {
     // System
     "11111111111111111111111111111111" => ParsableProgram::System,
     // BPF Loader Deprecated
     "BPFLoader1111111111111111111111111111111111" => ParsableProgram::BPFLoaderDeprecated,
     // BPF Loader
     "BPFLoader2111111111111111111111111111111111" => ParsableProgram::BPFLoader,
     // BPF Loader Upgradeable
     "BPFLoaderUpgradeab1e11111111111111111111111" => ParsableProgram::BPFLoaderUpgradeable,
     // Stake
     "Stake11111111111111111111111111111111111111" => ParsableProgram::Stake,
     // Vote
     "Vote111111111111111111111111111111111111111" => ParsableProgram::Vote,
     // SPL Memo v1
     "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo" => ParsableProgram::SPLMemoV1,
     // SPL Memo (current)
     "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr" => ParsableProgram::SPLMemo,
     // SPL Token
     "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => ParsableProgram::SPLToken,
     // SPL Associated Token Account
     "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => ParsableProgram::SPLAssociatedTokenAccount
};

#[derive(Error, Debug)]
pub enum ParseInstructionError {
    #[error("{0:?} instruction not parsable")]
    InstructionNotParsable(ParsableProgram),

    #[error("{0:?} instruction key mismatch")]
    InstructionKeyMismatch(ParsableProgram),

    #[error("Program not parsable")]
    ProgramNotParsable,

    #[error("Internal error, please report")]
    SerdeJsonError(#[from] serde_json::error::Error),
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ParsedInstructionEnum {
    #[serde(rename = "type")]
    pub instruction_type: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub info: Value,
}

pub fn parse(
    program_id: &Pubkey,
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<DisplayParsedInstruction, ParseInstructionError> {
    let program_name = PARSABLE_PROGRAM_IDS
        .get(&program_id.to_string())
        .ok_or(ParseInstructionError::ProgramNotParsable)?;

    let parsed_json = match program_name {
        ParsableProgram::System => serde_json::to_value(parse_system(instruction, account_keys)?)?,
        ParsableProgram::BPFLoaderDeprecated | ParsableProgram::BPFLoader => {
            serde_json::to_value(parse_bpf_loader(instruction, account_keys)?)?
        }
        ParsableProgram::BPFLoaderUpgradeable => {
            serde_json::to_value(parse_bpf_upgradeable_loader(instruction, account_keys)?)?
        }
        ParsableProgram::Stake => serde_json::to_value(parse_stake(instruction, account_keys)?)?,
        ParsableProgram::Vote => serde_json::to_value(parse_vote(instruction, account_keys)?)?,
        ParsableProgram::SPLMemoV1 | ParsableProgram::SPLMemo => {
            serde_json::to_value(parse_memo(instruction)?)?
        }
        ParsableProgram::SPLToken => serde_json::to_value(parse_token(instruction, account_keys)?)?,
        ParsableProgram::SPLAssociatedTokenAccount => {
            serde_json::to_value(parse_associated_token_account(instruction, account_keys)?)?
        }
    };

    Ok(DisplayParsedInstruction {
        program: format!("{program_name:?}"),
        program_id: program_id.to_string(),
        parsed: parsed_json,
    })
}

pub fn check_num_accounts(
    accounts: &[u8],
    num: usize,
    parsable_program: ParsableProgram,
) -> Result<(), ParseInstructionError> {
    if accounts.len() < num {
        Err(ParseInstructionError::InstructionKeyMismatch(
            parsable_program,
        ))
    } else {
        Ok(())
    }
}

pub fn partially_parse(
    program_id: &Pubkey,
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> DisplayPartiallyParsedInstruction {
    DisplayPartiallyParsedInstruction {
        program_id: program_id.to_string(),
        accounts: instruction
            .accounts
            .iter()
            .map(|&i| account_keys[i as usize].to_string())
            .collect(),
        data: bs58::encode(instruction.data.clone()).into_string(),
    }
}
