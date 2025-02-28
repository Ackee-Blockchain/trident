use petgraph::algo::toposort;
use petgraph::Graph;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use crate::types::trident_accounts::TridentAccountField;
use crate::types::trident_accounts::TridentAccountsStruct;

impl ToTokens for TridentAccountsStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.ident;
        let instruction_data = &self.instruction_type;
        let storage_type = &self.storage_type;
        // Build dependency graph and sort fields
        let dependencies = self.analyze_seed_dependencies();
        let mut graph = Graph::new();
        let mut node_indices = HashMap::new();
        let mut field_positions = HashMap::new();

        // Add all fields as nodes and track their positions
        for (idx, field) in self.fields.iter().enumerate() {
            let node_idx = graph.add_node(idx); // Store position instead of ident
            node_indices.insert(field.ident().to_string(), node_idx);
            field_positions.insert(field.ident().to_string(), idx);
        }

        // Add edges for dependencies - IMPORTANT: Reverse the direction!
        for dep in dependencies {
            let from = node_indices[&dep.dependent_field.to_string()];
            let to = node_indices[&dep.required_field.to_string()];
            // We were adding edges in the wrong direction
            graph.add_edge(to, from, ()); // Changed order here
        }

        // Perform topological sort
        let sorted_fields = match toposort(&graph, None) {
            Ok(nodes) => nodes
                .into_iter()
                .map(|idx| &self.fields[graph[idx]]) // Use the stored position
                .collect::<Vec<_>>(),
            Err(_) => {
                panic!("Circular dependencies detected in account seeds");
            }
        };

        // Generate individual resolution blocks for each field
        let resolve_storage = sorted_fields.iter().map(|field| {
            match field {
                TridentAccountField::Field(f) => {
                    let field_name = &f.ident;
                    let is_signer = f.constraints.signer;
                    let is_mutable = f.constraints.mutable;

                    // Generate the account resolution code based on constraints
                    if let Some(address) = &f.constraints.address {
                        quote! {
                            let #field_name = {
                                let account = #address;
                                self.#field_name.set_address(account);

                                if #is_signer {
                                    self.#field_name.set_is_signer();
                                }
                                if #is_mutable {
                                    self.#field_name.set_is_writable();
                                }

                                account
                            };
                        }
                    } else if let Some(storage_ident) = &f.constraints.storage {
                        let account_resolution = if let Some(seeds) = &f.constraints.seeds {
                            // Get program_id from constraint if available, otherwise use the passed program_id
                            let program_id_to_use = if let Some(program_id) = &f.constraints.program_id {
                                quote!(#program_id)
                            } else {
                                quote!(program_id)
                            };

                            quote! {
                                storage_accounts
                                    .#storage_ident
                                    .get_or_create(self.#field_name.account_id, client, Some(PdaSeeds::new(&[#(#seeds),*], #program_id_to_use)), None)
                            }
                        } else {
                            quote! {
                                storage_accounts
                                    .#storage_ident
                                    .get_or_create(self.#field_name.account_id, client, None, None)
                            }
                        };

                        quote! {
                            let #field_name = {
                                let account = #account_resolution;
                                self.#field_name.set_address(account);

                                if #is_signer {
                                    self.#field_name.set_is_signer();
                                }
                                if #is_mutable {
                                    self.#field_name.set_is_writable();
                                }

                                account
                            };
                        }
                    } else {
                        // No address or storage specified, just set flags
                        quote! {
                            let #field_name = {
                                if #is_signer {
                                    self.#field_name.set_is_signer();
                                }
                                if #is_mutable {
                                    self.#field_name.set_is_writable();
                                }
                            };
                        }
                    }
                }
                TridentAccountField::CompositeField(f) => {
                    let field_name = &f.ident;
                    quote! {
                        let #field_name = {
                            self.#field_name.resolve_accounts(client, storage_accounts, program_id, instruction_data);
                            &self.#field_name
                        };
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
                type IxAccounts = #storage_type;
                type IxData = #instruction_data;

                #[allow(unused_variables)]
                fn resolve_accounts(
                    &mut self,
                    client: &mut impl FuzzClient,
                    storage_accounts: &mut Self::IxAccounts,
                    program_id: Pubkey,
                    instruction_data: &Self::IxData,
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
