use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields, Lit};

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
        .find(|f| f.ident.as_ref().is_some_and(|id| id == "accounts"))
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
                .is_some_and(|id| id == "remaining_accounts")
        })
        .map(|f| f.ident.as_ref().unwrap().to_string());

    // Parse program ID
    let program_id = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("program_id"))
        .ok_or_else(|| {
            ParseError::new(
                input.span(),
                "Please specify program ID with #[program_id(\"program_id\")]",
            )
        })?
        .parse_args::<syn::LitStr>()?
        .value();

    // Parse discriminator
    let discriminator = parse_discriminator_attr(&input.attrs)?;

    Ok(TridentInstructionStruct {
        ident,
        accounts_field,
        remaining_accounts_field,
        program_id,
        discriminator,
    })
}

fn parse_discriminator_attr(attrs: &[Attribute]) -> ParseResult<Vec<u8>> {
    let discriminator_attr = attrs
        .iter()
        .find(|attr| attr.path().is_ident("discriminator"))
        .ok_or_else(|| {
            ParseError::new(
                proc_macro2::Span::call_site(),
                "Please specify discriminator with #[discriminator([u8, ...])]",
            )
        })?;

    let array = discriminator_attr.parse_args::<syn::ExprArray>()?;

    array
        .elems
        .into_iter()
        .map(|elem| {
            if let syn::Expr::Lit(expr_lit) = elem {
                if let Lit::Int(int) = expr_lit.lit {
                    int.base10_parse::<u8>()
                        .map_err(|_| ParseError::new(int.span(), "Invalid discriminator byte"))
                } else {
                    Err(ParseError::new(
                        expr_lit.span(),
                        "Discriminator must contain only integer literals",
                    ))
                }
            } else {
                Err(ParseError::new(
                    elem.span(),
                    "Discriminator must contain only integer literals",
                ))
            }
        })
        .collect()
}
