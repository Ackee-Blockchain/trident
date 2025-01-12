use quote::format_ident;
use syn::parse_quote;

use super::{idl_type_to_syn_type, types::Idl};

// New function to generate custom types
pub(crate) fn get_types(idl: &Idl) -> Vec<syn::Item> {
    idl.types.iter().fold(Vec::new(), |mut types, type_def| {
        match &type_def.ty {
            super::types::IdlTypeDefinitionTy::Struct { fields } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                let fields = fields.iter().fold(Vec::new(), |mut named_fields, field| {
                    let field_name = &field.name;
                    let field_ident = format_ident!("{}", field_name);

                    let (field_type, _is_custom) = idl_type_to_syn_type(&field.ty, 0);

                    let field: syn::FnArg = parse_quote!(#field_ident: #field_type);

                    named_fields.push(field);
                    named_fields
                });
                let struct_definition: syn::Item = parse_quote! {
                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                    pub struct #type_ident {
                        #(#fields),*
                    }
                };
                types.push(struct_definition);
            }
            super::types::IdlTypeDefinitionTy::Enum { variants } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                let enum_variants = variants.iter().fold(Vec::new(), |mut variants, variant| {
                    let variant_name = &variant.name;
                    let variant_ident = format_ident!("{}", variant_name);

                    match &variant.fields {
                        Some(fields) => match fields {
                            super::types::EnumFields::Named(vec) => {
                                let fields =
                                    vec.iter().fold(Vec::new(), |mut named_fields, field| {
                                        let field_name = &field.name;
                                        let (field_type, _is_custom) =
                                            idl_type_to_syn_type(&field.ty, 0);
                                        let field: syn::FnArg =
                                            parse_quote!(#field_name: #field_type);
                                        named_fields.push(field);
                                        named_fields
                                    });
                                let variant: syn::Variant =
                                    parse_quote!(#variant_name{#(#fields),*},);
                                variants.push(variant);
                            }
                            super::types::EnumFields::Tuple(vec) => {
                                let tuple_fields: Vec<syn::Type> = vec
                                    .iter()
                                    .map(|idl_type| {
                                        let (syn_type, _is_custom) =
                                            idl_type_to_syn_type(idl_type, 0);
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
            super::types::IdlTypeDefinitionTy::Alias { value: _ } => todo!(),
        }
        types
    })
}

// New function to generate custom types
pub(crate) fn get_accounts(idl: &Idl) -> Vec<syn::Item> {
    idl.accounts.iter().fold(Vec::new(), |mut types, type_def| {
        match &type_def.ty {
            super::types::IdlTypeDefinitionTy::Struct { fields } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                let fields = fields.iter().fold(Vec::new(), |mut named_fields, field| {
                    let field_name = &field.name;
                    let field_ident = format_ident!("{}", field_name);

                    let (field_type, _is_custom) = idl_type_to_syn_type(&field.ty, 0);

                    let field: syn::FnArg = parse_quote!(#field_ident: #field_type);

                    named_fields.push(field);
                    named_fields
                });
                let struct_definition: syn::Item = parse_quote! {
                    #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                    pub struct #type_ident {
                        #(#fields),*
                    }
                };
                types.push(struct_definition);
            }
            super::types::IdlTypeDefinitionTy::Enum { variants } => {
                let type_name = &type_def.name;
                let type_ident = format_ident!("{}", type_name);

                let enum_variants = variants.iter().fold(Vec::new(), |mut variants, variant| {
                    let variant_name = &variant.name;
                    let variant_ident = format_ident!("{}", variant_name);

                    match &variant.fields {
                        Some(fields) => match fields {
                            super::types::EnumFields::Named(vec) => {
                                let fields =
                                    vec.iter().fold(Vec::new(), |mut named_fields, field| {
                                        let field_name = &field.name;
                                        let (field_type, _is_custom) =
                                            idl_type_to_syn_type(&field.ty, 0);
                                        let field: syn::FnArg =
                                            parse_quote!(#field_name: #field_type);
                                        named_fields.push(field);
                                        named_fields
                                    });
                                let variant: syn::Variant =
                                    parse_quote!(#variant_name{#(#fields),*},);
                                variants.push(variant);
                            }
                            super::types::EnumFields::Tuple(vec) => {
                                let tuple_fields: Vec<syn::Type> = vec
                                    .iter()
                                    .map(|idl_type| {
                                        let (syn_type, _is_custom) =
                                            idl_type_to_syn_type(idl_type, 0);
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
            super::types::IdlTypeDefinitionTy::Alias { value: _ } => todo!(),
        }
        types
    })
}
