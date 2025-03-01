use syn::parse::{Error, Result as ParseResult};
use syn::{Data, DeriveInput, Fields};

use crate::types::trident_fuzz_test_executor::TridentFuzzTestExecutor;

pub fn parse_trident_fuzz_test_executor(
    input: &DeriveInput,
) -> ParseResult<TridentFuzzTestExecutor> {
    // Extract the client field from the struct
    let client_type = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // Find the client field
                    let client_field = fields
                        .named
                        .iter()
                        .find(|field| {
                            field
                                .ident
                                .as_ref()
                                .map_or(false, |ident| ident == "client")
                        })
                        .ok_or_else(|| {
                            Error::new_spanned(
                                &input.ident,
                                "FuzzTestExecutor requires a 'client' field",
                            )
                        })?;

                    client_field.ty.clone()
                }
                _ => {
                    return Err(Error::new_spanned(
                        &input.ident,
                        "FuzzTestExecutor can only be derived for structs with named fields",
                    ))
                }
            }
        }
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "FuzzTestExecutor can only be derived for structs",
            ))
        }
    };

    Ok(TridentFuzzTestExecutor {
        ident: input.ident.clone(),
        generics: input.generics.clone(),
        client_type,
    })
}
