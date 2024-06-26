#[trident_client::test::rstest]
#[trident_client::test::tokio::test(flavor = "multi_thread")]
#[trident_client::test::serial_test::serial]
async fn test_with_defined_root() -> trident_client::test::anyhow::Result<()> {
    let mut tester = trident_client::test::Tester::with_root("i_am_root");
    let localnet_handle = tester.before().await?;
    let test = async {
        {}
        Ok::<(), trident_client::test::anyhow::Error>(())
    };
    let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
    tester.after(localnet_handle).await?;
    if !result.is_ok() {
        ::core::panicking::panic("assertion failed: result.is_ok()")
    }
    let final_result = result.unwrap();
    if let Err(error) = final_result {
        trident_client::test::report_error(&error);
        return Err(error);
    }
    Ok(())
}
