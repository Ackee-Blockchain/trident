use quote::ToTokens;
use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::Field;
use syn::ItemStruct;

use crate::types::trident_transaction::TridentTransactionField;
use crate::types::trident_transaction::TridentTransactionStruct;

pub fn parse_trident_transaction(
    item_struct: &ItemStruct,
) -> ParseResult<TridentTransactionStruct> {
    let ident = item_struct.ident.clone();

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

    Ok(TridentTransactionStruct { ident, fields })
}

fn parse_transaction_field(field: &Field) -> ParseResult<TridentTransactionField> {
    let ident = field.ident.clone().unwrap();
    let ty = field.ty.to_token_stream().to_string();

    Ok(TridentTransactionField { ident, ty })
}
