#[macro_export]
macro_rules! fuzz_trident {
    ($ix:ident: $ix_dty:ident , |$buf:ident: $dty:ident| $body:block) => {
        fuzz(|$buf| {
            let mut $buf: FuzzData<$ix_dty, _> = {
                use arbitrary::Unstructured;

                let mut buf = Unstructured::new($buf);
                if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                    fuzz_data
                } else {
                    return;
                }
            };
            $body
        });
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
