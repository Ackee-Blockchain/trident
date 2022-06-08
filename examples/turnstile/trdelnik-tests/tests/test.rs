use fehler::throws;
use program_client::turnstile_instruction;
use std::mem;
use trdelnik_client::{anyhow::Result, *};
use turnstile;

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    // create a test fixture
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(1),
        state: keypair(42),
    };
    // deploy a tested program
    fixture.deploy().await?;

    // init instruction call
    turnstile_instruction::initialize(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.client.payer().pubkey(),
        System::id(),
        Some(fixture.state.clone()),
    )
    .await?;

    fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    // coin instruction call
    turnstile_instruction::coin(
        &fixture.client,
        "dummy_string".to_owned(),
        fixture.state.pubkey(),
        None,
    )
    .await?;
    // push instruction call
    turnstile_instruction::push(&fixture.client, fixture.state.pubkey(), None).await?;

    // check the test result
    let state = fixture.get_state().await?;

    // after pushing the turnstile should be locked
    assert_eq!(state.locked, true);
    // the last push was successfull
    assert_eq!(state.res, true);
}

#[trdelnik_test]
async fn test_unhappy_path(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    // pushing without prior coin insertion
    turnstile_instruction::push(&fixture.client, fixture.state.pubkey(), None).await?;

    // check the test result
    let state = fixture.get_state().await?;

    // after pushing the turnstile should be locked
    assert_eq!(state.locked, true);
    // the last push was successfull
    assert_eq!(state.res, false);
}

struct Fixture {
    client: Client,
    program: Keypair,
    state: Keypair,
}

impl Fixture {
    #[throws]
    async fn deploy(&mut self) {
        let reader = Reader::new();
        let mut program_data = reader.program_data("turnstile").await?;

        let amount = 5_000_000_000;

        self.client
            .airdrop(self.client.payer().pubkey(), amount)
            .await?;

        assert_eq!(
            self.client
                .get_balance(self.client.payer().pubkey())
                .await?,
            amount
        );

        self.client
            .deploy(self.program.clone(), mem::take(&mut program_data))
            .await?;
    }

    #[throws]
    async fn get_state(&self) -> turnstile::State {
        self.client.account_data(self.state.pubkey()).await?
    }
}
