# Fuzzing
Fuzzing is a technique for testing software that involves providing invalid, unexpected, or random data as inputs to a computer program.

## Introduction
Trdelnik testing framework provides a set of tools to help you speed up the development of your fuzz tests in a few steps:
- It automatically parses your Anchor-based programs and generates required implementations to deserialize instruction accounts.
- It generates templates that you will complete according to your desired fuzz test behavior.
- It provides several derive macros to implement required traits automatically.
- It provides a bank client and helper functions to handle accounts.
- It provides a CLI to run and debug the fuzz tests.

Trdelnik is designed to be customizable allowing you to fuzz:
- the execution order of instructions,
- the instruction parameters,
- the instruction accounts,
- any combination of the above.

## Fuzz test initialization
To initialize Trdelnik and generate fuzz test templates, navigate to your project's root directory and run:

```shell
trdelnik init
```

The command will generate the required folder structure and fuzz test files:
```shell
project-root
├── .trdelnik_client
├── trdelnik-tests
│   ├── src # fuzz tests folder
│   │   ├── bin
│   │   │   └── fuzz_target.rs # the binary target of your fuzz test
│   │   ├── fuzz_instructions.rs # the definition of your fuzz test
│   │   ├── accounts_snapshots.rs # generated accounts deserialization methods
│   │   └── lib.rs
│   ├── tests # integration tests folder
│   └── Cargo.toml
├── Trdelnik.toml
└── ...
```

## Running and debugging the fuzz test
Once you have finished the implementation of `get_accounts` and `get_data` methods (see below), you can run the fuzz test as follows:

```shell
# Replace <TARGET_NAME> with the name of your fuzz target (by default "fuzz_target")
trdelnik fuzz run <TARGET_NAME>
```

