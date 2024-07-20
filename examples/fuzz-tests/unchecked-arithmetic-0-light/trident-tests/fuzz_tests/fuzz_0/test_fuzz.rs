use fuzz_instructions::unchecked_arithmetic_0_light_fuzz_instructions::Initialize;
use fuzz_instructions::unchecked_arithmetic_0_light_fuzz_instructions::Update;
use unchecked_arithmetic_0_light::entry as entry_unchecked_arithmetic_0_light;
use unchecked_arithmetic_0_light::ID as PROGRAM_ID_UNCHECKED_ARITHMETIC_0_LIGHT;
const PROGRAM_NAME_UNCHECKED_ARITHMETIC_0_LIGHT: &str = "unchecked_arithmetic_0_light";
use fuzz_instructions::unchecked_arithmetic_0_light_fuzz_instructions::FuzzInstruction as FuzzInstruction_unchecked_arithmetic_0_light;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
// mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_unchecked_arithmetic_0_light> for MyFuzzData {
    fn pre_ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_unchecked_arithmetic_0_light>> {
        let init =
            FuzzInstruction_unchecked_arithmetic_0_light::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init])
    }
    fn ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_unchecked_arithmetic_0_light>> {
        let update = FuzzInstruction_unchecked_arithmetic_0_light::Update(Update::arbitrary(u)?);
        Ok(vec![update])
    }
    fn post_ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_unchecked_arithmetic_0_light>> {
        Ok(vec![])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_unchecked_arithmetic_0_light, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgramLight::new(
                PROGRAM_NAME_UNCHECKED_ARITHMETIC_0_LIGHT,
                &PROGRAM_ID_UNCHECKED_ARITHMETIC_0_LIGHT,
                Some(entry_unchecked_arithmetic_0_light)
            );

            let mut client =
                LightClient::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_UNCHECKED_ARITHMETIC_0_LIGHT, &mut client);
        });
    }
}
