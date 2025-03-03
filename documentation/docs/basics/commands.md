# Commands

Trident CLI supports the following commands:

## `trident init`

Initializes Trident Workspace and generates new Fuzz Test Template. Creates the following structure:

```bash
project-root
├── trident-tests
│   ├── fuzz_0 # particular fuzz test
│   │   ├── instructions # instructions folder
│   │   ├── transactions # transactions folder
│   │   ├── test_fuzz.rs # the binary target of your fuzz test
│   │   ├── types.rs # the types of your fuzz test
│   │   └── fuzz_instructions.rs # the definition of your fuzz test
│   ├── fuzz_1
│   ├── fuzz_X # possible multiple fuzz tests
│   ├── fuzzing # compilations and crashes folder
│   └── Cargo.toml
├── Trident.toml
└── ...
```

#### Options

- `-f, --force` - Force Trident initialization. Updates Trident dependencies based on the CLI version.
- `-p, --program-name <NAME>` - Specify the program name for which the fuzz test will be generated.
- `-t, --test-name <NAME>` - Specify a custom name for the fuzz test to initialize.

---

## `trident how`

Print How To message about writing fuzz tests.

---

## `trident fuzz`

Fuzzing-related commands with various subcommands:


### `trident fuzz run-afl <fuzz_target>`

!!! warning "Directory Note"
    Execute fuzz tests from the `trident-tests` directory.

!!! warning "Solana Logs"
    In case you want to see the logs of the fuzzed transactions, prefix the command with `TRIDENT_LOG=1`.
    ```bash
    TRIDENT_LOG=1 trident fuzz run-afl <fuzz_target>
    ```

Runs AFL on the specified Fuzz Target (e.g., fuzz_0).

---

### `trident fuzz run-hfuzz <fuzz_target>`

!!! warning "Directory Note"
    Execute fuzz tests from the `trident-tests` directory.

!!! warning "Solana Logs"
    In case you want to see the logs of the fuzzed transactions, prefix the command with `TRIDENT_LOG=1`.
    ```bash
    TRIDENT_LOG=1 trident fuzz run-hfuzz <fuzz_target>
    ```

Runs Honggfuzz on the specified Fuzz Target (e.g., fuzz_0).


The output includes:

1. **Iterations**: Number of fuzzing iterations completed
2. **Mode**: Feedback Driven Mode - generates data based on coverage progress
3. **Speed**: Average iterations per second
4. **Crashes**: Number of detected crashes (panics or failed invariant checks)

```bash
------------------------[  0 days 00 hrs 00 mins 01 secs ]----------------------
  Iterations : 688 (out of: 1000 [68%])
  Mode [3/3] : Feedback Driven Mode
      Target : .../release/fuzz_0
     Threads : 16, CPUs: 32, CPU%: 1262% [39%/CPU]
       Speed : 680/sec [avg: 688]
     Crashes : 1 [unique: 1, blocklist: 0, verified: 0]
    Timeouts : 0 [10 sec]
 Corpus Size : 98, max: 1048576 bytes, init: 0 files
    Coverage : edge: 10345/882951 [1%] pc: 163 cmp: 622547
```

---

### `trident fuzz debug-afl <fuzz_target> <crash_file_path>`

Debug AFL crashes by analyzing specific crash files.

---

### `trident fuzz debug-hfuzz <fuzz_target> <crash_file_path>`

Debug Honggfuzz crashes by analyzing specific crash files.

!!! note "Debug Output"
    The debug output currently includes verbose lldb information. It shows:

    1. Transaction logs
    2. Instruction data structures
    3. Panic/Crash information

---

### `trident fuzz add`

!!! warning "Directory Note"
    This commands needs to be executed from the project root directory.

Adds a new Fuzz Test Template.

#### Options

- `-p, --program-name <NAME>` - Specify the program name for which the fuzz test will be generated.
- `-t, --test-name <NAME>` - Specify a custom name for the fuzz test to add.

---

## `trident clean`

Executes `anchor clean` and cleans Honggfuzz targets while preserving crashfiles and fuzzing inputs.
