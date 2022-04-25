use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde_json::{json, Map};
use solana_sdk::{
    instruction::CompiledInstruction, pubkey::Pubkey, stake::instruction::StakeInstruction,
};

pub fn parse_stake(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let stake_instruction: StakeInstruction = bincode::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::Stake))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::Stake,
            ));
        }
    }
    match stake_instruction {
        StakeInstruction::Initialize(authorized, lockup) => {
            check_num_stake_accounts(&instruction.accounts, 2)?;
            let authorized = json!({
                "Staker": authorized.staker.to_string(),
                "Withdrawer": authorized.withdrawer.to_string(),
            });
            let lockup = json!({
                "Unix Timestamp": lockup.unix_timestamp,
                "Epoch": lockup.epoch,
                "Custodian": lockup.custodian.to_string(),
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "Initialize".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authorized": authorized,
                    "Lockup": lockup,
                }),
            })
        }
        StakeInstruction::Authorize(new_authorized, authority_type) => {
            check_num_stake_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                "New Authority": new_authorized.to_string(),
                "Authority Type": authority_type,
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() >= 4 {
                map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[3] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "Authorize".to_string(),
                info: value,
            })
        }
        StakeInstruction::DelegateStake => {
            check_num_stake_accounts(&instruction.accounts, 6)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Delegate".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Vote Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Stake History Sysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Stake Config Account": account_keys[instruction.accounts[4] as usize].to_string(),
                    "Stake Authority": account_keys[instruction.accounts[5] as usize].to_string(),
                }),
            })
        }
        StakeInstruction::Split(lamports) => {
            check_num_stake_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Split".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "New Split Account": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Stake Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Lamports": lamports,
                }),
            })
        }
        StakeInstruction::Withdraw(lamports) => {
            check_num_stake_accounts(&instruction.accounts, 5)?;
            let mut value = json!({
                "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                "Destination": account_keys[instruction.accounts[1] as usize].to_string(),
                "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                "Stake History Sysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                "Withdraw Authority": account_keys[instruction.accounts[4] as usize].to_string(),
                "Lamports": lamports,
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() >= 6 {
                map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[5] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "Withdraw".to_string(),
                info: value,
            })
        }
        StakeInstruction::Deactivate => {
            check_num_stake_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Deactivate".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Stake Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        StakeInstruction::SetLockup(lockup_args) => {
            check_num_stake_accounts(&instruction.accounts, 2)?;
            let mut lockup_map = Map::new();
            if let Some(timestamp) = lockup_args.unix_timestamp {
                lockup_map.insert("Unix Timestamp".to_string(), json!(timestamp));
            }
            if let Some(epoch) = lockup_args.epoch {
                lockup_map.insert("Epoch".to_string(), json!(epoch));
            }
            if let Some(custodian) = lockup_args.custodian {
                lockup_map.insert("Custodian".to_string(), json!(custodian.to_string()));
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "SetLockup".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Custodian": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Lockup": lockup_map,
                }),
            })
        }
        StakeInstruction::Merge => {
            check_num_stake_accounts(&instruction.accounts, 5)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Merge".to_string(),
                info: json!({
                    "Destination": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Source": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Stake History Sysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Stake Authority": account_keys[instruction.accounts[4] as usize].to_string(),
                }),
            })
        }
        StakeInstruction::AuthorizeWithSeed(args) => {
            check_num_stake_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Authority Base": account_keys[instruction.accounts[1] as usize].to_string(),
                    "New Authorized": args.new_authorized_pubkey.to_string(),
                    "Authority Type": args.stake_authorize,
                    "Authority Seed": args.authority_seed,
                    "Authority Owner": args.authority_owner.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() >= 3 {
                map.insert(
                    "Clock Sysvar".to_string(),
                    json!(account_keys[instruction.accounts[2] as usize].to_string()),
                );
            }
            if instruction.accounts.len() >= 4 {
                map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[3] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeWithSeed".to_string(),
                info: value,
            })
        }
        StakeInstruction::InitializeChecked => {
            check_num_stake_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "InitializeChecked".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Staker": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Withdrawer": account_keys[instruction.accounts[3] as usize].to_string(),
                }),
            })
        }
        StakeInstruction::AuthorizeChecked(authority_type) => {
            check_num_stake_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                "New Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                "Authority Type": authority_type,
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() >= 5 {
                map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[4] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeChecked".to_string(),
                info: value,
            })
        }
        StakeInstruction::AuthorizeCheckedWithSeed(args) => {
            check_num_stake_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Authority Base": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "New Authorized": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Authority Type": args.stake_authorize,
                    "Authority Seed": args.authority_seed,
                    "Authority Owner": args.authority_owner.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            if instruction.accounts.len() >= 5 {
                map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[4] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeCheckedWithSeed".to_string(),
                info: value,
            })
        }
        StakeInstruction::SetLockupChecked(lockup_args) => {
            check_num_stake_accounts(&instruction.accounts, 2)?;
            let mut lockup_map = Map::new();
            if let Some(timestamp) = lockup_args.unix_timestamp {
                lockup_map.insert("Unix Timestamp".to_string(), json!(timestamp));
            }
            if let Some(epoch) = lockup_args.epoch {
                lockup_map.insert("Epoch".to_string(), json!(epoch));
            }
            if instruction.accounts.len() >= 3 {
                lockup_map.insert(
                    "Custodian".to_string(),
                    json!(account_keys[instruction.accounts[2] as usize].to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "SetLockupChecked".to_string(),
                info: json!({
                    "Stake Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Custodian": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Lockup": lockup_map,
                }),
            })
        }
    }
}

