use proc_macro2::TokenStream;
use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::ItemImpl;
use syn::Meta;

use crate::types::trident_flow_executor::FlowExecutorArgs;
use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

impl Parse for FlowExecutorArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut args = FlowExecutorArgs::default();

        while !input.is_empty() {
            let meta: Meta = input.parse()?;

            match meta {
                Meta::NameValue(nv) => {
                    if nv.path.is_ident("random_tail") {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let syn::Lit::Bool(lit_bool) = expr_lit.lit {
                                args.random_tail = lit_bool.value();
                            } else {
                                return Err(ParseError::new(
                                    expr_lit.lit.span(),
                                    "random_tail must be a boolean value",
                                ));
                            }
                        }
                    } else {
                        return Err(ParseError::new(
                            nv.path.span(),
                            format!("unknown attribute: {}", nv.path.get_ident().unwrap()).as_str(),
                        ));
                    }
                }
                Meta::Path(path) => {
                    return Err(ParseError::new(
                        path.span(),
                        format!("unknown flag attribute: {}", path.get_ident().unwrap()).as_str(),
                    ));
                    // if path.is_ident("shuffle") {
                    //     args.shuffle = true;
                    // } else {
                    //     return Err(ParseError::new(
                    //         path.span(),
                    //         format!("unknown flag attribute: {}", path.get_ident().unwrap())
                    //             .as_str(),
                    //     ));
                    // }
                }
                _ => {
                    return Err(ParseError::new(
                        meta.span(),
                        "expected either a name-value pair or a flag attribute",
                    ));
                }
            }

            // Parse comma if there are more attributes
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(args)
    }
}

pub fn parse_trident_flow_executor(
    attr: TokenStream,
    input: &ItemImpl,
) -> ParseResult<TridentFlowExecutorImpl> {
    let args: FlowExecutorArgs = syn::parse2(attr)?;

    // Extract just the path without any generics
    let type_name = if let syn::Type::Path(type_path) = &*input.self_ty {
        let mut cleaned_path = type_path.clone();
        // Clear any generic arguments from the last segment
        if let Some(last_segment) = cleaned_path.path.segments.last_mut() {
            last_segment.arguments = syn::PathArguments::None;
        }
        Box::new(syn::Type::Path(cleaned_path))
    } else {
        input.self_ty.clone()
    };
    let generics = input.generics.clone();

    let mut init_method = None;
    let mut flow_methods = Vec::new();

    // Collect init and flow methods
    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            // First check for init methods
            if method.attrs.iter().any(|attr| attr.path().is_ident("init")) {
                if init_method.is_some() {
                    return Err(ParseError::new(
                        method.span(),
                        "Multiple #[init] methods found. Only one is allowed.",
                    ));
                }
                init_method = Some(method.sig.ident.clone());
                continue;
            }

            // Then check for flow methods
            if method.attrs.iter().any(|attr| attr.path().is_ident("flow")) {
                // Only check for ignore if it's a flow method
                let is_ignored = method
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("flow_ignore"));
                if !is_ignored {
                    flow_methods.push(method.sig.ident.clone());
                }
            }
        }
    }

    Ok(TridentFlowExecutorImpl {
        type_name,
        impl_block: input.items.clone(),
        flow_methods,
        init_method,
        generics,
        args,
    })
}
