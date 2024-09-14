---
hide:
  - navigation
---


### trident init

This command Initializes Trident Workspace and generates new Fuzz Test Template.

The command will generate the following folder structure:
```bash
project-root
├── trident-tests
│   ├── fuzz_tests # fuzz tests folder
│   │   ├── fuzz_0 # particular fuzz test
│   │   │   ├── test_fuzz.rs # the binary target of your fuzz test
│   │   │   └── fuzz_instructions.rs # the definition of your fuzz test
│   │   ├── fuzz_1
│   │   ├── fuzz_X # possible multiple fuzz tests
│   │   ├── fuzzing # compilations and crashes folder
│   │   └── Cargo.toml
├── Trident.toml
└── ...
```

### trident fuzz

This Comand behavior depends on the subcommand.

#### trident fuzz run <FUZZ_TARGET>

Run Fuzzer on the specified Fuzz Target (i.e. the Fuzz Template, for example fuzz_0).

#### trident fuzz run-debug <FUZZ_TARGET> <PATH_TO_CRASHFILE>

Run debug on the specified Fuzz Target (i.e. the Fuzz Template, for example fuzz_0), with specified crash file, to see where the crash file found an issue.

#### trident add

Adds new Fuzz Test Template.

### trident clean

Calls `anchor clean` and cleans targets created by the underlying Honggfuzz. Crashfiles and Fuzzing Inputs are preserved.
