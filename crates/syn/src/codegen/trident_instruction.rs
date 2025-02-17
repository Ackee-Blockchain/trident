use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_instruction::TridentInstructionStruct;

impl ToTokens for TridentInstructionStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let accounts = syn::Ident::new(&self.accounts_field, proc_macro2::Span::call_site());
        let program_id = &self.program_id;
        let discriminator_bytes = &self.discriminator;

        // Generate remaining accounts code if field exists
        let (
            remaining_accounts_extension,
            remaining_accounts_snapshots,
            remaining_accounts_snapshots_after,
        ) = if let Some(ref remaining_field) = self.remaining_accounts_field {
            let remaining = syn::Ident::new(remaining_field, proc_macro2::Span::call_site());
            (
                quote! { metas.extend(self.#remaining.to_account_meta()); },
                quote! { self.#remaining.capture_before(client); },
                quote! { self.#remaining.capture_after(client); },
            )
        } else {
            (quote! {}, quote! {}, quote! {})
        };

        let expanded = quote! {
            impl InstructionMethods for #name {
                fn get_discriminator(&self) -> Vec<u8> {
                    vec![#(#discriminator_bytes),*]
                }

                fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
                    pubkey!(#program_id)
                }

                fn set_snapshot_before(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    self.#accounts.capture_before(client);
                    #remaining_accounts_snapshots
                }

                fn set_snapshot_after(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    self.#accounts.capture_after(client);
                    #remaining_accounts_snapshots_after
                }

                fn to_account_metas(
                    &mut self,
                ) -> Vec<AccountMeta> {
                    let mut metas = Vec::new();
                    metas.extend(self.#accounts.to_account_meta());
                    #remaining_accounts_extension
                    metas
                }

                fn resolve_accounts(
                    &mut self,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut Self::IxAccounts,
                ) {
                    self.#accounts.resolve_accounts(client, fuzz_accounts);
                }
            }
        };

        tokens.extend(expanded);
    }
}
