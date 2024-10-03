# Fuzzing Statistics

Trident allows you to see statistics after the fuzzing session ended.

!!! important

    In order to show statistics set `fuzzing_with_stats` within the `Trident.toml` to `true`.

    ```toml
    [fuzz]
    # ...
    fuzzing_with_stats = true
    # ...
    ```

!!! warning

    Statistics about CU usage are dependent on the underlaying file format you are fuzzing.
    To speed up the fuzzing, Trident uses the native binary, which consumes significantly less CU.
    Hovewer, the programs you develop are always deployed in eBPF format to Solana network.
    So if you want to see relavant CU statistics and do not mind slowdown of fuzzing, please follow these steps:

    1. First compile all programs you want to fuzz to eBPF format with `anchor build` (or similar command).
    2. The compiled programs are stored inside `target/sbf-solana-solana/release` folder. Copy all `*.so` files with the names of the programs you want to fuzz to folder `trident-genesis` in the root of your current workspace (if it does not exists, please create it).
    3. In `test_fuzz.rs` file of your fuzz test, for which you want to have correct CU statistics displayed, remove entrypoint from all the programs you are fuzzing.

        For example, if you were to fuzz [unauthorized-access-2 example](https://github.com/Ackee-Blockchain/trident/tree/master/examples/fuzz-tests/unauthorized-access-2), you would need to change these lines inside `test_fuzz.rs` file:
        ```rust
        let fuzzing_program1 = FuzzingProgram::new(
            PROGRAM_NAME_UNAUTHORIZED_ACCESS_2,
            &PROGRAM_ID_UNAUTHORIZED_ACCESS_2,
            processor!(convert_entry!(entry_unauthorized_access_2))
        );
        ```
        to:
        ```rust
        let fuzzing_program1 = FuzzingProgram::new(
            PROGRAM_NAME_UNAUTHORIZED_ACCESS_2,
            &PROGRAM_ID_UNAUTHORIZED_ACCESS_2,
            None
        );
        ```


## Available Statistics

- `Invoked Total` - number of invocations of each instruction during the fuzzing session.
- `Ix Success` - number of successful invocations of each instruction during the fuzzing session.
- `Ix Failed` - number of failed invocations of each instruction during the fuzzing session (e.g. instruction failed because it did not passed checks inside the program).
- `Check Failed` - number of failed invariants checks for each instruction during the fuzzing session.
- `CU Used Max` - maximum amount of CU ([Compute Units](https://solana.com/docs/core/fees#compute-units)) used by the successful instruction invocation
- `CU Used Min` - minimum amount of CU used by the successful instruction invocation
- `CU Used Max for Ix Failed` - maximum amount of CU used by the failed instruction invocation
- `CU Used Min for Ix Failed` - minimum amount of CU used by the failed instruction invocation

??? note

    Keep in mind that the number of fuzz iterations does not directly correspond to the total number of invocations. In one fuzz iteration, the fuzzer might be unable to deserialize fuzz data into instructions, causing the entire iteration to be skipped.

    On the other hand this is expected behavior as the underlying data are randomly (with coverage feedback) generated, so the Honggfuzz will not necessarily find appropriate data each iteration.


!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
