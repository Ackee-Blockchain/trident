use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde_json::json;
use solana_sdk::{
    instruction::CompiledInstruction, pubkey::Pubkey, system_instruction::SystemInstruction,
};

pub fn parse_system(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let system_instruction: SystemInstruction = bincode::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::System))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::System,
            ));
        }
    }
    match system_instruction {
        SystemInstruction::CreateAccount {
            lamports,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateAccount".to_string(),
                info: json!({
                    "Source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "New Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Lamports": lamports,
                    "Space": space,
                    "Owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::Assign { owner } => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Assign".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::Transfer { lamports } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Transfer".to_string(),
                info: json!({
                    "Source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Lamports": lamports,
                }),
            })
        }
        SystemInstruction::CreateAccountWithSeed {
            base,
            seed,
            lamports,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateAccountWithSeed".to_string(),
                info: json!({
                    "Source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "New Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Base": base.to_string(),
                    "Seed": seed,
                    "Lamports": lamports,
                    "Space": space,
                    "Owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::AdvanceNonceAccount => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AdvanceNonce".to_string(),
                info: json!({
                    "Nonce Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Recent Blockhashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Nonce Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        SystemInstruction::WithdrawNonceAccount(lamports) => {
            check_num_system_accounts(&instruction.accounts, 5)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "WithdrawFromNonce".to_string(),
                info: json!({
                    "Nonce Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Recent Blockhashes Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Nonce Authority": account_keys[instruction.accounts[4] as usize].to_string(),
                    "Lamports": lamports,
                }),
            })
        }
        SystemInstruction::InitializeNonceAccount(authority) => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "InitializeNonce".to_string(),
                info: json!({
                    "Nonce Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Recent Blockhashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Nonce Authority": authority.to_string(),
                }),
            })
        }
        SystemInstruction::AuthorizeNonceAccount(authority) => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeNonce".to_string(),
                info: json!({
                    "Nonce Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Nonce Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "New Authorized": authority.to_string(),
                }),
            })
        }
        SystemInstruction::Allocate { space } => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Allocate".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Space": space,
                }),
            })
        }
        SystemInstruction::AllocateWithSeed {
            base,
            seed,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AllocateWithSeed".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Base": base.to_string(),
                    "Seed": seed,
                    "Space": space,
                    "Owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::AssignWithSeed { base, seed, owner } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AssignWithSeed".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Base": base.to_string(),
                    "Seed": seed,
                    "Owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::TransferWithSeed {
            lamports,
            from_seed,
            from_owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "TransferWithSeed".to_string(),
                info: json!({
                    "Source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Source Base": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Destination": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Lamports": lamports,
                    "Source Seed": from_seed,
                    "Source Owner": from_owner.to_string(),
                }),
            })
        }
    }
}

fn check_num_system_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::System)
}
