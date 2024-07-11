use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
// mod accounts_snapshots;

const PROGRAM_NAME: &str = "###PROGRAM_NAME###";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(convert_entry!(entry)))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
