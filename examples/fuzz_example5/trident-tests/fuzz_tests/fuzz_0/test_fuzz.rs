use fuzz_example5::entry;
use fuzz_example5::ID as PROGRAM_ID;
use fuzz_instructions::fuzz_example5_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::fuzz_example5_fuzz_instructions::InitVesting;
use fuzz_instructions::fuzz_example5_fuzz_instructions::WithdrawUnlocked;
use trident_client::{fuzz_trd, fuzzing::*};
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "fuzz_example5";

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
