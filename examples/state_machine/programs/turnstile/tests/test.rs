use anyhow::Error;
use fehler::throws;
use trdelnik::*;
use turnstile::State;

static ROOT: &str = "../../";

#[throws]
async fn before() -> LocalnetHandle {
    let commander = Commander::with_root(ROOT);
    commander.build_programs().await?;
    let localnet_handle = commander.start_localnet().await?;
    init_client().await?;
    localnet_handle
}

#[throws]
async fn after(localnet_handle: LocalnetHandle) {
    localnet_handle.stop().await?;
}

#[throws]
#[tokio::test(flavor = "multi_thread")]
async fn test_turnstile() {
    let localnet_handle = before().await?;

    let mut turnstile = Turnstile {
        locked: true,
        reader: Reader::with_root(ROOT),
    };
    turnstile.locked = turnstile.get_state_client().await?.locked;
    
    println!("coin");
    turnstile.coin().await?;
    println!("push_unlocked");
    turnstile.push_unlocked().await?;
    println!("push_locked");
    turnstile.push_locked().await?;

    after(localnet_handle).await?;
}

// fn run_test<T>(test: T) -> ()
//     where T: FnOnce() -> () + panic::UnwindSafe
// {
//     setup();

//     let result = panic::catch_unwind(|| {
//         test()
//     });

//     teardown();

//     assert!(result.is_ok())
// }



struct Turnstile {
    locked: bool,
    reader: Reader,
}

impl Turnstile {
    #[throws]
    async fn coin(&mut self) {
        // inserting a coin is just calling coin
        self.coin_client().await?;

        // update
        self.locked = false;

        // get current state
        let locked_after = self.get_state_client().await?.locked;

        // ensure that coin insert unlocks turnstile
        assert!(!locked_after);
    }

    #[throws]
    async fn push_locked(&mut self) {
        // get before state
        let locked_before = self.get_state_client().await?.locked;

        // pushing is just calling push
        self.push_client().await?;

        // get current state
        let state = self.get_state_client().await?;
        let (locked_after, res) = (state.locked, state.res);

        // update
        self.locked = true;

        // ensure that coin insert unlocks turnstile
        assert!(!res && locked_after && locked_before);
    }

    #[throws]
    async fn push_unlocked(&mut self) {
        // get before state
        let locked_before = self.get_state_client().await?.locked;

        // pushing is just calling push
        self.push_client().await?;

        // get current state
        let state = self.get_state_client().await?;
        let (locked_after, res) = (state.locked, state.res);

        // update
        self.locked = true;

        // ensure that coin insert unlocks turnstile
        assert!(res && locked_after && !locked_before);
    }

    #[throws]
    async fn get_state_client(&self) -> State {
        let reader = &self.reader;
        let account_pubkey = reader.pubkey("state").await?;
        let client = Client::new(reader.keypair("id").await?);
        client.account_data(account_pubkey).await?
    }

    #[throws]
    async fn coin_client(&self) {
        let reader = &self.reader;
        let payer = reader.keypair("id").await?;
        Client::new(payer).send_instruction(
            reader.pubkey("program").await?,
            turnstile::instruction::Coin,
            turnstile::accounts::UpdateState { 
                state: reader.pubkey("state").await?
            },
            None,
        ).await?;
    }

    #[throws]
    async fn push_client(&self) {
        let reader = &self.reader;
        let payer = reader.keypair("id").await?;
        Client::new(payer).send_instruction(
            reader.pubkey("program").await?,
            turnstile::instruction::Push,
            turnstile::accounts::UpdateState { 
                state: reader.pubkey("state").await?
            },
            None,
        ).await?;
    }
}

#[throws]
async fn init_client() {
    let reader = Reader::with_root(ROOT);

    let payer = reader.keypair("id").await?;
    let payer_pubkey = payer.pubkey();

    let program_keypair = reader.keypair("program").await?;
    let program_pubkey = program_keypair.pubkey();
    let program_data = reader.program_data("turnstile").await?;

    let client = Client::new(payer);

    println!("AIRDROP");
    client.airdrop(payer_pubkey, 5_000_000_000).await?;

    println!("DEPLOY");
    client.deploy(program_keypair, program_data).await?;

    println!("INIT STATE");
    let state = reader.keypair("state").await?;
    client.send_instruction(
        program_pubkey,
        turnstile::instruction::Initialize,
        turnstile::accounts::Initialize { 
            state: state.pubkey(),
            user: payer_pubkey,
            system_program: System::id()
        },
        Some(state),
    ).await?;

    println!("Initialized");
}

