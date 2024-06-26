//! The `trident_test` crate helps you to write Rust tests for your _programs_ with Trident.
//! See the macro [trident_test] for more info.
//!
//! _Dev Note_: You need `cargo expand` and nightly Rust to run tests. See [macrotest docs](https://docs.rs/macrotest/latest/macrotest/#workflow).

use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn};

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
///      - Example: `#[trident_test(root = "../../")]`
/// - You can see the macro expanded in the crate's tests.
///
/// # Example
///
/// ```rust,ignore
/// // tests/test.rs
/// use trident_client::*;
///
/// #[trident_test]
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
pub fn trident_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let macro_args = match MacroArgs::from_list(&attr_args) {
        Ok(macro_args) => macro_args,
        Err(error) => {
            return TokenStream::from(error.write_errors());
        }
    };
    let root = macro_args.root.unwrap_or_else(|| "../../".to_owned());

    let input_fn: ItemFn =
        syn::parse(input).expect("'trident_test' attribute is applicable only to async fn");

    let input_fn_span = input_fn.span();
    let input_fn_body = input_fn.block;
    let input_fn_name = input_fn.sig.ident;
    let input_fn_attrs = input_fn.attrs;
    let input_fn_inputs = input_fn.sig.inputs;

    quote::quote_spanned!(input_fn_span=>
        #(#input_fn_attrs)*
        // Note: The line `#(#input_fn_attrs)*` has to be above the line with the code
        // `#[trident_client::tokio::test...` to make macros like `#[rstest]` work -
        // see https://github.com/la10736/rstest#inject-test-attribute
        #[trident_client::test::rstest]
        #[trident_client::test::tokio::test(flavor = "multi_thread")]
        #[trident_client::test::serial_test::serial]
        async fn #input_fn_name(#input_fn_inputs) -> trident_client::test::anyhow::Result<()> {
            let mut tester = trident_client::test::Tester::with_root(#root);
            let localnet_handle = tester.before().await?;
            let test = async {
                #input_fn_body
                Ok::<(), trident_client::test::anyhow::Error>(())
            };
            let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
            tester.after(localnet_handle).await?;
            assert!(result.is_ok());
            let final_result = result.unwrap();
            if let Err(error) = final_result {
                trident_client::test::report_error(&error);
                return Err(error);
            }
            Ok(())
        }
    )
    .into()
}
