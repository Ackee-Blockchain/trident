use std::collections::HashMap;

use quote::format_ident;
use syn::parse_quote;
use trident_idl_spec::{idl_type_to_syn_type, Idl, IdlDefinedFields, IdlTypeDefTy};

// New function to generate custom types
pub(crate) fn get_types(idl: &Idl, program_accounts: HashMap<String, Vec<u8>>) -> Vec<syn::Item> {
    idl.types.iter().fold(Vec::new(), |mut types, type_def| {
        let is_program_account = program_accounts.get(&type_def.name);

        match &type_def.ty {
            IdlTypeDefTy::Struct { fields } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                match fields {
                    Some(non_empty) => match non_empty {
                        IdlDefinedFields::Named(vec) => {
                            let fields = vec.iter().fold(Vec::new(), |mut named_fields, field| {
                                let field_name = &field.name;
                                let field_ident = format_ident!("{}", field_name);

                                let (field_type, _is_custom) = match is_program_account {
                                    Some(_) => idl_type_to_syn_type(&field.ty, 0, false),
                                    None => idl_type_to_syn_type(&field.ty, 0, true),
                                };

                                let field: syn::FnArg = parse_quote!(#field_ident: #field_type);

                                named_fields.push(field);
                                named_fields
                            });
                            let struct_definition:syn::Item = match is_program_account{
                                Some(_) => parse_quote! {
                                    #[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
                                    pub struct #type_ident {
                                        #(#fields),*
                                    }
                                },
                                None => parse_quote! {
                                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                                    pub struct #type_ident {
                                        #(#fields),*
                                    }
                                },
                            };
                            types.push(struct_definition);
                        }
                        IdlDefinedFields::Tuple(vec) => {
                            let tuple_fields: Vec<syn::Type> = vec
                                .iter()
                                .map(|idl_type| {
                                    let (field_type, _is_custom) = match is_program_account {
                                        Some(_) => idl_type_to_syn_type(idl_type, 0, false),
                                        None => idl_type_to_syn_type(idl_type, 0, true),
                                    };
                                    field_type
                                })
                                .collect();

                            let struct_definition:syn::Item = match is_program_account{
                                Some(_) => parse_quote! {
                                    #[derive(Debug, BorshDeserialize, BorshSerialize)]
                                    struct #type_ident(#(pub #tuple_fields),*);
                                },
                                None => parse_quote! {
                                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
                                    struct #type_ident(#(pub #tuple_fields),*);
                                },
                            };

                            types.push(struct_definition);
                        }
                    },
                    None => {
                        let type_item:syn::Item = match is_program_account{
                            Some(_) => {
                                parse_quote! {
                                    #[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
                                    pub struct #type_ident {}
                                }
                            },
                            None => {
                                parse_quote! {
                                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                                    pub struct #type_ident {}
                                }
                            },
                        };

                        types.push(type_item);
                    }
                }
            }
            IdlTypeDefTy::Enum { variants } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                let enum_variants = variants.iter().fold(Vec::new(), |mut variants, variant| {
                    let variant_name = &variant.name;
                    let variant_ident = format_ident!("{}", variant_name);

                    match &variant.fields {
                        Some(fields) => match fields {
                            IdlDefinedFields::Named(vec) => {
                                let fields =
                                    vec.iter().fold(Vec::new(), |mut named_fields, field| {
                                        let field_name = &field.name;
                                        let (field_type, _is_custom) = match is_program_account {
                                            Some(_) => idl_type_to_syn_type(&field.ty, 0, false),
                                            None => idl_type_to_syn_type(&field.ty, 0, true),
                                        };
                                        let field: syn::FnArg =
                                            parse_quote!(#field_name: #field_type);
                                        named_fields.push(field);
                                        named_fields
                                    });
                                let variant: syn::Variant =
                                    parse_quote!(#variant_name{#(#fields),*},);
                                variants.push(variant);
                            }
                            IdlDefinedFields::Tuple(vec) => {
                                let tuple_fields: Vec<syn::Type> = vec
                                    .iter()
                                    .map(|idl_type| {
                                        let (syn_type, _is_custom) = match is_program_account {
                                            Some(_) => idl_type_to_syn_type(idl_type, 0, false),
                                            None => idl_type_to_syn_type(idl_type, 0, true),
                                        };
                                        syn_type
                                    })
                                    .collect();
                                let variant = parse_quote!(#variant_name(#(#tuple_fields),*),);
                                variants.push(variant);
                            }
                        },
                        None => {
                            let variant: syn::Variant = parse_quote!(#variant_ident);
                            variants.push(variant);
                        }
                    }

                    variants
                });

                let type_item: syn::Item = parse_quote! {
                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                    pub enum #type_ident {
                        #(#enum_variants),*
                    }
                };
                types.push(type_item);
            }
            IdlTypeDefTy::Type { alias: _ } => todo!(),
        }
        types
    })
}
