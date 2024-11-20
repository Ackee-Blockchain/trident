use std::collections::{hash_map::Entry, HashMap};

use anchor_lang_idl_spec::{Idl, IdlInstructionAccount, IdlPda};
use quote::{quote, ToTokens};

use super::{
    get_fuzz_accounts, get_instruction_inputs, get_instruction_ixops, get_instruction_variants,
    get_types,
};

pub(crate) enum ProgramAccount {
    // writable | signer
    Keypair(bool, bool),
    // writable | signer (PDA cannot be signer, but keep it simple)
    Pda(IdlPda, bool, bool),
    // writable | signer
    Constant(String, bool, bool),
}

// Main function to generate source code from IDLs
pub fn generate_source_code(idls: &[Idl]) -> String {
    // Collections to store generated items
    let mut all_instructions: Vec<syn::Variant> = Vec::new();
    let mut all_instruction_inputs: Vec<syn::ItemStruct> = Vec::new();
    let mut all_instructions_ixops_impls: Vec<syn::ItemImpl> = Vec::new();
    let mut all_fuzz_accounts: Vec<syn::FnArg> = Vec::new();
    let mut all_types: Vec<syn::Item> = Vec::new();

    // Iterate over each IDL to generate various parts of the code
    for idl in idls {
        let program_accounts = get_program_accounts(idl);

        all_instructions.extend(get_instruction_variants(idl));
        all_instruction_inputs.extend(get_instruction_inputs(idl));
        all_instructions_ixops_impls.extend(get_instruction_ixops(idl, &program_accounts));
        all_fuzz_accounts.extend(get_fuzz_accounts(idl, &program_accounts));
        all_types.extend(get_types(idl));
    }

    // Define the Rust module with all generated code
    let module_definition = quote! {
        use trident_fuzz::fuzzing::*;
        use borsh::{BorshDeserialize, BorshSerialize};

        /// FuzzInstruction contains all available Instructions.
        /// Below, the instruction arguments (accounts and data) are defined.
        #[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
        pub enum FuzzInstruction {
            #(#all_instructions),*
        }

        #(#all_instruction_inputs)*

        #(#all_instructions_ixops_impls)*

        /// Check supported AccountsStorages at
        /// https://ackee.xyz/trident/docs/latest/features/account-storages/
        #[derive(Default)]
        pub struct FuzzAccounts {
            #(#all_fuzz_accounts),*
        }

        #(#all_types)*
    };

    // Convert the module definition to a string and return it
    module_definition.into_token_stream().to_string()
}

fn get_program_accounts(idl: &Idl) -> HashMap<String, Option<ProgramAccount>> {
    idl.instructions.iter().fold(
        HashMap::<String, Option<ProgramAccount>>::new(),
        |mut program_accounts, instruction| {
            for account in &instruction.accounts {
                match account {
                    anchor_lang_idl_spec::IdlInstructionAccountItem::Composite(
                        idl_instruction_accounts,
                    ) => panic!(
                        "Composite accounts not supported. Composite account with name {} found",
                        idl_instruction_accounts.name
                    ),
                    anchor_lang_idl_spec::IdlInstructionAccountItem::Single(
                        idl_instruction_account,
                    ) => {
                        let account_name = &idl_instruction_account.name;
                        let program_account = decide_program_account_type(idl_instruction_account);

                        match program_accounts.entry(account_name.to_string()) {
                            Entry::Vacant(entry) => {
                                entry.insert(program_account);
                            }
                            Entry::Occupied(mut entry) => {
                                if entry.get().is_none() {
                                    entry.insert(program_account);
                                }
                            }
                        }
                    }
                }
            }
            program_accounts
        },
    )
}

fn decide_program_account_type(
    idl_instruction_account: &IdlInstructionAccount,
) -> Option<ProgramAccount> {
    if let Some(address) = &idl_instruction_account.address {
        Some(ProgramAccount::Constant(
            address.to_string(),
            idl_instruction_account.writable,
            idl_instruction_account.signer,
        ))
    } else if idl_instruction_account.signer {
        Some(ProgramAccount::Keypair(
            idl_instruction_account.writable,
            idl_instruction_account.signer,
        ))
    } else {
        idl_instruction_account.pda.as_ref().map(|idl_pda| {
            ProgramAccount::Pda(
                idl_pda.clone(),
                idl_instruction_account.writable,
                idl_instruction_account.signer,
            )
        })
    }
}
