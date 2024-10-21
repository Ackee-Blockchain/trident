# Genesis

## Genesis Programs


Trident allows you to use Cross Program Invocation of both Native and SBF programs.


### Native

In case of multiple programs within the Anchor Workspace. Make sure that all of the programs you would like to call Cross Program Invocation to are included in the initial state of the Fuzz Test Environment.

!!! important

    Source code below:

    - `fuzzing_program_callee` is included in the `ProgramTestClientBlocking`
    - `fuzzing_program_caller` is included in the `ProgramTestClientBlocking`

```rust
// test_fuzz.rs

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>
) {
    let fuzzing_program_callee = FuzzingProgram::new(
        PROGRAM_NAME_CALLEE,
        &PROGRAM_ID_CALLEE,
        processor!(convert_entry!(entry_callee)),
    );

    let fuzzing_program_caller = FuzzingProgram::new(
        PROGRAM_NAME_CALLER,
        &PROGRAM_ID_CALLER,
        processor!(convert_entry!(entry_caller)),
    );

    let mut client =
        ProgramTestClientBlocking::new(
            &[fuzzing_program_callee, fuzzing_program_caller],
            &[]
        ).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}
```

### SBF

In case of SBF targets, compiled or dumped from whatever cluster. You can also use these within the Fuzz Tests.

!!! tip

    If you want to obtain Program from Mainnet use

    ```bash
    # -u m specifies to dump from mainnet
    solana program dump -u m <PROGRAM_ID> <PROGRAM_NAME>.so
    ```

!!! important

    - To include the SBF target in the Fuzz Test for CPI, specify address and path to the program in the `Trident.toml`.

    - Including these types of programs will lead to performance decline of Trident.

```toml
[[fuzz.programs]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
program = "metaplex-program/metaplex-token-metadata.so"
```



## Genesis Accounts

Trident allows you to include Accounts with data in base64 format.

!!! tip

    If you want to obtain Account with `base64` data format, use

    ```bash
    # -u m specifies to dump from mainnet
    solana account -u m <ADDRESS> --output json
    ```

!!! important

    To include desired accounts in the fuzz testing environment, add then using the `Trident.toml`.

```toml
[[fuzz.accounts]]
address = "6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE"
filename = "tests/accounts/core_bridge_mainnet/guardian_set_5_mock.json"
```


!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
