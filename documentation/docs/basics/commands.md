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
│   ├── Cargo.toml
│   └── Trident.toml # Configuration file located in trident-tests directory
└── ...
```

#### Options

- `-f, --force` - Force Trident initialization. Trident dependencies will be updated based on the version of Trident CLI.
- `-s, --skip-build` - Skip building the program before initializing Trident.
- `-p, --program-name <FILE>` - Specify the name of the program for which fuzz test will be generated.
- `-t, --test-name <NAME>` - Name of the fuzz test to initialize.

---

## `trident how`

Show the HowTo message about writing fuzz tests.

---

## `trident fuzz`

Run fuzz subcommands. With fuzz subcommands you can add new fuzz test template or you can run fuzz test on already initialized one.

**Examples:**
```bash
trident fuzz add
trident fuzz run fuzz_0
trident fuzz debug <FUZZ_TARGET> <SEED>
```

### `trident fuzz add`

Generate new Fuzz Test template.

#### Options

- `-p, --program-name <FILE>` - Specify the name of the program for which the fuzz test will be generated.
- `-t, --test-name <NAME>` - Name of the fuzz test to add.
- `-s, --skip-build` - Skip building the program before adding new fuzz test.

---

### `trident fuzz run <fuzz_target>`

!!! warning "Directory Note"
    Execute fuzz tests from the `trident-tests` directory.

!!! warning "Solana Logs"
    In case you want to see the logs of the fuzzed transactions, prefix the command with `TRIDENT_LOG=1`.
    ```bash
    TRIDENT_LOG=1 trident fuzz run <fuzz_target>
    ```

Runs the specified Fuzz Target using Trident's Manually Guided Fuzzing (e.g., fuzz_0).

#### Arguments

- `<fuzz_target>` - Name of the desired fuzz template to execute (for example fuzz_0).
- `[seed]` - Master seed used for fuzzing, if not provided it will be generated randomly.

#### Options

- `-w, --with-exit-code` - Run the fuzzing with exit code, i.e. if it discovers crash the Trident will exit with exit code 1.

---

### `trident fuzz debug <fuzz_target> <seed>`

Debug crashes by analyzing specific crash files using the provided seed.

#### Arguments

- `<fuzz_target>` - Name of the desired fuzz template to execute (for example fuzz_0).
- `<seed>` - Master seed of the desired fuzz template to execute.

!!! note "Debug Output"
    The debug output includes:

    1. Transaction logs
    2. Instruction data structures
    3. Panic/Crash information

---

## `trident clean`

Clean build target, additionally perform `anchor clean`.


---

## `trident server`

Starts the Trident Server, in order to make it easier to visualize the fuzzing dashboard, more info in the [dashboard](../trident-advanced/dashboard/index.md) section.

---


## `trident compare`

Allows you to compare two fuzzing regression results. More info in the [regression](../trident-advanced/regression/index.md) section.

---