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
                    let (mut signers, metas) =
                        if let Ok(acc) = ix.get_accounts(client, &mut accounts.borrow_mut())
                        .map_err(|e| e.with_origin(Origin::Instruction(self.to_context_string()))) {
                            acc
                        } else {
                            return Ok(());
                        };
                    let mut snaphot = Snapshot::new(&metas, ix);
                    // this can return FuzzClientErrorWithOrigin
                    snaphot.capture_before(client).unwrap();
                    let data =
                        if let Ok(data) = ix.get_data(client, &mut accounts.borrow_mut())
                        .map_err(|e| e.with_origin(Origin::Instruction(self.to_context_string()))) {
                            data
                        } else {
                            return Ok(());
                        };
                    let ixx = Instruction {
                        program_id,
                        accounts: metas.clone(),
                        data: data.data(),
                    };
                    let mut transaction =
                        Transaction::new_with_payer(&[ixx], Some(&client.payer().pubkey()));
                    signers.push(client.payer().clone());
                    let sig: Vec<&Keypair> = signers.iter().collect();
                    transaction.sign(&sig, client.get_last_blockhash());

                    let res = client.process_transaction(transaction)
                    .map_err(|e| e.with_origin(Origin::Instruction(self.to_context_string())));

                    // this can return FuzzClientErrorWithOrigin
                    snaphot.capture_after(client).unwrap();
                    let (acc_before, acc_after) = snaphot.get_snapshot()
                        .map_err(|e| e.with_origin(Origin::Instruction(self.to_context_string()))).unwrap(); // we want to panic if we cannot unwrap to cause a crash

                    if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| e.with_origin(Origin::Instruction(self.to_context_string()))) {
                        eprintln!(
                            "Custom check after the {} instruction did not pass with the error message: {}",
                            self.to_context_string(), e
                        );
                        eprintln!("Instruction data submitted to the instruction were:"); // TODO data does not implement Debug trait -> derive Debug trait on InitializeIx and automaticaly implement conversion from Initialize to InitializeIx
                        panic!("{}", e)
                    }

                    if res.is_err() {
                        return Ok(());
                    }
                }
                }
            });

            quote! {
               impl FuzzTestExecutor<FuzzAccounts> for FuzzInstruction {
                   fn run_fuzzer(
                       &self,
                       program_id: Pubkey,
                       accounts: &RefCell<FuzzAccounts>,
                       client: &mut impl FuzzClient,
                   ) -> core::result::Result<(), Box<dyn std::error::Error + 'static>> {
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
