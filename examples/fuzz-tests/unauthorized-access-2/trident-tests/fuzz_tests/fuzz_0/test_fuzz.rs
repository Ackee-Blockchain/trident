use fuzz_instructions::unauthorized_access_2_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::unauthorized_access_2_fuzz_instructions::Initialize;
use trident_client::fuzzing::*;
use unauthorized_access_2::entry;
use unauthorized_access_2::ID as PROGRAM_ID;
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "unauthorized_access_2";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

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
