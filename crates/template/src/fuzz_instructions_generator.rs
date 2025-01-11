use std::collections::{hash_map::Entry, HashMap};

use quote::{quote, ToTokens};

use trident_idl_spec::{
    Idl, IdlInstructionAccount, IdlInstructionAccountItem, IdlInstructionAccounts,
};

use super::{
    get_fuzz_accounts, get_instruction_inputs, get_instruction_ixops, get_instruction_variants,
    get_types,
};
use crate::instruction_account::{InstructionAccount, InstructionAccountType};

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
        let instruction_accounts = get_instructions_accounts(idl);
        let program_accounts = get_program_accounts(idl);

        all_instructions.extend(get_instruction_variants(idl));
        all_instruction_inputs.extend(get_instruction_inputs(idl));
        all_instructions_ixops_impls.extend(get_instruction_ixops(idl, &instruction_accounts));
        all_fuzz_accounts.extend(get_fuzz_accounts(idl, &instruction_accounts));
        all_types.extend(get_types(idl, program_accounts));
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

fn get_instructions_accounts(idl: &Idl) -> HashMap<String, InstructionAccount> {
    idl.instructions.iter().fold(
        HashMap::<String, InstructionAccount>::new(),
        |mut instruction_accounts, instruction| {
            for account in &instruction.accounts {
                match account {
                    IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                        process_composite_account(idl_instruction_accounts)
                    }
                    IdlInstructionAccountItem::Single(idl_instruction_account) => {
                        process_single_account(
                            instruction.name.clone(),
                            idl_instruction_account,
                            &mut instruction_accounts,
                        );
                    }
                }
            }
            instruction_accounts
        },
    )
}

fn process_composite_account(idl_instruction_accounts: &IdlInstructionAccounts) {
    panic!(
        "Composite accounts not supported. Composite account with name {} found",
        idl_instruction_accounts.name
    )
}

fn process_single_account(
    instruction_name: String,
    idl_instruction_account: &IdlInstructionAccount,
    instruction_accounts: &mut HashMap<String, InstructionAccount>,
) {
    let account_name = &idl_instruction_account.name;
    let account_type = evaluate_account_type(idl_instruction_account);

    match instruction_accounts.entry(account_name.to_string()) {
        Entry::Vacant(entry) => {
            // if no entry so far, create new account
            let mut new_account = InstructionAccount::new(account_name.to_string());

            // insert infor about current instruction and the account type within the instruction
            new_account.insert(instruction_name, account_type);
            entry.insert(new_account);
        }
        Entry::Occupied(mut entry) => {
            // if there is an entry, insert infor about current instruction and the account type within the instruction
            let account = entry.get_mut();
            account.insert(instruction_name, account_type);
        }
    };
}

#[allow(clippy::manual_map)]
fn evaluate_account_type(
    idl_instruction_account: &IdlInstructionAccount,
) -> InstructionAccountType {
    // If the address is defined in the IDL, it is a constant account
    if let Some(address) = &idl_instruction_account.address {
        InstructionAccountType::Constant(
            address.to_string(),
            idl_instruction_account.writable,
            idl_instruction_account.signer,
        )
    // If the account is a signer, it is a keypair account
    } else if idl_instruction_account.signer {
        InstructionAccountType::Keypair(
            idl_instruction_account.writable,
            idl_instruction_account.signer,
        )
    // If the account is a PDA, it is a PDA account
    } else if let Some(idl_pda) = &idl_instruction_account.pda {
        InstructionAccountType::Pda(
            idl_pda.clone(),
            idl_instruction_account.writable,
            idl_instruction_account.signer,
        )
    // if we cannot decide based on the above rules, do not return anything
    // the accouunt might be decided in other instructions
    // or will be generated as Keypair by default
    } else {
        InstructionAccountType::default()
    }
}

fn get_program_accounts(idl: &Idl) -> HashMap<String, Vec<u8>> {
    // get account that program uses to store data
    // i.e. data accounts
    idl.accounts
        .iter()
        .fold(HashMap::new(), |mut program_accounts, account| {
            program_accounts.insert(account.name.clone(), account.discriminator.clone());
            program_accounts
        })
}
