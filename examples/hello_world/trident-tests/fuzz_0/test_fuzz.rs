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
        // Drift program
        let account = self
            .trident
            .get_account(&pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH"));

        assert!(account.executable());

        let executable_data = self
            .trident
            .get_account(&pubkey!("7dLgmtcTavcguNoynVimF9ZNVb13FvhXVRfj2HyrDGaP"));

        assert_eq!(executable_data.data().len(), 6473114);

        // Jupiter program
        let account = self
            .trident
            .get_account(&pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"));

        assert!(account.executable());

        let executable_data = self
            .trident
            .get_account(&pubkey!("4Ec7ZxZS6Sbdg5UGSLHbAnM7GQHp2eFd4KYWRexAipQT"));

        assert_eq!(executable_data.data().len(), 2892269);

        // ------------------------------------------------------------

        let author = self.fuzz_accounts.author.insert(&mut self.trident, None);

        let hello_world = self.fuzz_accounts.hello_world_account.insert(
            &mut self.trident,
            Some(PdaSeeds {
                seeds: &[b"hello_world_seed"],
                program_id: hello_world::program_id(),
            }),
        );

        self.trident.airdrop(&author, 10 * LAMPORTS_PER_SOL);

        let input = self.trident.random_from_range(0..u8::MAX);

        let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(input))
            .accounts(InitializeFnInstructionAccounts::new(author, hello_world))
            .instruction();

        let forward_time = self.trident.random_from_range(1..100_000_000);
        self.trident.forward_in_time(forward_time);

        let res = self.trident.process_transaction(&[ix], Some("Initialize"));

        self.trident.forward_in_time(forward_time);

        if res.is_success() {
            let hello_world_account = self
                .trident
                .get_account_with_type::<crate::types::StoreHelloWorld>(&hello_world, 8);
            if let Some(hello_world_account) = hello_world_account {
                assert!(hello_world_account.input == input);
                assert!(hello_world_account.timestamp == res.get_transaction_timestamp());
            }
        }
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
