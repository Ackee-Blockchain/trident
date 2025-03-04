use quote::format_ident;
use std::collections::HashMap;
use syn::parse_quote;

use crate::Template;

impl Template {
    pub(crate) fn test_fuzz(
        &mut self,
        program_ids: &HashMap<String, String>,
        lib_names: &[String],
    ) {
        let mut use_statements: Vec<syn::ItemUse> = Vec::new();

        let mut programs: Vec<syn::Stmt> = Vec::new();

        for program in lib_names {
            let (use_statement, program) =
                process_program_entries(program, program_ids.get(program));
            // add to the use statements
            use_statements.push(use_statement);
            // add to the programs
            programs.push(program);
        }

        let test_fuzz_definition: syn::File = parse_quote! {
            use trident_fuzz::fuzzing::*;
            use fuzz_transactions::*;
            mod fuzz_transactions;
            mod instructions;
            mod transactions;
            mod types;
            pub use transactions::*;

            #(#use_statements)*

            #[derive(Default)]
            struct FuzzTest<C> {
                client: C,
            }

            /// Use flows to specify custom sequences of behavior
            /// #[init]
            /// fn start(&mut self) {
            ///     // Initialization goes here
            /// }
            /// #[flow]
            /// fn flow1(
            ///     &mut self,
            ///     fuzzer_data: &mut FuzzerData,
            ///     accounts: &mut FuzzAccounts,
            /// ) -> Result<(), FuzzingError> {
            ///     // Flow logic goes here
            ///     Ok(())
            /// }
            #[flow_executor]
            impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
                fn new(client: C) -> Self {
                    Self { client }
                }
                #[init]
                fn start(&mut self) {
                    #(#programs)*
                }
            }


            fn main() {

                let client = TridentSVM::new_client(&[], &TridentConfig::new());

                FuzzTest::new(client).fuzz();
            }
        };

        self.test_fuzz = Some(test_fuzz_definition);
    }
}

fn process_program_entries(
    lib_name: &String,
    program_id: Option<&String>,
) -> (syn::ItemUse, syn::Stmt) {
    // library name as identifier
    let library = format_ident!("{}", lib_name);
    // entry name as identifier
    let library_entry = format_ident!("entry_{}", lib_name);
    // initial use statement
    let use_statement = parse_quote!(use #library::entry as #library_entry;);

    // program id if present, otherwise fill with placeholder
    let program_id = match program_id {
        Some(address) if !address.trim().is_empty() => address,
        _ => "fill corresponding program ID here",
    };

    // program definition and deployment as a single expression
    let program_stmt: syn::Stmt = parse_quote! {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!(#program_id),
            None,
            processor!(#library_entry)
        ));
    };

    (use_statement, program_stmt)
}
