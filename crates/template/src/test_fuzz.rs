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

        let mut input_array: Vec<syn::Ident> = Vec::new();

        for program in lib_names {
            let (use_statement, program, program_variable) =
                process_program_entries(program, program_ids.get(program));
            // add to the use statements
            use_statements.push(use_statement);
            // add to the programs
            programs.push(program);
            // add to the input array
            input_array.push(program_variable);
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

            #[derive(Default, FuzzTestExecutor)]
            struct FuzzTest<'a> {
                config: TridentConfig,
                client: TridentSVM<'a>,
            }

            #[flow_executor]
            impl<'a> FuzzTest {
                #[init]
                fn start(&mut self) {}
            }


            fn main() {

                #(#programs)*

                let config = TridentConfig::new();
                let client = TridentSVM::new_client(&[ #(#input_array),* ], &config);

                let mut fuzz_test = FuzzTest::new(client, config);

                fuzz_test.fuzz();
            }
        };

        self.test_fuzz = Some(test_fuzz_definition);
    }
}

fn process_program_entries(
    lib_name: &String,
    program_id: Option<&String>,
) -> (syn::ItemUse, syn::Stmt, syn::Ident) {
    // library name as identifier
    let library = format_ident!("{}", lib_name);
    // entry name as identifier
    let library_entry = format_ident!("entry_{}", lib_name);
    // variable name as identifier
    let variable_name = format_ident!("program_{}", library);

    // initial use statement
    let use_statement = parse_quote!(use #library::entry as #library_entry;);

    // program id if present, otherwise fill with placeholder
    let program_id = match program_id {
        Some(address) if !address.trim().is_empty() => address,
        _ => "fill corresponding program ID here",
    };

    // program definition
    let program = parse_quote! {
        let #variable_name = ProgramEntrypoint::new(
            pubkey!(#program_id),
            None,
            processor!(#library_entry)
        );
    };

    (use_statement, program, variable_name)
}