To debug your fuzz target crash with parameters from a crash file:
```shell
trdelnik fuzz run-debug <TARGET_NAME> <CRASH_FILE_PATH>
```

 Under the hood Trdelnik uses [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs). You can pass [supported parameters](https://github.com/Ackee-Blockchain/trdelnik/blob/develop/crates/client/src/config.rs#L57) via the Trdelnik.toml configuration file. For example:
 ```toml
 # Content of Trdelnik.toml
[fuzz]
timeout = 10 # Timeout in seconds
iterations = 1000 # Number of fuzzing iterations
exit_upon_crash = true # Stop the fuzzer upon crash.
```

Or you can pass any parameter via [environment variables](https://github.com/rust-fuzz/honggfuzz-rs#environment-variables). A list of hongfuzz parameters can be found in honggfuzz [usage documentation](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help). The parameters passed via environment variables have higher priority. For example:
 ```shell
# Time-out: 10 secs
# Number of concurrent fuzzing threads: 1
# Number of fuzzing iterations: 10000
# Display Solana logs in the terminal
HFUZZ_RUN_ARGS="-t 10 -n 1 -N 10000 -Q" trdelnik fuzz run <TARGET_NAME>
```

## Fuzz test lifecycle
In the sequence diagram below you can see a simplified fuzz test lifecycle.

Some diagram states are labeled with emojis:
- ⚡ Mandatory methods that must be implemented by the user.
- 👤 Optional methods that can be implemented by the user.


1. The fuzzer is running until:
    1. The maximal number of iterations is reached (if specified).
    2. A crash was detected and the `exit_upon_crash` parameter was set.
    3. User interrupted the test manually (for example by hitting `CTRL+C`).
2. In each iteration, the fuzzer generates a sequence of random instructions to execute.
    - User can optionally customize how the instructions are generated and can specify the instructions that should be executed at the beginning (`pre_ixs`), in the middle (`ixs`) and at the end (`post_ixs`) of each iteration. This can be useful for example if your program needs an initialization or you want to fuzz some specific program state.
3. For each instruction:
    1. User defined mandatory method `get_accounts()` is called to collect necessary instruction accounts.
    2. User defined mandatory method `get_data()` is called to collect instruction data.
    3. A snapshot of all instruction accounts before the instruction execution is saved.
    4. The instruction is executed.
    5. A snapshot of all instruction accounts after the instruction execution is saved.
    6. User defined optional method `check()` is called to check accounts data and evaluate invariants.

![Fuzzing lifecycle](fuzzing_lifecycle.svg)

## Write a fuzz test
At the current development stage, there are some manual steps required to make your fuzz test compile:
1. Add dependencies specific to your program to `trdelnik-tests/Cargo.toml` (such as anchor-spl etc.).
2. Add necessary `use` statements into `trdelnik-tests/src/accounts_snapshots.rs` to import missing types.

### Specify accounts to reuse
Trdelnik fuzzer helps you to generate only a limited amount of pseudo-random accounts and reuse them in the instructions. Always generating only random accounts would in most cases lead to a situation where the fuzzer would be stuck because the accounts would be almost every time rejected by your Anchor program. Therefore it is necessary to specify, what accounts should be used and also limit the number of newly created accounts to reduce the space complexity.

Go to the `trdelnik-tests/src/fuzz_instructions.rs` file and complete the pre-generated `FuzzAccounts` structure. It contains all accounts used in your program. You have to determine, if the account is a signer, a PDA, a token account or program account. Than use the corresponding `AccountsStorage` types such as:
```rust
pub struct FuzzAccounts {
    signer: AccountsStorage<Keypair>,
    some_pda: AccountsStorage<PdaStore>,
    token_vault: AccountsStorage<TokenStore>,
    mint: AccountsStorage<MintStore>,
}
```

### Specify instruction data
Trdelnik fuzzer generates random instruction data for you. Currently it is however required, that you manually assign the random fuzzer data to the instruction data. It is done using the `IxOps` trait and its method `get_data`. Go to the `trdelnik-tests/src/fuzz_instructions.rs` file and complete the pre-generated `get_data` methods for each instruction such as:
```rust
fn get_data(
    &self,
    _client: &mut impl FuzzClient,
    _fuzz_accounts: &mut FuzzAccounts,
) -> Result<Self::IxData, FuzzingError> {
    let data = fuzz_example1::instruction::Invest {
        amount: self.data.amount,
    };
    Ok(data)
}
```

### Specify instruction accounts
Trdelnik fuzzer generates random indexes of accounts to use in each instruction. Each created account is saved in the `FuzzAccounts` structure which helps you to reuse already existing accounts. You are required to define, how these accounts should be created and which accounts should be passed to an instruction. It is done using the `IxOps` trait and its method `get_accounts`. Go to the `trdelnik-tests/src/fuzz_instructions.rs` file and complete the pre-generated `get_accounts` methods for each instruction such as:
```rust
fn get_accounts(
    &self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut FuzzAccounts,
) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    let author = fuzz_accounts.author.get_or_create_account(
        self.accounts.author,
        client,
        5000000000000,
    );
    let signers = vec![author.clone()];
    let state = fuzz_accounts
        .state
        .get_or_create_account(
            self.accounts.state,
            &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &fuzz_example1::ID,
        )
        .ok_or(FuzzingError::CannotGetAccounts)?
        .pubkey();
    let acc_meta = fuzz_example1::accounts::EndRegistration {
        author: author.pubkey(),
        state,
    }
    .to_account_metas(None);
    Ok((signers, acc_meta))
}
```
Notice especially the helper method `fuzz_accounts.<account_name>.get_or_create_account` that is used to create or retrieve a Keypair or public key of an account.

### Define invariants checks
After each successful instruction execution, the `check()` method is called to check the account data invariants. For each instruction, you can compare the account data before and after the instruction execution such as:
```rust
fn check(
    &self,
    pre_ix: Self::IxSnapshot,
    post_ix: Self::IxSnapshot,
    _ix_data: Self::IxData,
) -> Result<(), &'static str> {
    if let Some(escrow_pre) = pre_ix.escrow {
        // we can unwrap the receiver account because it has to be initialized before the instruction
        // execution and it is not supposed to be closed after the instruction execution either
        let receiver = pre_ix.receiver.unwrap();
        let receiver_lamports_before = receiver.lamports();
        let receiver_lamports_after = post_ix.receiver.unwrap().lamports();

        if receiver.key() != escrow_pre.receiver.key()
            && receiver_lamports_before < receiver_lamports_after
        {
            return Err("Un-authorized withdrawal");
        }
    }

    Ok(())
}
```

### Customize instructions generation
It is possible to customize how the instructions are generated and which instructions will be executed at the beginning (`pre_ixs`), in the middle (`ixs`) and at the end (`post_ixs`) of each fuzz iteration. This can be useful for example if your program needs an initialization or you want to fuzz some specific program state. Go to the bin target file of your fuzz test and implement the corresponding optional method of the `FuzzDataBuilder<FuzzInstruction>` trait. For example, in order to always call the `initialize` instruction for the default fuzz target, modify the trait's implementation in `trdelnik-tests/src/bin/fuzz_target.rs` file as follows:
```rust
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init_ix])
    }
}
```

## Fuzz test examples
- [Fuzz test example 1](examples/fuzz_example1)
- [Fuzz test example 2](examples/fuzz_example2)
