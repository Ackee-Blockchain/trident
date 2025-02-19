#[macro_export]
macro_rules! fuzz_trident {
    (
        |
            $buf:ident: $dty:ident,
            $client:ident: $client_dty:ident,
            $config:ident: $config_dty:ident
        |) => {
        if cfg!(honggfuzz) {
            loop {
                fuzz_honggfuzz(|$buf| {
                    let mut $buf = {
                        use arbitrary::Unstructured;

                        let mut buf = Unstructured::new($buf);
                        if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                            fuzz_data
                        } else {
                            return;
                        }
                    };
                    // TODO: the function might not need to return anything
                    let _ = $buf.run_with_runtime(&mut $client, &$config);
                });
            }
        } else if cfg!(afl) {
            fuzz_afl(true, |$buf| {
                let mut $buf = {
                    use arbitrary::Unstructured;

                    let mut buf = Unstructured::new($buf);
                    if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                        fuzz_data
                    } else {
                        return;
                    }
                };
                // TODO: the function might not need to return anything
                let _ = $buf.run_with_runtime(&mut $client, &$config);
            });
        } else if cfg!(honggfuzz_debug) {
            let mut crash_file = String::new();
            std::io::stdin()
                .read_line(&mut crash_file)
                .expect("Failed to read crash file path from stdin");
            let crash_file = crash_file.trim();

            let mut $buf = {
                let fuzz_data = std::fs::read(crash_file).expect("Failed to read crash file");
                use arbitrary::Unstructured;

                let mut buf = Unstructured::new(&fuzz_data);
                if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                    fuzz_data
                } else {
                    return;
                }
            };
            // TODO: the function might not need to return anything
            let _ = $buf.run_with_runtime(&mut $client, &$config);
        } else {
            panic!("Select Honggfuzz or AFL for fuzzing!!!")
        }
    };
}
