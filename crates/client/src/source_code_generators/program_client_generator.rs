use convert_case::{Case, Casing};
use quote::{format_ident, ToTokens};
use syn::{parse_quote, parse_str};

use anchor_lang_idl_spec::{Idl, IdlInstructionAccountItem, IdlType};

pub fn generate_source_code(idl_instructions: &[Idl], use_modules: &[syn::ItemUse]) -> String {
    let mut output = "// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module\n".to_owned();
    let code = idl_instructions
        .iter()
        .map(|idl| {
            let program_name = &idl.metadata.name.to_case(Case::Snake);
            let program_id = &idl.address;

            let instruction_module_name = format_ident!("{}_instruction", program_name);
            let module_name: syn::Ident = parse_str(program_name).unwrap();

            let instructions = idl
                .instructions
                .iter()
                .fold(Vec::new(), |mut instructions, instruction| {
                    let instruction_fn_name: syn::Ident =
                        parse_str(&instruction.name.to_case(Case::Snake)).unwrap();
                    let instruction_struct_name: syn::Ident =
                        parse_str(&instruction.name.to_case(Case::UpperCamel)).unwrap();
                    let account_struct_name: syn::Ident =
                        parse_str(&instruction.name.to_case(Case::UpperCamel)).unwrap();
                    let instruction_name: syn::Ident =
                        parse_str(&(instruction.name.to_case(Case::Snake) + "_ix")).unwrap();

                    let parameters = instruction
                        .args
                        .iter()
                        .map(|arg| {
                            let name = format_ident!("i_{}", arg.name);
                            let arg_type = idl_type_to_syn_type(&arg.ty);
                            let parameter: syn::FnArg = parse_quote!(#name: #arg_type);
                            parameter
                        })
                        .collect::<Vec<_>>();

                    let accounts = instruction
                        .accounts
                        .iter()
                        .map(|account| {
                            let inner_account = match account {
                                IdlInstructionAccountItem::Composite(_composite) => {
                                    panic!("Composite Accounts are not supported yet!")
                                }
                                IdlInstructionAccountItem::Single(single) => single,
                            };

                            let name = format_ident!("a_{}", inner_account.name);
                            let ty: syn::Type = parse_str("Pubkey").unwrap();
                            let account: syn::FnArg = parse_quote!(#name: #ty);
                            account
                        })
                        .collect::<Vec<_>>();

                    let field_parameters = instruction
                        .args
                        .iter()
                        .map(|arg| {
                            let name: syn::Ident = parse_str(&arg.name).unwrap();
                            let value = format_ident!("i_{name}");
                            let parameter: syn::FieldValue = parse_quote!(#name: #value);
                            parameter
                        })
                        .collect::<Vec<_>>();

                    let field_accounts = instruction
                        .accounts
                        .iter()
                        .map(|account| {
                            let inner_account = match account {
                                IdlInstructionAccountItem::Composite(_composite) => {
                                    panic!("Composite Accounts are not supported yet!")
                                }
                                IdlInstructionAccountItem::Single(single) => single,
                            };

                            let name: syn::Ident = parse_str(&inner_account.name).unwrap();
                            let value = format_ident!("a_{name}");
                            let account: syn::FieldValue = parse_quote!(#name: #value);
                            account
                        })
                        .collect::<Vec<_>>();

                    let instruction: syn::ItemFn = parse_quote! {
                        pub async fn #instruction_fn_name(
                            client: &Client,
                            #(#parameters,)*
                            #(#accounts,)*
                            signers: impl IntoIterator<Item = Keypair> + Send + 'static,
                        ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
                            client.send_instruction(
                                PROGRAM_ID,
                                #module_name::instruction::#instruction_struct_name {
                                    #(#field_parameters,)*
                                },
                                #module_name::accounts::#account_struct_name {
                                    #(#field_accounts,)*
                                },
                                signers,
                            ).await
                        }
                    };

                    let instruction_raw: syn::ItemFn = parse_quote! {
                        pub  fn #instruction_name(
                            #(#parameters,)*
                            #(#accounts,)*
                        ) -> Instruction {
                            Instruction{
                                program_id: PROGRAM_ID,
                                data: #module_name::instruction::#instruction_struct_name {
                                    #(#field_parameters,)*
                                }.data(),
                                accounts: #module_name::accounts::#account_struct_name {
                                    #(#field_accounts,)*
                                }.to_account_metas(None),
                            }
                        }
                    };

                    instructions.push(instruction);
                    instructions.push(instruction_raw);
                    instructions
                })
                .into_iter();

            let program_module: syn::ItemMod = parse_quote! {
                pub mod #instruction_module_name {
                    #(#use_modules)*
                    pub const PROGRAM_ID: Pubkey = pubkey!(#program_id);

                    // pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array(#pubkey_bytes);
                    #(#instructions)*
                }
            };
            program_module.into_token_stream().to_string()
        })
        .collect::<String>();
    output.push_str(&code);
    output
}

