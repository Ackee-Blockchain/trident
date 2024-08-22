use std::collections::HashMap;

use syn::{parse_quote, parse_str};

use convert_case::{Case, Casing};
use quote::{format_ident, ToTokens};

use anchor_lang_idl_spec::{Idl, IdlInstruction, IdlInstructionAccountItem, IdlType};

pub fn generate_source_code(idls: &[Idl]) -> String {
    let code = idls
        .iter()
        .map(|idl| {
            let program_name = idl.metadata.name.to_case(Case::Snake);
            let fuzz_instructions_module_name = format_ident!("{}_fuzz_instructions", program_name);

            let instructions = idl.instructions.iter().map(|instruction| {
                let instruction_name = instruction.name.to_case(Case::UpperCamel);
                let instruction_struct_name: syn::Ident = parse_str(&instruction_name).unwrap();

                // Generate enum variant for the instruction
                let enum_variant: syn::Variant = parse_quote! {
                    #instruction_struct_name(#instruction_struct_name)
                };

                enum_variant
            });

            let instruction_inputs = get_instruction_inputs(&idl.instructions);
            let instructions_ixops_impls = get_instruction_ixops(&idl.instructions, program_name);
            let fuzz_accounts = get_fuzz_accounts(&idl.instructions);
            let snapshot_types = get_snapshot_types(&idl.instructions);

            let module_definition: syn::ItemMod = parse_quote! {
                pub mod #fuzz_instructions_module_name {
                    use trident_client::fuzzing::*;

                    #(#snapshot_types)*

                    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
                    pub enum FuzzInstruction {
                        #(#instructions),*
                    }

                    #(#instruction_inputs)*

                    #(#instructions_ixops_impls)*

                    // FIX this is just a workaround to propagate a comment to the source code easily
                    /// Use AccountsStorage<T> where T can be one of:
                    /// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
                    #[derive(Default)]
                    pub struct FuzzAccounts {
                        #(#fuzz_accounts),*
                    }

                }
            };

            module_definition.into_token_stream().to_string()
        })
        .collect::<String>();

    code
}

fn get_snapshot_types(instructions: &[IdlInstruction]) -> Vec<syn::ItemType> {
    instructions
        .iter()
        .fold(Vec::new(), |mut snapshot_types, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            let ix_snapshot: syn::Ident = format_ident!("{}Snapshot", &instruction_name);
            let ix_alias: syn::Ident = format_ident!("{}Alias", &instruction_name);

            let snapshot_type: syn::ItemType =
                parse_quote!(type #ix_snapshot<'info> = #ix_alias<'info>;);

            snapshot_types.push(snapshot_type);
            snapshot_types
        })
}

fn get_instruction_inputs(instructions: &[IdlInstruction]) -> Vec<syn::ItemStruct> {
    instructions
        .iter()
        .fold(Vec::new(), |mut instructions_data, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            let instruction_name_ident: syn::Ident = format_ident!("{}", &instruction_name);

            let instruction_data_name: syn::Ident = format_ident!("{}Data", &instruction_name);

            let instruction_accounts_name: syn::Ident =
                format_ident!("{}Accounts", &instruction_name);

            let accounts = instruction
                .accounts
                .iter()
                .map(|account| match account {
                    IdlInstructionAccountItem::Composite(_composite) => {
                        panic!("Composite Accounts are not supported yet!")
                    }
                    IdlInstructionAccountItem::Single(single) => {
                        let name = format_ident!("{}", single.name);
                        let account: syn::FnArg = parse_quote!(#name: AccountId);
                        account
                    }
                })
                .collect::<Vec<_>>();

            let parameters = instruction
                .args
                .iter()
                .map(|arg| {
                    let arg_name = format_ident!("{}", arg.name);

                    let arg_type = idl_type_to_syn_type(&arg.ty);

                    let parameter: syn::FnArg = parse_quote!(#arg_name: #arg_type);
                    parameter
                })
                .collect::<Vec<_>>();

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
                #[derive(Arbitrary, Debug)]
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

fn get_instruction_ixops(
    instructions: &[IdlInstruction],
    program_name: String,
) -> Vec<syn::ItemImpl> {
    let module_name: syn::Ident = parse_str(&program_name).unwrap();

    instructions
        .iter()
        .fold(Vec::new(), |mut instructions_ixops_impl, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            let instruction_ident_name: syn::Ident = format_ident!("{}", &instruction_name);

            let ix_snapshot: syn::Ident = format_ident!("{}Snapshot", &instruction_name);

            let parameters = instruction
                .args
                .iter()
                .map(|arg| {
                    let arg_name = format_ident!("{}", arg.name);

                    let parameter: syn::FieldValue = match arg.ty {
                        IdlType::Pubkey => parse_quote!(#arg_name: todo!()),
                        IdlType::String => {
                            let arg_value: syn::Expr = parse_quote!(self.data.#arg_name.clone());
                            parse_quote!(#arg_name: #arg_value)
                        }
                        IdlType::Bytes => {
                            let arg_value: syn::Expr = parse_quote!(self.data.#arg_name.clone());
                            parse_quote!(#arg_name: #arg_value)
                        }
                        IdlType::Vec(_) => {
                            let arg_value: syn::Expr = parse_quote!(self.data.#arg_name.clone());
                            parse_quote!(#arg_name: #arg_value)
                        }
                        IdlType::Defined {
                            name: _,
                            generics: _,
                        } => parse_quote!(#arg_name: todo!()),
                        _ => {
                            let arg_value: syn::Expr = parse_quote!(self.data.#arg_name);
                            parse_quote!(#arg_name: #arg_value)
                        }
                    };

                    parameter
                })
                .collect::<Vec<_>>();

            let ix_impl: syn::ItemImpl = parse_quote! {
                impl<'info> IxOps<'info> for #instruction_ident_name {
                    type IxData = #module_name::instruction::#instruction_ident_name;
                    type IxAccounts = FuzzAccounts;
                    type IxSnapshot = #ix_snapshot<'info>;

                    fn get_data(
                        &self,
                        _client: &mut impl FuzzClient,
                        _fuzz_accounts: &mut FuzzAccounts,
                    ) -> Result<Self::IxData, FuzzingError> {
                        let data = #module_name::instruction::#instruction_ident_name {
                            #(#parameters),*
                        };
                        Ok(data)
                    }

                    fn get_accounts(
                    &self,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut FuzzAccounts,
                    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
                        let signers = vec![todo!()];
                        let acc_meta = todo!();

                        Ok((signers, acc_meta))
                    }
                }

            };

            instructions_ixops_impl.push(ix_impl);
            instructions_ixops_impl
        })
}

fn get_fuzz_accounts(instructions: &[IdlInstruction]) -> Vec<syn::FnArg> {
    let fuzz_accounts = instructions.iter().fold(
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
                            let name: syn::Ident = format_ident!("{}", &single.name);
                            let account = match single.pda {
                                Some(_) => parse_quote! { #name: AccountsStorage<PdaStore> },
                                None => parse_quote! { #name: AccountsStorage<todo!()> },
                            };
                            fuzz_accounts.entry(name).or_insert(account);
                        }
                    };
                    fuzz_accounts
                });
            fuzz_accounts
        },
    );
    let mut sorted_accounts: Vec<_> = fuzz_accounts.into_iter().collect();

    sorted_accounts.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    // Extract the FnArg values into a Vec
    sorted_accounts.into_iter().map(|(_, v)| v).collect()
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
        IdlType::Pubkey => parse_quote!(AccountId), // Replace with AccountId if needed
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
        IdlType::Pubkey => parse_quote!(AccountId),
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
