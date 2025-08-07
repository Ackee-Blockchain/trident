use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::ItemImpl;
use syn::Meta;

use crate::types::trident_flow_executor::FlowConstraints;
use crate::types::trident_flow_executor::FlowMethod;
use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

pub fn parse_trident_flow_executor(input: &ItemImpl) -> ParseResult<TridentFlowExecutorImpl> {
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
    let mut end_method = None;
    let mut flow_methods = Vec::new();

    // Collect init, end, and flow methods
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

            // Then check for end methods
            if method.attrs.iter().any(|attr| attr.path().is_ident("end")) {
                if end_method.is_some() {
                    return Err(ParseError::new(
                        method.span(),
                        "Multiple #[end] methods found. Only one is allowed.",
                    ));
                }
                end_method = Some(method.sig.ident.clone());
                continue;
            }

            // Then check for flow methods
            if let Some(flow_attr) = method
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("flow"))
            {
                let constraints = parse_flow_constraints(flow_attr)?;
                let flow_method = FlowMethod {
                    ident: method.sig.ident.clone(),
                    constraints,
                };
                flow_methods.push(flow_method);
            }
        }
    }

    // Validate weight consistency
    let flows_with_weights: Vec<_> = flow_methods
        .iter()
        .filter(|f| f.constraints.weight.is_some())
        .collect();
    let flows_without_weights: Vec<_> = flow_methods
        .iter()
        .filter(|f| f.constraints.weight.is_none())
        .collect();

    if !flows_with_weights.is_empty() && !flows_without_weights.is_empty() {
        return Err(ParseError::new(
            proc_macro2::Span::call_site(),
            format!("Weight consistency error: If any flow has a weight specified, all flows must have weights. Flows without weights: {}",
                flows_without_weights.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>().join(", "))
        ));
    }

    // Validate that total weight equals exactly 100
    if !flows_with_weights.is_empty() {
        let total_weight: u32 = flows_with_weights
            .iter()
            .map(|f| f.constraints.weight.unwrap())
            .sum();

        if total_weight != 100 {
            return Err(ParseError::new(
                proc_macro2::Span::call_site(),
                format!("Total weight must equal exactly 100: The sum of all flow weights is {} but must be exactly 100 to represent clear percentages.", total_weight)
            ));
        }
    }

    Ok(TridentFlowExecutorImpl {
        type_name,
        impl_block: input.items.clone(),
        flow_methods,
        init_method,
        end_method,
        generics,
    })
}

fn parse_flow_constraints(attr: &Attribute) -> ParseResult<FlowConstraints> {
    let mut constraints = FlowConstraints::default();

    // Handle both #[flow] (no args) and #[flow(ignore)] (with args)
    match &attr.meta {
        Meta::Path(_) => {
            // #[flow] with no arguments - use defaults
            Ok(constraints)
        }
        Meta::List(_) => {
            // #[flow(...)] with arguments
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        "ignore" => {
                            constraints.ignore = true;
                            Ok(())
                        }
                        "weight" => {
                            if meta.input.peek(syn::Token![=]) {
                                meta.input.parse::<syn::Token![=]>()?;
                                let weight_lit: syn::LitInt = meta.input.parse()?;
                                let weight_val = weight_lit.base10_parse::<u32>()?;

                                if weight_val > 100 {
                                    return Err(meta.error("Weight must be between 0 and 100"));
                                }

                                constraints.weight = Some(weight_val);
                            }
                            Ok(())
                        }
                        _ => Err(meta.error("unsupported flow constraint")),
                    }
                } else {
                    Err(meta.error("unsupported flow constraint"))
                }
            })?;
            Ok(constraints)
        }
        _ => Err(ParseError::new_spanned(attr, "Invalid flow attribute")),
    }
}
