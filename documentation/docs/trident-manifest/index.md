# Trident Manifest

Trident supports its own Trident Manifest, called `Trident.toml`. This section describes all possible configuration fields.

## Fuzzing Metrics

```bash
[fuzz.metrics]
enabled = true
dashboard = false
json = false
```


- `enabled` - Trident will show and save statistics after the fuzzing session.
- `dashboard` - Trident will save a HTML dashboard after the fuzzing session.
- `json` - Trident will save a JSON file after the fuzzing session.

---

## Regression testing

```bash
[fuzz.regression]
enabled = true
```

- `enabled` - Trident will save a JSON file with all monitored states after the fuzzing session.

!!! note "Regression testing"

    - Keep in mind that the feature hashes contents of accounts, if the contents are dependant for example on time, the final hash might not be the same as the one from the first fuzzing session.

---


## Fuzzing Coverage

```bash
[fuzz.coverage]
enabled = false
server_port = 58432
loopcount = 0
format = "json"
attach_extension = false
```

The coverage configuration enables code coverage analysis during fuzzing sessions, providing insights into which parts of your program are being tested.

- `enabled` - When set to `true`, Trident collects code coverage data throughout the fuzzing session and generates a comprehensive report upon completion.
- `server_port` - Specifies the HTTP server port used for communication between Trident and the [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana).
- `loopcount` - Defines the number of execution flows processed before generating an intermediate coverage data file. Lower values provide more frequent coverage updates at the cost of performance. When set to `0`, coverage files are only generated at the end of the fuzzing session.
- `format` - Determines the output format for coverage reports. Trident supports both `json` and `html` formats.
- `attach_extension` - Enables real-time coverage visualization in VS Code when the extension is active. This feature requires `loopcount` to be greater than `0` and `format` to be set to `json`. 

!!! warning

    Coverage is only gathered for the programs deployed through an entrypoint!

<a id="entrypoint-deployment"></a>
??? note "Entrypoint deployment guide"
  
    1. Comment out the program in `Trident.toml`
     ```toml
      # [[fuzz.programs]]
      # address = "example_address"
      # program = "../target/deploy/example_program.so"
     ``` 
    2. Add the program dependency to `Cargo.toml`
     ```toml
      [dependencies.example_program]
      path = "../programs/example_program"
     ```
    3. Add `syscall-v1` or `syscall-v2` feature depending on the version of `solana-program`. Programs written for `solana-program` v1.x should use `syscall-v1`, programs written for `solana-program` v2.x should use `syscall-v2` feature.
     ```toml
      [dependencies.trident-fuzz]
      features = ["syscall-v2"]
     ```
    4. Import the programs entrypoint in `test_fuzz.rs`
     ```rust
      use example_program::entry as example_entrypoint;
     ``` 
    5. Deploy the entrypoint in `test_fuzz.rs`
     ```rust
      impl FuzzTest {
        fn new() -> Self {
            let mut trident = Trident::default();

            // Deploy through the entrypoint
            let program = TridentEntrypoint::new(example_program::ID, None, processor!(example_entrypoint));
            trident.get_client().deploy_entrypoint(program);

            Self {
                trident,
                fuzz_accounts: FuzzAccounts::default(),
            }
        }
        ...
     ``` 

---

## Programs

- Use dumped program from desired cluster, during fuzzing.

```bash
[[fuzz.programs]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
upgrade_authority = "account address goes here"
program = "metaplex-program/metaplex-token-metadata.so"
```

---

## Accounts

- Use dumped accounts from desired cluster, during fuzzing.

```bash
[[fuzz.accounts]]
address = "6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE"
filename = "tests/accounts/core_bridge_mainnet/guardian_set_5_mock.json"
```

---



