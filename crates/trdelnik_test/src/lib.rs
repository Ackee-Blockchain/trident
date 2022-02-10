use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn};

// #[trdelnik_test(root = "../../")]
// async fn test_turnstile() {
//     init_client().await?;
//     let mut turnstile = Turnstile {
//         locked: get_state_client().await?.locked
//     };
//     turnstile.coin().await?;
//     turnstile.push_unlocked().await?;
//     turnstile.push_locked().await?;
// }
//
// to
//
// #[trdelnik::tokio::test(flavor = "multi_thread")]
// #[trdelnik::serial_test::serial]
// async fn test_turnstile() -> trdelnik::anyhow::Result<()> {
//     let mut tester = trdelnik::Tester::with_root(#root);
//     let localnet_handle = tester.before().await?;
//     let test = async {
//         init_client().await?;
//         let mut turnstile = Turnstile {
//             locked: get_state_client().await?.locked
//         };
//         turnstile.coin().await?;
//         turnstile.push_unlocked().await?;
//         turnstile.push_locked().await?;
//         Ok::<(), trdelnik::anyhow::Error>(())
//     };
//     println!("____ TEST ____");
//     let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
//     tester.after(localnet_handle).await?;
//     assert!(result.is_ok());
//     result.unwrap()?;
//     Ok(())
// }

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    root: Option<String>,
}

/// The macro starts the Solana validator (localnet), runs your program test and then shuts down the validator.
/// - The test implicitly returns [anyhow::Result<()>](https://docs.rs/anyhow/latest/anyhow/type.Result.html).
/// - All tests are run sequentially - each test uses a new/reset validator. (See [serial_test::serial](https://docs.rs/serial_test/latest/serial_test/attr.serial.html)) 
/// - Async support is provided by Tokio: [tokio::test(flavor = "multi_thread")](https://docs.rs/tokio/latest/tokio/attr.test.html).
/// - The macro accepts one optional argument `root` with the default value `"../../"`.
///      - Example: `#[trdelnik_test(root = "../../")]`
/// 
/// # Example 
/// 
/// ```rust,no_run
/// // tests/test.rs
/// use trdelnik::*;
///
/// #[trdelnik_test]
/// async fn test_turnstile() {
///     let reader = Reader::new();
///     let mut turnstile = Turnstile {
///         client: Client::new(reader.keypair("id").await?),
///         state: reader.keypair("state").await?,
///         program: reader.keypair("program").await?,
///         program_data: reader.program_data("turnstile").await?,
///         locked: bool::default(),
///     };
///     turnstile.initialize().await?;
/// }
/// ```
#[proc_macro_attribute]
pub fn trdelnik_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let macro_args = match MacroArgs::from_list(&attr_args) {
        Ok(macro_args) => macro_args,
        Err(error) => {
            return TokenStream::from(error.write_errors());
        }
    };
    let root = macro_args.root.unwrap_or_else(|| "../../".to_owned());

    let input_fn: ItemFn =
        syn::parse(input).expect("'trdelnik_test' attribute is applicable only to async fn");

    let input_fn_span = input_fn.span();
    let input_fn_body = input_fn.block;
    let input_fn_name = input_fn.sig.ident;

    quote::quote_spanned!(input_fn_span=>
        #[trdelnik::tokio::test(flavor = "multi_thread")]
        #[trdelnik::serial_test::serial]
        async fn #input_fn_name() -> trdelnik::anyhow::Result<()> {
            let mut tester = trdelnik::Tester::with_root(#root);
            let localnet_handle = tester.before().await?;
            let test = async {
                #input_fn_body
                Ok::<(), trdelnik::anyhow::Error>(())
            };
            println!("____ TEST ____");
            let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
            tester.after(localnet_handle).await?;
            assert!(result.is_ok());
            result.unwrap()?;
            Ok(())
        }
    )
    .into()
}
