# Commands

Trident CLI supports the following commands:

## `trident init`

Initializes Trident Workspace and generates new Fuzz Test Template. Works with both **Anchor** and **vanilla Solana** programs.

Creates the following structure:

```bash
project-root
├── trident-tests
│   ├── fuzz_0 # particular fuzz test
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
- `-p, --program-name <FILE>` - Specify the name of the program for which fuzz test will be generated (Anchor projects only).
- `-t, --test-name <NAME>` - Name of the fuzz test to initialize.
- `--idl-path <FILE>...` - Path(s) to IDL file(s). Multiple files can be specified separated by spaces. Required for vanilla Solana programs.

#### Anchor Projects

```bash
# Default - auto-detects Anchor.toml and uses target/idl/
trident init

# With custom IDL paths
trident init --idl-path ./custom-idls/my_program.json

# Specific program only
trident init --program-name my_program
```

#### Vanilla Solana Projects

For programs not written with Anchor, you must provide IDL file(s):

```bash
# Single program
trident init --skip-build --idl-path ./idl/my_program.json

# Multiple programs
trident init --idl-path ./idl/program1.json ./idl/program2.json
```

!!! note "IDL Generation for Vanilla Solana"
    Vanilla Solana programs require IDL files to be generated using external tools or written manually following the Anchor IDL format. The build command uses `cargo build-sbf` instead of `anchor build`.

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
trident fuzz refresh fuzz_0
```

### `trident fuzz add`

Generate new Fuzz Test template. Can be executed from the project root or from within the `trident-tests` directory.

#### Options

- `-p, --program-name <FILE>` - Specify the name of the program for which the fuzz test will be generated (Anchor projects only).
- `-t, --test-name <NAME>` - Name of the fuzz test to add.
- `-s, --skip-build` - Skip building the program before adding new fuzz test.
- `--idl-path <FILE>...` - Path(s) to IDL file(s). Multiple files can be specified separated by spaces. Required for vanilla Solana programs.


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

- `-w, --with-exit-code` - Run the fuzzing with exit code, i.e. if it discovers invariant failures or panics the Trident will exit with exit code.

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

### `trident fuzz refresh <fuzz_target>`

Refresh an existing fuzz test by regenerating the types file based on the current program state. Can be executed from the project root or from within the `trident-tests` directory.

#### Arguments

- `<fuzz_target>` - Name of the fuzz test to refresh (for example fuzz_0).

#### Options

- `-p, --program-name <FILE>` - Specify the name of the program for which the fuzz test will be refreshed (Anchor projects only).
- `-s, --skip-build` - Skip building the program before refreshing the types file.
- `--idl-path <FILE>...` - Path(s) to IDL file(s). Multiple files can be specified separated by spaces. Required for vanilla Solana programs.


---

## `trident clean`

Clean build target, additionally perform `anchor clean`.


---

## `trident server`

Start HTTP server to serve fuzzing dashboards. More info in the [dashboard](../trident-advanced/dashboard/index.md) section.

#### Options

- `-d, --directory <DIR>` - Directory to monitor for dashboard files (default: `.fuzz-artifacts`).
- `-p, --port <PORT>` - Port to run the server on (default: `8000`).
- `--host <HOST>` - Host to bind the server to (default: `localhost`).

---


## `trident compare`

Compare two regression JSON files and identify differing iteration seeds. More info in the [regression](../trident-advanced/regression/index.md) section.

#### Arguments

- `<FILE1>` - Path to the first regression JSON file.
- `<FILE2>` - Path to the second regression JSON file.

---