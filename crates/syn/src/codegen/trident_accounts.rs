use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_accounts::TridentAccountField;
use crate::types::trident_accounts::TridentAccountsStruct;

impl ToTokens for TridentAccountsStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.ident;

        // Generate storage resolution code for resolve_accounts
        let resolve_storage = self.fields.iter().map(|field| {
            match field {
                TridentAccountField::Field(f) => {
                    let field_name = &f.ident;
                    let constraints = &f.constraints;

                    let mut inner_code = TokenStream::new();

                    if let Some(ref storage_ident) = constraints.storage {
                        inner_code.extend(quote! {
                            let address = ix_accounts
                                .#storage_ident
                                .get_or_create(self.#field_name.account_id, client, None, None);
                            self.#field_name.set_address(address);
                        });
                    }

                    if let Some(ref address) = constraints.address {
                        inner_code.extend(quote! {
                            self.#field_name.set_address(#address);
                        });
                    }

                    if constraints.signer {
                        inner_code.extend(quote! {
                            self.#field_name.set_is_signer();
                        });
                    }

                    if constraints.mutable {
                        inner_code.extend(quote! {
                            self.#field_name.set_is_writable();
                        });
                    }

                    if !inner_code.is_empty() {
                        quote! {
                            {
                                // Resolve and configure account
                                #inner_code
                            }
                        }
                    } else {
                        quote! {}
                    }
                }
                TridentAccountField::CompositeField(f) => {
                    let field_name = &f.ident;
                    quote! {
                        {
                            // Resolve composite accounts
                            self.#field_name.resolve_accounts(client, ix_accounts);
                        }
                    }
                }
            }
        });

        // Generate account meta conversion code
        let account_metas_fields = self.fields.iter().map(|field| {
            match field {
                TridentAccountField::Field(f) => {
                    let field_name = &f.ident;
                    quote! {
                        {
                            // Add account meta
                            metas.push(self.#field_name.to_account_meta());
                        }
                    }
                }
                TridentAccountField::CompositeField(f) => {
                    let field_name = &f.ident;
                    quote! {
                        {
                            // Add composite accounts
                            metas.extend(self.#field_name.to_account_meta());
                        }
                    }
                }
            }
        });

        let snapshot_fields: Vec<_> = self
            .fields
            .iter()
            .filter(|field| match field {
                TridentAccountField::Field(f) => !f.constraints.skip_snapshot,
                TridentAccountField::CompositeField(f) => !f.constraints.skip_snapshot,
            })
            .map(|field| field.ident())
            .collect();

        let expanded = quote! {
            impl AccountsMethods for #name {
                type IxAccounts = FuzzAccounts;

                fn resolve_accounts(
                    &mut self,
                    client: &mut impl FuzzClient,
                    ix_accounts: &mut Self::IxAccounts,
                ) {
                    #(#resolve_storage)*
                }

                fn to_account_meta(&mut self) -> Vec<AccountMeta> {
                    let mut metas = Vec::new();
                    #(#account_metas_fields)*
                    metas
                }

                fn capture_before(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    #(
                        {
                            // Capture snapshot before
                            self.#snapshot_fields.capture_before(client);
                        }
                    )*
                }

                fn capture_after(
                    &mut self,
                    client: &mut impl FuzzClient,
                ) {
                    #(
                        {
                            // Capture snapshot after
                            self.#snapshot_fields.capture_after(client);
                        }
                    )*
                }
            }
        };

        tokens.extend(expanded);
    }
}
