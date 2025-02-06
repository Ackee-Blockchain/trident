use serde::{Deserialize, Serialize};
use shared_memory::{Shmem, ShmemConf};
use std::sync::{Arc, Mutex};

use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;
use hello_world::entry as entry_hello_world;
struct InstructionsSequence;
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(InitializeFn);
    middle_sequence!();
    post_sequence!();
}
fn main() {
    let program_hello_world = ProgramEntrypoint::new(
        pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
        None,
        processor!(entry_hello_world),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_hello_world], &config);

    let struct_size = size_of::<FuzzingMetrics>();
    let struct_align = align_of::<FuzzingMetrics>();

    let shmem = ShmemConf::new()
        .size(1024 * 1024) // 1MB should be plenty for metrics
        .os_id("fuzzing_metrics-abcdf")
        .create()
        .expect("Failed to create shared memory");

    let ptr = shmem.as_ptr();
    let aligned_ptr = unsafe {
        let offset = ptr.align_offset(struct_align);
        ptr.add(offset) as *mut FuzzingMetrics
    };

    let metrics = FuzzingMetrics::new();

    unsafe {
        std::ptr::write(aligned_ptr, metrics);
    }

    let mut signals = Signals::new([SIGINT]).unwrap();

    std::thread::spawn(move || {
        if let Some(_) = signals.forever().next() {
            let shmem = ShmemConf::new()
                .os_id("fuzzing_metrics-abcdf")
                .open()
                .unwrap();
            let ptr = shmem.as_ptr();

            let struct_align = align_of::<FuzzingMetrics>();
            let aligned_ptr = unsafe {
                let offset = ptr.align_offset(struct_align);
                ptr.add(offset) as *const FuzzingMetrics // Note: const pointer
            };

            unsafe {
                let data = &*aligned_ptr; // Get read-only reference
                let mut file = std::fs::File::create("signal_triggered.txt").unwrap();
                writeln!(file, "SIGINT was triggered!\nFinal metrics:\n{:#?}", data).unwrap();
            }

            std::process::exit(0);
        }
    });

    fuzz_afl(true, |fuzz_data| {
        unsafe {
            // Get mutable reference to modify fields
            let data = &mut *aligned_ptr;
            data.increase_invoked("test".to_string());
        }

        let mut fuzz_data: FuzzData<FuzzInstruction, _> = {
            use arbitrary::Unstructured;

            let mut buf = Unstructured::new(fuzz_data);
            if let Ok(fuzz_data) = build_ix_fuzz_data(InstructionsSequence {}, &mut buf) {
                fuzz_data
            } else {
                return;
            }
        };
        let _ = fuzz_data.run_with_runtime(&mut client, &config);
    });
    // fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: InstructionsSequence, client: TridentSVM, config: TridentConfig|);
}
