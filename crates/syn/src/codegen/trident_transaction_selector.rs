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
                    tx.set_instructions(client, fuzz_accounts, rng);
                    tx.execute(client, stats_logger, rng)
                }
            }
        });

        let _process_transaction_no_hooks_match_arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(ref mut tx) => {
                    tx.set_instructions(client, accounts, rng);
                    tx.execute_no_hooks(client).map_err(FuzzingError::TransactionFailed)
                }
            }
        });

        let expanded = quote! {
            impl TransactionSelector<FuzzAccounts> for #name {
                fn transaction_selector(
                    &mut self,
                    stats_logger: &mut FuzzingStatistics,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut FuzzAccounts,
                    rng: &mut TridentRng,
                ) -> Result<(), FuzzingError> {
                    match self {
                        #(#process_transaction_match_arms)*
                    }
                }

                fn select_n_execute(
                    stats_logger: &mut FuzzingStatistics,
                    client: &mut impl FuzzClient,
                    accounts: &mut FuzzAccounts,
                    rng: &mut TridentRng,
                ) -> Result<(), FuzzingError> {
                    // let mut transaction = Self::arbitrary(fuzzer_data)?;
                    // transaction.transaction_selector(stats_logger, client, accounts, rng)
                    Ok(())
                }

                fn select_n_execute_no_hooks(
                    client: &mut impl FuzzClient,
                    accounts: &mut FuzzAccounts,
                    rng: &mut TridentRng,
                ) -> Result<(), FuzzingError> {
                    // let mut transaction = Self::arbitrary(fuzzer_data)?;
                    // match transaction {
                    //     #(#process_transaction_no_hooks_match_arms)*
                    // }
                    Ok(())

                }
            }
        };

        tokens.extend(expanded);
    }
}
