use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};

/// Derives the `TridentInstruction` trait implementation for a struct.
///
/// # Attributes
///
/// * `accounts` - Specifies the field name containing the accounts structure
///   Example: `#[accounts("accounts")]`
///
/// * `program_id` - Specifies the program ID for the instruction
///   Example: `#[program_id("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")]`
///
/// * `discriminator` - Specifies the 8-byte discriminator for the instruction
///   Example: `#[discriminator([0, 1, 2, 3, 4, 5, 6, 7])]`
///
/// # Example
/// ```
/// #[derive(TridentInstruction)]
/// #[accounts("accounts")]
/// #[program_id("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")]
/// #[discriminator([0, 1, 2, 3, 4, 5, 6, 7])]
/// struct MyInstruction {
///     accounts: MyAccounts,
///     // ... other fields
/// }
/// ```
#[proc_macro_derive(
    TridentInstruction,
    attributes(accounts, remaining_accounts, program_id, discriminator)
)]
pub fn trident_instruction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the target field from the derive attribute parameters
    let accounts_field = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("accounts"))
        .expect("Please specify target accounts field with #[accounts(\"field_name\")]")
        .parse_args::<syn::LitStr>()
        .expect("Failed to parse field name")
        .value();

    let remaining_accounts_field = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("remaining_accounts"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Failed to parse field name")
                .value()
        });

    // Get the program ID from the derive attribute parameters
    let program_id = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("program_id"))
        .expect("Please specify program ID with #[program_id(\"program_id\")]")
        .parse_args::<syn::LitStr>()
        .expect("Failed to parse program ID")
        .value();

    // Get the discriminator from the derive attribute parameters
    let discriminator = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("discriminator"))
        .expect("Please specify discriminator with #[discriminator([u8, ...])]")
        .parse_args::<syn::ExprArray>()
        .expect("Failed to parse discriminator array");

    let discriminator_bytes = discriminator
        .elems
        .into_iter()
        .map(|elem| {
            if let syn::Expr::Lit(expr_lit) = elem {
                if let Lit::Int(int) = expr_lit.lit {
                    int.base10_parse::<u8>()
                        .expect("Invalid discriminator byte")
                } else {
                    panic!("Discriminator must contain only integer literals")
                }
            } else {
                panic!("Discriminator must contain only integer literals")
            }
        })
        .collect::<Vec<u8>>();

    let accounts = syn::Ident::new(&accounts_field, proc_macro2::Span::call_site());
    let remaining_accounts = remaining_accounts_field
        .map(|field| syn::Ident::new(&field, proc_macro2::Span::call_site()));

    let remaining_accounts_extension = if let Some(ref remaining_field) = remaining_accounts {
        quote! {
            metas.extend(self.#remaining_field.to_account_meta());
        }
    } else {
        quote! {}
    };

    let remaining_accounts_snapshots = if let Some(ref remaining_field) = remaining_accounts {
        quote! {
            self.#remaining_field.capture_before(client);
        }
    } else {
        quote! {}
    };

    let remaining_accounts_snapshots_after = if let Some(ref remaining_field) = remaining_accounts {
        quote! {
            self.#remaining_field.capture_after(client);
        }
    } else {
        quote! {}
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

            fn to_account_metas(&mut self) -> Vec<AccountMeta> {
                let mut metas = Vec::new();
                metas.extend(self.#accounts.to_account_meta());
                #remaining_accounts_extension
                metas
            }
        }
    };

    TokenStream::from(expanded)
}
