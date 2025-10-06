use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::hello_world::{
    InitializeFnInstruction, InitializeFnInstructionAccounts, InitializeFnInstructionData,
};

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Trident client for interacting with the Solana program
    trident: Trident,
    /// Storage for all account addresses used in fuzz testing
    fuzz_accounts: AccountAddresses,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: AccountAddresses::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Perform any initialization here, this method will be executed
        // at the start of each iteration

        let author = self.fuzz_accounts.author.insert(&mut self.trident, None);

        let hello_world = self.fuzz_accounts.hello_world_account.insert(
            &mut self.trident,
            Some(PdaSeeds {
                seeds: &[b"hello_world_seed"],
                program_id: hello_world::program_id(),
            }),
        );

        self.trident.airdrop(&author, 10 * LAMPORTS_PER_SOL);
        let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(
            self.trident.gen_range(0..u8::MAX),
        ))
        .accounts(InitializeFnInstructionAccounts::new(author, hello_world))
        .instruction();

        let _ = self.trident.execute(&[ix], "Initialize");
    }

    #[flow]
    fn flow1(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
    }

    #[flow]
    fn flow2(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // Perform any cleanup here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
