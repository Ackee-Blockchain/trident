use quote::ToTokens;
use syn::parse_quote;

use crate::Template;

impl Template {
    pub fn get_instructions(&self) -> Vec<(String, String)> {
        self.instructions
            .iter()
            .map(|(name, file)| (name.clone(), file.into_token_stream().to_string()))
            .collect()
    }
    pub fn get_transactions(&self) -> Vec<(String, String)> {
        self.transactions
            .iter()
            .map(|(name, file)| (name.clone(), file.into_token_stream().to_string()))
            .collect()
    }
    pub fn get_instructions_mod(&self) -> String {
        let (modules, uses): (Vec<syn::ItemMod>, Vec<syn::ItemUse>) = self
            .instructions_mod
            .iter()
            .map(|mod_definition| {
                (
                    mod_definition.module.clone(),
                    mod_definition.use_statement.clone(),
                )
            })
            .unzip();

        let mod_file: syn::File = parse_quote! {
            #(#modules)*
            #(#uses)*
        };

        mod_file.into_token_stream().to_string()
    }
    pub fn get_transactions_mod(&self) -> String {
        let (modules, uses): (Vec<syn::ItemMod>, Vec<syn::ItemUse>) = self
            .transactions_mod
            .iter()
            .map(|mod_definition| {
                (
                    mod_definition.module.clone(),
                    mod_definition.use_statement.clone(),
                )
            })
            .unzip();

        let mod_file: syn::File = parse_quote! {
            #(#modules)*
            #(#uses)*
        };

        mod_file.into_token_stream().to_string()
    }
    pub fn get_fuzz_transactions(&self) -> String {
        let transaction_variants = self.fuzz_transactions.clone();
        let account_storages: Vec<syn::Field> = self
            .account_storages
            .iter()
            .map(|account_storage| account_storage.1.clone())
            .collect();

        let module_definition: syn::File = parse_quote! {
            use trident_fuzz::fuzzing::*;
            use crate::transactions::*;

            /// FuzzTransactions contains all available transactions
            ///
            /// You can create your own transactions by adding new variants to the enum.
            ///
            /// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
            #[derive(Arbitrary, TransactionSelector)]
            pub enum FuzzTransactions {
                #(#transaction_variants),*
            }

            /// FuzzAccounts contains all available accounts
            ///
            /// You can create your own accounts by adding new fields to the struct.
            ///
            /// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
            #[derive(Default)]
            pub struct FuzzAccounts {
                #(#account_storages),*
            }

        };
        module_definition.into_token_stream().to_string()
    }
    pub fn get_custom_types(&self) -> String {
        let custom_types = self.custom_types.clone();
        let common_header = quote::quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use trident_fuzz::fuzzing::*;

            /// File containing all custom types which can be used
            /// in transactions and instructions or invariant checks.
            ///
            /// You can define your own custom types here.
        };

        let module_definition: syn::File = match custom_types.len() {
            0 => parse_quote! {
                #common_header

                #[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
                pub struct ExampleType {
                    example_data: u8,
                }
            },
            _ => parse_quote! {
                #common_header

                #(#custom_types)*
            },
        };
        module_definition.into_token_stream().to_string()
    }

    pub fn get_test_fuzz(&self) -> String {
        match &self.test_fuzz {
            Some(file) => file.into_token_stream().to_string(),
            None => panic!("test_fuzz.rs not prepared, aborting"),
        }
    }
}
