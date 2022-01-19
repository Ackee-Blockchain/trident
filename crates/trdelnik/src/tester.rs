use fehler::throws;
use crate::{Commander, LocalnetHandle, commander::Error};

#[derive(Default)]
pub struct Tester;

impl Tester {
    pub fn new() -> Self {
        Self::default()
    }

    #[throws]
    pub async fn before(&self) -> LocalnetHandle {
        println!("_____________________");
        println!("____ BEFORE TEST ____");
        let commander = Commander::new();
        commander.build_programs().await?;
        commander.generate_idls().await?;
        panic!("idls generated, ending, @TODO remove me");
        commander.start_localnet().await?
    }

    #[throws]
    pub async fn after(&self, localnet_handle: LocalnetHandle) {
        println!("____ AFTER TEST ____");
        localnet_handle.stop().await?;
        println!("_____________________");
    }
}
