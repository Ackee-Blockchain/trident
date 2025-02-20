use quote::ToTokens;
use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;

use syn::spanned::Spanned;
use syn::Attribute;
use syn::Field;
use syn::ItemStruct;
use syn::Type;

use crate::types::trident_accounts::CompositeField;
use crate::types::trident_accounts::TridentAccountField;
use crate::types::trident_accounts::TridentAccountTy;
use crate::types::trident_accounts::TridentAccountType;
use crate::types::trident_accounts::TridentAccountsStruct;
use crate::types::trident_accounts::TridentConstraints;
use crate::types::trident_accounts::TridentField;

pub fn parse_trident_accounts(item_struct: &ItemStruct) -> ParseResult<TridentAccountsStruct> {
    let ident = item_struct.ident.clone();

    let fields = match &item_struct.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(parse_account_field)
            .collect::<ParseResult<Vec<TridentAccountField>>>()?,
        _ => {
            return Err(ParseError::new(
                item_struct.span(),
                "TridentAccounts structs must have named fields",
            ))
        }
    };

    Ok(TridentAccountsStruct { ident, fields })
}

fn parse_account_field(field: &Field) -> ParseResult<TridentAccountField> {
    let ident = field.ident.clone().unwrap();
    let constraints = parse_constraints(&field.attrs)?;

    // Check if this is a composite field (not a TridentAccount)
    if !is_trident_account_type(&field.ty) {
        return Ok(TridentAccountField::CompositeField(CompositeField {
            ident,
            constraints,
            ty: field.ty.to_token_stream().to_string(),
        }));
    }

    // Handle regular TridentAccount fields
    Ok(TridentAccountField::Field(TridentField {
        ident,
        ty: parse_account_type(&field.ty)?,
        constraints,
    }))
}

fn is_trident_account_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident == "TridentAccount")
            .unwrap_or(false),
        _ => false,
    }
}

fn parse_account_type(ty: &Type) -> ParseResult<TridentAccountType> {
    match ty {
        Type::Path(type_path) => {
            let path_segment = type_path
                .path
                .segments
                .last()
                .ok_or_else(|| ParseError::new(ty.span(), "Invalid type path"))?;

            match path_segment.ident.to_string().as_str() {
                "TridentAccount" => Ok(TridentAccountType::TridentAccount(TridentAccountTy {
                    program_type_path: type_path.clone(),
                })),
                _ => Err(ParseError::new(ty.span(), "Unsupported account type")),
            }
        }
        _ => Err(ParseError::new(ty.span(), "Invalid account type")),
    }
}

fn parse_constraints(attrs: &[Attribute]) -> ParseResult<TridentConstraints> {
    let mut constraints = TridentConstraints::default();

    for attr in attrs {
        if !attr.path().is_ident("account") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            let ident = meta
                .path
                .get_ident()
                .ok_or_else(|| meta.error("expected identifier"))?;

            match ident.to_string().as_str() {
                "mut" => {
                    constraints.mutable = true;
                    Ok(())
                }
                "signer" => {
                    constraints.signer = true;
                    Ok(())
                }
                "address" => {
                    if meta.input.peek(syn::Token![=]) {
                        meta.input.parse::<syn::Token![=]>()?;
                        // Parse a string literal
                        let addr_str: syn::LitStr = meta.input.parse()?;
                        // Convert the string literal into a pubkey expression
                        constraints.address = Some(syn::parse_str::<syn::Expr>(&format!(
                            "pubkey!(\"{}\")",
                            addr_str.value()
                        ))?);
                    }
                    Ok(())
                }
                // "storage" => {
                //     if meta.input.peek(syn::Token![=]) {
                //         meta.input.parse::<syn::Token![=]>()?;
                //         let storage_ident: Ident = meta.input.parse()?;
                //         constraints.storage = Some(storage_ident);
                //     }
                //     Ok(())
                // }
                "skip_snapshot" => {
                    constraints.skip_snapshot = true;
                    Ok(())
                }
                _ => Err(meta.error("unsupported constraint")),
            }
        })?;
    }

    Ok(constraints)
}
