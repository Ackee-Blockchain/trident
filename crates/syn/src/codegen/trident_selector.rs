use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_selector::TridentSelectorEnum;

impl ToTokens for TridentSelectorEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let variants = &self.variants;

        let process_transaction_match_arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(tx) => {
                    tx.execute_with_hooks(client, fuzz_accounts)
                }
            }
        });

        let expanded = quote! {
            impl Selector for #name {
                type IxAccounts = FuzzAccounts;
                fn execute_with_hooks(
                    &mut self,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut FuzzAccounts,
                ) -> Result<(), FuzzingError> {
                    match self {
                        #(#process_transaction_match_arms)*
                    }
                }
            }
        };

        tokens.extend(expanded);
    }
}
