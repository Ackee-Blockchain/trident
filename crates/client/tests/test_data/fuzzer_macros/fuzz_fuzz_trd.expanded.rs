use trdelnik_client::__private::fuzz_trd;
fn main() {
    loop {
        fuzz(|fuzz_data| {
            let mut fuzz_data: FuzzData<FuzzInstruction, _> = {
                use arbitrary::Unstructured;
                let mut buf = Unstructured::new(fuzz_data);
                if let Ok(fuzz_data) = build_ix_fuzz_data(MyFuzzData {}, &mut buf) {
                    fuzz_data
                } else {
                    return;
                }
            };
            {
                let mut client = ProgramTestClientBlocking::new(
                        PROGRAM_NAME,
                        PROGRAM_ID,
                        xyz,
                    )
                    .unwrap();
                let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
            }
        });
    }
}
