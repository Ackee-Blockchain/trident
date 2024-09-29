use fuzz_instructions::Initialize;
use fuzz_instructions::Update;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use arbitrary_custom_types_4::entry as entry_arbitrary_custom_types_4;
use arbitrary_custom_types_4::ID as PROGRAM_ID_ARBITRARY_CUSTOM_TYPES_4;
use fuzz_instructions::FuzzInstruction;

const PROGRAM_NAME_ARBITRARY_CUSTOM_TYPES_4: &str = "arbitrary_custom_types_4";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        let update = FuzzInstruction::Update(Update::arbitrary(u)?);
        Ok(vec![init, update])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_arbitrary_custom_types_4 = FuzzingProgram::new(
        PROGRAM_NAME_ARBITRARY_CUSTOM_TYPES_4,
        &PROGRAM_ID_ARBITRARY_CUSTOM_TYPES_4,
        processor!(convert_entry!(entry_arbitrary_custom_types_4)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_arbitrary_custom_types_4], &[]).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client, config);
}

fn main() {
    let config = Config::new();
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data,&config) ; });
    }
}
