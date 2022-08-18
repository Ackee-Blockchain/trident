#[trdelnik_client::rstest]
#[trdelnik_client::tokio::test(flavor = "multi_thread")]
#[trdelnik_client::serial_test::serial]
async fn test_turnstile() -> trdelnik_client::anyhow::Result<()> {
    let mut tester = trdelnik_client::Tester::with_root("../../");
    let localnet_handle = tester.before().await?;
    let test = async {
        {
            init_client().await?;
            let mut turnstile = Turnstile {
                locked: get_state_client().await?.locked,
            };
            turnstile.coin().await?;
            turnstile.push_unlocked().await?;
            turnstile.push_locked().await?;
        }
        Ok::<(), trdelnik_client::anyhow::Error>(())
    };
    let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
    tester.after(localnet_handle).await?;
    if !result.is_ok() {
        ::core::panicking::panic("assertion failed: result.is_ok()")
    }
    let final_result = result.unwrap();
    if let Err(error) = final_result {
        trdelnik_client::error_reporter::report_error(&error);
        return Err(error);
    }
    Ok(())
}
