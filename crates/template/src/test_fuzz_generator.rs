use quote::{format_ident, ToTokens};
use std::collections::HashMap;
use syn::parse_quote;
use trident_idl_spec::Idl;

pub fn generate_source_code(idl_instructions: &[Idl], lib_names: &[String]) -> String {
    let mut program_ids = HashMap::new();

    for idl in idl_instructions {
        let program_name = idl.metadata.name.clone();
        let program_id = idl.address.clone();
        program_ids.insert(program_name, program_id);
    }

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
        mod fuzz_instructions;
        use fuzz_instructions::FuzzInstruction;
        use fuzz_instructions::*;

        #(#use_statements)*

        struct InstructionsSequence;


        /// Define instruction sequences for invocation.
        /// `pre` runs at the start, `middle` in the middle, and `post` at the end.
        /// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during each fuzzing iteration:
        /// ```
        /// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
        ///     pre_sequence!(InitializeFn,UpdateFn);
        ///     middle_sequence!(WithdrawFn);
        ///}
        /// ```
        /// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
        impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {}

        fn main() {

            #(#programs)*

            let config = TridentConfig::new();
            let mut client = TridentSVM::new_client(&[ #(#input_array),* ], &config);
            fuzz_trident!(
                fuzz_ix: FuzzInstruction,
                |fuzz_data: InstructionsSequence, client: TridentSVM, config: TridentConfig|
            );
        }
    };

    test_fuzz_definition.into_token_stream().to_string()
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
