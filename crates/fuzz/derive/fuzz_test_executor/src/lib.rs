use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FuzzTestExecutor)]
pub fn trident_fuzz_test_executor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let display_impl = match &input.data {
        Data::Enum(enum_data) => {
            let process_transaction_match_arm = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {
                        ix.process_transaction(client, config, &mut fuzz_accounts.borrow_mut())
                    }
                }
            });
            quote! {
               impl FuzzTestExecutor<FuzzAccounts> for #enum_name {
                    fn process_transaction(&mut self, client: &mut TridentSVM, config: &TridentConfig, fuzz_accounts: &RefCell<FuzzAccounts>) -> Result<(), FuzzingError> {
                        match self {
                            #(#process_transaction_match_arm)*
                        }
                    }
                }
            }
        }
        _ => panic!("FuzzTestExecutor can only be derived for enums"),
    };

    TokenStream::from(display_impl)
}
