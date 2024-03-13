
# Add new Fuzz Test
If you have already initialized {{ config.site_name }} within your project, and you are interested in initializing a new fuzz test run.
```bash
trident fuzz add
```

The command will generate a new fuzz test as follows:
```bash
project-root
├── trident-tests
│   ├── fuzz_tests # fuzz tests folder
│   │   ├── fuzz_X # new fuzz test folder
│   │   │   ├── accounts_snapshots.rs
│   │   │   ├── test_fuzz.rs
│   │   │   └── fuzz_instructions.rs
│   │   ├── fuzzing
│   │   └── Cargo.toml # already present
├── Trident.toml # already present
└── ...
```
