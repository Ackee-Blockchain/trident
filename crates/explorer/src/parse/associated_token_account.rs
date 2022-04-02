use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde_json::json;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

pub fn parse_associated_token_account(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::SPLAssociatedTokenAccount,
            ));
        }
    }
    check_num_associated_token_accounts(&instruction.accounts, 7)?;
    Ok(ParsedInstructionEnum {
        instruction_type: "Create".to_string(),
        info: json!({
            "Source": account_keys[instruction.accounts[0] as usize].to_string(),
            "Account": account_keys[instruction.accounts[1] as usize].to_string(),
            "Wallet": account_keys[instruction.accounts[2] as usize].to_string(),
            "Mint": account_keys[instruction.accounts[3] as usize].to_string(),
            "System Program": account_keys[instruction.accounts[4] as usize].to_string(),
            "Token Program": account_keys[instruction.accounts[5] as usize].to_string(),
            "Rent Sysvar": account_keys[instruction.accounts[6] as usize].to_string(),
        }),
    })
}

fn check_num_associated_token_accounts(
    accounts: &[u8],
    num: usize,
) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::SPLAssociatedTokenAccount)
}
