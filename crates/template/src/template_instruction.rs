use crate::{template::ModDefinition, Template};

use convert_case::Case;
use convert_case::Casing;
use quote::format_ident;
use syn::{parse_quote, parse_str, FnArg};
use trident_idl_spec::{
    IdlField, IdlInstruction, IdlInstructionAccount, IdlInstructionAccountItem,
};

use crate::idl_type_to_syn_type;

use crate::process_discriminator;

impl Template {
    pub(crate) fn instruction(&mut self, instruction: &IdlInstruction, program_id: &String) {
        let instruction_name = self.get_camel_identifier(instruction);

        // Get instruction name
        let instruction_name = format!("{}Instruction", instruction_name);
        // Get instruction data name
        let instruction_data_name: syn::Ident = format_ident!("{}Data", &instruction_name);
        // Get instruction accounts name
        let instruction_accounts_name: syn::Ident = format_ident!("{}Accounts", &instruction_name);

        let instruction_struct_name: syn::Ident = parse_str(&instruction_name).unwrap();

        let accounts = get_instruction_accounts(instruction);

        let data = get_instruction_data(instruction);

        let instruction_discriminator = process_discriminator(instruction);

        // Generate composite account structs before main instruction struct
        let composite_structs = get_composite_account_structs(instruction);

        let instruction_struct: syn::ItemStruct = parse_quote! {
            #[derive(Arbitrary, Debug, TridentInstruction)]
            #[accounts("accounts")]
            #[program_id(#program_id)]
            #[discriminator([#(#instruction_discriminator,)*])]
            pub struct #instruction_struct_name {
                pub accounts: #instruction_accounts_name,
                pub data: #instruction_data_name
            }
        };

        let instruction_input_accounts: syn::ItemStruct = parse_quote! {
            /// Instruction Accounts
            #[derive(Arbitrary, Debug, Clone, TridentAccounts)]
            pub struct #instruction_accounts_name {
                 #(pub #accounts),*
            }
        };

        let instruction_input_data: syn::ItemStruct = parse_quote! {
            /// Instruction Data
            #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
            pub struct #instruction_data_name {
                 #(pub #data),*
            }
        };

        let ix_setters: syn::ItemImpl = parse_quote! {
            /// Implementation of instruction setters for fuzzing
            ///
            /// Provides methods to:
            /// - Set instruction data during fuzzing
            /// - Configure instruction accounts during fuzzing
            /// - (Optional) Set remaining accounts during fuzzing
            impl InstructionSetters for #instruction_struct_name {
                type IxAccounts = FuzzAccounts;

                fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
                    todo!()
                }

                fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
                    todo!()
                }
            }
        };

        let instruction_file: syn::File = parse_quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use trident_fuzz::fuzzing::*;
            use crate::types::*;
            use crate::fuzz_transactions::FuzzAccounts;

            #instruction_struct

            #instruction_input_accounts

            #(#composite_structs)*

            #instruction_input_data

            #ix_setters
        };

        let instruction_file_name = self.get_snake_identifier(instruction);

        self.instructions
            .insert(instruction_file_name.to_owned(), instruction_file);

        self.instruction_mod(instruction);
    }

    fn instruction_mod(&mut self, instruction: &IdlInstruction) {
        let instruction_file_name = self.get_snake_identifier(instruction);
        let instruction_file_name_ident: syn::Ident = parse_str(&instruction_file_name).unwrap();

        let instructions_mod: syn::ItemMod = parse_quote!(
            pub mod #instruction_file_name_ident;
        );

        let instructions_use_statement: syn::ItemUse = parse_quote!(
            pub use #instruction_file_name_ident::*;
        );

        self.instructions_mod.push(ModDefinition {
            module: instructions_mod,
            use_statement: instructions_use_statement,
        });
    }
}

fn get_instruction_accounts(instruction: &IdlInstruction) -> Vec<syn::FnArg> {
    instruction
        .accounts
        .iter()
        .fold(Vec::new(), |mut account_parameters, account| {
            match account {
                IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                    // Create a field using the composite account name
                    let composite_name = format_ident!("{}", idl_instruction_accounts.name);
                    // Convert to camel case for the type name
                    let composite_type = format_ident!(
                        "{}Accounts",
                        idl_instruction_accounts.name.to_case(Case::UpperCamel)
                    );
                    let account: syn::FnArg = parse_quote!(#composite_name: #composite_type);
                    account_parameters.push(account);
                }
                IdlInstructionAccountItem::Single(idl_instruction_account) => {
                    process_single_account(idl_instruction_account, &mut account_parameters);
                }
            };
            account_parameters
        })
}

fn process_single_account(
    idl_instruction_account: &IdlInstructionAccount,
    account_parameters: &mut Vec<syn::FnArg>,
) {
    // If the account has constant address it is not needed to fuzz it
    // So it will not be generated as a parameter.
    let name = format_ident!("{}", idl_instruction_account.name);
    let account: syn::FnArg = parse_quote!(#name: TridentAccount);
    account_parameters.push(account);
}

fn get_instruction_data(instruction: &IdlInstruction) -> Vec<syn::FnArg> {
    instruction
        .args
        .iter()
        .fold(Vec::new(), |mut arguments, argument| {
            process_instruction_argument(argument, &mut arguments);
            arguments
        })
}

fn process_instruction_argument(argument: &IdlField, arguments: &mut Vec<FnArg>) {
    let arg_name = format_ident!("{}", argument.name);

    // convert type to syn type
    let (arg_type, _is_custom) = idl_type_to_syn_type(&argument.ty);

    let parameter: syn::FnArg = parse_quote!(#arg_name: #arg_type);

    arguments.push(parameter);
}

fn get_composite_account_structs(instruction: &IdlInstruction) -> Vec<syn::ItemStruct> {
    let mut composite_structs = Vec::new();

    for account in &instruction.accounts {
        process_composite_account_item(account, &mut composite_structs);
    }

    composite_structs
}

fn process_composite_account_item(
    account: &IdlInstructionAccountItem,
    composite_structs: &mut Vec<syn::ItemStruct>,
) {
    if let IdlInstructionAccountItem::Composite(composite) = account {
        // Process all nested composite accounts first
        for nested_account in &composite.accounts {
            process_composite_account_item(nested_account, composite_structs);
        }

        // Convert to camel case for the struct name
        let struct_name = format_ident!("{}Accounts", composite.name.to_case(Case::UpperCamel));

        let fields = composite
            .accounts
            .iter()
            .fold(Vec::new(), |mut fields, account| {
                match account {
                    IdlInstructionAccountItem::Single(acc) => {
                        let name = format_ident!("{}", acc.name);
                        let field: syn::FnArg = parse_quote!(#name: TridentAccount);
                        fields.push(field);
                    }
                    IdlInstructionAccountItem::Composite(nested) => {
                        let name = format_ident!("{}", nested.name);
                        // Convert nested composite type to camel case as well
                        let type_name =
                            format_ident!("{}Accounts", nested.name.to_case(Case::UpperCamel));
                        let field: syn::FnArg = parse_quote!(#name: #type_name);
                        fields.push(field);
                    }
                }
                fields
            });

        let struct_def: syn::ItemStruct = parse_quote! {
            #[derive(Arbitrary, Debug, Clone, TridentAccounts)]
            pub struct #struct_name {
                #(pub #fields),*
            }
        };

        composite_structs.push(struct_def);
    }
}
