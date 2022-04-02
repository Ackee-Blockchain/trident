use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde_json::json;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};
use solana_vote_program::vote_instruction::VoteInstruction;

pub fn parse_vote(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let vote_instruction: VoteInstruction = bincode::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::Vote))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::Vote,
            ));
        }
    }
    match vote_instruction {
        VoteInstruction::InitializeAccount(vote_init) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Initialize".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Rent Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Node": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Authorized Voter": vote_init.authorized_voter.to_string(),
                    "Authorized Withdrawer": vote_init.authorized_withdrawer.to_string(),
                    "Commission": vote_init.commission,
                }),
            })
        }
        VoteInstruction::Authorize(new_authorized, authority_type) => {
            check_num_vote_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Authorize".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "New Authority": new_authorized.to_string(),
                    "Authority Type": authority_type,
                }),
            })
        }
        VoteInstruction::Vote(vote) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            let vote = json!({
                "Slots": vote.slots,
                "Hash": vote.hash.to_string(),
                "Timestamp": vote.timestamp,
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "Vote".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Slot Hashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Vote": vote,
                }),
            })
        }
        VoteInstruction::UpdateVoteState(vote_state_update) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            let vote_state_update = json!({
                "Lockouts": vote_state_update.lockouts,
                "Root": vote_state_update.root,
                "Hash": vote_state_update.hash.to_string(),
                "Timestamp": vote_state_update.timestamp,
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateVoteState".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Slot Hashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Vote State Update": vote_state_update,
                }),
            })
        }
        VoteInstruction::UpdateVoteStateSwitch(vote_state_update, hash) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            let vote_state_update = json!({
                "Lockouts": vote_state_update.lockouts,
                "Root": vote_state_update.root,
                "Hash": vote_state_update.hash.to_string(),
                "Timestamp": vote_state_update.timestamp,
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateVoteStateSwitch".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Slot Hashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Vote State Update": vote_state_update,
                    "Hash": hash.to_string(),
                }),
            })
        }
        VoteInstruction::Withdraw(lamports) => {
            check_num_vote_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Withdraw".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Withdraw Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Lamports": lamports,
                }),
            })
        }
        VoteInstruction::UpdateValidatorIdentity => {
            check_num_vote_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateValidatorIdentity".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "New Validator Identity": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Withdraw Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        VoteInstruction::UpdateCommission(commission) => {
            check_num_vote_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateCommission".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Withdraw Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Commission": commission,
                }),
            })
        }
        VoteInstruction::VoteSwitch(vote, hash) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            let vote = json!({
                "Slots": vote.slots,
                "Hash": vote.hash.to_string(),
                "Timestamp": vote.timestamp,
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "VoteSwitch".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Slot Hashes Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Vote": vote,
                    "Hash": hash.to_string(),
                }),
            })
        }
        VoteInstruction::AuthorizeChecked(authority_type) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeChecked".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "New Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Authority Type": authority_type,
                }),
            })
        }
    }
}

fn check_num_vote_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::Vote)
}
