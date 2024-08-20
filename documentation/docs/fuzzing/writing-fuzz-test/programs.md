# Include Programs

With Trident you can and need to specify which programs will be included in the Testing Environment.

!!! important

    As Trident supports Cross-Program Invocations you need to specify all of the programs that the Solana Environment should start with, same as for the ProgramTest. Go to the `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/test_fuzz.rs` and customize the Solana Environment as shown below.

```rust
// use statements ...
use ... as FuzzInstruction_hello_world;

use hello_world::entry as entry_hello_world;
use hello_world::ID as PROGRAM_ID_HELLO_WORLD;
const PROGRAM_NAME_HELLO_WORLD: &str = "hello_world";

// TODO specify this type for Instructions generation.
pub type FuzzInstruction = FuzzInstruction_hello_world;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Include the program and its entrypoint in the Testing Environment
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_HELLO_WORLD,
                &PROGRAM_ID_HELLO_WORLD,
                processor!(convert_entry!(entry_hello_world))
            );

            let mut client =
                ProgramTestClientBlocking::new(
                    &[fuzzing_program1]
                ).unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(
                PROGRAM_ID_HELLO_WORLD,
                &mut client
            );
        });
    }
}
```

## SBF

!!! important

    If you want to include Solana Programs from some of the clusters (for example Mainnet). Follow these steps:

     1. Dump the program from the desired cluster (`solana program dump -u m <PROGRAM_ID> <PROGRAM_NAME>.so`).
     2. Create folder `trident-genesis` in the root of the Anchor Workspace (i.e. next to the Anchor.toml)
     3. Copy the dumped file to the `trident-genesis`.
     4. Include the program as described below.

!!! warning

    The name of the dumped .so file has to be the same as the program name you are including (i.e. you are including metaplex-token-metadata, then the file name has to be metaplex-token-metadata.so)

```rust
// use statements ...
use ... as FuzzInstruction_hello_world;

use hello_world::entry as entry_hello_world;
use hello_world::ID as PROGRAM_ID_HELLO_WORLD;
const PROGRAM_NAME_HELLO_WORLD: &str = "hello_world";

// TODO specify this type for Instructions generation.
pub type FuzzInstruction = FuzzInstruction_hello_world;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Include the program and its entrypoint in the Testing Environment
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_CPI_METAPLEX_7,
                &PROGRAM_ID_CPI_METAPLEX_7,
                processor!(convert_entry!(entry_cpi_metaplex_7))
            );


            let metaplex = FuzzingProgram::new(
                "metaplex-token-metadata",
                &mpl_token_metadata::ID,
                None
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1,metaplex])
                    .unwrap();

            // fill Program ID of program the fuzzer will call.
            let _ = fuzz_data.run_with_runtime(
                PROGRAM_NAME_PROGRAM_1,
                &mut client
            );
        });
    }
}

```

## Native

!!! important

    If you want to include additional Solana programs within the Testing Environment for example for CPI purposes you can include them with the same approach as including only one program. More details in the snippet below.

```rust
// use statements ...
use ... as FuzzInstruction_program1;
use ... as FuzzInstruction_program2;


use program1::entry as entry_program1;
use program1::ID as PROGRAM_ID_PROGRAM_1;
const PROGRAM_NAME_PROGRAM_1: &str = "program1";

use program2::entry as entry_program2;
use program2::ID as PROGRAM_ID_PROGRAM_2;
const PROGRAM_NAME_PROGRAM_2: &str = "program2";


// TODO specify this type for Instructions generation.
// These instructions are going to be called by the Fuzzer
pub type FuzzInstruction = FuzzInstruction_program1;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Include the program and its entrypoint in the Testing Environment
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_PROGRAM_1,
                &PROGRAM_ID_PROGRAM_1,
                processor!(convert_entry!(entry_program1))
            );

            let fuzzing_program2 = FuzzingProgram::new(
                PROGRAM_NAME_PROGRAM_2,
                &PROGRAM_ID_PROGRAM_2,
                processor!(convert_entry!(entry_program2))
            );


            let mut client =
                ProgramTestClientBlocking::new(
                    &[fuzzing_program1,fuzzing_program2]
                ).unwrap();

            // fill Program ID of program the fuzzer will call.
            let _ = fuzz_data.run_with_runtime(
                PROGRAM_NAME_PROGRAM_1,
                &mut client
            );
        });
    }
}
```


## Example

!!! tip

    For a practical example of CPI, please refer to the [Examples](../extra/examples.md) section.
