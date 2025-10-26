use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::cpi::{
    InitializeCallerInstruction, InitializeCallerInstructionAccounts,
    InitializeCallerInstructionData,
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

        let signer = self.fuzz_accounts.signer.insert(&mut self.trident, None);

        let ix = InitializeCallerInstruction::data(InitializeCallerInstructionData::new(
            self.trident.random_from_range(0..u16::MAX),
        ))
        .accounts(InitializeCallerInstructionAccounts::new(signer))
        .instruction();

        let _ = self.trident.process_transaction(&[ix], "initialize_caller");
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
