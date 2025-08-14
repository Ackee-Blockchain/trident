use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;

use crate::types::trident_fuzz_test_methods::TridentFuzzTestMethodsStruct;

pub fn parse_trident_fuzz_test_methods(
    input: &DeriveInput,
) -> ParseResult<TridentFuzzTestMethodsStruct> {
    let ident = input.ident.clone();

    // Get the struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(ParseError::new(
                    input.span(),
                    "FuzzTestMethods can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(ParseError::new(
                input.span(),
                "FuzzTestMethods can only be derived for structs",
            ))
        }
    };

    // Find required fields
    let trident_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|id| id == "trident"))
        .ok_or_else(|| ParseError::new(input.span(), "Struct must contain a 'trident' field"))?
        .ident
        .as_ref()
        .unwrap()
        .clone();

    Ok(TridentFuzzTestMethodsStruct {
        ident,
        trident_field,
    })
}
