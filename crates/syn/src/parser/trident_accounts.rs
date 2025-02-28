use quote::ToTokens;
use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;

use syn::spanned::Spanned;
use syn::Attribute;
use syn::Field;
use syn::Ident;
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

    // Parse the instruction type attribute (required)
    let instruction_type = item_struct
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("instruction_data"))
        .ok_or_else(|| {
            ParseError::new(
                item_struct.span(),
                "Missing required #[instruction_data(Type)] attribute",
            )
        })?
        .parse_args::<syn::Type>()?;

    // Parse the storage type attribute (required)
    let storage_type = item_struct
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("storage"))
        .ok_or_else(|| {
            ParseError::new(
                item_struct.span(),
                "Missing required #[storage(Type)] attribute",
            )
        })?
        .parse_args::<syn::Type>()?;

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

    Ok(TridentAccountsStruct {
        ident,
        fields,
        instruction_type,
        storage_type,
    })
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
                "storage" => {
                    if meta.input.peek(syn::Token![=]) {
                        meta.input.parse::<syn::Token![=]>()?;
                        let storage_ident: Ident = meta.input.parse()?;
                        constraints.storage = Some(storage_ident);
                    }
                    Ok(())
                }
                "skip_snapshot" => {
                    constraints.skip_snapshot = true;
                    Ok(())
                }
                "seeds" => {
                    if meta.input.peek(syn::Token![=]) {
                        meta.input.parse::<syn::Token![=]>()?;
                        let content;
                        syn::bracketed!(content in meta.input);

                        let mut seeds = Vec::new();
                        while !content.is_empty() {
                            let expr: syn::Expr = content.parse()?;
                            seeds.push(expr);

                            if !content.is_empty() {
                                content.parse::<syn::Token![,]>()?;
                            }
                        }
                        constraints.seeds = Some(seeds);
                    }
                    Ok(())
                }
                "program_id" => {
                    if meta.input.peek(syn::Token![=]) {
                        meta.input.parse::<syn::Token![=]>()?;
                        let expr: syn::Expr = meta.input.parse()?;
                        constraints.program_id = Some(expr);
                    }
                    Ok(())
                }
                _ => Err(meta.error("unsupported constraint")),
            }
        })?;
    }

    // Validate constraints
    if (constraints.seeds.is_some() || constraints.program_id.is_some())
        && constraints.storage.is_none()
    {
        return Err(ParseError::new(
            proc_macro2::Span::call_site(),
            "seeds require non-optional storage attribute",
        ));
    }

    Ok(constraints)
}
