//! AFL++ Persistent Mode with fuzz! macro
//!
//! This uses the afl::fuzz! macro which handles the loop properly.

use trident_fuzz::fuzzing::*;

mod fuzz_accounts;
mod types;
use types::*;

use trident_fuzz::fuzzing::prelude::{
    reset_coverage_map, traces_to_coverage_map, COVERAGE_MAP_SIZE,
};

/// Get AFL's shared memory coverage map
#[cfg(target_family = "unix")]
fn get_afl_shm() -> Option<&'static mut [u8]> {
    let shm_id: i32 = std::env::var("__AFL_SHM_ID").ok()?.parse().ok()?;
    let ptr = unsafe { libc::shmat(shm_id, std::ptr::null(), 0) };
    if ptr == libc::MAP_FAILED as *mut libc::c_void || ptr.is_null() {
        return None;
    }
    Some(unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, COVERAGE_MAP_SIZE) })
}

#[cfg(not(target_family = "unix"))]
fn get_afl_shm() -> Option<&'static mut [u8]> {
    None
}

use crate::fuzz_accounts::AccountAddresses;
// #[macro_use]
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
use afl::fuzz;

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

        let payer = self.trident.random_pubkey();

        self.trident.airdrop(&payer, 10 + LAMPORTS_PER_SOL);
        let empty_account = self.trident.random_keypair();

        let ix = self.trident.create_account(
            &payer,
            &empty_account.pubkey(),
            10 + LAMPORTS_PER_SOL,
            82,
            &types::maze::program_id(),
        );

        let res = self
            .trident
            .process_transaction(&[ix], Some("CreateAccount"));
        println!("CreateAccount: {:?}", res.get_result());
        println!("CreateAccount: {}", res.logs());
        println!("CreateAccount: {:?}", res.get_execution_traces());

        let account = self.trident.get_account(&empty_account.pubkey());
        println!("Account: {:?}", account);

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

        let res = self
            .trident
            .process_transaction(&[init], Some("Initialize"));

        let account = self.trident.get_account(&state);
        println!("Account: {:?}", account);

        println!("Initialize: {:?}", res.get_result());
        println!("Initialize: {}", res.logs());
        // println!("Initialize: {:?}", res.get_execution_traces());
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
    // =========================================================================
    // SETUP (done ONCE before the fuzz loop)
    // =========================================================================

    // Get AFL's shared memory for coverage feedback
    // let coverage_map: &'static mut [u8] =
    //     get_afl_shm().expect("Must run under AFL (needs __AFL_SHM_ID). Use: cargo afl fuzz ...");

    // Initialize Trident ONCE (loads your Solana program)
    let mut trident = Trident::default();

    // =========================================================================
    // THE AFL LOOP - using afl::fuzz! macro
    // =========================================================================
    //
    // The fuzz! macro:
    //   1. Handles the AFL persistent mode protocol
    //   2. Reads input from AFL for each iteration
    //   3. Calls your closure with the input bytes
    //   4. Loops efficiently without process restart
    //
    afl::fuzz!(|input: &[u8]| {
        // -----------------------------------------------------------------
        // Step 1: Reset coverage for this iteration
        // -----------------------------------------------------------------
        // reset_coverage_map(coverage_map);

        // -----------------------------------------------------------------
        // Step 2: Convert input → instruction (your fuzzing logic)
        // -----------------------------------------------------------------
        // Here you parse the input bytes to create meaningful instructions.
        // For this simple example, we just call Initialize.
        //
        // In a real fuzzer, you'd do something like:
        //   let instruction = match input.get(0) {
        //       Some(0) => build_initialize_ix(&input[1..]),
        //       Some(1) => build_transfer_ix(&input[1..]),
        //       _ => return,  // Skip invalid input
        //   };

        let instruction =
            maze::InitializeInstruction::data(maze::InitializeInstructionData::new()).instruction();

        // -----------------------------------------------------------------
        // Step 3: Execute transaction and get TRACES
        // -----------------------------------------------------------------
        let result = trident.process_transaction(&[instruction], None);
        let traces = result.get_execution_traces();

        // -----------------------------------------------------------------
        // Step 4: Convert traces → AFL coverage (THE KEY CONNECTION!)
        // -----------------------------------------------------------------
        // This writes the execution path information to AFL's shared memory.
        // AFL reads this to decide if the input found new code paths.
        // traces_to_coverage_map(traces, coverage_map);
    });
}
