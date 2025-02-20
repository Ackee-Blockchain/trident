use quote::quote;
use quote::ToTokens;

use crate::types::trident_accounts::TridentAccountField;
use crate::types::trident_accounts::TridentAccountsStruct;

impl ToTokens for TridentAccountsStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.ident;

        let account_metas_fields = self.fields.iter().map(|field| {
            match field {
                TridentAccountField::Field(f) => {
                    let field_name = &f.ident;
                    let constraints = &f.constraints;

                    let mut code = proc_macro2::TokenStream::new();

                    // Handle regular field constraints...
                    if let Some(ref address) = constraints.address {
                        code.extend(quote! {
                            self.#field_name.set_address(#address);
                        });
                    }

                    if constraints.signer {
                        code.extend(quote! {
                            self.#field_name.set_is_signer();
                        });
                    }

                    if constraints.mutable {
                        code.extend(quote! {
                            self.#field_name.set_is_writable();
                        });
                    }

                    code.extend(quote! {
                        metas.push(self.#field_name.to_account_meta());
                    });

                    code
                }
                TridentAccountField::CompositeField(f) => {
                    let field_name = &f.ident;
                    // For composite fields, just extend the metas
                    quote! {
                        metas.extend(self.#field_name.to_account_meta());
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
                fn capture_before(&mut self, client: &mut impl FuzzClient) {
                    #(self.#snapshot_fields.capture_before(client);)*
                }

                fn capture_after(&mut self, client: &mut impl FuzzClient) {
                    #(self.#snapshot_fields.capture_after(client);)*
                }

                fn to_account_meta(&mut self) -> Vec<AccountMeta> {
                    let mut metas = Vec::new();
                    #(#account_metas_fields)*
                    metas
                }
            }
        };

        tokens.extend(expanded);
    }
}
