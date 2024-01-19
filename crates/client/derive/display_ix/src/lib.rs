use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(DisplayIx)]
pub fn display_ix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let display_impl = match &input.data {
        Data::Enum(enum_data) => {
            let display_match_arms = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                match &variant.fields {
                    syn::Fields::Unnamed(fields) => {
                        if fields.unnamed.len() == 1 {
                            quote! {
                                #enum_name::#variant_name(ref content) => {
                                    write!(f, stringify!(#variant_name))?;
                                    write!(f, "({:#?})", content)
                                },
                            }
                        } else {
                            quote! {
                                #enum_name::#variant_name (_) => write!(f, stringify!(#variant_name)),
                            }
                        }
                    },
                    _ => quote! {
                        #enum_name::#variant_name => write!(f, stringify!(#variant_name)),
                    },
                }
            });

            quote! {
                impl std::fmt::Display for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#display_match_arms)*
                        }
                    }
                }
            }
        }
        _ => panic!("DisplayIx can only be derived for enums"),
    };

    TokenStream::from(display_impl)
}
