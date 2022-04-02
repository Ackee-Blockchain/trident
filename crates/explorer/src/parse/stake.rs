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
                    "StakeAuthority": account_keys[instruction.accounts[5] as usize].to_string(),
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
