use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde_json::json;
use solana_sdk::{
    instruction::CompiledInstruction, loader_upgradeable_instruction::UpgradeableLoaderInstruction,
    pubkey::Pubkey,
};

pub fn parse_bpf_upgradeable_loader(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let bpf_upgradeable_loader_instruction: UpgradeableLoaderInstruction =
        bincode::deserialize(&instruction.data).map_err(|_| {
            ParseInstructionError::InstructionNotParsable(ParsableProgram::BPFLoaderUpgradeable)
        })?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::BPFLoaderUpgradeable,
            ));
        }
    }
    match bpf_upgradeable_loader_instruction {
        UpgradeableLoaderInstruction::InitializeBuffer => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 1)?;
            let mut value = json!({
                "Account": account_keys[instruction.accounts[0] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() > 1 {
                map.insert(
                    "Authority".to_string(),
                    json!(account_keys[instruction.accounts[1] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "InitializeBuffer".to_string(),
                info: value,
            })
        }
        UpgradeableLoaderInstruction::Write { offset, bytes } => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Write".to_string(),
                info: json!({
                    "Offset": offset,
                    "Bytes": base64::encode(bytes),
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                }),
            })
        }
        UpgradeableLoaderInstruction::DeployWithMaxDataLen { max_data_len } => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 8)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "DeployWithMaxDataLen".to_string(),
                info: json!({
                    "MaxDataLen": max_data_len,
                    "Payer Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "ProgramData Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Program Account": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Buffer Account": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[4] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[5] as usize].to_string(),
                    "System Program": account_keys[instruction.accounts[6] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[7] as usize].to_string(),
                }),
            })
        }
        UpgradeableLoaderInstruction::Upgrade => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Upgrade".to_string(),
                info: json!({
                    "ProgramData Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Program Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Buffer Account": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Spill Account": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[4] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[5] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[6] as usize].to_string(),
                }),
            })
        }
        UpgradeableLoaderInstruction::SetAuthority => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "SetAuthority".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "NewAuthority": if instruction.accounts.len() > 2 {
                        Some(account_keys[instruction.accounts[2] as usize].to_string())
                    } else {
                        None
                    },
                }),
            })
        }
        UpgradeableLoaderInstruction::Close => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Close".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Recipient": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[2] as usize].to_string()
                }),
            })
        }
    }
}

fn check_num_bpf_upgradeable_loader_accounts(
    accounts: &[u8],
    num: usize,
) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::BPFLoaderUpgradeable)
}
