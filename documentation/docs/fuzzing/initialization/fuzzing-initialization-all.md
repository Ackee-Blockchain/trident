
# All-Suite

To initialize {{ config.site_name }} and generate all-suite test templates, navigate to your project's root directory and run

```bash
trident init both
```

The command will generate the following folder structure:
```bash
project-root
├── .program_client
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
│   └── poc_tests # integration tests folder
├── Trident.toml
└── ...
```
