use proc_macro::TokenStream;
use syn::{ItemFn, spanned::Spanned, parse_macro_input, AttributeArgs};
use darling::FromMeta;

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

#[proc_macro_attribute]
pub fn trdelnik_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let macro_args = match MacroArgs::from_list(&attr_args) {
        Ok(macro_args) => macro_args,
        Err(error) => { return TokenStream::from(error.write_errors()); }
    };
    let root = macro_args.root.unwrap_or_else(|| "../../".to_owned());

    let input_fn: ItemFn = syn::parse(input)
        .expect("'trdelnik_test' attribute is applicable only to async fn");

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
