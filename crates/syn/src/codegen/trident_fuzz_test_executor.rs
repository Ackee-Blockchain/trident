use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_fuzz_test_executor::TridentFuzzTestExecutorEnum;

impl ToTokens for TridentFuzzTestExecutorEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let variants = &self.variants;

        let process_transaction_match_arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(tx) => {
                    tx.process_transaction(client, config, &mut fuzz_accounts.borrow_mut())
                }
            }
        });

        let expanded = quote! {
            impl FuzzTestExecutor<FuzzAccounts> for #name {
                fn process_transaction(
                    &mut self,
                    client: &mut TridentSVM,
                    config: &TridentConfig,
                    fuzz_accounts: &RefCell<FuzzAccounts>,
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
