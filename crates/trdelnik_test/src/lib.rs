use proc_macro::TokenStream;
use syn::{ItemFn, spanned::Spanned,};

// #[trdelnik_test]
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
//     let tester = Tester::new();
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
//     Ok(())
// }
#[proc_macro_attribute]
pub fn trdelnik_test(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn: ItemFn = syn::parse(input)
        .expect("'trdelnik_test' attribute is applicable only to async fn");

    let input_fn_span = input_fn.span();
    let input_fn_body = input_fn.block;
    let input_fn_name = input_fn.sig.ident;

    quote::quote_spanned!(input_fn_span=>
        #[trdelnik::tokio::test(flavor = "multi_thread")]
        #[trdelnik::serial_test::serial]
        async fn #input_fn_name() -> trdelnik::anyhow::Result<()> {
            let tester = trdelnik::Tester::new();
            let localnet_handle = tester.before().await?;
            let test = async {
                #input_fn_body
                Ok::<(), trdelnik::anyhow::Error>(())
            };
            println!("____ TEST ____");
            let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
            tester.after(localnet_handle).await?;
            assert!(result.is_ok());
            Ok(())
        }
    )
    .into()
}
