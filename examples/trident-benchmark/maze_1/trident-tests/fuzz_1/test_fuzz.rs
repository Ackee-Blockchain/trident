//! AFL++ Deferred Fork Server with Persistent Mode
//!
//! This implements AFL's fork server protocol manually with optimizations:
//! - Deferred fork: heavy init happens ONCE before forking
//! - Batch mode: child runs multiple iterations before exiting
//! - Coverage comes ONLY from sBPF traces (no LLVM instrumentation)
//!
//! Build with: cargo build --bin fuzz_1
//! Run with:   cargo afl fuzz -i seeds -o out -c - target/debug/fuzz_1

use std::io::{Read, Write};
use std::os::unix::io::FromRawFd;

use trident_fuzz::fuzzing::processor::InstructionError;
use trident_fuzz::fuzzing::*;

mod fuzz_accounts;
mod types;
use types::*;

// use maze1::entry as maze1_entry;

use trident_fuzz::fuzzing::prelude::{
    reset_coverage_map, traces_to_coverage_map, COVERAGE_MAP_SIZE,
};

// AFL communication constants
const FORKSRV_FD: i32 = 198;

/// Get AFL's shared memory coverage map
fn get_afl_shm() -> Option<&'static mut [u8]> {
    #[cfg(target_family = "unix")]
    {
        let shm_id: i32 = std::env::var("__AFL_SHM_ID").ok()?.parse().ok()?;
        let ptr = unsafe { libc::shmat(shm_id, std::ptr::null(), 0) };
        if std::ptr::eq(ptr, libc::MAP_FAILED) || ptr.is_null() {
            return None;
        }
        Some(unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, COVERAGE_MAP_SIZE) })
    }
    #[cfg(not(target_family = "unix"))]
    {
        None
    }
}

/// Check if we're running under AFL
fn is_afl_mode() -> bool {
    unsafe { libc::fcntl(FORKSRV_FD, libc::F_GETFD) != -1 }
}

/// Read input from stdin
fn read_input() -> Vec<u8> {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap_or(0);
    input
}

/// Deferred Fork Server - forks AFTER expensive initialization
struct DeferredForkServer {
    ctl_fd: std::fs::File,
    st_fd: std::fs::File,
}

impl DeferredForkServer {
    fn new() -> Option<Self> {
        if !is_afl_mode() {
            return None;
        }
        unsafe {
            Some(Self {
                ctl_fd: std::fs::File::from_raw_fd(FORKSRV_FD),
                st_fd: std::fs::File::from_raw_fd(FORKSRV_FD + 1),
            })
        }
    }

    fn handshake(&mut self) -> std::io::Result<()> {
        self.st_fd.write_all(&0u32.to_ne_bytes())?;
        Ok(())
    }

    fn run_loop<F>(&mut self, mut run_one: F) -> !
    where
        F: FnMut(&[u8]),
    {
        loop {
            // Wait for AFL's "go" signal
            let mut buf = [0u8; 4];
            if self.ctl_fd.read_exact(&mut buf).is_err() {
                std::process::exit(0);
            }

            // Fork child
            let pid = unsafe { libc::fork() };

            if pid < 0 {
                std::process::exit(1);
            } else if pid == 0 {
                // === CHILD PROCESS ===
                // Read input and run
                let input = read_input();
                run_one(&input);
                std::process::exit(0);
            } else {
                // === PARENT PROCESS ===
                // Tell AFL the child PID
                let _ = self.st_fd.write_all(&(pid as u32).to_ne_bytes());

                // Wait for child
                let mut status: i32 = 0;
                unsafe { libc::waitpid(pid, &mut status, 0) };

                // Report status to AFL
                let _ = self.st_fd.write_all(&(status as u32).to_ne_bytes());
            }
        }
    }
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

/// Run one fuzzing iteration with the given input
fn run_fuzz_iteration(input: &[u8], coverage_map: Option<&'static mut [u8]>) {
    // Initialize Trident (fresh state for each iteration)
    let mut fuzz_test = FuzzTest::new();

    // Reset coverage map at the start of each iteration
    if let Some(map) = coverage_map.as_ref() {
        let map_mut =
            unsafe { std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE) };
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

    if !init_result.is_success() {
        return;
    }

    // Write Initialize traces to coverage map
    if let Some(map) = coverage_map.as_ref() {
        let map_mut =
            unsafe { std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE) };
        traces_to_coverage_map(init_result.get_execution_traces(), map_mut);
    }

