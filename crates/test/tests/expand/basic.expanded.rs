#[trident_client::test::rstest]
#[trident_client::test::tokio::test(flavor = "multi_thread")]
#[trident_client::test::serial_test::serial]
async fn test_turnstile() -> trident_client::test::anyhow::Result<()> {
    let mut tester = trident_client::test::Tester::with_root("../../");
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
