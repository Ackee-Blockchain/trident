use fuzz_example3::entry;
use program_client::fuzz_example3_instruction::*;
use trdelnik_client::{fuzz_trd, fuzzing::*};
use trdelnik_tests::fuzz_instructions::fuzz_example3_fuzz_instructions::{
    FuzzInstruction, InitVesting, WithdrawUnlocked,
};

const PROGRAM_NAME: &str = "fuzz_example3";

struct MyFuzzData;

// impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);
        let withdraw_ix = FuzzInstruction::WithdrawUnlocked(WithdrawUnlocked::arbitrary(u)?);

        Ok(vec![init_ix, withdraw_ix])
    }
}

fn main() {
    loop {
        fuzz_trd!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
