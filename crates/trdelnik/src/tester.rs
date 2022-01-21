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
        // @TODO: the `generate_program_client_lib_rs` method has to be run through
        // Trdelnik CLI (as a part of `trdelnik test`?) to generate the `lib.rs`
        // code before compiler tries to compile tests.
        // Note: It can't be run in `build.rs` otherwise it causes cargo deadlocks.
        commander.generate_program_client_lib_rs().await?;
        commander.start_localnet().await?
    }

    #[throws]
    pub async fn after(&self, localnet_handle: LocalnetHandle) {
        println!("____ AFTER TEST ____");
        localnet_handle.stop().await?;
        println!("_____________________");
    }
}
