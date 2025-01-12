use anchor_lang_idl_spec::{Idl, IdlInstruction, IdlInstructionAccountItem};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::parse_quote;

use super::idl_type_to_syn_type;

// Generate input structures for each instruction
pub(crate) fn get_instruction_inputs(idl: &Idl) -> Vec<syn::ItemStruct> {
    let _program_name = idl.metadata.name.to_case(Case::UpperCamel);

    idl.instructions
        .iter()
        .fold(Vec::new(), |mut instructions_data, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            let instruction_name_ident: syn::Ident = format_ident!("{}", &instruction_name);
            let instruction_data_name: syn::Ident = format_ident!("{}Data", &instruction_name);
            let instruction_accounts_name: syn::Ident =
                format_ident!("{}Accounts", &instruction_name);

            // Generate accounts and parameters

            let accounts = get_instruction_accounts(instruction);

            let parameters = get_instruction_arguments(instruction);

            // Define the input structures
            let instructions_inputs: syn::ItemStruct = parse_quote! {
                #[derive(Arbitrary, Debug)]
                pub struct #instruction_name_ident {
                     pub accounts: #instruction_accounts_name,
                     pub data: #instruction_data_name
                }
            };

            let instructions_input_accounts: syn::ItemStruct = parse_quote! {
                #[derive(Arbitrary, Debug)]
                pub struct #instruction_accounts_name {
                     #(pub #accounts),*
                }
            };

            let instructions_input_data: syn::ItemStruct = parse_quote! {
                /// Custom data types must derive `Debug` and `Arbitrary`.
                /// To do this, redefine the type in the fuzz test and implement the `From` trait
                /// to convert it into the type defined in the program.
                /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
                #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
                pub struct #instruction_data_name {
                     #(pub #parameters),*
                }
            };

            instructions_data.push(instructions_inputs);
            instructions_data.push(instructions_input_accounts);
            instructions_data.push(instructions_input_data);
            instructions_data
        })
}

fn get_instruction_accounts(instruction: &IdlInstruction) -> Vec<syn::FnArg> {
    instruction.accounts.iter().fold(
        Vec::new(),
        |mut account_parameters, account| match account {
            IdlInstructionAccountItem::Composite(_composite) => {
                panic!("Composite Accounts are not supported yet!")
            }
            IdlInstructionAccountItem::Single(single) => {
                if single.address.is_none() {
                    let name = format_ident!("{}", single.name);
                    let account: syn::FnArg = parse_quote!(#name: AccountId);
                    account_parameters.push(account);
                }
                account_parameters
            }
        },
    )
}

fn get_instruction_arguments(instruction: &IdlInstruction) -> Vec<syn::FnArg> {
    instruction
        .args
        .iter()
        .fold(Vec::new(), |mut data_parameters, arg| {
            let arg_name = format_ident!("{}", arg.name);
            let (arg_type, _is_custom) = idl_type_to_syn_type(&arg.ty, 0);
            let parameter: syn::FnArg = parse_quote!(#arg_name: #arg_type);
            data_parameters.push(parameter);
            data_parameters
        })
}
