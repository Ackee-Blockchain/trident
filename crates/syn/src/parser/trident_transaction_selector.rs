use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput};

use crate::types::trident_transaction_selector::TridentSelectorEnum;
use crate::types::trident_transaction_selector::TridentSelectorVariant;

pub fn parse_trident_selector(input: &DeriveInput) -> ParseResult<TridentSelectorEnum> {
    let ident = input.ident.clone();

    let variants = match &input.data {
        Data::Enum(enum_data) => enum_data
            .variants
            .iter()
            .map(|variant| {
                Ok(TridentSelectorVariant {
                    ident: variant.ident.clone(),
                })
            })
            .collect::<ParseResult<Vec<_>>>(),
        _ => Err(ParseError::new(
            input.span(),
            "Selector can only be derived for enums",
        )),
    }?;

    Ok(TridentSelectorEnum { ident, variants })
}
