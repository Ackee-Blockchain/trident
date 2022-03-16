use fehler::throws;
use program_client::turnstile_instruction;
use std::mem;
use trdelnik_client::*;

#[trdelnik_test]
async fn test_happy_path() {
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
    // coint instruction call
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

struct Fixture {
    client: Client,
    program: Keypair,
    state: Keypair,
    mint: Keypair,
    treasury: Keypair,
    user: Keypair,
    token_program: Pubkey,
    system_program: Pubkey,
}

impl Fixture {
    #[throws]
    async fn deploy(&mut self) {
        let reader = Reader::new();
        let mut program_data = reader.program_data("turnstile").await?;

        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy(self.program.clone(), mem::take(&mut program_data))
            .await?;
    }

    #[throws]
    async fn get_state(&self) -> turnstile::State {
        self.client.account_data(self.state.pubkey()).await?
    }
}
