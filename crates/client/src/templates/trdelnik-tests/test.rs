use fehler::throws;
use trdelnik_client::{anyhow::Result, *};

// @todo: create and deploy your fixture
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    // @todo: here you can call your <program>::initialize instruction
    fixture.deploy().await?;
    fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
    // @todo: add your happy path test scenario and the other test cases
    let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;
    assert_eq!(fixture.program, default_fixture.program);
}

// @todo: design and implement all the logic you need for your fixture(s)
struct Fixture {
    client: Client,
    program: Keypair,
    state: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            program: anchor_keypair("###PROGRAM_NAME###").unwrap(),
            state: keypair(42),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&self.program.clone(), "###PROGRAM_NAME###")
            .await?;
    }
}
