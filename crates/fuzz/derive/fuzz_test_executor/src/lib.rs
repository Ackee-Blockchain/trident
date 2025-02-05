use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FuzzTestExecutor)]
pub fn fuzz_test_executor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let display_impl = match &input.data {
        Data::Enum(enum_data) => {
            let instruction_name_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (_) => String::from(stringify!(#variant_name)),
                }
            });
            let discriminator_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.get_discriminator()
                    }
                }
            });
            let program_id_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.get_program_id()
                    }
                }
            });
            let get_data_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.get_data(client, &mut accounts.borrow_mut())
                    }
                }
            });
            let get_accounts_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.get_accounts(client, &mut accounts.borrow_mut())
                    }
                }
            });
            let invariant_check_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.transaction_invariant_check(pre_tx, post_tx)
                    }
                }
            });
            let tx_error_handler_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.transaction_error_handler(e, pre_tx)
                    }
                }
            });
            let post_instruction_handler_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.post_transaction(client, post_tx)
                    }
                }
            });
            quote! {
               impl FuzzTestExecutor<FuzzAccounts> for FuzzInstruction {
                    fn instruction_name(&self) -> String {
                        match self {
                            #(#instruction_name_match_arm)*
                        }
                    }
                   fn get_discriminator(&self) -> Vec<u8> {
                       match self {
                            #(#discriminator_match_arm)*
                        }
                   }
                   fn get_program_id(&self) -> Pubkey {
                       match self {
                            #(#program_id_match_arm)*
                        }
                   }
                   fn get_data(&self, client: &mut impl FuzzClient, accounts: &RefCell<FuzzAccounts>) -> Result<Vec<u8>, FuzzingError> {
                        match self {
                            #(#get_data_match_arm)*
                        }
                   }
                   fn get_accounts(&self, client: &mut impl FuzzClient, accounts: &RefCell<FuzzAccounts>) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
                        match self {
                            #(#get_accounts_match_arm)*
                        }
                   }
                   fn transaction_invariant_check(&self, pre_tx: &TransactionSnapshot, post_tx: &TransactionSnapshot) -> Result<(), FuzzingError> {
                        match self {
                            #(#invariant_check_match_arm)*
                        }
                   }
                   fn transaction_error_handler(&self, e: TransactionError,  pre_tx: &TransactionSnapshot) -> Result<(), TransactionError> {
                        match self {
                            #(#tx_error_handler_match_arm)*
                        }
                   }
                   fn post_transaction(&self, client: &mut impl FuzzClient, post_tx: &TransactionSnapshot) {
                        match self {
                            #(#post_instruction_handler_match_arm)*
                        }
                   }
                }
            }
        }
        _ => panic!("FuzzTestExecutor can only be derived for enums"),
    };

    TokenStream::from(display_impl)
}
