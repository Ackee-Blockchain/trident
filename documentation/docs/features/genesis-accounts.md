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

!!! important

    To include the SBF target in the Fuzz Test for CPI

    - Specify program entry to `None`.
    - Store the SBF target in the `trident-genesis` folder in the root of the workspace.
    - **Name of the program has to be the same as name of the dumbed SBF target** in the trident-genesis (without .so).

```rust
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>
) {

    let fuzzing_program_cpi_metaplex_7 = FuzzingProgram::new(
        PROGRAM_NAME_CPI_METAPLEX_7,
        &PROGRAM_ID_CPI_METAPLEX_7,
        processor!(convert_entry!(entry_cpi_metaplex_7)),
    );

    let metaplex = FuzzingProgram::new(
        "metaplex-token-metadata",
        &mpl_token_metadata::ID,
        None
    );

    let mut client =
        ProgramTestClientBlocking::new(
            &[fuzzing_program_cpi_metaplex_7, metaplex],
            &[]
        ).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}
```




## Genesis Accounts

Trident allows you to include Accounts with data in base64 format.

!!! tip

    If you want to obtain Account with `base64` data format, use

    ```bash
    # -u m specifies to dump from mainnet
    solana account -u m <ADDRESS> --output json
    ```

```rust
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>
) {

    let fuzzing_program_cpi_metaplex_7 = FuzzingProgram::new(
        PROGRAM_NAME_CPI_METAPLEX_7,
        &PROGRAM_ID_CPI_METAPLEX_7,
        processor!(convert_entry!(entry_cpi_metaplex_7)),
    );

    let custom_account = FuzzingAccountBase64::new(
        ACCOUNT_ADDRESS,
        LAMPORTS,
        OWNER,
        "AQAAALL8+z4UH4TCY/N8hU0XmNZYMU9r04EadnC4rxQcc", // base64 data format
    );

    let mut client =
        ProgramTestClientBlocking::new(
            &[fuzzing_program_cpi_metaplex_7],
            &[custom_account] // Include here
        ).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}
```


!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
