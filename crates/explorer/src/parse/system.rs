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
        SystemInstruction::UpgradeNonceAccount => todo!("What should be returned here?"),
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

#[cfg(test)]
mod test {
    use super::*;
    use solana_sdk::{message::Message, pubkey::Pubkey, system_instruction, sysvar};

    #[test]
    fn test_parse_system_create_account_ix() {
        let lamports = 55;
        let space = 128;
        let from_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();

        let instruction = system_instruction::create_account(
            &from_pubkey,
            &to_pubkey,
            lamports,
            space,
            &owner_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "CreateAccount".to_string(),
                info: json!({
                    "Source": from_pubkey.to_string(),
                    "New Account": to_pubkey.to_string(),
                    "Lamports": lamports,
                    "Owner": owner_pubkey.to_string(),
                    "Space": space,
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_system_assign_ix() {
        let account_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::assign(&account_pubkey, &owner_pubkey);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Assign".to_string(),
                info: json!({
                    "Account": account_pubkey.to_string(),
                    "Owner": owner_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &[]).is_err());
    }

    #[test]
    fn test_parse_system_transfer_ix() {
        let lamports = 55;
        let from_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Transfer".to_string(),
                info: json!({
                    "Source": from_pubkey.to_string(),
                    "Destination": to_pubkey.to_string(),
                    "Lamports": lamports,
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_system_create_account_with_seed_ix() {
        let lamports = 55;
        let space = 128;
        let seed = "test_seed";
        let from_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let base_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::create_account_with_seed(
            &from_pubkey,
            &to_pubkey,
            &base_pubkey,
            seed,
            lamports,
            space,
            &owner_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "CreateAccountWithSeed".to_string(),
                info: json!({
                    "Source": from_pubkey.to_string(),
                    "New Account": to_pubkey.to_string(),
                    "Lamports": lamports,
                    "Base": base_pubkey.to_string(),
                    "Seed": seed,
                    "Owner": owner_pubkey.to_string(),
                    "Space": space,
                }),
            }
        );

        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_system_allocate_ix() {
        let space = 128;
        let account_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::allocate(&account_pubkey, space);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Allocate".to_string(),
                info: json!({
                    "Account": account_pubkey.to_string(),
                    "Space": space,
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &[]).is_err());
    }

    #[test]
    fn test_parse_system_allocate_with_seed_ix() {
        let space = 128;
        let seed = "test_seed";
        let account_pubkey = Pubkey::new_unique();
        let base_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::allocate_with_seed(
            &account_pubkey,
            &base_pubkey,
            seed,
            space,
            &owner_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AllocateWithSeed".to_string(),
                info: json!({
                    "Account": account_pubkey.to_string(),
                    "Base": base_pubkey.to_string(),
                    "Seed": seed,
                    "Owner": owner_pubkey.to_string(),
                    "Space": space,
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_system_assign_with_seed_ix() {
        let seed = "test_seed";
        let account_pubkey = Pubkey::new_unique();
        let base_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::assign_with_seed(
            &account_pubkey,
            &base_pubkey,
            seed,
            &owner_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AssignWithSeed".to_string(),
                info: json!({
                    "Account": account_pubkey.to_string(),
                    "Base": base_pubkey.to_string(),
                    "Seed": seed,
                    "Owner": owner_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_system_transfer_with_seed_ix() {
        let lamports = 55;
        let seed = "test_seed";
        let from_pubkey = Pubkey::new_unique();
        let from_base_pubkey = Pubkey::new_unique();
        let from_owner_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::transfer_with_seed(
            &from_pubkey,
            &from_base_pubkey,
            seed.to_string(),
            &from_owner_pubkey,
            &to_pubkey,
            lamports,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "TransferWithSeed".to_string(),
                info: json!({
                    "Source": from_pubkey.to_string(),
                    "Source Base": from_base_pubkey.to_string(),
                    "Source Seed": seed,
                    "Source Owner": from_owner_pubkey.to_string(),
                    "Lamports": lamports,
                    "Destination": to_pubkey.to_string()
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_system_advance_nonce_account_ix() {
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();

        let instruction =
            system_instruction::advance_nonce_account(&nonce_pubkey, &authorized_pubkey);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AdvanceNonce".to_string(),
                info: json!({
                    "Nonce Account": nonce_pubkey.to_string(),
                    "Recent Blockhashes Sysvar": sysvar::recent_blockhashes::ID.to_string(),
                    "Nonce Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_system_withdraw_nonce_account_ix() {
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();

        let lamports = 55;
        let instruction = system_instruction::withdraw_nonce_account(
            &nonce_pubkey,
            &authorized_pubkey,
            &to_pubkey,
            lamports,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "WithdrawFromNonce".to_string(),
                info: json!({
                    "Nonce Account": nonce_pubkey.to_string(),
                    "Destination": to_pubkey.to_string(),
                    "Recent Blockhashes Sysvar": sysvar::recent_blockhashes::ID.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Nonce Authority": authorized_pubkey.to_string(),
                    "Lamports": lamports
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..4]).is_err());
    }

    #[test]
    fn test_parse_system_initialize_nonce_ix() {
        let lamports = 55;
        let from_pubkey = Pubkey::new_unique();
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();

        let instructions = system_instruction::create_nonce_account(
            &from_pubkey,
            &nonce_pubkey,
            &authorized_pubkey,
            lamports,
        );
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_system(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "InitializeNonce".to_string(),
                info: json!({
                    "Nonce Account": nonce_pubkey.to_string(),
                    "Recent Blockhashes Sysvar": sysvar::recent_blockhashes::ID.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Nonce Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_system(&message.instructions[1], &message.account_keys[0..3]).is_err());
    }

    #[test]
    fn test_parse_system_authorize_nonce_account_ix() {
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authority_pubkey = Pubkey::new_unique();

        let instruction = system_instruction::authorize_nonce_account(
            &nonce_pubkey,
            &authorized_pubkey,
            &new_authority_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeNonce".to_string(),
                info: json!({
                    "Nonce Account": nonce_pubkey.to_string(),
                    "New Authorized": new_authority_pubkey.to_string(),
                    "Nonce Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }
}
