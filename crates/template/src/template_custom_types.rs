use quote::format_ident;
use syn::{parse_quote, Variant};
use trident_idl_spec::{
    Idl, IdlDefinedFields, IdlEnumVariant, IdlField, IdlType, IdlTypeDef, IdlTypeDefTy,
};

use crate::idl_type_to_syn_type;

use crate::Template;

impl Template {
    pub(crate) fn custom_types(&mut self, idl: &Idl) {
        // New function to generate custom types
        idl.types.iter().for_each(|type_def| match &type_def.ty {
            IdlTypeDefTy::Struct {
                fields: struct_fields,
            } => {
                self.process_struct(type_def, struct_fields);
            }
            IdlTypeDefTy::Enum {
                variants: enum_variants,
            } => {
                self.process_enum(type_def, enum_variants);
            }
            IdlTypeDefTy::Type { alias: _ } => self.process_type(),
        });
    }
    fn process_struct(&mut self, type_def: &IdlTypeDef, struct_fields: &Option<IdlDefinedFields>) {
        let type_name = &type_def.name;
        let type_ident = format_ident!("{}", type_name);

        match struct_fields {
            // If there are fields, we need to process them
            Some(fields) => match fields {
                IdlDefinedFields::Named(idl_fields) => {
                    self.process_struct_named(type_def, idl_fields);
                }
                IdlDefinedFields::Tuple(idl_types) => {
                    self.process_struct_tuple(type_def, idl_types);
                }
            },
            // If there are no fields, we need to create an empty struct
            None => {
                let type_item: syn::Item = {
                    parse_quote! {
                        #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                        pub struct #type_ident;
                    }
                };

                self.custom_types.push(type_item);
            }
        }
    }

    fn process_enum(&mut self, type_def: &IdlTypeDef, enum_variants: &[IdlEnumVariant]) {
        let enum_variants = enum_variants
            .iter()
            .fold(Vec::new(), |mut variants, variant| {
                match &variant.fields {
                    // we process variant with fields
                    Some(fields) => match fields {
                        // process named fields
                        IdlDefinedFields::Named(idl_fields) => {
                            self.process_enum_variant_fields_named(
                                idl_fields,
                                variant,
                                &mut variants,
                            );
                        }
                        // process tuple fields
                        IdlDefinedFields::Tuple(idl_types) => {
                            self.process_enum_variant_fields_tuple(
                                idl_types,
                                variant,
                                &mut variants,
                            );
                        }
                    },
                    // we process empty variant
                    None => {
                        self.process_empty_variant(variant, &mut variants);
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
        self.custom_types.push(type_item);
    }

    fn process_type(&mut self) {
        panic!("Parsing Type is not implemented yet")
    }

    fn process_struct_named(&mut self, type_def: &IdlTypeDef, idl_fields: &[IdlField]) {
        let type_name = &type_def.name;
        let type_ident = format_ident!("{}", type_name);

        let fields = idl_fields
            .iter()
            .fold(Vec::new(), |mut named_fields, field| {
                // process each field in struct
                self.process_struct_field(field, &mut named_fields);
                named_fields
            });

        // if the struct corresponds to a program account we do not add arbitrary
        let struct_definition: syn::Item = parse_quote! {
                #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                pub struct #type_ident {
                    #(#fields),*
                }
        };
        self.custom_types.push(struct_definition);
    }

    fn process_struct_field(&mut self, field: &IdlField, named_fields: &mut Vec<syn::FnArg>) {
        let field_name = &field.name;
        let field_ident = format_ident!("{}", field_name);

        let (field_type, _is_custom) = idl_type_to_syn_type(&field.ty);
        // we create the field
        let field: syn::FnArg = parse_quote!(#field_ident: #field_type);

        // we add the field to the struct
        named_fields.push(field);
    }

    fn process_struct_tuple(&mut self, type_def: &IdlTypeDef, idl_types: &[IdlType]) {
        let type_name = &type_def.name;
        let type_ident = format_ident!("{}", type_name);

        let tuple_fields: Vec<syn::Type> = idl_types
            .iter()
            .map(|idl_type| {
                // processing each IDL Type in the tuple
                let (field_type, _is_custom) = idl_type_to_syn_type(idl_type);
                field_type
            })
            .collect();

        // if the struct corresponds to a program account we do not add arbitrary
        let struct_definition: syn::Item = parse_quote! {
            #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
            pub struct #type_ident(#(pub #tuple_fields),*);
        };

        self.custom_types.push(struct_definition);
    }

    fn process_enum_variant_fields_named(
        &mut self,
        idl_fields: &[IdlField],
        variant: &IdlEnumVariant,
        variants: &mut Vec<Variant>,
    ) {
        let variant_name = &variant.name;
        let variant_ident = format_ident!("{}", variant_name);

        let fields = idl_fields
            .iter()
            .fold(Vec::new(), |mut named_fields, field| {
                // process each named field, similar to struct fields
                self.process_struct_field(field, &mut named_fields);
                named_fields
            });

        // we create the variant
        let variant = parse_quote!(#variant_ident { #(#fields),* });

        // we add the variant to the enum
        variants.push(variant);
    }
    fn process_enum_variant_fields_tuple(
        &mut self,
        idl_types: &[IdlType],
        variant: &IdlEnumVariant,
        variants: &mut Vec<Variant>,
    ) {
        let variant_name = &variant.name;
        let variant_ident = format_ident!("{}", variant_name);

        let tuple_fields: Vec<syn::Type> = idl_types
            .iter()
            .map(|idl_type| {
                // processing each IDL Type in the tuple
                let (syn_type, _is_custom) = idl_type_to_syn_type(idl_type);
                syn_type
            })
            .collect();
        let variant = parse_quote!(#variant_ident(#(#tuple_fields),*));
        variants.push(variant);
    }

    fn process_empty_variant(&mut self, variant: &IdlEnumVariant, variants: &mut Vec<Variant>) {
        let variant_name = &variant.name;
        let variant_ident = format_ident!("{}", variant_name);

        let variant: syn::Variant = parse_quote!(#variant_ident);
        variants.push(variant);
    }
}