/// Converts an `IdlType` to a corresponding Rust `syn::Type`.
fn idl_type_to_syn_type(idl_type: &IdlType) -> syn::Type {
    match idl_type {
        IdlType::Bool => parse_quote!(bool),
        IdlType::U8 => parse_quote!(u8),
        IdlType::I8 => parse_quote!(i8),
        IdlType::U16 => parse_quote!(u16),
        IdlType::I16 => parse_quote!(i16),
        IdlType::U32 => parse_quote!(u32),
        IdlType::I32 => parse_quote!(i32),
        IdlType::F32 => parse_quote!(f32),
        IdlType::U64 => parse_quote!(u64),
        IdlType::I64 => parse_quote!(i64),
        IdlType::F64 => parse_quote!(f64),
        IdlType::U128 => parse_quote!(u128),
        IdlType::I128 => parse_quote!(i128),
        IdlType::U256 => parse_quote!(u256), // Assuming custom type for u256
        IdlType::I256 => parse_quote!(i256), // Assuming custom type for i256
        IdlType::Bytes => parse_quote!(Vec<u8>),
        IdlType::String => parse_quote!(String),
        IdlType::Pubkey => parse_quote!(Pubkey), // Replace with AccountId if needed
        IdlType::Option(inner) => {
            let inner_type = get_inner_type(inner, 0);
            parse_quote!(Option<#inner_type>)
        }
        IdlType::Vec(inner) => {
            let inner_type = get_inner_type(inner, 0);
            parse_quote!(Vec<#inner_type>)
        }
        IdlType::Array(inner, len) => {
            let inner_type = get_inner_type(inner, 0);

            let len = match len {
                anchor_lang_idl_spec::IdlArrayLen::Generic(_generic) => {
                    panic!("Generic within Array len not supported")
                }
                anchor_lang_idl_spec::IdlArrayLen::Value(len) => len,
            };
            parse_quote!([#inner_type;#len])
        }
        // TODO try to automatically generate the struct so we can simply
        // derive arbitrary
        IdlType::Defined { name, generics: _ } => {
            let name_ident: syn::Ident = format_ident!("{}", &name);
            parse_quote!(#name_ident)
        }
        IdlType::Generic(_name) => {
            panic!("Generic currently not supported")
        }
        _ => todo!(),
    }
}

fn get_inner_type(idl_type: &IdlType, nestings: u8) -> syn::Type {
    if nestings >= 5 {
        panic!("No more than 5 nestings allowed");
    }
    match idl_type {
        IdlType::Bool => parse_quote!(bool),
        IdlType::U8 => parse_quote!(u8),
        IdlType::I8 => parse_quote!(i8),
        IdlType::U16 => parse_quote!(u16),
        IdlType::I16 => parse_quote!(i16),
        IdlType::U32 => parse_quote!(u32),
        IdlType::I32 => parse_quote!(i32),
        IdlType::F32 => parse_quote!(f32),
        IdlType::U64 => parse_quote!(u64),
        IdlType::I64 => parse_quote!(i64),
        IdlType::F64 => parse_quote!(f64),
        IdlType::U128 => parse_quote!(u128),
        IdlType::I128 => parse_quote!(i128),
        IdlType::U256 => parse_quote!(u256),
        IdlType::I256 => parse_quote!(i256),
        IdlType::Bytes => parse_quote!(Vec<u8>),
        IdlType::String => parse_quote!(String),
        IdlType::Pubkey => parse_quote!(Pubkey),
        IdlType::Option(inner) => {
            let inner_type = get_inner_type(inner, nestings + 1);
            parse_quote!(Option<#inner_type>)
        }
        IdlType::Vec(inner) => {
            let inner_type = get_inner_type(inner, nestings + 1);
            parse_quote!(Vec<#inner_type>)
        }
        IdlType::Array(inner, len) => {
            let inner_type = get_inner_type(inner, nestings + 1);

            let len = match len {
                anchor_lang_idl_spec::IdlArrayLen::Generic(_generic) => {
                    panic!("Generic within Array len not supported")
                }
                anchor_lang_idl_spec::IdlArrayLen::Value(len) => len,
            };
            parse_quote!([#inner_type;#len])
        }
        IdlType::Defined {
            name: _,
            generics: _,
        } => {
            panic!("Defined not supported")
        }
        IdlType::Generic(_name) => {
            panic!("Generic not supported")
        }
        _ => todo!(),
    }
}
