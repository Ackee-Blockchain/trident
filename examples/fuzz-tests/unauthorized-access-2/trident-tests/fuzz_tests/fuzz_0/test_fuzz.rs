use fuzz_instructions::Initialize;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use fuzz_instructions::FuzzInstruction;
use unauthorized_access_2::entry as entry_unauthorized_access_2;
use unauthorized_access_2::ID as PROGRAM_ID_UNAUTHORIZED_ACCESS_2;

const PROGRAM_NAME_UNAUTHORIZED_ACCESS_2: &str = "unauthorized_access_2";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_unauthorized_access_2 = FuzzingProgram::new(
        PROGRAM_NAME_UNAUTHORIZED_ACCESS_2,
        &PROGRAM_ID_UNAUTHORIZED_ACCESS_2,
        processor!(convert_entry!(entry_unauthorized_access_2)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_unauthorized_access_2], config).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client, config);
}

fn main() {
    let config = Config::new();

    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
}
