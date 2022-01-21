use trdelnik::*;
use fehler::throws;
use program_client::turnstile_instruction;

#[trdelnik_test]
async fn test_turnstile() {
    let reader = Reader::new();
    let mut turnstile = Turnstile {
        client: Client::new(reader.keypair("id").await?),
        state: reader.keypair("state").await?,
        program: reader.keypair("program").await?,
        program_data: reader.program_data("turnstile").await?,
        locked: bool::default(),
    };
    println!("Initialize");
    turnstile.initialize().await?;
    println!("coin");
    turnstile.coin().await?;
    println!("push_unlocked");
    turnstile.push_unlocked().await?;
    println!("push_locked");
    turnstile.push_locked().await?;
}

struct Turnstile {
    client: Client,
    state: Keypair,
    program: Keypair,
    program_data: Vec<u8>,
    locked: bool,
}

impl Turnstile {
    #[throws]
    async fn get_state(&self) -> turnstile::State {
        self.client.account_data(self.state.pubkey()).await?
    }

    #[throws]
    async fn initialize(&mut self) {
        println!("AIRDROP");
        self.client.airdrop(self.client.payer_pubkey(), 5_000_000_000).await?;

        println!("DEPLOY");
        self.client.deploy(self.program.clone(), self.program_data).await?;

        println!("INIT STATE");
        turnstile_instruction::initialize(
            &self.client, 
            self.state.pubkey(), 
            self.client.payer_pubkey(), 
            System::id(), 
            Some(self.state.clone()),
        ).await?;

        self.locked = self.get_state().await?.locked;
    }

    #[throws]
    async fn coin(&mut self) {
        // inserting a coin is just calling coin
        turnstile_instruction::coin(
            &self.client, 
            "something".to_owned(), 
            self.state.pubkey(), 
            None,
        ).await?;

        // update
        self.locked = false;

        // get current state
        let locked_after = self.get_state().await?.locked;

        // ensure that coin insert unlocks turnstile
        assert!(!locked_after);
    }

    #[throws]
    async fn push_locked(&mut self) {
        // get before state
        let locked_before = self.get_state().await?.locked;

        // pushing is just calling push
        turnstile_instruction::push(
            &self.client, 
            self.state.pubkey(), 
            None
        ).await?;

        // get current state
        let state = self.get_state().await?;
        let (locked_after, res) = (state.locked, state.res);

        // update
        self.locked = true;

        // ensure that coin insert unlocks turnstile
        assert!(!res && locked_after && locked_before);
    }

    #[throws]
    async fn push_unlocked(&mut self) {
        // get before state
        let locked_before = self.get_state().await?.locked;

        // pushing is just calling push
        turnstile_instruction::push(
            &self.client, 
            self.state.pubkey(), 
            None
        ).await?;

        // get current state
        let state = self.get_state().await?;
        let (locked_after, res) = (state.locked, state.res);

        // update
        self.locked = true;

        // ensure that coin insert unlocks turnstile
        assert!(res && locked_after && !locked_before);
    }
}
