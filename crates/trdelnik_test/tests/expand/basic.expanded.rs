#[trdelnik::tokio::test(flavor = "multi_thread")]
#[trdelnik::serial_test::serial]
async fn test_turnstile() -> trdelnik::anyhow::Result<()> {
    let mut tester = trdelnik::Tester::with_root("../../");
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
        Ok::<(), trdelnik::anyhow::Error>(())
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
