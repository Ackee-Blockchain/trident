use anchor_lang_idl_spec::{Idl, IdlInstructionAccountItem};
use convert_case::{Case, Casing};
use quote::format_ident;
use std::collections::HashMap;
use syn::parse_quote;

use super::fuzz_instructions_generator::ProgramAccount;

// Generate accounts for fuzzing
pub(crate) fn get_fuzz_accounts(
    idl: &Idl,
    program_accounts: &HashMap<String, Option<ProgramAccount>>,
) -> Vec<syn::FnArg> {
    let program_name = idl.metadata.name.to_case(Case::Snake);

    // Create a HashMap to collect all fuzz accounts
    let fuzz_accounts = idl.instructions.iter().fold(
        HashMap::new(),
        |mut fuzz_accounts: HashMap<syn::Ident, syn::FnArg>, instruction| {
            instruction
                .accounts
                .iter()
                .fold(&mut fuzz_accounts, |fuzz_accounts, account| {
                    match account {
                        IdlInstructionAccountItem::Composite(_composite) => {
                            panic!("Composite Accounts are not supported yet!")
                        }
                        IdlInstructionAccountItem::Single(single) => {
                            let program_account =
                                program_accounts.get(&single.name).unwrap_or(&None);

                            let name: syn::Ident =
                                format_ident!("{}_{}", &single.name, program_name);
                            match program_account {
                                Some(program_account) => match program_account {
                                    ProgramAccount::Keypair(_, _) => {
                                        let account =
                                            parse_quote! { #name: AccountsStorage<KeypairStore> };
                                        fuzz_accounts.entry(name).or_insert(account);
                                    }
                                    ProgramAccount::Pda(_idl_pda, _, _) => {
                                        let account =
                                            parse_quote! { #name: AccountsStorage<PdaStore> };
                                        fuzz_accounts.entry(name).or_insert(account);
                                    }
                                    ProgramAccount::Constant(_, _, _) => {}
                                },
                                None => {
                                    let account =
                                        parse_quote! { #name: AccountsStorage<KeypairStore> };
                                    fuzz_accounts.entry(name).or_insert(account);
                                }
                            }
                        }
                    };
                    fuzz_accounts
                });
            fuzz_accounts
        },
    );

    // Sort and return the fuzz accounts
    let mut sorted_accounts: Vec<_> = fuzz_accounts.into_iter().collect();
    sorted_accounts.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    sorted_accounts.into_iter().map(|(_, v)| v).collect()
}
