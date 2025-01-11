use trident_idl_spec::{
    Idl, IdlInstructionAccount, IdlInstructionAccountItem, IdlInstructionAccounts,
};

use quote::format_ident;
use std::collections::HashMap;
use syn::parse_quote;

use crate::instruction_account::{FuzzAccountsType, InstructionAccount};

// Generate accounts for fuzzing
// TODO: if accounts have different names in multiple programs, the accounts will be generated
// as one account in the FuzzAccounts struct
pub(crate) fn get_fuzz_accounts(
    idl: &Idl,
    instructions_accounts: &HashMap<String, InstructionAccount>,
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
                        IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                            process_composite_account(idl_instruction_accounts);
                        }
                        IdlInstructionAccountItem::Single(idl_instruction_account) => {
                            process_single_account(
                                idl_instruction_account,
                                fuzz_accounts,
                                instructions_accounts,
                            );
                        }
                    };
                    fuzz_accounts
                });
            fuzz_accounts
        },
    );

    // Sort and return the fuzz accounts
    // Sorting will ensure that it is always generated in the same order
    let mut sorted_accounts: Vec<_> = fuzz_accounts.into_iter().collect();
    sorted_accounts.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    sorted_accounts.into_iter().map(|(_, v)| v).collect()
}

fn process_composite_account(idl_instruction_accounts: &IdlInstructionAccounts) {
    panic!(
        "Composite accounts not supported. Composite account with name {} found",
        idl_instruction_accounts.name
    )
}

fn process_single_account(
    idl_instruction_account: &IdlInstructionAccount,
    fuzz_accounts: &mut HashMap<syn::Ident, syn::FnArg>,
    instructions_accounts: &HashMap<String, InstructionAccount>,
) {
    let account = instructions_accounts
        .get(&idl_instruction_account.name)
        .expect("Account not found in types databse");

    let account_type = account.get_fuzz_accounts_type();

    match account_type {
        FuzzAccountsType::Keypair => {
            process_keypair_account(idl_instruction_account, fuzz_accounts);
        }
        FuzzAccountsType::Pda => {
            process_pda_account(idl_instruction_account, fuzz_accounts);
        }
        FuzzAccountsType::Constant => {
            // No need to store and fuzz constant addresses
        }
        FuzzAccountsType::Unknown => {
            process_unknown_account(idl_instruction_account, fuzz_accounts);
        }
    }
}

fn process_keypair_account(
    idl_instruction_account: &IdlInstructionAccount,
    fuzz_accounts: &mut HashMap<syn::Ident, syn::FnArg>,
) {
    let name: syn::Ident = format_ident!("{}", &idl_instruction_account.name);

    let account = parse_quote! { #name: AccountsStorage<KeypairStore> };
    fuzz_accounts.entry(name).or_insert(account);
}

fn process_pda_account(
    idl_instruction_account: &IdlInstructionAccount,
    fuzz_accounts: &mut HashMap<syn::Ident, syn::FnArg>,
) {
    let name: syn::Ident = format_ident!("{}", &idl_instruction_account.name);

    let account = parse_quote! { #name: AccountsStorage<PdaStore> };
    fuzz_accounts.entry(name).or_insert(account);
}

fn process_unknown_account(
    idl_instruction_account: &IdlInstructionAccount,
    fuzz_accounts: &mut HashMap<syn::Ident, syn::FnArg>,
) {
    let name: syn::Ident = format_ident!("{}", &idl_instruction_account.name);

    // By default do not decide if de not know what it is
    // if we generate it incorrectly, it can confuse user
    let account = parse_quote! { #name: AccountsStorage<todo!()> };
    fuzz_accounts.entry(name).or_insert(account);
}
