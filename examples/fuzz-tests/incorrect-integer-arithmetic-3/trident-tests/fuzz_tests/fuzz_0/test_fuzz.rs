use fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use incorrect_integer_arithmetic_3::entry as entry_incorrect_integer_arithmetic_3;
use incorrect_integer_arithmetic_3::ID as PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3;
const PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3: &str = "incorrect_integer_arithmetic_3";
struct MyFuzzData;
/// Define instruction sequences for invocation.
/// `pre_ixs` runs at the start, `ixs` in the middle, and `post_ixs` at the end.
/// For example, to call `InitializeFn` at the start of each fuzzing iteration:
/// ```
/// fn pre_ixs(u: &mut arbitrary::Unstructured) ->
/// arbitrary::Result<Vec<FuzzInstruction>> {
///     let init = FuzzInstruction::InitializeFn(InitializeFn::arbitrary(u)?);
///     Ok(vec![init])
/// }
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_incorrect_integer_arithmetic_3 = FuzzingProgram::new(
        PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3,
        &PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3,
        processor!(convert_entry!(entry_incorrect_integer_arithmetic_3)),
    );
    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_incorrect_integer_arithmetic_3], config)
            .unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
}
