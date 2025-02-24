use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput};

use crate::types::trident_fuzz_test_executor::{
    TridentFuzzTestExecutorEnum, TridentFuzzTestExecutorVariant,
};

pub fn parse_trident_fuzz_test_executor(
    input: &DeriveInput,
) -> ParseResult<TridentFuzzTestExecutorEnum> {
    let ident = input.ident.clone();

    let variants = match &input.data {
        Data::Enum(enum_data) => enum_data
            .variants
            .iter()
            .map(|variant| {
                Ok(TridentFuzzTestExecutorVariant {
                    ident: variant.ident.clone(),
                })
            })
            .collect::<ParseResult<Vec<_>>>(),
        _ => Err(ParseError::new(
            input.span(),
            "FuzzTestExecutor can only be derived for enums",
        )),
    }?;

    Ok(TridentFuzzTestExecutorEnum { ident, variants })
}
