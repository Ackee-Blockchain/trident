use fuzz_instructions::unchecked_arithmetic_0_fuzz_instructions::Initialize;
use unchecked_arithmetic_0::entry as entry_unchecked_arithmetic_0;
use unchecked_arithmetic_0::ID as PROGRAM_ID_UNCHECKED_ARITHMETIC_0;
const PROGRAM_NAME_UNCHECKED_ARITHMETIC_0: &str = "unchecked_arithmetic_0";
use fuzz_instructions::unchecked_arithmetic_0_fuzz_instructions::FuzzInstruction as FuzzInstruction_unchecked_arithmetic_0;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_unchecked_arithmetic_0> for MyFuzzData {
    fn pre_ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_unchecked_arithmetic_0>> {
        let init = FuzzInstruction_unchecked_arithmetic_0::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_unchecked_arithmetic_0, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_UNCHECKED_ARITHMETIC_0,
                &PROGRAM_ID_UNCHECKED_ARITHMETIC_0,
                processor!(convert_entry!(entry_unchecked_arithmetic_0))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_UNCHECKED_ARITHMETIC_0, &mut client);
        });
    }
}
