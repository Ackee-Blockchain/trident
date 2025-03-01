use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_transaction::TridentTransactionStruct;

impl ToTokens for TridentTransactionStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let field_idents = self.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

        // Generate the name implementation
        let name_impl = match &self.custom_name {
            Some(custom) => quote! { #custom.to_string() },
            None => quote! { stringify!(#name).to_string() },
        };

        // Generate instruction blocks for each field
        let instruction_blocks = self.fields.iter().map(|f| {
            let field_ident = &f.ident;
            quote! {
                {
                    self.#field_ident.set_data(client, fuzz_accounts);
                    self.#field_ident.resolve_accounts(client, fuzz_accounts);
                    self.#field_ident.set_accounts(client, fuzz_accounts);
                    self.#field_ident.set_remaining_accounts(client, fuzz_accounts);
                }
            }
        });

        let expanded = quote! {
            // Implement the getters trait
            impl TransactionGetters for #name {
                fn get_transaction_name(&self) -> String {
                    #name_impl
                }

                fn get_instruction_discriminators(&self) -> Vec<Vec<u8>> {
                    vec![
                        #(self.#field_idents.get_discriminator()),*
                    ]
                }

                fn get_instruction_program_ids(&self) -> Vec<solana_sdk::pubkey::Pubkey> {
                    vec![
                        #(self.#field_idents.get_program_id()),*
                    ]
                }

                fn get_instruction_data(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) -> Vec<Vec<u8>> {
                    vec![
                        #(borsh::to_vec(&self.#field_idents.data).unwrap()),*
                    ]
                }

                fn get_instruction_accounts(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) -> Vec<Vec<AccountMeta>> {
                    vec![
                        #(self.#field_idents.to_account_metas()),*
                    ]
                }
            }

            // Implement the setters trait
            impl TransactionSetters for #name {
                fn set_snapshot_before(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    #(self.#field_idents.set_snapshot_before(client);)*
                }

                fn set_snapshot_after(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    #(self.#field_idents.set_snapshot_after(client);)*
                }

                fn set_instructions(
                    &mut self,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut Self::IxAccounts,
                ) {
                    #(#instruction_blocks)*
                }
            }

            impl TransactionMethods for #name {
                fn build(
                    fuzzer_data: &mut FuzzerData,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut Self::IxAccounts,
                ) -> arbitrary::Result<Self> {
                    let mut tx = Self::arbitrary(fuzzer_data)?;
                    tx.set_instructions(client, fuzz_accounts);
                    Ok(tx)
                }
            }
        };

        tokens.extend(expanded);
    }
}
