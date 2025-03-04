use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Type};

use crate::types::trident_remaining_accounts::TridentRemainingAccountsStruct;

pub fn parse_trident_remaining_accounts(
    input: &DeriveInput,
) -> ParseResult<TridentRemainingAccountsStruct> {
    let ident = input.ident.clone();

    // Get the struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(ParseError::new(
                    input.span(),
                    "TridentRemainingAccounts structs must have named fields",
                ))
            }
        },
        _ => {
            return Err(ParseError::new(
                input.span(),
                "TridentRemainingAccounts can only be derived for structs",
            ))
        }
    };

    // Ensure there's exactly one field
    if fields.len() != 1 {
        return Err(ParseError::new(
            input.span(),
            "TridentRemainingAccounts struct must have exactly one field",
        ));
    }

    let field = fields.iter().next().unwrap();

    // Verify the field type is [TridentAccount; N]
    if !is_valid_remaining_accounts_type(&field.ty) {
        return Err(ParseError::new(
            field.ty.span(),
            "Field must be of type [TridentAccount; N] where N is a constant",
        ));
    }

    let field_name = field.ident.clone().unwrap();

    Ok(TridentRemainingAccountsStruct { ident, field_name })
}

fn is_valid_remaining_accounts_type(ty: &Type) -> bool {
    match ty {
        Type::Array(array) => {
            // Check if it's [TridentAccount; N]
            if let Type::Path(type_path) = &*array.elem {
                type_path
                    .path
                    .segments
                    .last()
                    .is_some_and(|seg| seg.ident == "TridentAccount")
            } else {
                false
            }
        }
        _ => false,
    }
}
