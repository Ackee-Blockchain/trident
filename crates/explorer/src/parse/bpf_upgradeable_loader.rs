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
                    "New Authority": if instruction.accounts.len() > 2 {
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
        UpgradeableLoaderInstruction::ExtendProgram { additional_bytes } => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "ExtendProgram".to_string(),
                info: json!({
                    "AdditionalBytes": additional_bytes,
                    "ProgramData Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Program Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "System Program": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Payer Account": account_keys[instruction.accounts[3] as usize].to_string(),
                }),
            })
        }
        UpgradeableLoaderInstruction::SetAuthorityChecked => {
            check_num_bpf_upgradeable_loader_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "SetAuthorityChecked".to_string(),
                info: json!({
                    "Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "New Authority": account_keys[instruction.accounts[2] as usize].to_string()
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

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;
    use solana_sdk::{
        bpf_loader_upgradeable, message::Message, pubkey::Pubkey, system_program, sysvar,
    };

    #[test]
    fn test_parse_bpf_upgradeable_loader_create_buffer_ix() {
        let max_data_len = 54321;

        let payer_address = Pubkey::new_unique();
        let buffer_address = Pubkey::new_unique();
        let authority_address = Pubkey::new_unique();
        let instructions = bpf_loader_upgradeable::create_buffer(
            &payer_address,
            &buffer_address,
            &authority_address,
            55,
            max_data_len,
        )
        .unwrap();
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "InitializeBuffer".to_string(),
                info: json!({
                    "Account": buffer_address.to_string(),
                    "Authority": authority_address.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[1],
            &message.account_keys[0..2]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_write_ix() {
        let offset = 4242;
        let bytes = vec![8; 99];

        let buffer_address = Pubkey::new_unique();
        let authority_address = Pubkey::new_unique();
        let instruction = bpf_loader_upgradeable::write(
            &buffer_address,
            &authority_address,
            offset,
            bytes.clone(),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Write".to_string(),
                info: json!({
                    "Offset": offset,
                    "Bytes": base64::encode(&bytes),
                    "Account": buffer_address.to_string(),
                    "Authority": authority_address.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..1]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_deploy_ix() {
        let max_data_len = 54321;

        let payer_address = Pubkey::new_unique();
        let program_address = Pubkey::new_unique();
        let buffer_address = Pubkey::new_unique();
        let upgrade_authority_address = Pubkey::new_unique();
        let programdata_address = Pubkey::find_program_address(
            &[program_address.as_ref()],
            &bpf_loader_upgradeable::id(),
        )
        .0;
        let instructions = bpf_loader_upgradeable::deploy_with_max_program_len(
            &payer_address,
            &program_address,
            &buffer_address,
            &upgrade_authority_address,
            55,
            max_data_len,
        )
        .unwrap();
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "DeployWithMaxDataLen".to_string(),
                info: json!({
                    "MaxDataLen": max_data_len,
                    "Payer Account": payer_address.to_string(),
                    "Program Account": program_address.to_string(),
                    "Authority": upgrade_authority_address.to_string(),
                    "ProgramData Account": programdata_address.to_string(),
                    "Buffer Account": buffer_address.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "System Program": system_program::ID.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[1],
            &message.account_keys[0..7]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_upgrade_ix() {
        let program_address = Pubkey::new_unique();
        let buffer_address = Pubkey::new_unique();
        let authority_address = Pubkey::new_unique();
        let spill_address = Pubkey::new_unique();
        let programdata_address = Pubkey::find_program_address(
            &[program_address.as_ref()],
            &bpf_loader_upgradeable::id(),
        )
        .0;
        let instruction = bpf_loader_upgradeable::upgrade(
            &program_address,
            &buffer_address,
            &authority_address,
            &spill_address,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Upgrade".to_string(),
                info: json!({
                    "Authority": authority_address.to_string(),
                    "ProgramData Account": programdata_address.to_string(),
                    "Program Account": program_address.to_string(),
                    "Buffer Account": buffer_address.to_string(),
                    "Spill Account": spill_address.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..6]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_set_buffer_authority_ix() {
        let buffer_address = Pubkey::new_unique();
        let current_authority_address = Pubkey::new_unique();
        let new_authority_address = Pubkey::new_unique();
        let instruction = bpf_loader_upgradeable::set_buffer_authority(
            &buffer_address,
            &current_authority_address,
            &new_authority_address,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetAuthority".to_string(),
                info: json!({
                    "Account": buffer_address.to_string(),
                    "Authority": current_authority_address.to_string(),
                    "New Authority": new_authority_address.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..1]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_set_upgrade_authority_ix() {
        let program_address = Pubkey::new_unique();
        let current_authority_address = Pubkey::new_unique();
        let new_authority_address = Pubkey::new_unique();
        let (programdata_address, _) = Pubkey::find_program_address(
            &[program_address.as_ref()],
            &bpf_loader_upgradeable::id(),
        );
        let instruction = bpf_loader_upgradeable::set_upgrade_authority(
            &program_address,
            &current_authority_address,
            Some(&new_authority_address),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetAuthority".to_string(),
                info: json!({
                    "Account": programdata_address.to_string(),
                    "Authority": current_authority_address.to_string(),
                    "New Authority": new_authority_address.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..1]
        )
        .is_err());

        let instruction = bpf_loader_upgradeable::set_upgrade_authority(
            &program_address,
            &current_authority_address,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetAuthority".to_string(),
                info: json!({
                    "Account": programdata_address.to_string(),
                    "Authority": current_authority_address.to_string(),
                    "New Authority": Value::Null,
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..1]
        )
        .is_err());
    }

    #[test]
    fn test_parse_bpf_upgradeable_loader_close_ix() {
        let close_address = Pubkey::new_unique();
        let recipient_address = Pubkey::new_unique();
        let authority_address = Pubkey::new_unique();
        let instruction =
            bpf_loader_upgradeable::close(&close_address, &recipient_address, &authority_address);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_bpf_upgradeable_loader(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Close".to_string(),
                info: json!({
                    "Account": close_address.to_string(),
                    "Recipient": recipient_address.to_string(),
                    "Authority": authority_address.to_string(),
                }),
            }
        );
        assert!(parse_bpf_upgradeable_loader(
            &message.instructions[0],
            &message.account_keys[0..1]
        )
        .is_err());
    }
}
