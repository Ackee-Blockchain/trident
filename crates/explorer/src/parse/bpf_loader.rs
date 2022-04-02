use crate::parse::{ParsableProgram, ParseInstructionError, ParsedInstructionEnum};
use serde_json::json;
use solana_sdk::{
    instruction::CompiledInstruction, loader_instruction::LoaderInstruction, pubkey::Pubkey,
};

pub fn parse_bpf_loader(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let bpf_loader_instruction: LoaderInstruction = bincode::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::BPFLoader))?;
    if instruction.accounts.is_empty() || instruction.accounts[0] as usize >= account_keys.len() {
        return Err(ParseInstructionError::InstructionKeyMismatch(
            ParsableProgram::BPFLoader,
        ));
    }
    match bpf_loader_instruction {
        LoaderInstruction::Write { offset, bytes } => Ok(ParsedInstructionEnum {
            instruction_type: "Write".to_string(),
            info: json!({
                "Offset": offset,
                "Bytes": base64::encode(bytes),
                "Account": account_keys[instruction.accounts[0] as usize].to_string(),
            }),
        }),
        LoaderInstruction::Finalize => Ok(ParsedInstructionEnum {
            instruction_type: "Finalize".to_string(),
            info: json!({
                "Account": account_keys[instruction.accounts[0] as usize].to_string(),
            }),
        }),
    }
}
