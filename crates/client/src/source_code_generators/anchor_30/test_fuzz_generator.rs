use anchor_lang_idl_spec::Idl;
use quote::ToTokens;
use syn::parse_quote;

pub fn generate_source_code(_idl_instructions: &[Idl]) -> String {
    // let program_imports = get_program_imports(idl_instructions);
    // let program_names = get_program_names(idl_instructions);
    // let (fuzzing_programs, programs_array) = get_fuzzing_programs(idl_instructions);

    let test_fuzz_definition: syn::File = parse_quote! {
        use trident_fuzz::fuzzing::*;
        mod fuzz_instructions;
        use fuzz_instructions::FuzzInstruction;
        use fuzz_instructions::*;

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

        /// `fn fuzz_iteration` runs during every fuzzing iteration.
        /// Modification is not required.
        fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
            fuzz_data: FuzzData<T, U>,
            config: &Config,
            client: &mut impl FuzzClient,
        ) {
            let _ = fuzz_data.run_with_runtime(client, config);
        }
        fn main() {
            let config = Config::new();
            let mut client = TridentSVM::new(&[], &config);
            fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config,&mut client) ; });
        }
    };

    test_fuzz_definition.into_token_stream().to_string()
}
