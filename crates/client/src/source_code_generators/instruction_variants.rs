use convert_case::{Case, Casing};
use syn::{parse_quote, parse_str};
use trident_idl_spec::Idl;

// Generate instruction variants for the enum
pub(crate) fn get_instruction_variants(idl: &Idl) -> Vec<syn::Variant> {
    let _program_name = idl.metadata.name.to_case(Case::UpperCamel);

    idl.instructions
        .iter()
        .fold(Vec::new(), |mut variants, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);

            let instruction_struct_name: syn::Ident = parse_str(&instruction_name).unwrap();
            let variant: syn::Variant = parse_quote! {
                #instruction_struct_name(#instruction_struct_name)
            };

            variants.push(variant);
            variants
        })
}
