# Initialize Fuzz Test

For initialization of workspace for fuzz tests, call:
```bash
trident init fuzz
```

The command will generate the following folder structure:
```bash
project-root
├── trident-tests
│   ├── fuzz_tests # fuzz tests folder
│   │   ├── fuzz_0 # particular fuzz test
│   │   │   ├── accounts_snapshots.rs # generated accounts deserialization methods
│   │   │   ├── test_fuzz.rs # the binary target of your fuzz test
│   │   │   └── fuzz_instructions.rs # the definition of your fuzz test
│   │   ├── fuzz_1
│   │   ├── fuzz_X # possible multiple fuzz tests
│   │   ├── fuzzing # compilations and crashes folder
│   │   └── Cargo.toml
├── Trident.toml
└── ...
```

## Add new Fuzz Test

If you wish to generate template for a new fuzz test, call:
```bash
trident fuzz add
```