fn check_num_stake_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::Stake)
}

#[cfg(test)]
mod test {
    use {
        super::*,
        solana_sdk::{
            message::Message,
            pubkey::Pubkey,
            stake::{
                config,
                instruction::{self, LockupArgs},
                state::{Authorized, Lockup, StakeAuthorize},
            },
            sysvar,
        },
    };

    #[test]
    fn test_parse_stake_initialize_ix() {
        let from_pubkey = Pubkey::new_unique();
        let stake_pubkey = Pubkey::new_unique();
        let authorized = Authorized {
            staker: Pubkey::new_unique(),
            withdrawer: Pubkey::new_unique(),
        };
        let lockup = Lockup {
            unix_timestamp: 1_234_567_890,
            epoch: 11,
            custodian: Pubkey::new_unique(),
        };
        let lamports = 55;

        let instructions = instruction::create_account(
            &from_pubkey,
            &stake_pubkey,
            &authorized,
            &lockup,
            lamports,
        );
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_stake(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Initialize".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Authorized": {
                        "Staker": authorized.staker.to_string(),
                        "Withdrawer": authorized.withdrawer.to_string(),
                    },
                    "Lockup": {
                        "Unix Timestamp": lockup.unix_timestamp,
                        "Epoch": lockup.epoch,
                        "Custodian": lockup.custodian.to_string(),
                    }
                }),
            }
        );
        assert!(parse_stake(&message.instructions[1], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_stake_authorize_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let custodian_pubkey = Pubkey::new_unique();
        let instruction = instruction::authorize(
            &stake_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Staker,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Authorize".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": StakeAuthorize::Staker,
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..2]).is_err());

        let instruction = instruction::authorize(
            &stake_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Withdrawer,
            Some(&custodian_pubkey),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Authorize".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": StakeAuthorize::Withdrawer,
                    "Custodian": custodian_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_stake_delegate_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let vote_pubkey = Pubkey::new_unique();
        let instruction =
            instruction::delegate_stake(&stake_pubkey, &authorized_pubkey, &vote_pubkey);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Delegate".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Vote Account": vote_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Stake History Sysvar": sysvar::stake_history::ID.to_string(),
                    "Stake Config Account": config::ID.to_string(),
                    "Stake Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..5]).is_err());
    }

    #[test]
    fn test_parse_stake_split_ix() {
        let lamports = 55;
        let stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let split_stake_pubkey = Pubkey::new_unique();
        let instructions = instruction::split(
            &stake_pubkey,
            &authorized_pubkey,
            lamports,
            &split_stake_pubkey,
        );
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_stake(&message.instructions[2], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Split".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "New Split Account": split_stake_pubkey.to_string(),
                    "Stake Authority": authorized_pubkey.to_string(),
                    "Lamports": lamports,
                }),
            }
        );
        assert!(parse_stake(&message.instructions[2], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_stake_withdraw_ix() {
        let lamports = 55;
        let stake_pubkey = Pubkey::new_unique();
        let withdrawer_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let custodian_pubkey = Pubkey::new_unique();
        let instruction = instruction::withdraw(
            &stake_pubkey,
            &withdrawer_pubkey,
            &to_pubkey,
            lamports,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Withdraw".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Destination": to_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Stake History Sysvar": sysvar::stake_history::ID.to_string(),
                    "Withdraw Authority": withdrawer_pubkey.to_string(),
                    "Lamports": lamports,
                }),
            }
        );
        let instruction = instruction::withdraw(
            &stake_pubkey,
            &withdrawer_pubkey,
            &to_pubkey,
            lamports,
            Some(&custodian_pubkey),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Withdraw".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Destination": to_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Stake History Sysvar": sysvar::stake_history::ID.to_string(),
                    "Withdraw Authority": withdrawer_pubkey.to_string(),
                    "Custodian": custodian_pubkey.to_string(),
                    "Lamports": lamports,
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..4]).is_err());
    }

    #[test]
    fn test_parse_stake_deactivate_stake_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let instruction = instruction::deactivate_stake(&stake_pubkey, &authorized_pubkey);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Deactivate".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Stake Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_stake_merge_ix() {
        let destination_stake_pubkey = Pubkey::new_unique();
        let source_stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let instructions = instruction::merge(
            &destination_stake_pubkey,
            &source_stake_pubkey,
            &authorized_pubkey,
        );
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Merge".to_string(),
                info: json!({
                    "Destination": destination_stake_pubkey.to_string(),
                    "Source": source_stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Stake History Sysvar": sysvar::stake_history::ID.to_string(),
                    "Stake Authority": authorized_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..4]).is_err());
    }

    #[test]
    fn test_parse_stake_authorize_with_seed_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authority_base_pubkey = Pubkey::new_unique();
        let authority_owner_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let custodian_pubkey = Pubkey::new_unique();

        let seed = "test_seed";
        let instruction = instruction::authorize_with_seed(
            &stake_pubkey,
            &authority_base_pubkey,
            seed.to_string(),
            &authority_owner_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Staker,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeWithSeed".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Authority Owner": authority_owner_pubkey.to_string(),
                    "New Authorized": new_authorized_pubkey.to_string(),
                    "Authority Base": authority_base_pubkey.to_string(),
                    "Authority Seed": seed,
                    "Authority Type": StakeAuthorize::Staker,
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..2]).is_err());

        let instruction = instruction::authorize_with_seed(
            &stake_pubkey,
            &authority_base_pubkey,
            seed.to_string(),
            &authority_owner_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Withdrawer,
            Some(&custodian_pubkey),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeWithSeed".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Authority Owner": authority_owner_pubkey.to_string(),
                    "New Authorized": new_authorized_pubkey.to_string(),
                    "Authority Base": authority_base_pubkey.to_string(),
                    "Authority Seed": seed,
                    "Authority Type": StakeAuthorize::Withdrawer,
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Custodian": custodian_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..3]).is_err());
    }

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_parse_stake_set_lockup() {
        let mut keys: Vec<Pubkey> = vec![];
        for _ in 0..3 {
            keys.push(Pubkey::new_unique());
        }
        let unix_timestamp = 1_234_567_890;
        let epoch = 11;
        let custodian = Pubkey::new_unique();

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: None,
            custodian: None,
        };
        let instruction = instruction::set_lockup(&keys[1], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockup".to_string(),
                info: json!({
                    "Stake Account": keys[1].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp
                    }
                }),
            }
        );

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: Some(epoch),
            custodian: None,
        };
        let instruction = instruction::set_lockup(&keys[1], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockup".to_string(),
                info: json!({
                    "Stake Account": keys[1].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp,
                        "Epoch": epoch,
                    }
                }),
            }
        );

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: Some(epoch),
            custodian: Some(custodian),
        };
        let instruction = instruction::set_lockup(&keys[1], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockup".to_string(),
                info: json!({
                    "Stake Account": keys[1].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp,
                        "Epoch": epoch,
                        "Custodian": custodian.to_string(),
                    }
                }),
            }
        );

        assert!(parse_stake(&message.instructions[0], &keys[0..1]).is_err());

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: None,
            custodian: None,
        };
        let instruction = instruction::set_lockup_checked(&keys[1], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockupChecked".to_string(),
                info: json!({
                    "Stake Account": keys[1].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp
                    }
                }),
            }
        );

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: Some(epoch),
            custodian: None,
        };
        let instruction = instruction::set_lockup_checked(&keys[1], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockupChecked".to_string(),
                info: json!({
                    "Stake Account": keys[1].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp,
                        "Epoch": epoch,
                    }
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &keys[0..1]).is_err());

        let lockup = LockupArgs {
            unix_timestamp: Some(unix_timestamp),
            epoch: Some(epoch),
            custodian: Some(keys[1]),
        };
        let instruction = instruction::set_lockup_checked(&keys[2], &lockup, &keys[0]);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &keys[0..3]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "SetLockupChecked".to_string(),
                info: json!({
                    "Stake Account": keys[2].to_string(),
                    "Custodian": keys[0].to_string(),
                    "Lockup": {
                        "Unix Timestamp": unix_timestamp,
                        "Epoch": epoch,
                        "Custodian": keys[1].to_string(),
                    }
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_stake_create_account_checked_ix() {
        let from_pubkey = Pubkey::new_unique();
        let stake_pubkey = Pubkey::new_unique();

        let authorized = Authorized {
            staker: Pubkey::new_unique(),
            withdrawer: Pubkey::new_unique(),
        };
        let lamports = 55;

        let instructions =
            instruction::create_account_checked(&from_pubkey, &stake_pubkey, &authorized, lamports);
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_stake(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "InitializeChecked".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Staker": authorized.staker.to_string(),
                    "Withdrawer": authorized.withdrawer.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[1], &message.account_keys[0..3]).is_err());
    }

    #[test]
    fn test_parse_stake_authorize_checked_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let custodian_pubkey = Pubkey::new_unique();

        let instruction = instruction::authorize_checked(
            &stake_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Staker,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeChecked".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": StakeAuthorize::Staker,
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..3]).is_err());

        let instruction = instruction::authorize_checked(
            &stake_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Withdrawer,
            Some(&custodian_pubkey),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeChecked".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": StakeAuthorize::Withdrawer,
                    "Custodian": custodian_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..4]).is_err());
    }

    #[test]
    fn test_parse_stake_authorize_checked_with_seed_ix() {
        let stake_pubkey = Pubkey::new_unique();
        let authority_base_pubkey = Pubkey::new_unique();
        let authority_owner_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let custodian_pubkey = Pubkey::new_unique();

        let seed = "test_seed";
        let instruction = instruction::authorize_checked_with_seed(
            &stake_pubkey,
            &authority_base_pubkey,
            seed.to_string(),
            &authority_owner_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Staker,
            None,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeCheckedWithSeed".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Authority Owner": authority_owner_pubkey.to_string(),
                    "New Authorized": new_authorized_pubkey.to_string(),
                    "Authority Base": authority_base_pubkey.to_string(),
                    "Authority Seed": seed,
                    "Authority Type": StakeAuthorize::Staker,
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..3]).is_err());

        let instruction = instruction::authorize_checked_with_seed(
            &stake_pubkey,
            &authority_base_pubkey,
            seed.to_string(),
            &authority_owner_pubkey,
            &new_authorized_pubkey,
            StakeAuthorize::Withdrawer,
            Some(&custodian_pubkey),
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_stake(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeCheckedWithSeed".to_string(),
                info: json!({
                    "Stake Account": stake_pubkey.to_string(),
                    "Authority Owner": authority_owner_pubkey.to_string(),
                    "New Authorized": new_authorized_pubkey.to_string(),
                    "Authority Base": authority_base_pubkey.to_string(),
                    "Authority Seed": seed,
                    "Authority Type": StakeAuthorize::Withdrawer,
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Custodian": custodian_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_stake(&message.instructions[0], &message.account_keys[0..4]).is_err());
    }
}
