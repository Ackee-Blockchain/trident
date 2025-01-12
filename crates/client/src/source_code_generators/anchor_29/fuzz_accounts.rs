use super::types::Idl;
use quote::format_ident;
use std::collections::HashMap;
use syn::parse_quote;

use super::fuzz_instructions_generator::ProgramAccount;

// Generate accounts for fuzzing
pub(crate) fn get_fuzz_accounts(
    idl: &Idl,
    program_accounts: &HashMap<String, Option<ProgramAccount>>,
) -> Vec<syn::FnArg> {
    // Create a HashMap to collect all fuzz accounts
    let fuzz_accounts = idl.instructions.iter().fold(
        HashMap::new(),
        |mut fuzz_accounts: HashMap<syn::Ident, syn::FnArg>, instruction| {
            instruction
                .accounts
                .iter()
                .fold(&mut fuzz_accounts, |fuzz_accounts, account| {
                    match account {
                        super::types::IdlAccountItem::IdlAccount(idl_account) => {
                            let program_account =
                                program_accounts.get(&idl_account.name).unwrap_or(&None);

                            let name: syn::Ident = format_ident!("{}", &idl_account.name);
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
                                },
                                None => {
                                    let account =
                                        parse_quote! { #name: AccountsStorage<KeypairStore> };
                                    fuzz_accounts.entry(name).or_insert(account);
                                }
                            }
                        }
                        super::types::IdlAccountItem::IdlAccounts(_idl_accounts) => {
                            panic!("Composite Accounts are not supported yet!")
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
