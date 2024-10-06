use fuzz_instructions::InitializeFn;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use fuzz_instructions::FuzzInstruction;
use hello_world::entry as entry_hello_world;
use hello_world::ID as PROGRAM_ID_HELLO_WORLD;

const PROGRAM_NAME_HELLO_WORLD: &str = "hello_world";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::InitializeFn(InitializeFn::arbitrary(u)?);
        Ok(vec![init])
    }
    fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_hello_world = FuzzingProgram::new(
        PROGRAM_NAME_HELLO_WORLD,
        &PROGRAM_ID_HELLO_WORLD,
        processor!(convert_entry!(entry_hello_world)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_hello_world], config).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client, config);
}

fn main() {
    let config = Config::new();

    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
    }
}
