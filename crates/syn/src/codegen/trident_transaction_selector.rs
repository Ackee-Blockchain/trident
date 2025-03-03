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
                #name::#variant_name(ref mut tx) => {
                    tx.set_instructions(client, fuzz_accounts);
                    tx.execute(client)
                }
            }
        });

        let process_transaction_no_hooks_match_arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(ref mut tx) => {
                    tx.set_instructions(client, accounts);
                    tx.execute_no_hooks(client).map_err(FuzzingError::TransactionFailed)
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

                fn select_n_execute(
                    fuzzer_data: &mut FuzzerData,
                    client: &mut impl FuzzClient,
                    accounts: &mut FuzzAccounts,
                ) -> Result<(), FuzzingError> {
                    let mut transaction = Self::arbitrary(fuzzer_data)?;
                    transaction.transaction_selector(client, accounts)
                }

                fn select_n_execute_no_hooks(
                    fuzzer_data: &mut FuzzerData,
                    client: &mut impl FuzzClient,
                    accounts: &mut FuzzAccounts,
                ) -> Result<(), FuzzingError> {
                    let mut transaction = Self::arbitrary(fuzzer_data)?;
                    match transaction {
                        #(#process_transaction_no_hooks_match_arms)*
                    }
                }
            }
        };

        tokens.extend(expanded);
    }
}
