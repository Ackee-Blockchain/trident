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

        let debug_remaining_accounts =
            if let Some(ref remaining_field) = self.remaining_accounts_field {
                let remaining = syn::Ident::new(remaining_field, proc_macro2::Span::call_site());
                quote! {
                    .field("\x1b[96mremaining_accounts\x1b[0m", &self.#remaining)
                }
            } else {
                quote! {}
            };

        let expanded = quote! {
            // Implement InstructionGetters trait
            impl InstructionGetters for #name {
                /// Get the instruction discriminator (identifier bytes)
                fn get_discriminator(&self) -> Vec<u8> {
                    vec![#(#discriminator_bytes),*]
                }

                /// Get the program ID that will process this instruction
                fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
                    pubkey!(#program_id)
                }

                /// Convert all accounts to AccountMeta format for Solana instructions
                fn to_account_metas(&mut self) -> Vec<AccountMeta> {
                    let mut metas = Vec::new();
                    metas.extend(self.#accounts.to_account_meta());
                    #remaining_accounts_extension
                    metas
                }
            }

            // Implement InstructionSetters trait
            impl InstructionSetters for #name {
                /// Capture the state of all accounts before transaction execution
                fn set_snapshot_before(&mut self, client: &mut impl FuzzClient) {
                    self.#accounts.capture_before(client);
                    #remaining_accounts_snapshots
                }

                /// Capture the state of all accounts after transaction execution
                fn set_snapshot_after(&mut self, client: &mut impl FuzzClient) {
                    self.#accounts.capture_after(client);
                    #remaining_accounts_snapshots_after
                }

                /// Resolve all accounts needed for this instruction
                fn resolve_accounts(
                    &mut self,
                    client: &mut impl FuzzClient,
                    fuzz_accounts: &mut Self::IxAccounts,
                ) {
                    self.#accounts.resolve_accounts(client, fuzz_accounts, self.get_program_id(), &self.data);
                }
            }

            // Debug implementation for better logging and visualization
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#name))
                        .field("\x1b[96mprogram_id\x1b[0m", &format_args!("\x1b[93m{}\x1b[0m", pubkey!(#program_id)))
                        .field("\x1b[96mdiscriminator\x1b[0m", &format_args!("{:?}", vec![#(#discriminator_bytes),*]))
                        .field("\x1b[96maccounts\x1b[0m", &self.#accounts)
                        #debug_remaining_accounts
                        .field("\x1b[96mdata\x1b[0m", &self.data)
                        .finish()
                }
            }
        };

        tokens.extend(expanded);
    }
}
