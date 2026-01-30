//! AFL++ Persistent Mode with fuzz! macro
//!
//! This uses the afl::fuzz! macro which handles the loop properly.

use trident_fuzz::fuzzing::prelude::traces_to_coverage_map_dataflow;
use trident_fuzz::fuzzing::prelude::traces_to_coverage_map_enhanced;
use trident_fuzz::fuzzing::processor::InstructionError;
use trident_fuzz::fuzzing::*;

mod fuzz_accounts;
mod types;
use types::*;

// use maze1::entry as maze1_entry;

use trident_fuzz::fuzzing::prelude::{
    reset_coverage_map, traces_to_coverage_map, COVERAGE_MAP_SIZE,
};

/// Get AFL's shared memory coverage map
#[cfg(target_family = "unix")]
fn get_afl_shm() -> Option<&'static mut [u8]> {
    let shm_id: i32 = std::env::var("__AFL_SHM_ID").ok()?.parse().ok()?;
    let ptr = unsafe { libc::shmat(shm_id, std::ptr::null(), 0) };
    if std::ptr::eq(ptr, libc::MAP_FAILED) || ptr.is_null() {
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

/// Size of one move command in bytes:
/// - 1 byte: direction (0=North, 1=South, 2=East, 3=West)
/// - 64 bytes: 8 x u64 parameters (p0-p7)
const MOVE_CMD_SIZE: usize = 1 + 8 * 8; // 65 bytes

/// Helper to parse a u64 from a byte slice
fn read_u64(data: &[u8]) -> u64 {
    if data.len() >= 8 {
        u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ])
    } else {
        0
    }
}

fn main() {
    // Get AFL's shared memory for coverage feedback
    let coverage_map = get_afl_shm();

    afl::fuzz!(|input: &[u8]| {
        // Initialize Trident INSIDE the loop (required for macOS fork compatibility)
        let mut fuzz_test = FuzzTest::new();

        // let program = TridentEntrypoint::new(maze1::ID, None, processor!(maze1_entry));

        // fuzz_test.trident.deploy_entrypoint(program);

        // Reset coverage map at the start of each iteration
        if let Some(map) = coverage_map.as_ref() {
            let map_mut = unsafe {
                std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE)
            };
            reset_coverage_map(map_mut);
        }

        // Setup accounts
        let state_author = fuzz_test
            .fuzz_accounts
            .state_author
            .insert(&mut fuzz_test.trident, None);

        fuzz_test
            .trident
            .airdrop(&state_author, 10 + LAMPORTS_PER_SOL);

        let state = fuzz_test.fuzz_accounts.state.insert(
            &mut fuzz_test.trident,
            Some(PdaSeeds {
                seeds: &[b"state"],
                program_id: types::maze::program_id(),
            }),
        );

        // =====================================================================
        // Step 1: Initialize the maze state
        // =====================================================================
        let init = maze::InitializeInstruction::data(InitializeInstructionData::new())
            .accounts(InitializeInstructionAccounts::new(state_author, state))
            .instruction();

        let init_result = fuzz_test.trident.process_transaction(&[init.clone()], None);

        println!("Executed Initialize - 1");
        assert!(init_result.is_success());

        // Write Initialize traces to coverage map
        if let Some(map) = coverage_map.as_ref() {
            let map_mut = unsafe {
                std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE)
            };
            traces_to_coverage_map_dataflow(init_result.get_execution_traces(), map_mut);
        }

        // =====================================================================
        // Step 2: Process Move instructions from fuzzer input
        // =====================================================================
        // Input format: [direction(1), p0(8), p1(8), p2(8), p3(8), p4(8), p5(8), p6(8), p7(8)]
        //               = 65 bytes per move command

        let mut offset = 0;
        let mut move_count = 0;
        let mut total_instructions = 0usize;

        while offset + MOVE_CMD_SIZE <= input.len() {
            // Parse direction (0-3)
            let direction = input[offset] % 4;
            offset += 1;

            // Parse p0-p7 parameters
            let p0 = read_u64(&input[offset..]);
            let p1 = read_u64(&input[offset + 8..]);
            let p2 = read_u64(&input[offset + 16..]);
            let p3 = read_u64(&input[offset + 24..]);
            let p4 = read_u64(&input[offset + 32..]);
            let p5 = read_u64(&input[offset + 40..]);
            let p6 = read_u64(&input[offset + 48..]);
            let p7 = read_u64(&input[offset + 56..]);
            offset += 64;

            // Build the Move instruction based on direction
            let dir_name = match direction {
                0 => "North",
                1 => "South",
                2 => "East",
                _ => "West",
            };
            eprintln!(
                "[Move #{}] {} | p0={} p1={} p2={} p3={} p4={} p5={} p6={} p7={}",
                move_count + 1,
                dir_name,
                p0,
                p1,
                p2,
                p3,
                p4,
                p5,
                p6,
                p7
            );

            let move_ix = match direction {
                0 => maze::MoveNorthInstruction::data(MoveNorthInstructionData::new(
                    p0, p1, p2, p3, p4, p5, p6, p7,
                ))
                .accounts(MoveNorthInstructionAccounts::new(state))
                .instruction(),
                1 => maze::MoveSouthInstruction::data(MoveSouthInstructionData::new(
                    p0, p1, p2, p3, p4, p5, p6, p7,
                ))
                .accounts(MoveSouthInstructionAccounts::new(state))
                .instruction(),
                2 => maze::MoveEastInstruction::data(MoveEastInstructionData::new(
                    p0, p1, p2, p3, p4, p5, p6, p7,
                ))
                .accounts(MoveEastInstructionAccounts::new(state))
                .instruction(),
                3 => maze::MoveWestInstruction::data(MoveWestInstructionData::new(
                    p0, p1, p2, p3, p4, p5, p6, p7,
                ))
                .accounts(MoveWestInstructionAccounts::new(state))
                .instruction(),
                _ => {
                    eprintln!("Invalid direction: {}", direction);
                    return;
                }
            };

            // Execute the move transaction
            let move_result = fuzz_test.trident.process_transaction(&[move_ix], None);

            if move_result.is_error() {
                let error = move_result.get_result();
                match error {
                    Ok(_) => continue,
                    Err(e) => match e {
                        TransactionError::InstructionError(x, y) => match y {
                            InstructionError::ProgramFailedToComplete => {
                                println!("ProgramFailedToComplete: {}", move_result.logs());
                                assert!(false)
                            }
                            _ => continue,
                        },
                        _ => continue,
                    },
                }
            }

            // Add this transaction's traces to coverage map
            if let Some(map) = coverage_map.as_ref() {
                let map_mut = unsafe {
                    std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE)
                };
                let traces = move_result.get_execution_traces();
                total_instructions += traces.iter().map(|t| t.len()).sum::<usize>();
                traces_to_coverage_map_dataflow(traces, map_mut);
            }

            move_count += 1;
        }

        // =====================================================================
        // Debug output
        // =====================================================================
        if let Some(map) = coverage_map.as_ref() {
            let map_ref = unsafe { std::slice::from_raw_parts(map.as_ptr(), COVERAGE_MAP_SIZE) };
            let edges_hit = map_ref.iter().filter(|&&x| x > 0).count();
            eprintln!(
                "[DEBUG] Processed {} moves, {} total instructions, {} edges hit",
                move_count, total_instructions, edges_hit
            );
        } else {
            eprintln!(
                "[DEBUG] Processed {} moves (no AFL coverage map)",
                move_count
            );
        }

        fuzz_test.trident.next_iteration();
    });
}
