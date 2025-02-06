#[macro_export]
macro_rules! fuzz_trident {
    (
        $ix:ident: $ix_dty:ident ,
        |
            $buf:ident: $dty:ident,
            $client:ident: $client_dty:ident,
            $config:ident: $config_dty:ident
        |) => {
        let mut metrics = std::sync::Arc::new(std::sync::Mutex::new(FuzzingMetrics::new()));
        let metrics_clone = std::sync::Arc::clone(&metrics);

        let mut signals = Signals::new([SIGINT]).unwrap();
        std::thread::spawn(move || {
            if let Some(_) = signals.forever().next() {
                let metrics_data = metrics_clone.lock().unwrap();
                let mut file = std::fs::File::create("signal_triggered.txt").unwrap();
                writeln!(
                    file,
                    "SIGINT was triggered!\nFinal metrics:\n{:#?}",
                    *metrics_data
                )
                .unwrap();
                // std::process::exit(0);
                panic!("SIGINT was triggered!");
            }
        });

        if cfg!(honggfuzz) {
            loop {
                fuzz_honggfuzz(|$buf| {
                    let mut $buf: FuzzData<$ix_dty, _> = {
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
            let metrics_clone = std::sync::Arc::clone(&metrics);

            fuzz_afl(true, |$buf| {
                metrics_clone
                    .lock()
                    .unwrap()
                    .increase_invoked("test".to_string());
                let mut $buf: FuzzData<$ix_dty, _> = {
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

/// Prints the details of a given account in a pretty-printed format.
///
/// This macro takes a single argument, which is an expression referring to the account
/// you want to print. The account data structure must implement or derive the [`Debug`]
/// trait for this macro to work, as it relies on `std::fmt::Debug` for formatting.
///
/// # Examples
///
/// ```rust,ignore
/// use trident_client::fuzzing::show_account;
///
/// #[derive(Debug)]
/// #[account]
/// struct Escrow {
///     recipeint: Pubkey,
///     id: u32,
///     balance: f64,
///     name: String,
/// }
///
/// fn check(
///     &self,
///     pre_ix: Self::IxSnapshot,
///     post_ix: Self::IxSnapshot,
///     ix_data: Self::IxData,
/// ) -> Result<(), FuzzingError> {
///     if let Some(escrow) = pre_ix.escrow{
///         show_account!(escrow);
///     }
/// }
/// ```
///
/// # Requirements
///
/// The `account` passed to `show_account!` must implement or derive the [`Debug`] trait.
/// Attempting to use this macro with a type that does not meet this requirement will
/// result in a compilation error.
#[macro_export]
macro_rules! show_account {
    ($account:expr) => {
        eprintln!("{:#?}", $account);
    };
}
