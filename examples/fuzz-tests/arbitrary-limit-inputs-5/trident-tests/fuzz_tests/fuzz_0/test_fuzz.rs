use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::InitVesting;
use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::WithdrawUnlocked;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use arbitrary_limit_inputs_5::entry as entry_arbitrary_limit_inputs_5;
use arbitrary_limit_inputs_5::ID as PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5;

const PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5: &str = "arbitrary_limit_inputs_5";

use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::FuzzInstruction as fuzz_instruction_arbitrary_limit_inputs_5;

pub type FuzzInstruction = fuzz_instruction_arbitrary_limit_inputs_5;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
    fn ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let withdraw_ix = FuzzInstruction::WithdrawUnlocked(WithdrawUnlocked::arbitrary(u)?);
        Ok(vec![withdraw_ix])
    }
    fn post_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {
    let fuzzing_program_arbitrary_limit_inputs_5 = FuzzingProgram::new(
        PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5,
        &PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5,
        processor!(convert_entry!(entry_arbitrary_limit_inputs_5)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_arbitrary_limit_inputs_5], &[]).unwrap();

    let _ = fuzz_data.run_with_runtime(PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5, &mut client);
}

fn main() {
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
