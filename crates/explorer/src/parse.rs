use crate::transaction::{DisplayParsedInstruction, DisplayPartiallyParsedInstruction};
use phf::phf_map;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};
use system_program::parse_system;
use thiserror::Error;

mod system_program;

#[derive(Clone, Debug)]
pub enum ParsableProgram {
    System,
    BPFLoaderDeprecated,
    BPFLoader,
    BPFLoaderUpgradeable,
}

static PARSABLE_PROGRAM_IDS: phf::Map<&'static str, ParsableProgram> = phf_map! {
     // System Program
     "11111111111111111111111111111111" => ParsableProgram::System,
     // BPF Loader Deprecated
     "BPFLoader1111111111111111111111111111111111" => ParsableProgram::BPFLoaderDeprecated,
     // BPF Loader
     "BPFLoader2111111111111111111111111111111111" => ParsableProgram::BPFLoader,
     // BPF Loader Upgradeable
     "BPFLoaderUpgradeab1e11111111111111111111111" => ParsableProgram::BPFLoaderUpgradeable,
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

pub struct ParsedInstruction {
    pub program: String,
    pub program_id: String,
    pub parsed: Value,
}

#[derive(Serialize)]
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
        _ => return Err(ParseInstructionError::ProgramNotParsable),
    };

    Ok(DisplayParsedInstruction {
        program: format!("{:?}", program_name),
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
