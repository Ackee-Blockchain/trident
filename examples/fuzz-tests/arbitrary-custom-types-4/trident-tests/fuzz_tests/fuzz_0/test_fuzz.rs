use arbitrary_custom_types_4::entry as entry_arbitrary_custom_types_4;
use arbitrary_custom_types_4::ID as PROGRAM_ID_ARBITRARY_CUSTOM_TYPES_4;
const PROGRAM_NAME_ARBITRARY_CUSTOM_TYPES_4: &str = "arbitrary_custom_types_4";
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::FuzzInstruction as FuzzInstruction_arbitrary_custom_types_4;
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::Initialize;
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::Update;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_arbitrary_custom_types_4> for MyFuzzData {
    fn pre_ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_arbitrary_custom_types_4>> {
        let init = FuzzInstruction_arbitrary_custom_types_4::Initialize(Initialize::arbitrary(u)?);
        let update = FuzzInstruction_arbitrary_custom_types_4::Update(Update::arbitrary(u)?);
        Ok(vec![init, update])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_arbitrary_custom_types_4, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_ARBITRARY_CUSTOM_TYPES_4,
                &PROGRAM_ID_ARBITRARY_CUSTOM_TYPES_4,
                processor!(convert_entry!(entry_arbitrary_custom_types_4))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_ARBITRARY_CUSTOM_TYPES_4, &mut client);
        });
    }
}