    // =====================================================================
    // Step 2: Process Move instructions from fuzzer input
    // =====================================================================
    let mut offset = 0;
    let mut move_count = 0;
    let mut total_instructions = 0usize;

    while offset + MOVE_CMD_SIZE <= input.len() {
        let direction = input[offset] % 4;
        offset += 1;

        let p0 = read_u64(&input[offset..]);
        let p1 = read_u64(&input[offset + 8..]);
        let p2 = read_u64(&input[offset + 16..]);
        let p3 = read_u64(&input[offset + 24..]);
        let p4 = read_u64(&input[offset + 32..]);
        let p5 = read_u64(&input[offset + 40..]);
        let p6 = read_u64(&input[offset + 48..]);
        let p7 = read_u64(&input[offset + 56..]);
        offset += 64;

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
            _ => return,
        };

        let move_result = fuzz_test.trident.process_transaction(&[move_ix], None);

        if move_result.is_error() {
            if let Err(TransactionError::InstructionError(_x, ref y)) = move_result.get_result() {
                if matches!(y, InstructionError::ProgramFailedToComplete) {
                    eprintln!("ProgramFailedToComplete: {}", move_result.logs());
                    std::process::exit(1); // Signal crash to AFL
                }
            }
            continue;
        }

        // Add traces to coverage map (ONLY source of coverage!)
        if let Some(map) = coverage_map.as_ref() {
            let map_mut = unsafe {
                std::slice::from_raw_parts_mut(map.as_ptr() as *mut u8, COVERAGE_MAP_SIZE)
            };
            let traces = move_result.get_execution_traces();
            total_instructions += traces.iter().map(|t| t.len()).sum::<usize>();
            traces_to_coverage_map(traces, map_mut);
        }

        move_count += 1;
    }

    // Debug output
    if let Some(map) = coverage_map.as_ref() {
        let map_ref = unsafe { std::slice::from_raw_parts(map.as_ptr(), COVERAGE_MAP_SIZE) };
        let edges_hit = map_ref.iter().filter(|&&x| x > 0).count();
        eprintln!(
            "[DEBUG] Processed {} moves, {} instructions, {} edges",
            move_count, total_instructions, edges_hit
        );
    }

    fuzz_test.trident.next_iteration();
}

// Global coverage map pointer (set once, used in each iteration)
static mut COVERAGE_MAP_PTR: *mut u8 = std::ptr::null_mut();

fn main() {
    // Get AFL's shared memory for coverage feedback and store globally
    if let Some(map) = get_afl_shm() {
        unsafe { COVERAGE_MAP_PTR = map.as_mut_ptr() };
    }

    // Try to set up AFL fork server (returns None if not running under AFL)
    if let Some(mut fork_server) = DeferredForkServer::new() {
        eprintln!("[*] Running in AFL deferred fork server mode");

        if let Err(e) = fork_server.handshake() {
            eprintln!("Fork server handshake failed: {}", e);
            std::process::exit(1);
        }

        // Run the fork server loop - each child runs one iteration
        fork_server.run_loop(|input| {
            let coverage_map = unsafe {
                if COVERAGE_MAP_PTR.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts_mut(
                        COVERAGE_MAP_PTR,
                        COVERAGE_MAP_SIZE,
                    ))
                }
            };
            run_fuzz_iteration(input, coverage_map);
        });
    } else {
        // Standalone mode: read from stdin, run once
        eprintln!("[*] Running in standalone mode (no AFL)");
        let coverage_map = unsafe {
            if COVERAGE_MAP_PTR.is_null() {
                None
            } else {
                Some(std::slice::from_raw_parts_mut(
                    COVERAGE_MAP_PTR,
                    COVERAGE_MAP_SIZE,
                ))
            }
        };
        let input = read_input();
        run_fuzz_iteration(&input, coverage_map);
    }
}
