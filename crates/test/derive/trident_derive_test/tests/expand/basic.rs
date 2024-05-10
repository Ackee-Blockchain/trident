#[trident_test::trident_test]
async fn test_turnstile() {
    init_client().await?;
    let mut turnstile = Turnstile {
        locked: get_state_client().await?.locked
    };
    turnstile.coin().await?;
    turnstile.push_unlocked().await?;
    turnstile.push_locked().await?;
}
