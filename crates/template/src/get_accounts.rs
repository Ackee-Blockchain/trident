use quote::format_ident;
use std::collections::HashMap;
use syn::{parse_quote, Block};

use trident_idl_spec::{
    IdlInstruction, IdlInstructionAccount, IdlInstructionAccountItem, IdlInstructionAccounts,
};

use crate::instruction_account::{InstructionAccount, InstructionAccountType};

pub(crate) fn get_accounts(
    instruction: &IdlInstruction,
    instructions_accounts: &HashMap<String, InstructionAccount>,
) -> Vec<syn::Block> {
    let mut account_implementations = vec![];

    for account in &instruction.accounts {
        match account {
            IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                process_composite_account(idl_instruction_accounts)
            }
            IdlInstructionAccountItem::Single(idl_instruction_account) => {
                process_single_account(
                    &instruction.name,
                    idl_instruction_account,
                    instructions_accounts,
                    &mut account_implementations,
                );
            }
        }
    }

    account_implementations
}

fn process_composite_account(idl_instruction_accounts: &IdlInstructionAccounts) {
    panic!(
        "Composite accounts not supported. Composite account with name {} found",
        idl_instruction_accounts.name
    )
}

fn process_single_account(
    instruction: &str,
    account: &IdlInstructionAccount,
    instructions_accounts: &HashMap<String, InstructionAccount>,
    account_implementations: &mut Vec<Block>,
) {
    let account = instructions_accounts
        .get(&account.name)
        .expect("Account not found in types databse");

    let account_name = &account.account_name;
    let account_name_ident = format_ident!("{}", account_name);

    let kind = account
        .kind
        .get(instruction)
        .unwrap_or(&InstructionAccountType::Unknown);

    let account_implementation = match kind {
        InstructionAccountType::Keypair(writable, signer) => {
            process_keypair_account(&account_name_ident, *writable, *signer)
        }

        InstructionAccountType::Pda(_idl_pda, writable, signer) => {
            process_pda_account(&account_name_ident, *writable, *signer)
        }

        InstructionAccountType::Constant(address, writable, signer) => {
            process_constant_account(address, *writable, *signer)
        }
        InstructionAccountType::Unknown => process_unknown_account(&account_name_ident),
    };
    account_implementations.push(account_implementation);
}

fn process_keypair_account(account_name: &syn::Ident, writable: bool, signer: bool) -> syn::Block {
    match (writable, signer) {
        // If the account is writable and also signer
        // It has to be used with AccountMeta::new
        // It has to be appended to the signers vector
        (true, true) => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new(#account_name.pubkey(), #signer));
                    signers.push(#account_name.insecure_clone());
                }
            )
        }
        // If the account is writable and not signer
        // It has to be used with AccountMeta::new
        // It has to be appended to the account_metas vector
        (true, false) => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new(#account_name.pubkey(), #signer));
                }

            )
        }
        // If the account is not writable and signer
        // It has to be used with AccountMeta::new_readonly
        // It has to be appended to the account_metas vector
        (false, true) => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new_readonly(#account_name.pubkey(), #signer));
                    signers.push(#account_name.insecure_clone());
                }
            )
        }
        // If the account is not writable and not signer
        // It has to be used with AccountMeta::new_readonly
        // It has to be appended to the account_metas vector
        (false, false) => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new_readonly(#account_name.pubkey(), #signer));
                }

            )
        }
    }
}

fn process_pda_account(account_name: &syn::Ident, writable: bool, _signer: bool) -> syn::Block {
    match writable {
        // if the account is writable
        // It has to be used with AccountMeta::new
        true => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        &[todo!()],
                        &self.get_program_id(),
                    );
                    account_metas.push(AccountMeta::new(#account_name, false));
                }

            )
        }
        // if the account is not writable
        // It has to be used with AccountMeta::new_readonly
        false => {
            parse_quote!(
                {
                    let #account_name = fuzz_accounts.#account_name.get_or_create_account(
                        self.accounts.#account_name,
                        client,
                        &[todo!()],
                        &self.get_program_id(),
                    );
                    account_metas.push(AccountMeta::new_readonly(#account_name, false));
                }
            )
        }
    }
}

fn process_constant_account(address: &str, writable: bool, signer: bool) -> syn::Block {
    match (writable, signer) {
        // If the account is writable and also signer
        // It has to be used with AccountMeta::new
        // It has to be appended to the signers vector
        (true, true) => {
            parse_quote!({
                account_metas.push(AccountMeta::new(pubkey!(#address), true));
                signers.push(todo!());
            })
        }
        // If the account is writable and not signer
        // It has to be used with AccountMeta::new
        (true, false) => {
            parse_quote!({
                account_metas.push(AccountMeta::new(pubkey!(#address), false));
            })
        }
        // If the account is not writable and signer
        // It has to be used with AccountMeta::new_readonly
        // It has to be appended to the signers vector
        (false, true) => {
            parse_quote!({
                account_metas.push(AccountMeta::new_readonly(pubkey!(#address), true));
                signers.push(todo!());
            })
        }
        // If the account is not writable and not signer
        // It has to be used with AccountMeta::new_readonly
        (false, false) => {
            parse_quote!({
                account_metas.push(AccountMeta::new_readonly(pubkey!(#address), false));
            })
        }
    }
}

fn process_unknown_account(account_name: &syn::Ident) -> syn::Block {
    // if we do not have any information about the account
    // we can just create a placeholder
    parse_quote!(
        {
            let #account_name = todo!();
            account_metas.push(todo!());
        }
    )
}
