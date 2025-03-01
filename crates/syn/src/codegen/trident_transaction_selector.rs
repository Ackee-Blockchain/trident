use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_transaction_selector::TridentSelectorEnum;

impl ToTokens for TridentSelectorEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let variants = &self.variants;

        let process_transaction_match_arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(tx) => {
                    tx.execute(client,fuzz_accounts)
                }
            }
        });

        let expanded = quote! {
            impl TransactionSelector<FuzzAccounts> for #name {
                fn transaction_selector(
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
