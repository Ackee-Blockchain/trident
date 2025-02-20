use quote::ToTokens;
use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Field, ItemStruct};

use crate::types::trident_transaction::TridentTransactionField;
use crate::types::trident_transaction::TridentTransactionStruct;

pub fn parse_trident_transaction(
    item_struct: &ItemStruct,
) -> ParseResult<TridentTransactionStruct> {
    let ident = item_struct.ident.clone();

    // Parse the custom name attribute if present
    let custom_name = item_struct
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("name"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Failed to parse field name")
                .value()
        });

    let fields = match &item_struct.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(parse_transaction_field)
            .collect::<ParseResult<Vec<TridentTransactionField>>>()?,
        _ => {
            return Err(ParseError::new(
                item_struct.span(),
                "TridentTransaction structs must have named fields",
            ))
        }
    };

    Ok(TridentTransactionStruct {
        ident,
        fields,
        custom_name,
    })
}

fn parse_transaction_field(field: &Field) -> ParseResult<TridentTransactionField> {
    let ident = field.ident.clone().unwrap();
    let ty = field.ty.to_token_stream().to_string();

    Ok(TridentTransactionField { ident, ty })
}
