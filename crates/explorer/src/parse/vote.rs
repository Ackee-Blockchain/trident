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
        VoteInstruction::UpdateVoteState(state) => {
            check_num_vote_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateVoteState".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "State Hash": state.hash.to_string(),
                }),
            })
        }
        VoteInstruction::UpdateVoteStateSwitch(state, hash) => {
            check_num_vote_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateVoteStateSwitch".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Hash": hash.to_string(),
                    "State Hash": state.hash.to_string(),
                }),
            })
        }
        VoteInstruction::AuthorizeWithSeed(authority_type) => {
            check_num_vote_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeWithSeed".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "Authority Type": authority_type,
                }),
            })
        }
        VoteInstruction::AuthorizeCheckedWithSeed(authority_type) => {
            check_num_vote_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "AuthorizeCheckedWithSeed".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Clock Sysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Authority": account_keys[instruction.accounts[2] as usize].to_string(),
                    "New Authority": account_keys[instruction.accounts[3] as usize].to_string(),
                    "Authority Type": authority_type,
                }),
            })
        },
        VoteInstruction::CompactUpdateVoteState(state) => {
            check_num_vote_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CompactUpdateVoteState".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "State Hash": state.hash.to_string(),
                }),
            })
        },
        VoteInstruction::CompactUpdateVoteStateSwitch(state, hash) => {
            check_num_vote_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CompactUpdateVoteStateSwitch".to_string(),
                info: json!({
                    "Vote Account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "Vote Authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "Hash": hash.to_string(),
                    "State Hash": state.hash.to_string(),
                }),
            })
        },
    }
}

fn check_num_vote_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::Vote)
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_sdk::{hash::Hash, message::Message, pubkey::Pubkey, sysvar};
    use solana_vote_program::{
        vote_instruction,
        vote_state::{Vote, VoteAuthorize, VoteInit},
    };

    #[test]
    fn test_parse_vote_initialize_ix() {
        let lamports = 55;

        let commission = 10;
        let node_pubkey = Pubkey::new_unique();
        let vote_pubkey = Pubkey::new_unique();
        let authorized_voter = Pubkey::new_unique();
        let authorized_withdrawer = Pubkey::new_unique();
        let vote_init = VoteInit {
            node_pubkey,
            authorized_voter,
            authorized_withdrawer,
            commission,
        };

        let instructions = vote_instruction::create_account(
            &Pubkey::new_unique(),
            &vote_pubkey,
            &vote_init,
            lamports,
        );
        let message = Message::new(&instructions, None);
        assert_eq!(
            parse_vote(&message.instructions[1], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Initialize".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Rent Sysvar": sysvar::rent::ID.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Node": node_pubkey.to_string(),
                    "Authorized Voter": authorized_voter.to_string(),
                    "Authorized Withdrawer": authorized_withdrawer.to_string(),
                    "Commission": commission,
                }),
            }
        );
        assert!(parse_vote(&message.instructions[1], &message.account_keys[0..3]).is_err());
    }

    #[test]
    fn test_parse_vote_authorize_ix() {
        let vote_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let authority_type = VoteAuthorize::Voter;
        let instruction = vote_instruction::authorize(
            &vote_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            authority_type,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Authorize".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": authority_type,
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_vote_ix() {
        let hash = Hash::new_from_array([1; 32]);
        let vote = Vote {
            slots: vec![1, 2, 4],
            hash,
            timestamp: Some(1_234_567_890),
        };

        let vote_pubkey = Pubkey::new_unique();
        let authorized_voter_pubkey = Pubkey::new_unique();
        let instruction = vote_instruction::vote(&vote_pubkey, &authorized_voter_pubkey, vote);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Vote".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Slot Hashes Sysvar": sysvar::slot_hashes::ID.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Vote Authority": authorized_voter_pubkey.to_string(),
                    "Vote": {
                        "Slots": [1, 2, 4],
                        "Hash": hash.to_string(),
                        "Timestamp": 1_234_567_890,
                    },
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..3]).is_err());
    }

    #[test]
    fn test_parse_vote_withdraw_ix() {
        let lamports = 55;
        let vote_pubkey = Pubkey::new_unique();
        let authorized_withdrawer_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let instruction = vote_instruction::withdraw(
            &vote_pubkey,
            &authorized_withdrawer_pubkey,
            lamports,
            &to_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "Withdraw".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Destination": to_pubkey.to_string(),
                    "Withdraw Authority": authorized_withdrawer_pubkey.to_string(),
                    "Lamports": lamports,
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_vote_update_validator_identity_ix() {
        let vote_pubkey = Pubkey::new_unique();
        let authorized_withdrawer_pubkey = Pubkey::new_unique();
        let node_pubkey = Pubkey::new_unique();
        let instruction = vote_instruction::update_validator_identity(
            &vote_pubkey,
            &authorized_withdrawer_pubkey,
            &node_pubkey,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "UpdateValidatorIdentity".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "New Validator Identity": node_pubkey.to_string(),
                    "Withdraw Authority": authorized_withdrawer_pubkey.to_string(),
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..2]).is_err());
    }

    #[test]
    fn test_parse_vote_update_commission_ix() {
        let commission = 10;
        let vote_pubkey = Pubkey::new_unique();
        let authorized_withdrawer_pubkey = Pubkey::new_unique();
        let instruction = vote_instruction::update_commission(
            &vote_pubkey,
            &authorized_withdrawer_pubkey,
            commission,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "UpdateCommission".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Withdraw Authority": authorized_withdrawer_pubkey.to_string(),
                    "Commission": commission,
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..1]).is_err());
    }

    #[test]
    fn test_parse_vote_switch_ix() {
        let hash = Hash::new_from_array([1; 32]);
        let vote = Vote {
            slots: vec![1, 2, 4],
            hash,
            timestamp: Some(1_234_567_890),
        };

        let vote_pubkey = Pubkey::new_unique();
        let authorized_voter_pubkey = Pubkey::new_unique();
        let proof_hash = Hash::new_from_array([2; 32]);
        let instruction =
            vote_instruction::vote_switch(&vote_pubkey, &authorized_voter_pubkey, vote, proof_hash);
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "VoteSwitch".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Slot Hashes Sysvar": sysvar::slot_hashes::ID.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Vote Authority": authorized_voter_pubkey.to_string(),
                    "Vote": {
                        "Slots": [1, 2, 4],
                        "Hash": hash.to_string(),
                        "Timestamp": 1_234_567_890,
                    },
                    "Hash": proof_hash.to_string(),
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..3]).is_err());
    }

    #[test]
    fn test_parse_vote_authorized_checked_ix() {
        let vote_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authorized_pubkey = Pubkey::new_unique();
        let authority_type = VoteAuthorize::Voter;
        let instruction = vote_instruction::authorize_checked(
            &vote_pubkey,
            &authorized_pubkey,
            &new_authorized_pubkey,
            authority_type,
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_vote(&message.instructions[0], &message.account_keys).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "AuthorizeChecked".to_string(),
                info: json!({
                    "Vote Account": vote_pubkey.to_string(),
                    "Clock Sysvar": sysvar::clock::ID.to_string(),
                    "Authority": authorized_pubkey.to_string(),
                    "New Authority": new_authorized_pubkey.to_string(),
                    "Authority Type": authority_type,
                }),
            }
        );
        assert!(parse_vote(&message.instructions[0], &message.account_keys[0..3]).is_err());
    }
}
