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
        } else {
            panic!("Select Honggfuzz or AFL for fuzzing!!!")
        }
    };
}
