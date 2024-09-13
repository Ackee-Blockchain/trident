use fuzz_instructions::cpi_metaplex_7_fuzz_instructions::Initialize;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use cpi_metaplex_7::entry as entry_cpi_metaplex_7;
use cpi_metaplex_7::ID as PROGRAM_ID_CPI_METAPLEX_7;

const PROGRAM_NAME_CPI_METAPLEX_7: &str = "cpi_metaplex_7";

use fuzz_instructions::cpi_metaplex_7_fuzz_instructions::FuzzInstruction as fuzz_instruction_cpi_metaplex_7;

pub type FuzzInstruction = fuzz_instruction_cpi_metaplex_7;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init])
    }
    fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {
    let fuzzing_program_cpi_metaplex_7 = FuzzingProgram::new(
        PROGRAM_NAME_CPI_METAPLEX_7,
        &PROGRAM_ID_CPI_METAPLEX_7,
        processor!(convert_entry!(entry_cpi_metaplex_7)),
    );

    let metaplex = FuzzingProgram::new("metaplex-token-metadata", &mpl_token_metadata::ID, None);

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_cpi_metaplex_7, metaplex], &[]).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}

fn main() {
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
