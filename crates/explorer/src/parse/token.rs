use crate::parse::{
    check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use solana_account_decoder::parse_token::token_amount_to_ui_amount;
use solana_program::{program_option::COption, pubkey::Pubkey};
use solana_sdk::instruction::CompiledInstruction;
use spl_token::instruction::{AuthorityType, TokenInstruction};

pub fn parse_token(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let token_instruction = TokenInstruction::unpack(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::SPLToken))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::SPLToken,
            ));
        }
    }
    match token_instruction {
        TokenInstruction::InitializeMint {
            decimals,
            mint_authority,
            freeze_authority,
        } => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "decimals": decimals,
                "mintAuthority": mint_authority.to_string(),
                "rentSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            if let COption::Some(freeze_authority) = freeze_authority {
                map.insert(
                    "freezeAuthority".to_string(),
                    json!(freeze_authority.to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeMint".to_string(),
                info: value,
            })
        }
        TokenInstruction::InitializeAccount => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeAccount".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[2] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::InitializeAccount2 { owner } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeAccount2".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                    "owner": owner.to_string(),
                    "rentSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::InitializeMultisig { m } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut signers: Vec<String> = vec![];
            for i in instruction.accounts[2..].iter() {
                signers.push(account_keys[*i as usize].to_string());
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeMultisig".to_string(),
                info: json!({
                    "multisig": account_keys[instruction.accounts[0] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "signers": signers,
                    "m": m,
                }),
            })
        }
        TokenInstruction::Transfer { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: value,
            })
        }
        TokenInstruction::Approve { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "delegate": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "approve".to_string(),
                info: value,
            })
        }
        TokenInstruction::Revoke => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                1,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "revoke".to_string(),
                info: value,
            })
        }
        TokenInstruction::SetAuthority {
            authority_type,
            new_authority,
        } => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let owned = match authority_type {
                AuthorityType::MintTokens | AuthorityType::FreezeAccount => "mint",
                AuthorityType::AccountOwner | AuthorityType::CloseAccount => "account",
            };
            let mut value = json!({
                owned: account_keys[instruction.accounts[0] as usize].to_string(),
                "authorityType": Into::<UiAuthorityType>::into(authority_type),
                "newAuthority": match new_authority {
                    COption::Some(authority) => Some(authority.to_string()),
                    COption::None => None,
                },
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                1,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "setAuthority".to_string(),
                info: value,
            })
        }
        TokenInstruction::MintTo { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "account": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "mintAuthority",
                "multisigMintAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "mintTo".to_string(),
                info: value,
            })
        }
        TokenInstruction::Burn { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "burn".to_string(),
                info: value,
            })
        }
        TokenInstruction::CloseAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "destination": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "closeAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::FreezeAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "freezeAuthority",
                "multisigFreezeAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "freezeAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::ThawAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "freezeAuthority",
                "multisigFreezeAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "thawAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::TransferChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "destination": account_keys[instruction.accounts[2] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                3,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "transferChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::ApproveChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "delegate": account_keys[instruction.accounts[2] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                3,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "approveChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::MintToChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "account": account_keys[instruction.accounts[1] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "mintAuthority",
                "multisigMintAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "mintToChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::BurnChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "burnChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::SyncNative => {
            check_num_token_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "syncNative".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                }),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiAuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}

impl From<AuthorityType> for UiAuthorityType {
    fn from(authority_type: AuthorityType) -> Self {
        match authority_type {
            AuthorityType::MintTokens => UiAuthorityType::MintTokens,
            AuthorityType::FreezeAccount => UiAuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => UiAuthorityType::AccountOwner,
            AuthorityType::CloseAccount => UiAuthorityType::CloseAccount,
        }
    }
}

fn parse_signers(
    map: &mut Map<String, Value>,
    last_nonsigner_index: usize,
    account_keys: &[Pubkey],
    accounts: &[u8],
    owner_field_name: &str,
    multisig_field_name: &str,
) {
    if accounts.len() > last_nonsigner_index + 1 {
        let mut signers: Vec<String> = vec![];
        for i in accounts[last_nonsigner_index + 1..].iter() {
            signers.push(account_keys[*i as usize].to_string());
        }
        map.insert(
            multisig_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
        map.insert("signers".to_string(), json!(signers));
    } else {
        map.insert(
            owner_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
    }
}

fn check_num_token_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::SPLToken)
}
