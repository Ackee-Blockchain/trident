use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

#[proc_macro_derive(TridentAccounts, attributes(address, skip_snapshot))]
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

        // Check if field has #[program("xyz")] attribute
        let address = f
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("address"))
            .and_then(|attr| {
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = meta_list.nested.first() {
                        Some(lit_str.value())
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        // Check if the type is TridentAccount
        let is_single_account = field_type.into_token_stream().to_string() == "TridentAccount";

        if let Some(pid) = address {
            quote! {
                self.#field_name.set_account_meta(pubkey!(#pid), false, false);
                metas.push(self.#field_name.to_account_meta());
            }
        } else if is_single_account {
            quote! {
                metas.push(self.#field_name.to_account_meta());
            }
        } else {
            quote! {
                metas.extend(self.#field_name.to_account_meta());
            }
        }
    });

    // Extract all named fields except those marked with #[skip_snapshot]
    let snapshots_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .filter(|f| {
                    // Skip fields that have #[skip_snapshot] attribute
                    !f.attrs
                        .iter()
                        .any(|attr| attr.path.is_ident("skip_snapshot"))
                })
                .collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

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
