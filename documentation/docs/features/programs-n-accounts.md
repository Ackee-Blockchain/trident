# Programs and Accounts

Trident allows for usage of programs and accounts from the desired cluster (Mainnet, Devnet, etc.).

### Include Mainnet Programs

In case you want to include programs from Mainnet, you can do so by specifying the address and path to the program in the `Trident.toml`.

```toml
[[fuzz.programs]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
program = "metaplex-program/metaplex-token-metadata.so"
```

!!! tip

    If you want to dump a program from Mainnet, use

    ```bash
    # -u m specifies to dump from mainnet
    solana program dump -u m <PROGRAM_ID> <PROGRAM_NAME>.so
    ```


### Include Mainnet Accounts

In case you want to include accounts from Mainnet, you can do so by specifying the address and path to the account in the `Trident.toml`.

```toml
[[fuzz.accounts]]
address = "6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE"
filename = "tests/accounts/core_bridge_mainnet/guardian_set_5_mock.json"
```

!!! tip

    If you want to obtain an account with `base64` data format, use

    ```bash
    # -u m specifies to dump from mainnet
    solana account -u m <ADDRESS> --output json
    ```


### Include Programs Through the Entrypoint

There is another way to include programs in the Fuzz Test Environment. Including programs through the entrypoint is better for fuzzing, as the program is built together with the fuzz test, so that it will contain instrumentation which helps the fuzzer to better cover all possible program branches. However, the source code is required to do so.


You can include additional programs using the program entrypoint, as shown in the example below:

```rust
// test_fuzz.rs

use callee::entry as entry_callee;
use caller::entry as entry_caller;

// ...

fn main() {
    // Program 1
    let program_callee = ProgramEntrypoint::new(
        pubkey!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL"),
        None,
        processor!(entry_callee),
    );

    // Program 2
    let program_caller = ProgramEntrypoint::new(
        pubkey!("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz"),
        None,
        processor!(entry_caller),
    );
    let config = Config::new();

    let mut client = TridentSVM::new_client(
        &[program_callee, program_caller],
        &config
    );

    fuzz_trident !
        (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence |
            { fuzz_iteration (fuzz_data , & config , & mut client) ; });
}
```

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
