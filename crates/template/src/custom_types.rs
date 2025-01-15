use std::collections::HashMap;

use quote::format_ident;
use syn::{parse_quote, Variant};
use trident_idl_spec::{
    idl_type_to_syn_type, Idl, IdlDefinedFields, IdlEnumVariant, IdlField, IdlType, IdlTypeDef,
    IdlTypeDefTy,
};

// New function to generate custom types
pub(crate) fn get_types(idl: &Idl, program_accounts: HashMap<String, Vec<u8>>) -> Vec<syn::Item> {
    idl.types.iter().fold(Vec::new(), |mut types, type_def| {
        match &type_def.ty {
            IdlTypeDefTy::Struct {
                fields: struct_fields,
            } => {
                process_struct(type_def, struct_fields, &mut types, &program_accounts);
            }
            IdlTypeDefTy::Enum {
                variants: enum_variants,
            } => {
                process_enum(type_def, enum_variants, &mut types, &program_accounts);
            }
            IdlTypeDefTy::Type { alias: _ } => process_type(),
        }
        types
    })
}

fn process_struct(
    type_def: &IdlTypeDef,
    struct_fields: &Option<IdlDefinedFields>,
    types: &mut Vec<syn::Item>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let is_program_account = program_accounts.get(&type_def.name);

    let type_name = &type_def.name;
    let type_ident = format_ident!("{}", type_name);

    match struct_fields {
        // If there are fields, we need to process them
        Some(fields) => match fields {
            IdlDefinedFields::Named(idl_fields) => {
                process_struct_named(type_def, idl_fields, types, program_accounts);
            }
            IdlDefinedFields::Tuple(idl_types) => {
                process_struct_tuple(type_def, idl_types, types, program_accounts);
            }
        },
        // If there are no fields, we need to create an empty struct
        None => {
            let type_item: syn::Item = match is_program_account {
                Some(_) => {
                    parse_quote! {
                        #[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
                        pub struct #type_ident;
                    }
                }
                None => {
                    parse_quote! {
                        #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                        pub struct #type_ident;
                    }
                }
            };

            types.push(type_item);
        }
    }
}

fn process_enum(
    type_def: &IdlTypeDef,
    enum_variants: &[IdlEnumVariant],
    types: &mut Vec<syn::Item>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let enum_variants = enum_variants
        .iter()
        .fold(Vec::new(), |mut variants, variant| {
            match &variant.fields {
                // we process variant with fields
                Some(fields) => match fields {
                    // process named fields
                    IdlDefinedFields::Named(idl_fields) => {
                        process_enum_variant_fields_named(
                            type_def,
                            idl_fields,
                            variant,
                            &mut variants,
                            program_accounts,
                        );
                    }
                    // process tuple fields
                    IdlDefinedFields::Tuple(idl_types) => {
                        process_enum_variant_fields_tuple(
                            type_def,
                            idl_types,
                            variant,
                            &mut variants,
                            program_accounts,
                        );
                    }
                },
                // we process empty variant
                None => {
                    process_empty_variant(variant, &mut variants);
                }
            }

            variants
        });

    let type_name = &type_def.name;
    let type_ident = format_ident!("{}", type_name);

    let type_item: syn::Item = parse_quote! {
        #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
        pub enum #type_ident {
            #(#enum_variants),*
        }
    };
    types.push(type_item);
}

fn process_type() {
    panic!("Parsing Type is not implemented yet")
}

