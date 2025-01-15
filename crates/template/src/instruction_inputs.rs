use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{parse_quote, FnArg};

use trident_idl_spec::IdlInstructionAccounts;
use trident_idl_spec::{
    idl_type_to_syn_type, Idl, IdlField, IdlInstruction, IdlInstructionAccount,
    IdlInstructionAccountItem,
};

// Generate input structures for each instruction
pub(crate) fn get_instruction_inputs(idl: &Idl) -> Vec<syn::ItemStruct> {
    let _program_name = idl.metadata.name.to_case(Case::UpperCamel);

    idl.instructions
        .iter()
        .fold(Vec::new(), |mut instructions_data, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            // get instruction name
            let instruction_name_ident: syn::Ident = format_ident!("{}", &instruction_name);
            // get instruction data name
            let instruction_data_name: syn::Ident = format_ident!("{}Data", &instruction_name);
            // get instruction accounts name
            let instruction_accounts_name: syn::Ident =
                format_ident!("{}Accounts", &instruction_name);

            // Generate accounts and parameters
            let accounts = get_instruction_accounts(instruction);

            let data = get_instruction_data(instruction);

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
                     #(pub #data),*
                }
            };

            instructions_data.push(instructions_inputs);
            instructions_data.push(instructions_input_accounts);
            instructions_data.push(instructions_input_data);
            instructions_data
        })
}

fn get_instruction_accounts(instruction: &IdlInstruction) -> Vec<syn::FnArg> {
    instruction
        .accounts
        .iter()
        .fold(Vec::new(), |mut account_parameters, account| {
            match account {
                IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                    process_composite_account(idl_instruction_accounts, &mut account_parameters);
                }
                IdlInstructionAccountItem::Single(idl_instruction_account) => {
                    process_single_account(idl_instruction_account, &mut account_parameters);
                }
            };
            account_parameters
        })
}

fn process_composite_account(
    idl_instruction_accounts: &IdlInstructionAccounts,
    account_parameters: &mut Vec<syn::FnArg>,
) {
    for account in &idl_instruction_accounts.accounts {
        match account {
            IdlInstructionAccountItem::Single(idl_instruction_account) => {
                process_single_account(idl_instruction_account, account_parameters);
            }
            IdlInstructionAccountItem::Composite(idl_instruction_accounts) => {
                process_composite_account(idl_instruction_accounts, account_parameters);
            }
        }
    }
}

fn process_single_account(
    idl_instruction_account: &IdlInstructionAccount,
    account_parameters: &mut Vec<syn::FnArg>,
) {
    // If the account has constant address it is not needed to fuzz it
    // So it will not be generated as a parameter.
    if idl_instruction_account.address.is_none() {
        let name = format_ident!("{}", idl_instruction_account.name);
        let account: syn::FnArg = parse_quote!(#name: AccountId);
        account_parameters.push(account);
    }
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
    let (arg_type, _is_custom) = idl_type_to_syn_type(&argument.ty, 0, true);

    let parameter: syn::FnArg = parse_quote!(#arg_name: #arg_type);

    arguments.push(parameter);
}
