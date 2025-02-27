use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

use crate::types::trident_instruction::TridentInstructionStruct;

pub fn parse_trident_instruction(input: &DeriveInput) -> ParseResult<TridentInstructionStruct> {
    let ident = input.ident.clone();

    // Get the struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(ParseError::new(
                    input.span(),
                    "TridentInstruction structs must have named fields",
                ))
            }
        },
        _ => {
            return Err(ParseError::new(
                input.span(),
                "TridentInstruction can only be derived for structs",
            ))
        }
    };

    // Find the accounts field
    let accounts_field = fields
        .iter()
        .find(|f| f.ident.as_ref().map_or(false, |id| id == "accounts"))
        .ok_or_else(|| ParseError::new(input.span(), "Struct must contain an 'accounts' field"))?
        .ident
        .as_ref()
        .unwrap()
        .to_string();

    // Check for remaining_accounts field
    let remaining_accounts_field = fields
        .iter()
        .find(|f| {
            f.ident
                .as_ref()
                .map_or(false, |id| id == "remaining_accounts")
        })
        .map(|f| f.ident.as_ref().unwrap().to_string());

    Ok(TridentInstructionStruct {
        ident,
        accounts_field,
        remaining_accounts_field,
    })
}