fn process_struct_named(
    type_def: &IdlTypeDef,
    idl_fields: &[IdlField],
    types: &mut Vec<syn::Item>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let is_program_account = program_accounts.get(&type_def.name);

    let type_name = &type_def.name;
    let type_ident = format_ident!("{}", type_name);

    let fields = idl_fields
        .iter()
        .fold(Vec::new(), |mut named_fields, field| {
            // process each field in struct
            process_struct_field(field, &mut named_fields, is_program_account);
            named_fields
        });

    // if the struct corresponds to a program account we do not add arbitrary
    let struct_definition: syn::Item = match is_program_account {
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

fn process_struct_field(
    field: &IdlField,
    named_fields: &mut Vec<syn::FnArg>,
    is_program_account: Option<&Vec<u8>>,
) {
    let field_name = &field.name;
    let field_ident = format_ident!("{}", field_name);

    let (field_type, _is_custom) = match is_program_account {
        // if it is program account Public Keys are not turned into AccountIds
        Some(_) => idl_type_to_syn_type(&field.ty, 0, false),
        // if it is not program account Public Keys are turned into AccountIds
        None => idl_type_to_syn_type(&field.ty, 0, true),
    };

    // we create the field
    let field: syn::FnArg = parse_quote!(#field_ident: #field_type);

    // we add the field to the struct
    named_fields.push(field);
}

fn process_struct_tuple(
    type_def: &IdlTypeDef,
    idl_types: &[IdlType],
    types: &mut Vec<syn::Item>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let is_program_account = program_accounts.get(&type_def.name);

    let type_name = &type_def.name;
    let type_ident = format_ident!("{}", type_name);

    let tuple_fields: Vec<syn::Type> = idl_types
        .iter()
        .map(|idl_type| {
            // processing each IDL Type in the tuple
            let (field_type, _is_custom) = match is_program_account {
                Some(_) => idl_type_to_syn_type(idl_type, 0, false),
                None => idl_type_to_syn_type(idl_type, 0, true),
            };
            field_type
        })
        .collect();

    // if the struct corresponds to a program account we do not add arbitrary
    let struct_definition: syn::Item = match is_program_account {
        Some(_) => parse_quote! {
            #[derive(Debug, BorshDeserialize, BorshSerialize)]
            struct #type_ident(#(pub #tuple_fields),*);
        },
        None => parse_quote! {
            #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
            struct #type_ident(#(pub #tuple_fields),*);
        },
    };

    types.push(struct_definition);
}

fn process_enum_variant_fields_named(
    type_def: &IdlTypeDef,
    idl_fields: &[IdlField],
    variant: &IdlEnumVariant,
    variants: &mut Vec<Variant>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let is_program_account = program_accounts.get(&type_def.name);

    let variant_name = &variant.name;
    let variant_ident = format_ident!("{}", variant_name);

    let fields = idl_fields
        .iter()
        .fold(Vec::new(), |mut named_fields, field| {
            // process each named field, similar to struct fields
            process_struct_field(field, &mut named_fields, is_program_account);
            named_fields
        });

    // we create the variant
    let variant = parse_quote!(#variant_ident { #(#fields),* });

    // we add the variant to the enum
    variants.push(variant);
}
fn process_enum_variant_fields_tuple(
    type_def: &IdlTypeDef,
    idl_types: &[IdlType],
    variant: &IdlEnumVariant,
    variants: &mut Vec<Variant>,
    program_accounts: &HashMap<String, Vec<u8>>,
) {
    let is_program_account = program_accounts.get(&type_def.name);

    let variant_name = &variant.name;
    let variant_ident = format_ident!("{}", variant_name);

    let tuple_fields: Vec<syn::Type> = idl_types
        .iter()
        .map(|idl_type| {
            // processing each IDL Type in the tuple
            let (syn_type, _is_custom) = match is_program_account {
                Some(_) => idl_type_to_syn_type(idl_type, 0, false),
                None => idl_type_to_syn_type(idl_type, 0, true),
            };
            syn_type
        })
        .collect();
    let variant = parse_quote!(#variant_ident(#(#tuple_fields),*));
    variants.push(variant);
}

fn process_empty_variant(variant: &IdlEnumVariant, variants: &mut Vec<Variant>) {
    let variant_name = &variant.name;
    let variant_ident = format_ident!("{}", variant_name);

    let variant: syn::Variant = parse_quote!(#variant_ident);
    variants.push(variant);
}
