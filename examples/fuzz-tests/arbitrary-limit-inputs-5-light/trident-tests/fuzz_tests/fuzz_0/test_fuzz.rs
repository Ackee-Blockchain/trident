use arbitrary_limit_inputs_5_light::entry as entry_arbitrary_limit_inputs_5_light;
use arbitrary_limit_inputs_5_light::ID as PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5_LIGHT;
const PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5_LIGHT: &str = "arbitrary_limit_inputs_5_light";
use fuzz_instructions::arbitrary_limit_inputs_5_light_fuzz_instructions::FuzzInstruction as FuzzInstruction_arbitrary_limit_inputs_5_light;
use fuzz_instructions::arbitrary_limit_inputs_5_light_fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
// mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_arbitrary_limit_inputs_5_light> for MyFuzzData {
    fn pre_ixs(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Vec<FuzzInstruction_arbitrary_limit_inputs_5_light>> {
        let init_ix =
            FuzzInstruction_arbitrary_limit_inputs_5_light::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_arbitrary_limit_inputs_5_light, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgramLight::new(
                PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5_LIGHT,
                &PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5_LIGHT,
                Some(entry_arbitrary_limit_inputs_5_light)
            );

            let mut client =
                LightClient::new(&[fuzzing_program1])
                    .unwrap();

            client.init();
            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5_LIGHT, &mut client);
        });
    }
}
