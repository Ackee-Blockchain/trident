#[trdelnik_client::tokio::test(flavor = "multi_thread")]
#[trdelnik_client::serial_test::serial]
async fn test_with_defined_root() -> trdelnik_client::anyhow::Result<()> {
    let mut tester = trdelnik_client::Tester::with_root("i_am_root");
    let localnet_handle = tester.before().await?;
    let test = async {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello!\n"], &[]));
        }
        Ok::<(), trdelnik_client::anyhow::Error>(())
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["____ TEST ____\n"], &[]));
    };
    let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
    tester.after(localnet_handle).await?;
    if !result.is_ok() {
        ::core::panicking::panic("assertion failed: result.is_ok()")
    };
    result.unwrap()?;
    Ok(())
}
