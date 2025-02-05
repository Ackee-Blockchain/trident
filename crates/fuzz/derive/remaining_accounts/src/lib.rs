use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(TridentRemainingAccounts)]
pub fn trident_remaining_accounts_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // We expect exactly one field of type Vec<TridentAccount> or array [TridentAccount; N]
    let remaining_accounts_field = fields
        .first()
        .expect("Struct must have exactly one field for remaining accounts");
    let field_name = remaining_accounts_field.ident.as_ref().unwrap();

    let expanded = quote! {
        impl AccountsMethods for #name {
            fn capture_before(
                &mut self,
                client: &mut impl FuzzClient,
            ) {
                for account in self.#field_name.iter_mut() {
                    if !account.is_account_meta_set() {
                        break;
                    }
                    account.capture_before(client);
                }
            }

            fn capture_after(
                &mut self,
                client: &mut impl FuzzClient,
            ) {
                for account in self.#field_name.iter_mut() {
                    if !account.is_account_meta_set() {
                        break;
                    }
                    account.capture_after(client);
                }
            }

            fn to_account_meta(&mut self) -> Vec<AccountMeta> {
                let mut metas = Vec::new();
                for account in self.#field_name.iter() {
                    if !account.is_account_meta_set() {
                        break;
                    }
                    metas.push(account.to_account_meta());
                }
                metas
            }
        }
    };

    TokenStream::from(expanded)
}
