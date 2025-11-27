use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::maze::InitializeInstructionAccounts;
use crate::types::maze::InitializeInstructionData;
use crate::types::maze::MoveEastInstructionAccounts;
use crate::types::maze::MoveEastInstructionData;
use crate::types::maze::MoveNorthInstructionAccounts;
use crate::types::maze::MoveNorthInstructionData;
use crate::types::maze::MoveSouthInstructionAccounts;
use crate::types::maze::MoveSouthInstructionData;
use crate::types::maze::MoveWestInstructionAccounts;
use crate::types::maze::MoveWestInstructionData;

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
        // perform any initialization here, this method will be executed
        // at start of each iteration

        let state_author = self
            .fuzz_accounts
            .state_author
            .insert(&mut self.trident, None);

        self.trident.airdrop(&state_author, 10 + LAMPORTS_PER_SOL);

        let state = self.fuzz_accounts.state.insert(
            &mut self.trident,
            Some(PdaSeeds {
                seeds: &[b"state"],
                program_id: types::maze::program_id(),
            }),
        );

        let init = maze::InitializeInstruction::data(InitializeInstructionData::new())
            .accounts(InitializeInstructionAccounts::new(state_author, state))
            .instruction();

        let _ = self
            .trident
            .process_transaction(&[init], Some("Initialize"));
    }

    #[flow]
    fn flow1(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self
            .fuzz_accounts
            .state
            .get(&mut self.trident)
            .expect("Storage empty");

        let move_north = maze::MoveNorthInstruction::data(MoveNorthInstructionData::new(
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
        ))
        .accounts(MoveNorthInstructionAccounts::new(state))
        .instruction();

        let _ = self
            .trident
            .process_transaction(&[move_north], Some("MoveNorth"));
    }

    #[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self
            .fuzz_accounts
            .state
            .get(&mut self.trident)
            .expect("Storage empty");

        let move_north = maze::MoveSouthInstruction::data(MoveSouthInstructionData::new(
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
        ))
        .accounts(MoveSouthInstructionAccounts::new(state))
        .instruction();

        let _ = self
            .trident
            .process_transaction(&[move_north], Some("MoveSouth"));
    }

    #[flow]
    fn flow3(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self
            .fuzz_accounts
            .state
            .get(&mut self.trident)
            .expect("Storage empty");

        let move_north = maze::MoveEastInstruction::data(MoveEastInstructionData::new(
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
        ))
        .accounts(MoveEastInstructionAccounts::new(state))
        .instruction();

        let _ = self
            .trident
            .process_transaction(&[move_north], Some("MoveEast"));
    }

    #[flow]
    fn flow4(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self
            .fuzz_accounts
            .state
            .get(&mut self.trident)
            .expect("Storage empty");

        let move_north = maze::MoveWestInstruction::data(MoveWestInstructionData::new(
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
            self.trident.random_from_range(0..u64::MAX),
        ))
        .accounts(MoveWestInstructionAccounts::new(state))
        .instruction();

        let _ = self
            .trident
            .process_transaction(&[move_north], Some("MoveWest"));
    }

    #[end]
    fn end(&mut self) {
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
