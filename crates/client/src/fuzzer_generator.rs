use crate::idl::Idl;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, parse_str};

/// Generates `program_client`'s `lib.rs` from [Idl] created from Anchor programs.
/// Disable regenerating the `use` statements with a used imports `use_modules`
///
/// _Note_: See the crate's tests for output example.
pub fn generate_source_code(idl: &Idl, use_modules: &[syn::ItemUse]) -> String {
    let mut output = "// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module\n".to_owned();
    let code = idl
        .programs
        .iter()
        .map(|idl_program| {
            let program_name = idl_program.name.snake_case.replace('-', "_");
            let fuzz_instructions_module_name = format_ident!("{}_fuzz_instructions", program_name);
            let module_name: syn::Ident = parse_str(&program_name).unwrap();
            // let pubkey_bytes: syn::ExprArray = parse_str(&idl_program.id).unwrap();

            let instructions = idl_program
                .instruction_account_pairs
                .iter()
                .fold(
                    Vec::new(),
                    |mut instructions, (idl_instruction, idl_account_group)| {
                        let instruction_fn_name: syn::Ident =
                            parse_str(&idl_instruction.name.snake_case).unwrap();
                        let instruction_struct_name: syn::Ident =
                            parse_str(&idl_instruction.name.upper_camel_case).unwrap();
                        let instruction_data_name: syn::Ident =
                            format_ident!("{}Data", &idl_instruction.name.upper_camel_case);
                        let instruction_accounts_name: syn::Ident =
                            format_ident!("{}Accounts", &idl_instruction.name.upper_camel_case);
                        let account_struct_name: syn::Ident =
                            parse_str(&idl_account_group.name.upper_camel_case).unwrap();
                        let instruction_name: syn::Ident =
                            parse_str(&(idl_instruction.name.snake_case.clone() + "_ix")).unwrap();

                        let parameters = idl_instruction
                            .parameters
                            .iter()
                            .map(|(name, ty)| {
                                let name = format_ident!("i_{name}");
                                let ty: syn::Type = parse_str(ty).unwrap();
                                let parameter: syn::FnArg = parse_quote!(#name: #ty);
                                parameter
                            })
                            .collect::<Vec<_>>();

                        let accounts = idl_account_group
                            .accounts
                            .iter()
                            .map(|(name, ty)| {
                                let name = format_ident!("a_{name}");
                                // do not use fully qualified type for Pubkey
                                let ty = parse_str(ty).unwrap();
                                let ty: syn::Type = match &ty {
                                    syn::Type::Path(tp) => {
                                        let last_type =
                                            &tp.path.segments.last().unwrap().ident.to_string();
                                        if last_type == "Pubkey" {
                                            let t: syn::Type = parse_str(last_type).unwrap();
                                            t
                                        } else {
                                            // we expect only Pubkey, but if it is something different, than return fully qualified type
                                            ty
                                        }
                                    }
                                    _ => ty,
                                };
                                let account: syn::FnArg = parse_quote!(#name: #ty);
                                account
                            })
                            .collect::<Vec<_>>();

                        let field_parameters = idl_instruction
                            .parameters
                            .iter()
                            .map(|(name, _)| {
                                let name: syn::Ident = parse_str(name).unwrap();
                                let value = format_ident!("i_{name}");
                                let parameter: syn::FieldValue = parse_quote!(#name: #value);
                                parameter
                            })
                            .collect::<Vec<_>>();

                        let field_accounts = idl_account_group
                            .accounts
                            .iter()
                            .map(|(name, _)| {
                                let name: syn::Ident = parse_str(name).unwrap();
                                let value = format_ident!("a_{name}");
                                let account: syn::FieldValue = parse_quote!(#name: #value);

                                account
                            })
                            .collect::<Vec<_>>();

                        let instruction: syn::Variant = parse_quote! {
                            #instruction_struct_name {
                                accounts: #instruction_accounts_name,
                                data: #instruction_data_name
                            }
                        };

                        instructions.push(instruction);
                        instructions
                    },
                )
                .into_iter();

            let instructions_data = idl_program
                .instruction_account_pairs
                .iter()
                .fold(
                    Vec::new(),
                    |mut instructions_data, (idl_instruction, _idl_account_group)| {
                        let instruction_data_name: syn::Ident =
                            format_ident!("{}Data", &idl_instruction.name.upper_camel_case);

                        let parameters = idl_instruction
                            .parameters
                            .iter()
                            .map(|(name, ty)| {
                                let name = format_ident!("i_{name}");
                                let ty: syn::Type = parse_str(ty).unwrap();
                                let parameter: syn::FnArg = parse_quote!(#name: #ty);
                                parameter
                            })
                            .collect::<Vec<_>>();

                        let ix_data: syn::ItemStruct = parse_quote! {
                            #[derive(Arbitrary, Clone)]
                            pub struct #instruction_data_name {
                                 #(pub #parameters),*
                            }

                        };

                        instructions_data.push(ix_data);
                        instructions_data
                    },
                )
                .into_iter();

            let fuzzer_module: syn::ItemMod = parse_quote! {
                pub mod #fuzz_instructions_module_name {
                    use trdelnik_client::fuzzing::*;

                    #[derive(Arbitrary, Clone)]
                    pub enum FuzzInstruction {
                        #(#instructions),*
                    }

                    #(#instructions_data)*
                }
            };
            fuzzer_module.into_token_stream().to_string()
        })
        .collect::<String>();
    output.push_str(&code);
    output
}
