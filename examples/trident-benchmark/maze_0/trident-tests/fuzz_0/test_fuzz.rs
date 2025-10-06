use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use maze0::entry as maze0;

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
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        let mut trident = Trident::default();

        // Deploy through the entrypoint
        let program = TridentEntrypoint::new(maze0::ID, None, processor!(maze0));
        trident.deploy_entrypoint(program);

        Self {
            trident,
            fuzz_accounts: FuzzAccounts::default(),
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
                program_id: maze0::ID,
            }),
        );

        let init = maze::InitializeInstruction::data(InitializeInstructionData::new())
            .accounts(InitializeInstructionAccounts::new(state_author, state))
            .instruction();

        let _ = self.trident.execute(&[init], "Initialize");
    }

    #[flow]
    fn flow1(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self.fuzz_accounts.state.get(&mut self.trident);

        let move_north = maze::MoveNorthInstruction::data(MoveNorthInstructionData::new(
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
        ))
        .accounts(MoveNorthInstructionAccounts::new(state))
        .instruction();

        let _ = self.trident.execute(&[move_north], "MoveNorth");
    }

    #[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self.fuzz_accounts.state.get(&mut self.trident);

        let move_north = maze::MoveSouthInstruction::data(MoveSouthInstructionData::new(
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
        ))
        .accounts(MoveSouthInstructionAccounts::new(state))
        .instruction();

        let _ = self.trident.execute(&[move_north], "MoveSouth");
    }

    #[flow]
    fn flow3(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self.fuzz_accounts.state.get(&mut self.trident);

        let move_north = maze::MoveEastInstruction::data(MoveEastInstructionData::new(
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
        ))
        .accounts(MoveEastInstructionAccounts::new(state))
        .instruction();

        let _ = self.trident.execute(&[move_north], "MoveEast");
    }

    #[flow]
    fn flow4(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows

        let state = self.fuzz_accounts.state.get(&mut self.trident);

        let move_north = maze::MoveWestInstruction::data(MoveWestInstructionData::new(
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
            self.trident.gen_range(0..u64::MAX),
        ))
        .accounts(MoveWestInstructionAccounts::new(state))
        .instruction();

        let _ = self.trident.execute(&[move_north], "MoveWest");
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
