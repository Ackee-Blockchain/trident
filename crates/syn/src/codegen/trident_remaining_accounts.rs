use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_remaining_accounts::TridentRemainingAccountsStruct;

impl ToTokens for TridentRemainingAccountsStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let field_name = &self.field_name;

        let expanded = quote! {
            impl RemainingAccountsMethods for #name {
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

        tokens.extend(expanded);
    }
}
