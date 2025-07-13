use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

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
    let client_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|id| id == "client"))
        .ok_or_else(|| ParseError::new(input.span(), "Struct must contain a 'client' field"))?
        .ident
        .as_ref()
        .unwrap()
        .clone();

    let metrics_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|id| id == "metrics"))
        .ok_or_else(|| ParseError::new(input.span(), "Struct must contain a 'metrics' field"))?
        .ident
        .as_ref()
        .unwrap()
        .clone();

    let rng_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|id| id == "rng"))
        .ok_or_else(|| ParseError::new(input.span(), "Struct must contain an 'rng' field"))?
        .ident
        .as_ref()
        .unwrap()
        .clone();

    Ok(TridentFuzzTestMethodsStruct {
        ident,
        client_field,
        metrics_field,
        rng_field,
    })
}
