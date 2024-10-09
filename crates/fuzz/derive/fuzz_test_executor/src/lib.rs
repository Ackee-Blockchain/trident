use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FuzzTestExecutor)]
pub fn fuzz_test_executor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let display_impl = match &input.data {
        Data::Enum(enum_data) => {
            let display_match_arms = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #enum_name::#variant_name (ix) => {

                        // snapshot has to live as long as ix, thus we declare it here
                        let mut snaphot = Snapshot::new_empty(ix);

                        if cfg!(honggfuzz){
                            TransactionExecutor::process_transaction_honggfuzz(
                                &self.to_context_string(),
                                client,
                                ix,
                                &mut snaphot,
                                sent_txs,
                                config,
                                accounts
                            )?;
                        }else if cfg!(afl){
                            TransactionExecutor::process_transaction_afl(
                                &self.to_context_string(),
                                client,
                                ix,
                                &mut snaphot,
                                sent_txs,
                                config,
                                accounts
                            )?;
                        }

                    }
                }
            });

            quote! {
               impl FuzzTestExecutor<FuzzAccounts> for FuzzInstruction {
                   fn run_fuzzer(
                       &self,
                       accounts: &RefCell<FuzzAccounts>,
                       client: &mut impl FuzzClient,
                       sent_txs: &mut HashMap<Hash, ()>,
                       config: &Config,
                   ) -> core::result::Result<(), FuzzClientErrorWithOrigin> {
                           match self {
                               #(#display_match_arms)*
                           }
                           Ok(())
                   }
                }
            }
        }
        _ => panic!("FuzzTestExecutor can only be derived for enums"),
    };

    TokenStream::from(display_impl)
}
