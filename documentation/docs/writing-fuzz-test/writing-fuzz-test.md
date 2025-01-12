---
hide:
  - navigation
---

## Initialize Trident

Initialize Trident in the Anchor-based workspace.

```bash
trident init
```

!!! info

    Trident under the hood

    - Builds the Anchor-based project.
    - Reads the generated IDL.
    - Based on the IDL creates the fuzzing template.

!!! tip

    If you have Trident already initialized, you can add a new fuzz test using `trident fuzz add`.


## Fill the Fuzz test Template

### Define Fuzz Accounts

Define `AccountsStorage` type for each Account you would like to use.

!!! important

    Keep in mind:

    - You do not need to specify every `AccountStorage`; some accounts do not necessarily need to be stored in their corresponding storage.
        - For example, `System Program` does not need to be stored and can be used from the `solana_sdk`.
    - You can rename `FuzzAccounts` fields to whatever you want. The default names were generated based on the Program's `IDL`.

```rust
#[doc = r" Use AccountsStorage<T> where T can be one of:"]
#[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
#[derive(Default)]
pub struct FuzzAccounts {
    author: AccountsStorage<KeypairStore>,
    hello_world_account: AccountsStorage<PdaStore>,
    // No need to fuzz system_program
    // system_program: AccountsStorage<todo!()>,
}
```

!!! tip

    For more details about the `AccountsStorage`, check [AccountsStorage](../features/account-storages.md).

### Implement Fuzz Instructions

Each Instruction in the Fuzz Test must define the following functions:

- `get_discriminator`
    - Defines Instruction Discriminator, which is used to identify the Instruction.
- `get_program_id()`
    - Specifies to which program the Instruction belongs. This function is **automatically defined** and should not need any updates. The importance is such that if you have multiple programs in your workspace, Trident can generate Instruction Sequences corresponding to different programs.
- `get_data()`
    - Specifies what Instruction inputs are sent to the Program Instructions.
- `get_accounts()`
    - Specifies what Accounts are sent to the Program Instructions.

!!! tip

    - For more info about how to write these functions, check the [Fuzz Instructions](../features/fuzz-instructions.md).
    - For examples of how to write these functions, check the [Examples](../examples/examples.md).


## Execute

### Run Fuzz Test

In principle there are two possible fuzzing engines that the Trident supports, [Honggfuzz](https://github.com/google/honggfuzz) and [AFL](https://aflplus.plus/).

To execute the desired fuzz test using the Honggfuzz, run the following command from the `trident-tests` directory:

```bash
# Replace <TARGET_NAME> with the name of particular
# fuzz test (for example: "fuzz_0")
trident fuzz run-hfuzz <TARGET_NAME>
```

To execute the desired fuzz test using the AFL, run the following command from the `trident-tests` directory:

```bash
# Replace <TARGET_NAME> with the name of particular
# fuzz test (for example: "fuzz_0")
trident fuzz run-afl <TARGET_NAME>
```



### Debug Fuzz Test

To debug your program using Honggfuzz with values from a crash file, run the following command from the `trident-tests` directory:

```bash
# The fuzzer will run the <TARGET_NAME> with the specified <CRASH_FILE_PATH>
trident fuzz debug-hfuzz <TARGET_NAME> <CRASH_FILE_PATH>
```

To debug your program using AFL with values from a crash file, run the following command from the `trident-tests` directory:

```bash
# The fuzzer will run the <TARGET_NAME> with the specified <CRASH_FILE_PATH>
trident fuzz debug-afl <TARGET_NAME> <CRASH_FILE_PATH>
```

!!! tip

    By default, the crash files are stored in:

    - `trident-tests/fuzzing/honggfuzz/hfuzz_workspace/<FUZZ_TARGET>` for Honggfuzz and
    - `trident-tests/fuzzing/afl/afl_workspace/out/default/crashes` for AFL.

!!! tip

    For more info about the fuzzing outputs, check the [Commands](../commands/commands.md).
