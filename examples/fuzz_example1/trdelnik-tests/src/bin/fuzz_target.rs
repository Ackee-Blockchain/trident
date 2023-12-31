use fuzz_example1::{entry, instructions::register};
use program_client::fuzz_example1_instruction::*;
use trdelnik_client::{fuzz_trd, fuzzing::*};
use trdelnik_tests::fuzz_instructions::fuzz_example1_fuzz_instructions::{
    EndRegistrations, FuzzInstruction, Initialize, Register,
};

const PROGRAM_NAME: &str = "fuzz_example1";

struct MyFuzzData;

// impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);

        let register_ix = FuzzInstruction::Register(Register::arbitrary(u)?);
        let end_register_ix = FuzzInstruction::EndRegistrations(EndRegistrations::arbitrary(u)?);

        Ok(vec![init_ix, register_ix, end_register_ix])
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
