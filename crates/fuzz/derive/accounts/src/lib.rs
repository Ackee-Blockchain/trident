use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

#[proc_macro_derive(TridentAccounts, attributes(account))]
pub fn trident_accounts_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let account_metas_fields = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        // Parse the account attribute if it exists
        let account_attr = f.attrs.iter().find(|attr| attr.path.is_ident("account"));

        let mut is_mutable = false;
        let mut is_signer = false;
        let mut address = None;

        if let Some(attr) = account_attr {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                for nested in meta_list.nested.iter() {
                    match nested {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            if path.is_ident("mut") {
                                is_mutable = true;
                            } else if path.is_ident("signer") {
                                is_signer = true;
                            }
                        }
                        NestedMeta::Meta(Meta::NameValue(nv)) => {
                            if nv.path.is_ident("address") {
                                if let Lit::Str(lit_str) = &nv.lit {
                                    address = Some(lit_str.value());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Check if the type is TridentAccount
        let is_single_account = field_type.into_token_stream().to_string() == "TridentAccount";

        if let Some(address) = address {
            let mut q = quote! {};
            q.extend(quote! { self.#field_name.set_address(pubkey!(#address)); });
            if is_signer {
                q.extend(quote! { self.#field_name.set_is_signer(); });
            }
            if is_mutable {
                q.extend(quote! { self.#field_name.set_is_writable(); });
            }
            q.extend(quote! { metas.push(self.#field_name.to_account_meta()); });
            q
        } else if is_single_account {
            let mut q = quote! {};
            if is_signer {
                q.extend(quote! { self.#field_name.set_is_signer(); });
            }
            if is_mutable {
                q.extend(quote! { self.#field_name.set_is_writable(); });
            }
            q.extend(quote! { metas.push(self.#field_name.to_account_meta()); });
            q
        } else {
            quote! {
                metas.extend(self.#field_name.to_account_meta());
            }
        }
    });

    // Extract all named fields except those marked with skip_snapshot
    let snapshots_fields = fields
        .iter()
        .filter(|f| {
            !f.attrs.iter().any(|attr| {
                if attr.path.is_ident("account") {
                    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                        return meta_list.nested.iter().any(|nested| {
                            if let NestedMeta::Meta(Meta::Path(path)) = nested {
                                path.is_ident("skip_snapshot")
                            } else {
                                false
                            }
                        });
                    }
                }
                false
            })
        })
        .collect::<Vec<_>>();

    let snapshots_fields_idents = snapshots_fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl AccountsMethods for #name {
            fn capture_before(
                &mut self,
                client: &mut impl FuzzClient,
            ) {
                #(self.#snapshots_fields_idents.capture_before(client);)*
            }

            fn capture_after(
                &mut self,
                client: &mut impl FuzzClient,
            ) {
                #(self.#snapshots_fields_idents.capture_after(client);)*
            }
            fn to_account_meta(&mut self) -> Vec<AccountMeta> {
                let mut metas = Vec::new();
                #(#account_metas_fields)*
                metas
            }
        }
    };

    TokenStream::from(expanded)
}
