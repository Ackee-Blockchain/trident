use anchor_lang_idl_spec::Idl;
use convert_case::{Case, Casing};
use quote::{format_ident, ToTokens};
use syn::parse_quote;

pub fn generate_source_code(idl_instructions: &[Idl]) -> String {
    let program_imports = get_program_imports(idl_instructions);
    let program_names = get_program_names(idl_instructions);
    let fuzz_instructions = get_fuzz_instructions(idl_instructions);
    let (fuzzing_programs, programs_array) = get_fuzzing_programs(idl_instructions);

    let main_fuzz_instruction: syn::ItemType = match fuzz_instructions.len() {
        1 => {
            let program_name = idl_instructions[0].metadata.name.to_case(Case::Snake);
            let alias = format_ident!("fuzz_instruction_{}", program_name);

            parse_quote!(
                pub type FuzzInstruction = #alias;
            )
        }
        _ => parse_quote!(
            pub type FuzzInstruction = todo!();
        ),
    };

    let run_with_runtime: syn::Stmt = match fuzz_instructions.len() {
        1 => {
            let program_name_upper = idl_instructions[0].metadata.name.to_case(Case::UpperSnake);
            let program_id_name_ident = format_ident!("PROGRAM_ID_{}", program_name_upper);

            parse_quote!(
                let _ = fuzz_data.run_with_runtime(#program_id_name_ident, &mut client);
            )
        }
        _ => parse_quote!(
            let _ = fuzz_data.run_with_runtime(todo!(), &mut client);
        ),
    };

    let test_fuzz_definition: syn::File = parse_quote! {
        use trident_client::fuzzing::*;
        mod fuzz_instructions;

        #(#program_imports)*

        #(#program_names)*

        #(#fuzz_instructions)*

        #main_fuzz_instruction

        struct MyFuzzData;

        impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

        fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {


            #(#fuzzing_programs)*

            let mut client = ProgramTestClientBlocking::new(&#programs_array).unwrap();

            #run_with_runtime


        }

        fn main() {
            loop {
                fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {


                    fuzz_iteration(fuzz_data);


                });
            }
        }
    };

    test_fuzz_definition.into_token_stream().to_string()
}

fn get_fuzz_instructions(idl_instructions: &[Idl]) -> Vec<syn::ItemUse> {
    idl_instructions.iter().map(|idl| {
        let program_name = idl.metadata.name.to_case(Case::Snake);
        let fuzz_instructions_program_ident = format_ident!("{}_fuzz_instructions", program_name);
        let alias = format_ident!("fuzz_instruction_{}", program_name);

        parse_quote!(use fuzz_instructions::#fuzz_instructions_program_ident::FuzzInstruction as #alias;)
    }).collect()
}

fn get_program_names(idl_instructions: &[Idl]) -> Vec<syn::Stmt> {
    idl_instructions
        .iter()
        .map(|idl| {
            let program_name = &idl.metadata.name;
            let program_name_upper = idl.metadata.name.to_case(Case::UpperSnake);
            let program_name_ident = format_ident!("PROGRAM_NAME_{}", program_name_upper);

            parse_quote!(const #program_name_ident: &str = #program_name;)
        })
        .collect()
}

fn get_program_imports(idl_instructions: &[Idl]) -> Vec<syn::ItemUse> {
    idl_instructions
        .iter()
        .flat_map(|idl| {
            let program_name = idl.metadata.name.to_case(Case::Snake);
            let program_name_upper = idl.metadata.name.to_case(Case::UpperSnake);
            let program_name_ident = format_ident!("{}", program_name);
            let program_entry_ident = format_ident!("entry_{}", program_name);
            let program_id_name_ident = format_ident!("PROGRAM_ID_{}", program_name_upper);

            vec![
                parse_quote!(use #program_name_ident::entry as #program_entry_ident;),
                parse_quote!(use #program_name_ident::ID as #program_id_name_ident;),
            ]
        })
        .collect()
}

fn get_fuzzing_programs(idl_instructions: &[Idl]) -> (Vec<syn::Stmt>, syn::ExprArray) {
    // Vector to collect the `fuzzing_program_name_ident` identifiers
    let mut fuzzing_program_idents = Vec::new();

    // Collect the statements and the identifiers for the `ExprArray`
    let fuzzing_programs: Vec<syn::Stmt> = idl_instructions
        .iter()
        .map(|idl| {
            let program_name = idl.metadata.name.to_case(Case::Snake);
            let program_name_upper = idl.metadata.name.to_case(Case::UpperSnake);
            let fuzzing_program_name_ident = format_ident!("fuzzing_program_{}", program_name);
            let program_id_name_ident = format_ident!("PROGRAM_ID_{}", program_name_upper);
            let program_name_ident = format_ident!("PROGRAM_NAME_{}", program_name_upper);
            let program_entry_ident = format_ident!("entry_{}", program_name);

            // Add the identifier to the vector
            fuzzing_program_idents.push(fuzzing_program_name_ident.clone());

            // Generate the statement
            parse_quote! {
                let #fuzzing_program_name_ident = FuzzingProgram::new(
                    #program_name_ident,
                    &#program_id_name_ident,
                    processor!(convert_entry!(#program_entry_ident))
                );
            }
        })
        .collect();

    // Create the ExprArray from the collected identifiers
    let expr_array: syn::ExprArray = syn::ExprArray {
        attrs: vec![],
        bracket_token: syn::token::Bracket::default(),
        elems: fuzzing_program_idents
            .into_iter()
            .map(|ident| {
                syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: ident.into(),
                })
            })
            .collect(),
    };

    // Return the vector of statements and the ExprArray
    (fuzzing_programs, expr_array)
}
