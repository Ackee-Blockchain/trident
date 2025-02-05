use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(TridentTransaction, attributes(name))]
pub fn trident_transaction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the target field from the derive attribute parameters
    let custom_name = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("name"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Failed to parse field name")
                .value()
        });

    // Generate the name implementation
    let name_impl = match custom_name {
        Some(custom) => quote! { #custom.to_string() },
        None => quote! { stringify!(#name).to_string() },
    };

    // Extract field names that end with "instruction"
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_idents = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl TransactionMethods for #name {
            type IxAccounts = FuzzAccounts;

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
                fuzz_accounts: &mut FuzzAccounts,
            ) -> Vec<Vec<u8>> {
                #(self.#field_idents.set_data(client, fuzz_accounts);)*
                vec![
                    #(borsh::to_vec(&self.#field_idents.data).unwrap()),*
                ]
            }

            fn get_instruction_accounts(
                &mut self,
                client: &mut impl FuzzClient,
                fuzz_accounts: &mut FuzzAccounts,
            ) -> Vec<Vec<AccountMeta>> {
                #(self.#field_idents.set_accounts(client, fuzz_accounts);)*
                #(self.#field_idents.set_remaining_accounts(client, fuzz_accounts);)*
                vec![
                    #(self.#field_idents.to_account_metas()),*
                ]
            }

            fn set_snapshot_before(
                &mut self,
                client: &mut impl FuzzClient,
            ){
                #(self.#field_idents.set_snapshot_before(client);)*
            }

            fn set_snapshot_after(
                &mut self,
                client: &mut impl FuzzClient,
            ){
                #(self.#field_idents.set_snapshot_after(client);)*
            }
        }
    };

    TokenStream::from(expanded)
}
